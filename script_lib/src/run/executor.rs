use std::cell::Cell;
use std::future::Future;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use super::reactor::{BitburnerReactor, WakeDelay};
use super::waker::{PinnedFuture, SimpleWaker, Task};

pub type TaskResult = Result<(), String>;

/// Executes tasks as RAM becomes available.
pub struct ConstrainedPriorityExecutor<F>
where
    F: Future<Output = ()>,
{
    pub available_ram: Cell<f64>,
    pub woken_tx: Sender<Arc<Task>>,
    woken_rx: Receiver<Arc<Task>>,
    reactor: BitburnerReactor,
    sleep_fn: fn(f64) -> F,
}

impl<F> ConstrainedPriorityExecutor<F>
where
    F: Future<Output = ()>,
{
    pub fn new(max_ram: f64, instant_fn: fn() -> f64, sleep_fn: fn(f64) -> F) -> Self {
        let (woken_tx, woken_rx) = channel::<Arc<Task>>();
        let reactor = BitburnerReactor::new(instant_fn);
        ConstrainedPriorityExecutor {
            available_ram: max_ram.into(),
            woken_tx,
            woken_rx,
            reactor,
            sleep_fn,
        }
    }

    pub fn register(&self, future: PinnedFuture) {
        let task: Task = Mutex::new(future);
        let waker = SimpleWaker::waker(Arc::new(task), self.woken_tx.clone());
        self.schedule_wake(WakeDelay::Immediate, waker);
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
            match future.as_mut().poll(&mut cx) {
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

    pub fn schedule_wake(&self, wake_delay: WakeDelay, waker: Waker) {
        self.reactor
            .reactor_tx
            .send((wake_delay, waker))
            .expect("Reactor closed the queue");
    }

    pub fn use_ram(&self, ram: f64) {
        let available_ram = self.available_ram.get();
        self.available_ram.set(available_ram - ram);
    }

    pub fn free_ram(&self, ram: f64) {
        let available_ram = self.available_ram.get();
        self.available_ram.set(available_ram + ram);
    }

    pub fn can_launch(&self, ram: f64) -> bool {
        self.available_ram.get() >= ram
    }
}
