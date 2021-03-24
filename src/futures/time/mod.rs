use crate::futures::prelude::*;
use core::sync::atomic::{AtomicU64, Ordering};
use pin_project_lite::pin_project;
use smallvec::SmallVec;
use spin::Mutex;

fn runtime_time<F, T>(f: F) -> T
where
    F: FnOnce(&Time) -> T,
{
    let runtime = &crate::futures::RUNTIME;
    f(&runtime.time)
}

#[derive(Default)]
pub struct Time {
    tasks: Mutex<SmallVec<[Waker; 10]>>,
    ticks: AtomicU64,
}

impl Time {
    fn register_waker(&self, waker: Waker) {
        let mut tasks = self.tasks.lock();
        tasks.push(waker);
    }

    pub fn notify_time_tick(&self) {
        self.ticks.fetch_add(1, Ordering::Relaxed);
        let tasks = {
            let mut tasks = self.tasks.lock();
            core::mem::replace(&mut *tasks, SmallVec::new())
        };

        for task in tasks {
            task.wake();
        }
    }
}

pub fn wait_for_interrupt() -> impl Future<Output = ()> {
    TimeInterruptFuture {
        start_ticks: runtime_time(|t| t.ticks.load(Ordering::Relaxed)),
    }
}

pin_project! {
    struct TimeInterruptFuture {
        start_ticks: u64,
    }
}

impl Future for TimeInterruptFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let ticks = runtime_time(|time| time.ticks.load(Ordering::Relaxed));
        if ticks != *this.start_ticks {
            Poll::Ready(())
        } else {
            runtime_time(|time| time.register_waker(cx.waker().clone()));
            Poll::Pending
        }
    }
}
