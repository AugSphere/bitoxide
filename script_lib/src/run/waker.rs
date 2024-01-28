use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::task::Waker;

use cooked_waker::{IntoWaker, WakeRef};

use super::executor::TaskResult;

pub type PinnedFuture = Pin<Box<dyn Future<Output = TaskResult>>>;
pub type Task = Mutex<PinnedFuture>;
pub type ArcTask = Arc<Task>;

#[derive(Clone)]
pub struct SimpleWaker {
    task: ArcTask,
    woken_tx: Sender<ArcTask>,
}

// A lie, but we are in a browser, so only 1 thread
unsafe impl Send for SimpleWaker {}
unsafe impl Sync for SimpleWaker {}

impl SimpleWaker {
    pub fn new(task: ArcTask, woken_tx: Sender<ArcTask>) -> Self {
        SimpleWaker { task, woken_tx }
    }

    pub fn waker(task: ArcTask, woken_tx: Sender<ArcTask>) -> Waker {
        let waker = Self::new(task, woken_tx);
        Arc::new(waker).into_waker()
    }
}

impl WakeRef for SimpleWaker {
    fn wake_by_ref(&self) {
        self.woken_tx
            .send(self.task.clone())
            .expect("Executor closed the queue");
    }
}

#[allow(dead_code)]
pub fn get_task_with_waker<F>(future: F, woken_tx: Sender<ArcTask>) -> (ArcTask, Waker)
where
    F: Future<Output = TaskResult> + Send + 'static,
{
    let task: Task = Mutex::new(Box::pin(future));
    let arc_task: ArcTask = Arc::new(task);
    let waker: Waker = SimpleWaker::waker(arc_task.clone(), woken_tx);
    (arc_task, waker)
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use std::task::Waker;

    use super::{get_task_with_waker, ArcTask};

    #[test]
    fn test_wake() {
        let (woken_tx, woken_rx) = channel::<ArcTask>();
        let future = std::future::ready(Ok(()));
        let (arc_task, waker) = get_task_with_waker(future, woken_tx);
        let waker_ref: &Waker = &waker;

        // One in rc_task and one inside the waker
        assert!(Arc::strong_count(&arc_task) == 2);

        waker_ref.wake_by_ref();
        // New ref in the queue
        assert!(Arc::strong_count(&arc_task) == 3);
        assert!(woken_rx.recv().is_ok_and(|t| Arc::ptr_eq(&t, &arc_task)));
        assert!(Arc::strong_count(&arc_task) == 2);

        // Can wake multiple times, always pointing to the same task
        waker_ref.wake_by_ref();
        assert!(Arc::strong_count(&arc_task) == 3);
        assert!(woken_rx.recv().is_ok_and(|t| Arc::ptr_eq(&t, &arc_task)));
        assert!(Arc::strong_count(&arc_task) == 2);

        // Can wake by value, ref inside waker is dropped, ref inside queue created
        waker.wake();
        assert!(Arc::strong_count(&arc_task) == 2);
        assert!(woken_rx.recv().is_ok_and(|t| Arc::ptr_eq(&t, &arc_task)));
        assert!(Arc::strong_count(&arc_task) == 1);

        // No unexpected sends
        assert!(woken_rx.try_recv().is_err());
    }
}
