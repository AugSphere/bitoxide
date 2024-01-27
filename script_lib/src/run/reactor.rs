use std::collections::{BTreeMap, VecDeque};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::task::Waker;

use crate::{F64Total, RETRY_WAIT};

pub type WakersByTime = BTreeMap<F64Total, Waker>;
pub type WakersWithTime = (WakeDelay, Waker);

#[derive(Debug, Clone, Copy)]
pub enum WakeDelay {
    Immediate,
    AfterNextRamRelease,
    WakeAt(f64),
}

pub struct BitburnerReactor {
    pub reactor_tx: Sender<WakersWithTime>,
    reactor_rx: Receiver<WakersWithTime>,
    wakers_running: WakersByTime,
    wakers_waiting: VecDeque<Waker>,
    instant_fn: fn() -> f64,
}

impl BitburnerReactor {
    pub fn new(instant_fn: fn() -> f64) -> Self {
        let (reactor_tx, reactor_rx) = channel::<WakersWithTime>();
        BitburnerReactor {
            reactor_tx,
            reactor_rx,
            wakers_running: WakersByTime::new(),
            wakers_waiting: VecDeque::new(),
            instant_fn,
        }
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
        self.wakers_waiting.is_empty() & self.wakers_running.is_empty()
    }

    fn drain_queue(&mut self) {
        let now = self.now();
        for (delay, mut waker) in self.reactor_rx.try_iter() {
            let wake_at: Option<f64> = match delay {
                WakeDelay::Immediate => Some(now + RETRY_WAIT),
                WakeDelay::AfterNextRamRelease => None,
                WakeDelay::WakeAt(time) => Some(time),
            };
            if let Some(mut time) = wake_at {
                while let Some(old_waker) = self.wakers_running.insert(time.into(), waker) {
                    time += RETRY_WAIT;
                    waker = old_waker;
                }
            } else {
                self.wakers_waiting.push_back(waker);
            }
        }
    }

    pub fn wake_waiting(&mut self) -> usize {
        self.drain_queue();
        let woken = self.wakers_waiting.len();
        for waker in self.wakers_waiting.drain(..) {
            waker.wake();
        }
        woken
    }

    pub fn wake_running(&mut self) -> usize {
        self.drain_queue();
        let now = self.now();
        let still_waiting = self.wakers_running.split_off(&now.into());
        let woken = self.wakers_running.len();
        for waker in self.wakers_running.values() {
            waker.wake_by_ref();
        }
        self.wakers_running = still_waiting;
        woken
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use std::sync::Arc;

    use super::{BitburnerReactor, WakeDelay};
    use crate::run::waker::{get_task_with_waker, ArcTask, SimpleWaker};
    use crate::{F64Total, RETRY_WAIT};

    #[test]
    fn test_new() {
        let mut reactor = BitburnerReactor::new(|| 0.0);
        assert!(reactor.is_empty());
        assert!(reactor.next_wake().is_none())
    }

    #[test]
    fn test_drain_queue() {
        let mut reactor = BitburnerReactor::new(|| 0.0);
        let (woken_tx, _woken_rx) = channel::<ArcTask>();
        let (task_1, waker_1) = get_task_with_waker(std::future::ready(Ok(())), woken_tx.clone());
        let immediate = (WakeDelay::Immediate, waker_1);

        // Refs are in task_1 and waker_1
        assert!(Arc::strong_count(&task_1) == 2);
        reactor.drain_queue();

        reactor.reactor_tx.send(immediate).unwrap();
        // Refs are in task_1 and reactor_tx
        assert!(Arc::strong_count(&task_1) == 2);

        reactor.drain_queue();
        // Refs are in task_1 and wakers_running
        assert!(Arc::strong_count(&task_1) == 2);
        // Scheduled to run after RETRY_WAIT
        assert!(reactor
            .wakers_running
            .first_entry()
            .is_some_and(|e| *e.key() == F64Total::from(RETRY_WAIT)));

        // Queue now empty
        assert!(reactor.reactor_rx.try_recv().is_err());

        let waker_2 = SimpleWaker::waker(task_1.clone(), woken_tx.clone());
        let delayed = (WakeDelay::WakeAt(4.0), waker_2);
        // Now also referred to from waker_2
        assert!(Arc::strong_count(&task_1) == 3);

        reactor.reactor_tx.send(delayed).unwrap();
        assert!(Arc::strong_count(&task_1) == 3);

        reactor.drain_queue();
        let running: Vec<&F64Total> = reactor.wakers_running.keys().collect();
        assert_eq!(
            running,
            vec![&F64Total::from(4.0), &F64Total::from(RETRY_WAIT)]
        );
    }
}
