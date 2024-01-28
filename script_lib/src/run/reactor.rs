use std::collections::BTreeMap;
use std::task::Waker;

use crate::simple_channel::{self, Receiver, Sender};
use crate::{F64Total, RETRY_WAIT};

type WakersByTime = BTreeMap<F64Total, Vec<Waker>>;
type WakersInOrder = Vec<Waker>;
pub type WakerWithTime = (WakeDelay, Waker);

#[derive(Debug, Clone, Copy)]
pub enum WakeDelay {
    Immediate,
    Retry,
    AfterNextRamRelease,
    WakeAt(f64),
}

pub struct BitburnerReactor {
    reactor_tx: Sender<WakerWithTime>,
    reactor_rx: Receiver<WakerWithTime>,
    wakers_running: WakersByTime,
    wakers_ram: WakersInOrder,
    instant_fn: fn() -> f64,
}

impl BitburnerReactor {
    pub fn new(instant_fn: fn() -> f64) -> Self {
        let (reactor_tx, reactor_rx) = simple_channel::channel::<WakerWithTime>();
        BitburnerReactor {
            reactor_tx,
            reactor_rx,
            wakers_running: WakersByTime::new(),
            wakers_ram: WakersInOrder::new(),
            instant_fn,
        }
    }

    pub fn get_schedule_queue(&self) -> Sender<WakerWithTime> {
        self.reactor_tx.clone()
    }

    pub fn now(&self) -> f64 {
        (self.instant_fn)()
    }

    pub fn next_wake(&self) -> Option<f64> {
        if let Some((&time, _)) = self.wakers_running.first_key_value() {
            Some(time.into())
        } else {
            None
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.drain_queue();
        self.wakers_ram.is_empty() && self.wakers_running.is_empty()
    }

    fn drain_queue(&mut self) {
        let now = self.now();
        let mut schedule = |delay: WakeDelay, waker: Waker| {
            Self::schedule_waker(
                &mut self.wakers_running,
                &mut self.wakers_ram,
                now,
                delay,
                waker,
            )
        };
        for (delay, waker) in self.reactor_rx.try_iter() {
            schedule(delay, waker);
        }
    }

    fn schedule_waker(
        wakers_running: &mut WakersByTime,
        wakers_ram: &mut WakersInOrder,
        now: f64,
        delay: WakeDelay,
        waker: Waker,
    ) {
        let wake_at: Option<f64> = match delay {
            WakeDelay::Immediate => Some(now),
            WakeDelay::Retry => Some(now + RETRY_WAIT),
            WakeDelay::WakeAt(time) => Some(time),
            WakeDelay::AfterNextRamRelease => None,
        };
        if let Some(time) = wake_at {
            let key = F64Total::from(time);
            if let Some(current) = wakers_running.get_mut(&key) {
                current.push(waker);
            } else {
                let new = vec![waker];
                let present = wakers_running.insert(key, new);
                debug_assert!(present.is_none());
            }
        } else {
            wakers_ram.push(waker);
        }
    }

    pub fn wake_on_ram_release(&mut self) -> usize {
        self.drain_queue();
        let mut woken = 0;
        for waker in self.wakers_ram.drain(..) {
            waker.wake();
            woken += 1;
        }
        woken
    }

    pub fn wake_running(&mut self) -> usize {
        self.drain_queue();
        let now = self.now();
        let still_waiting = self.wakers_running.split_off(&now.into());
        let mut woken = 0;
        for wakers in self.wakers_running.values() {
            for waker in wakers.iter() {
                waker.wake_by_ref();
                woken += 1;
            }
        }
        self.wakers_running = still_waiting;
        woken
    }
}

#[cfg(test)]
mod tests {
    use std::task::Waker;

    use super::{BitburnerReactor, WakeDelay, WakersByTime, WakersInOrder};
    use crate::run::waker::{get_task_with_waker, RcTask};
    use crate::{simple_channel, F64Total, RETRY_WAIT};

    #[test]
    fn test_new() {
        let mut reactor = BitburnerReactor::new(|| 0.0);
        assert!(reactor.is_empty());
        assert!(reactor.next_wake().is_none())
    }

    #[test]
    fn test_drain_queue() {
        let mut reactor = BitburnerReactor::new(|| 0.0);
        let (woken_tx, _) = simple_channel::channel::<RcTask>();
        let (_, waker_1) = get_task_with_waker(std::future::ready(Ok(())), woken_tx.clone());

        reactor.drain_queue();

        reactor
            .reactor_tx
            .send((WakeDelay::Immediate, waker_1))
            .unwrap();
        reactor.drain_queue();
        // Scheduled in the running process queue
        assert!(reactor.wakers_running.first_entry().is_some());
        // Queue now empty
        assert!(reactor.reactor_rx.try_recv().is_err());

        let (_, waker_2) = get_task_with_waker(std::future::ready(Ok(())), woken_tx.clone());
        reactor
            .reactor_tx
            .send((WakeDelay::WakeAt(4.0), waker_2))
            .unwrap();
        reactor.drain_queue();
        assert_eq!(reactor.wakers_running.len(), 2);

        let (_, waker_3) = get_task_with_waker(std::future::ready(Ok(())), woken_tx.clone());
        let (_, waker_4) = get_task_with_waker(std::future::ready(Ok(())), woken_tx.clone());
        let (_, waker_5) = get_task_with_waker(std::future::ready(Ok(())), woken_tx.clone());
        reactor
            .reactor_tx
            .send((WakeDelay::Retry, waker_3))
            .unwrap();
        reactor
            .reactor_tx
            .send((WakeDelay::AfterNextRamRelease, waker_4))
            .unwrap();
        reactor
            .reactor_tx
            .send((WakeDelay::WakeAt(10.0), waker_5))
            .unwrap();
        // Not in queues yet
        assert_eq!(reactor.wakers_ram.len(), 0);
        assert_eq!(reactor.wakers_running.len(), 2);

        reactor.drain_queue();
        assert!(reactor.reactor_rx.try_recv().is_err());
        assert_eq!(reactor.wakers_ram.len(), 1);
        assert_eq!(reactor.wakers_running.len(), 4);
    }

    #[test]
    fn test_schedule_waker() {
        let now = 0.0;
        let mut wakers_running = WakersByTime::new();
        let mut wakers_ram = WakersInOrder::new();
        let (woken_tx, _woken_rx) = simple_channel::channel::<RcTask>();

        let delay_fn = |idx: usize| -> WakeDelay {
            match idx {
                idx @ 0..=2 => WakeDelay::WakeAt(idx as f64),
                3 => WakeDelay::AfterNextRamRelease,
                4 => WakeDelay::Retry,
                idx @ 5..=7 => WakeDelay::WakeAt(idx as f64),
                8 => WakeDelay::Retry,
                9 => WakeDelay::Immediate,
                _ => panic!(),
            }
        };

        let mut w: Vec<Waker> = vec![];
        for idx in 0..10 {
            let future = std::future::ready(Ok(()));
            let (_, waker) = get_task_with_waker(future, woken_tx.clone());
            w.push(waker.clone());
            let delay = delay_fn(idx);
            BitburnerReactor::schedule_waker(
                &mut wakers_running,
                &mut wakers_ram,
                now,
                delay,
                waker,
            )
        }

        let assert_same_wakers = |actual: &Vec<Waker>, expected: Vec<&Waker>| {
            assert_eq!(actual.len(), expected.len());
            let mut zipped = actual.iter().zip(expected);
            assert!(zipped.all(|(a, e)| a.will_wake(e)));
        };

        assert_eq!(wakers_running.len(), 7);
        assert_eq!(wakers_ram.len(), 1);

        let expected = vec![
            (now + 0.0, vec![&w[0], &w[9]]),
            (now + 1.0, vec![&w[1]]),
            (now + 2.0, vec![&w[2]]),
            (now + 5.0, vec![&w[5]]),
            (now + 6.0, vec![&w[6]]),
            (now + 7.0, vec![&w[7]]),
            (now + RETRY_WAIT, vec![&w[4], &w[8]]),
        ];
        for (time, expected_wakers) in expected {
            let key = F64Total::from(time);
            let actual_wakers = &wakers_running[&key];
            assert_same_wakers(actual_wakers, expected_wakers);
        }

        assert!(wakers_ram[0].will_wake(&w[3]))
    }
}
