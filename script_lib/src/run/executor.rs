use std::future::Future;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use super::reactor::{BitburnerReactor, WakeDelay, WakerWithTime};
use super::waker::{PinnedFuture, SimpleWaker, Task};

pub type TaskResult = Result<(), String>;
pub trait SleepFuture: Future<Output = ()> {}
impl<T: Future<Output = ()>> SleepFuture for T {}

#[derive(Debug, Clone, Copy)]
pub enum RamChange {
    Use(f64),
    Release(f64),
}

/// Executes tasks as RAM becomes available.
pub struct BitburnerExecutor<F>
where
    F: SleepFuture,
{
    available_ram: f64,
    woken_tx: Sender<Arc<Task>>,
    woken_rx: Receiver<Arc<Task>>,
    ram_tx: Sender<RamChange>,
    ram_rx: Receiver<RamChange>,
    reactor: BitburnerReactor,
    sleep_fn: fn(f64) -> F,
}

impl<F> BitburnerExecutor<F>
where
    F: SleepFuture,
{
    pub fn new(max_ram: f64, instant_fn: fn() -> f64, sleep_fn: fn(f64) -> F) -> Self {
        let (woken_tx, woken_rx) = channel::<Arc<Task>>();
        let (ram_tx, ram_rx) = channel::<RamChange>();
        let reactor = BitburnerReactor::new(instant_fn);
        BitburnerExecutor {
            available_ram: max_ram,
            woken_tx,
            woken_rx,
            ram_tx,
            ram_rx,
            reactor,
            sleep_fn,
        }
    }

    pub fn register(&self, future: PinnedFuture) {
        let task: Task = Mutex::new(future);
        let waker = SimpleWaker::waker(Arc::new(task), self.woken_tx.clone());
        self.get_schedule_queue()
            .send((WakeDelay::Immediate, waker))
            .expect("Reactor closed the queue");
    }

    pub async fn run(&mut self) -> TaskResult {
        loop {
            if let Some(time) = self.reactor.next_wake() {
                let duration = time - self.now();
                if duration > 0.0 {
                    self.sleep(duration).await;
                }
            }
            self.reactor.wake_running();
            self.reactor.wake_on_ram_release();

            self.poll()?;
            if self.reactor.is_empty() {
                return Ok(());
            }
        }
    }

    async fn sleep(&self, duration: f64) {
        (self.sleep_fn)(duration).await
    }

    fn poll(&mut self) -> TaskResult {
        for woken in self.woken_rx.try_iter() {
            let waker = SimpleWaker::waker(woken.clone(), self.woken_tx.clone());
            let mut cx = Context::from_waker(&waker);
            let mut future = woken
                .try_lock()
                .expect("Could not borrow the future from the task");
            let poll = future.as_mut().poll(&mut cx);
            Self::track_ram_use(&mut self.ram_rx, &mut self.available_ram);
            match poll {
                Poll::Ready(e @ Err(_)) => {
                    return e;
                }
                Poll::Ready(Ok(_)) => {
                    self.reactor.wake_on_ram_release();
                }
                Poll::Pending => {}
            };
        }
        Ok(())
    }

    pub fn now(&self) -> f64 {
        self.reactor.now()
    }

    pub fn get_ram_change_queue(&self) -> Sender<RamChange> {
        self.ram_tx.clone()
    }

    pub fn get_schedule_queue(&self) -> Sender<WakerWithTime> {
        self.reactor.get_schedule_queue()
    }

    fn track_ram_use(ram_rx: &mut Receiver<RamChange>, available_ram: &mut f64) {
        for change in ram_rx.try_iter() {
            match change {
                RamChange::Release(ram) => *available_ram += ram,
                RamChange::Use(ram) => *available_ram -= ram,
            }
            assert!(available_ram >= &mut 0.0);
        }
    }
}
