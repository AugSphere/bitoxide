use std::cell::RefCell;
use std::future::Future;
use std::panic::panic_any;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::Waker;
use std::thread::{self, ThreadId};

use cooked_waker::{IntoWaker, WakeRef};

use super::executor::TaskResult;
use crate::simple_channel::Sender;

pub type PinnedFuture = Pin<Box<dyn Future<Output = TaskResult>>>;
pub type Task = RefCell<PinnedFuture>;
pub type RcTask = Rc<Task>;

const SEND_PANIC: &str = "SimpleWaker can not be sent across threads";

#[derive(Clone)]
pub struct SimpleWaker {
    task: RcTask,
    woken_tx: Sender<RcTask>,
    _thread_id: ThreadId,
}

// Immediately panic on being woken from a different thread
unsafe impl Send for SimpleWaker {}
unsafe impl Sync for SimpleWaker {}

impl SimpleWaker {
    pub fn new(task: RcTask, woken_tx: Sender<RcTask>) -> Self {
        SimpleWaker {
            task,
            woken_tx,
            _thread_id: thread::current().id(),
        }
    }

    pub fn waker(task: RcTask, woken_tx: Sender<RcTask>) -> Waker {
        let waker = Self::new(task, woken_tx);
        Arc::new(waker).into_waker()
    }
}

impl WakeRef for SimpleWaker {
    fn wake_by_ref(&self) {
        if thread::current().id() != self._thread_id {
            panic_any(SEND_PANIC);
        }
        self.woken_tx
            .send(self.task.clone())
            .expect("Executor closed the queue");
    }
}

#[allow(dead_code)]
pub fn get_task_with_waker<F>(future: F, woken_tx: Sender<RcTask>) -> (RcTask, Waker)
where
    F: Future<Output = TaskResult> + Send + 'static,
{
    let task: RcTask = RcTask::new(Task::new(Box::pin(future)));
    let waker: Waker = SimpleWaker::waker(task.clone(), woken_tx);
    (task, waker)
}

#[cfg(test)]
mod tests {
    use std::task::Waker;
    use std::thread;

    use super::{get_task_with_waker, RcTask, SEND_PANIC};
    use crate::simple_channel;

    #[test]
    fn test_wake() {
        let (woken_tx, woken_rx) = simple_channel::channel::<RcTask>();
        let future = std::future::ready(Ok(()));
        let (arc_task, waker) = get_task_with_waker(future, woken_tx);
        let waker_ref: &Waker = &waker;

        // One in rc_task and one inside the waker
        assert!(RcTask::strong_count(&arc_task) == 2);

        waker_ref.wake_by_ref();
        // New ref in the queue
        assert!(RcTask::strong_count(&arc_task) == 3);
        assert!(woken_rx.recv().is_ok_and(|t| RcTask::ptr_eq(&t, &arc_task)));
        assert!(RcTask::strong_count(&arc_task) == 2);

        // Can wake multiple times, always pointing to the same task
        waker_ref.wake_by_ref();
        assert!(RcTask::strong_count(&arc_task) == 3);
        assert!(woken_rx.recv().is_ok_and(|t| RcTask::ptr_eq(&t, &arc_task)));
        assert!(RcTask::strong_count(&arc_task) == 2);

        // Can wake by value, ref inside waker is dropped, ref inside queue created
        waker.wake();
        assert!(RcTask::strong_count(&arc_task) == 2);
        assert!(woken_rx.recv().is_ok_and(|t| RcTask::ptr_eq(&t, &arc_task)));
        assert!(RcTask::strong_count(&arc_task) == 1);

        // No unexpected sends
        assert!(woken_rx.try_recv().is_err());
    }

    #[test]
    fn test_send_panic() {
        let (woken_tx, _woken_rx) = simple_channel::channel::<RcTask>();
        let future = std::future::ready(Ok(()));
        let (_, waker) = get_task_with_waker(future, woken_tx);

        let result = thread::spawn(move || {
            waker.wake();
        })
        .join();

        assert!(
            result.is_err_and(|msg| { msg.downcast::<&str>().is_ok_and(|msg| *msg == SEND_PANIC) })
        );
    }
}
