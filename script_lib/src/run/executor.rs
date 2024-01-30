use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use std::task::{Context, Poll};

use super::reactor::{BitburnerReactor, WakeDelay, WakerWithTime};
use super::waker::{PinnedFuture, RcTask, SimpleWaker, Task};
use crate::simple_channel::{self, Receiver, Sender};

pub type TaskResult = Result<(), String>;
pub trait SleepFuture: Future<Output = ()> {}
impl<T: Future<Output = ()>> SleepFuture for T {}

const EXECUTOR_YIELD_MSEC: f64 = 1.0;

#[derive(Debug, Clone, Copy)]
pub enum RamChange {
    Use(f64),
    Release(f64),
}

impl std::fmt::Display for RamChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = |tag: &str, val: &f64| {
            let prec = f.precision().unwrap_or(3);
            write!(f, "{tag}({val:.0$})", prec)
        };
        match self {
            RamChange::Use(v) => fmt("Use", v),
            RamChange::Release(v) => fmt("Release", v),
        }
    }
}

/// Executes tasks as RAM becomes available.
pub struct BitburnerExecutor<F>
where
    F: SleepFuture,
{
    woken_tx: Sender<RcTask>,
    woken_rx: Receiver<RcTask>,
    reactor: BitburnerReactor,
    ram_cell: Rc<RefCell<f64>>,
    sleep_fn: fn(f64) -> F,
}

impl<F> BitburnerExecutor<F>
where
    F: SleepFuture,
{
    pub fn new(max_ram: f64, instant_fn: fn() -> f64, sleep_fn: fn(f64) -> F) -> Self {
        let (woken_tx, woken_rx) = simple_channel::channel::<RcTask>();
        let ram_cell = Rc::new(RefCell::new(max_ram));
        let reactor = BitburnerReactor::new(instant_fn);
        BitburnerExecutor {
            woken_tx,
            woken_rx,
            ram_cell,
            reactor,
            sleep_fn,
        }
    }

    pub fn register(&self, future: PinnedFuture) {
        let task: Task = Task::new(future);
        let waker = SimpleWaker::waker(RcTask::new(task), self.woken_tx.clone());
        self.get_schedule_queue()
            .send((WakeDelay::Immediate, waker))
            .expect("Reactor closed the queue");
    }

    pub async fn run(&mut self) -> TaskResult {
        loop {
            assert!(*self.ram_cell.borrow() >= 0.0);
            let sleep_for = self.reactor.next_wake().map_or(0.0, |t| t - self.now());
            // Always yield to avoid starving the browser
            self.sleep(sleep_for.max(EXECUTOR_YIELD_MSEC)).await;

            self.reactor.wake_running();
            self.reactor.wake_on_ram_release();

            self.poll()?;
            if self.reactor.is_empty() {
                return Ok(());
            }
            if self.reactor.has_no_running() {
                return Err("All tasks waiting on RAM release".to_owned());
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
                .try_borrow_mut()
                .expect("Could not borrow the future from the task");
            let poll = future.as_mut().poll(&mut cx);
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

    pub fn get_ram_cell(&self) -> Rc<RefCell<f64>> {
        self.ram_cell.clone()
    }

    pub fn get_schedule_queue(&self) -> Sender<WakerWithTime> {
        self.reactor.get_schedule_queue()
    }
}
