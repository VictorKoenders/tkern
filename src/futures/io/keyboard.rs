use crate::futures::prelude::*;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crossbeam::atomic::AtomicCell;
use pin_project_lite::pin_project;
use smallvec::SmallVec;
use spin::Mutex;

fn runtime_keyboard<F, T>(f: F) -> T
where
    F: FnOnce(&Keyboard) -> T,
{
    let runtime = &crate::futures::RUNTIME;
    let t = f(&runtime.keyboard);
    t
}

#[derive(Default)]
pub struct Keyboard {
    tasks: Mutex<SmallVec<[Waker; 10]>>,
    last_key: AtomicCell<Option<KeyInput>>,
    last_key_idx: AtomicUsize,
    has_new_key: AtomicBool,
}

impl Keyboard {
    pub fn next_key() -> impl Future<Output = KeyInput> + Send {
        let last_key_idx = runtime_keyboard(|kb| kb.last_key_idx.load(Ordering::Relaxed));
        KeyboardNextKeyFuture { last_key_idx }
    }

    pub fn notify_key_pressed(&self, character: char, modifiers: KeyboardModifiers) {
        self.last_key.store(Some(KeyInput {
            character,
            modifiers,
        }));
        self.last_key_idx.fetch_add(1, Ordering::Relaxed);
        self.has_new_key.store(true, Ordering::Relaxed);

        if self.has_new_key.load(Ordering::Relaxed) {
            let tasks = {
                let mut tasks = self.tasks.lock();
                core::mem::replace(&mut *tasks, SmallVec::new())
            };

            while self.has_new_key.swap(false, Ordering::Relaxed) {
                for task in &tasks {
                    task.wake_by_ref();
                }
            }
        }
    }

    fn register_waker(&self, waker: Waker) {
        let mut tasks = self.tasks.lock();
        tasks.push(waker);
    }
}

pin_project! {
    struct KeyboardNextKeyFuture {
        last_key_idx: usize,
    }
}

impl Future for KeyboardNextKeyFuture {
    type Output = KeyInput;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let (last_key_idx, last_key) =
            runtime_keyboard(|kb| (kb.last_key_idx.load(Ordering::Relaxed), kb.last_key.load()));
        if *this.last_key_idx != last_key_idx {
            if let Some(last_key) = last_key {
                *this.last_key_idx = last_key_idx;
                return Poll::Ready(last_key);
            }
        }
        runtime_keyboard(|kb| kb.register_waker(cx.waker().clone()));
        Poll::Pending
    }
}

#[derive(Copy, Clone, Debug)]
pub struct KeyInput {
    pub character: char,
    pub modifiers: KeyboardModifiers,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct KeyboardModifiers {
    pub ctrl_pressed: bool,
}
