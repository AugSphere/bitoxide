use std::future::Future;
use std::pin::Pin;
use std::task::{Poll, Waker};

use bitburner_api::netscript::{Arg, ThreadOrOptions};
use bitburner_api::NS;

use super::executor::{ConstrainedPriorityExecutor, TaskResult};
use super::reactor::WakeDelay;

pub struct BitburnerProcess<'a, F: Future<Output = ()>> {
    ns: &'a NS,
    executor: &'a ConstrainedPriorityExecutor<F>,
    script: String,
    thread_or_options: Option<ThreadOrOptions>,
    args: Vec<Arg>,
    duration_hint: f64,
    /// Total RAM requirement hint, not per thread
    ram_hint: f64,
    pid: Option<u32>,
    is_released: bool,
    start_instant: Option<f64>,
    last_polled: Option<f64>,
}
impl<'a, F: Future<Output = ()>> BitburnerProcess<'a, F> {
    pub fn new(
        ns: &'a NS,
        executor: &'a ConstrainedPriorityExecutor<F>,
        script: String,
        thread_or_options: Option<ThreadOrOptions>,
        args: Vec<Arg>,
        duration_hint: f64,
        ram_hint: f64,
    ) -> Self {
        BitburnerProcess {
            ns,
            executor,
            script,
            thread_or_options,
            args,
            duration_hint,
            ram_hint,
            pid: None,
            is_released: false,
            start_instant: None,
            last_polled: None,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        if self.is_launched() {
            return Err("Cannot only run once".to_owned());
        }
        let pid = self
            .ns
            .run(&self.script, self.thread_or_options, self.args.clone());
        if pid != 0 {
            self.executor.use_ram(self.ram_hint);
            self.start_instant = Some(self.executor.now());
            self.pid = Some(pid);
            Ok(())
        } else {
            Err("Failed to launch".to_owned())
        }
    }

    pub fn can_launch(&self) -> bool {
        self.executor.can_launch(self.ram_hint)
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
        self.is_launched() & !self.is_finished()
    }

    pub fn kill(&mut self) -> bool {
        let Some(pid) = self.pid else {
            return false;
        };
        if self.ns.kill(pid) {
            self.release();
            true
        } else {
            false
        }
    }

    fn release(&mut self) {
        if !self.is_released & self.is_finished() {
            self.executor.free_ram(self.ram_hint);
            self.is_released = true;
        }
    }

    fn schedule_wake(&self, waker: &Waker) {
        let wake_at: WakeDelay = match (self.start_instant, self.last_polled) {
            (None, _) => WakeDelay::AfterNextRamRelease,
            (Some(start_instant), last_polled) => {
                let expected_finish = start_instant + self.duration_hint;
                let now = self.executor.now();
                match last_polled {
                    Some(last_polled) if last_polled > expected_finish => WakeDelay::Retry,
                    Some(_) => WakeDelay::WakeAt(expected_finish),
                    None if now >= expected_finish => WakeDelay::Immediate,
                    None => WakeDelay::WakeAt(expected_finish),
                }
            }
        };
        self.executor.schedule_wake(wake_at, waker.clone());
    }
}

impl<'a, F: Future<Output = ()>> Future for BitburnerProcess<'a, F> {
    type Output = TaskResult;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
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
                if let Err(msg) = self.run() {
                    return Poll::Ready(Err(msg));
                }
            }
            self.schedule_wake(cx.waker());
            Poll::Pending
        }
    }
}

impl<'a, F: Future<Output = ()>> Drop for BitburnerProcess<'a, F> {
    fn drop(&mut self) {
        self.kill();
    }
}
