pub mod io;
pub mod time;

mod fut_id;
mod runtime_future;

use self::{
    fut_id::FutId,
    runtime_future::{MaybeFut, RuntimeFuture},
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{
    future::Future,
    sync::atomic::{AtomicUsize, Ordering},
};
use futures::future::FutureExt;
use lazy_static::lazy_static;
use spin::Mutex;

pub(self) mod prelude {
    pub use core::{
        future::Future,
        pin::Pin,
        task::{Context, Poll, Waker},
    };
}

lazy_static! {
    pub static ref RUNTIME: Runtime = Runtime::default();
}

#[derive(Default)]
pub struct Runtime {
    // TODO: these mutexes seem very inefficient for multithreaded access,
    // specifically the entire futures mutex being locked when a single future is executing.
    // We should validate this assumption and find a better way to access
    // this data if it turns out to be an issue
    futures: Mutex<Vec<MaybeFut>>,
    awoken_futures: Mutex<Vec<FutId>>,
    keyboard: self::io::Keyboard,
    time: self::time::Time,
    awoken_future_count: AtomicUsize,
}

impl Runtime {
    pub fn notify_key_pressed(&self, character: char, modifier: self::io::KeyboardModifiers) {
        self.keyboard.notify_key_pressed(character, modifier);
    }

    fn notify_awake(&self, id: FutId) {
        let mut futures = self.futures.lock();

        let future = match futures.get_mut(id.id()) {
            Some(MaybeFut::Present { future }) if future.id == id => future,
            _ => return,
        };
        future.woken = true;

        let mut awoken_futures = self.awoken_futures.lock();
        awoken_futures.push(future.id);
        self.awoken_future_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn spawn<T>(&self, future: T) -> FutId
    where
        T: Future<Output = ()> + Send + 'static,
    {
        // Make sure we get a lock on these 2 mutexes first
        let mut futures = self.futures.lock();
        let mut awoken_futures = self.awoken_futures.lock();

        // find the first new future ID
        let mut id = 0;
        let future_id = loop {
            if futures.len() >= id {
                futures.push(MaybeFut::Vacant {
                    next_id: FutId::new(id),
                });
                break FutId::new(id);
            } else if let Some(MaybeFut::Vacant { next_id }) = futures.get(id) {
                break *next_id;
            }
            id += 1;
        };

        futures[future_id.id()] = MaybeFut::Present {
            future: RuntimeFuture {
                id: future_id,
                woken: true,
                future: Box::pin(future),
            },
        };

        awoken_futures.push(future_id);
        self.awoken_future_count.fetch_add(1, Ordering::Relaxed);
        future_id
    }

    fn drive(&self, future_id: FutId) {
        use core::task::*;

        let mut futures = self.futures.lock();
        let future = match futures.get_mut(future_id.id()) {
            Some(MaybeFut::Present { future }) if future.id == future_id => future,
            _ => return,
        };
        let waker = future_id.waker();
        let mut context = Context::from_waker(&waker);
        if let Poll::Ready(()) = future.future.as_mut().poll(&mut context) {
            futures[future_id.id()] = MaybeFut::Vacant {
                next_id: future_id.next_generation(),
            };
        } else {
            future.woken = false;
        }
    }

    fn drive_all(&self) -> usize {
        let awoken_futures = {
            let mut awoken_futures = self.awoken_futures.lock();
            let actual_futures = core::mem::replace(&mut *awoken_futures, Vec::new());
            // Do this here while the `self.awoken_futures` lock is still active
            // So we make sure these 2 variables are updated in sync
            self.awoken_future_count.store(0, Ordering::Relaxed);
            actual_futures
        };
        let count = awoken_futures.len();
        for future_id in awoken_futures {
            self.drive(future_id);
        }

        count
    }

    pub fn run<T>(&self, future: T) -> !
    where
        T: Future<Output = !> + Send + 'static,
    {
        self.spawn(future.map(|_| ()));

        loop {
            let count = self.drive_all();
            if count == 0 {
                crate::arch::halt();
            }
        }
    }

    pub fn is_running(&self, future_id: FutId) -> bool {
        let mut futures = self.futures.lock();
        let future_status = futures.get_mut(future_id.id());
        matches!(future_status, Some(MaybeFut::Present { future }) if future.id == future_id)
    }
}

pub fn block_on<F, T>(f: F) -> T
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let result: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));
    let result_clone = result.clone();
    let fut_id = RUNTIME.spawn(async move {
        let t = f.await;
        let mut guard = result_clone.lock();
        *guard = Some(t);
    });

    while RUNTIME.is_running(fut_id) {
        RUNTIME.drive_all();
    }
    let value = match Arc::try_unwrap(result) {
        Ok(v) => v,
        Err(_) => {
            panic!("Could not unwrap crate::futures::block_on value, is the other arc not dropped?")
        }
    };
    value
        .into_inner()
        .expect("crates::future::block_on did not complete, no value was set")
}
