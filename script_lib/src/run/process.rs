use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Poll, Waker};

use bitburner_api::netscript::{Arg, ThreadOrOptions};
use bitburner_api::NS;

use super::executor::{RamChange, TaskResult};
use super::reactor::{WakeDelay, WakerWithTime};
use crate::simple_channel::Sender;

pub struct BitburnerProcess {
    ns: Rc<NS>,
    script: String,
    thread_or_options: Option<ThreadOrOptions>,
    args: Vec<Arg>,
    duration_hint: f64,
    /// Total RAM requirement hint, not per thread
    ram_hint: f64,
    pid: Option<u32>,
    instant_fn: fn() -> f64,
    ram_cell: Rc<RefCell<f64>>,
    schedule_tx: Sender<WakerWithTime>,
    is_released: bool,
    start_instant: Option<f64>,
    last_polled: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ExecutorData {
    instant_fn: fn() -> f64,
    ram_cell: Rc<RefCell<f64>>,
    schedule_tx: Sender<WakerWithTime>,
}

impl ExecutorData {
    pub fn new(
        instant_fn: fn() -> f64,
        ram_cell: Rc<RefCell<f64>>,
        schedule_tx: Sender<WakerWithTime>,
    ) -> Self {
        ExecutorData {
            instant_fn,
            ram_cell,
            schedule_tx,
        }
    }
}

impl BitburnerProcess {
    pub fn new(
        ns: Rc<NS>,
        script: String,
        thread_or_options: Option<ThreadOrOptions>,
        args: Vec<Arg>,
        duration_hint: f64,
        ram_hint: f64,
        executor_data: ExecutorData,
    ) -> Self {
        let ExecutorData {
            instant_fn,
            ram_cell,
            schedule_tx,
        } = executor_data;
        BitburnerProcess {
            ns,
            script,
            thread_or_options,
            args,
            duration_hint,
            ram_hint,
            pid: None,
            instant_fn,
            ram_cell,
            schedule_tx,
            is_released: false,
            start_instant: None,
            last_polled: None,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        assert!(!self.is_launched());
        let pid = self
            .ns
            .run(&self.script, self.thread_or_options, self.args.clone());
        if pid != 0 {
            self.update_ram(RamChange::Use(self.ram_hint));
            self.start_instant = Some(self.now());
            self.pid = Some(pid);
            Ok(())
        } else {
            Err("Failed to launch".to_owned())
        }
    }

    pub fn is_launched(&self) -> bool {
        self.pid.is_some()
    }

    pub fn is_finished(&self) -> bool {
        let Some(pid) = self.pid else {
            return false;
        };
        !self.ns.is_running(pid.into(), None, self.args.clone())
    }

    pub fn is_running(&self) -> bool {
        self.is_launched() && !self.is_finished()
    }

    pub fn kill(&mut self) -> bool {
        if let Some(pid) = self.pid {
            // only call kill if not running, to prevent spamming the tail log
            if self.is_running() && self.ns.kill(pid) {
                self.release();
                self.ns.print(&format!("{} killed", self.addr()));
                return true;
            }
        }
        false
    }

    fn release(&mut self) {
        assert!(self.is_finished());
        if !self.is_released {
            self.update_ram(RamChange::Release(self.ram_hint));
            self.is_released = true;
        }
    }

    fn schedule_wake(&self, waker: &Waker) {
        let wake_at: WakeDelay = match (self.start_instant, self.last_polled) {
            (None, _) => WakeDelay::AfterNextRamRelease,
            (Some(start_instant), last_polled) => {
                let expected_finish = start_instant + self.duration_hint;
                let now = self.now();
                match last_polled {
                    Some(last_polled) if last_polled > expected_finish => WakeDelay::Retry,
                    Some(_) => WakeDelay::WakeAt(expected_finish),
                    None if now >= expected_finish => WakeDelay::Immediate,
                    None => WakeDelay::WakeAt(expected_finish),
                }
            }
        };
        self.schedule_tx
            .send((wake_at, waker.clone()))
            .expect("Reactor closed the scheduling queue");
        self.ns
            .print(&format!("{} sheduled wake: {:.3}", self.addr(), wake_at));
    }

    fn now(&self) -> f64 {
        (self.instant_fn)()
    }

    fn can_launch(&self) -> bool {
        *self.ram_cell.borrow() >= self.ram_hint
    }

    fn update_ram(&self, ram_change: RamChange) {
        let mut ram = self.ram_cell.borrow_mut();
        match ram_change {
            RamChange::Use(r) => *ram -= r,
            RamChange::Release(r) => *ram += r,
        }
        self.ns
            .print(&format!("{} updated ram: {:.3}", self.addr(), ram_change));
    }

    fn addr(&self) -> usize {
        self as *const Self as usize
    }
}

impl Future for BitburnerProcess {
    type Output = TaskResult;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.ns.print(&format!(
            "{} being polled at {:.3}",
            self.addr(),
            self.now() / 1e3
        ));
        self.last_polled = Some(self.now());
        if self.is_launched() {
            if self.is_finished() {
                self.release();
                Poll::Ready(Ok(()))
            } else {
                self.schedule_wake(cx.waker());
                Poll::Pending
            }
        } else {
            if self.can_launch() {
                if let e @ Err(_) = self.run() {
                    return Poll::Ready(e);
                }
            }
            self.schedule_wake(cx.waker());
            Poll::Pending
        }
    }
}

impl Drop for BitburnerProcess {
    fn drop(&mut self) {
        self.ns.print(&format!("{} dropping", self.addr()));
        self.kill();
    }
}
