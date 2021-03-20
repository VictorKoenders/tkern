use super::RUNTIME;
use core::task::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct FutId(usize);

impl FutId {
    const ID_BIT_LENGTH: usize = core::mem::size_of::<usize>() * 8 - 8;
    const MAX_VALUE: usize = 1usize << Self::ID_BIT_LENGTH - 1;
    const RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
        fut_id_waker_clone,
        fut_id_waker_wake,
        fut_id_waker_wake_by_ref,
        fut_id_waker_drop,
    );

    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn generation(&self) -> u8 {
        (self.0 >> Self::ID_BIT_LENGTH) as u8
    }
    pub fn id(&self) -> usize {
        self.0 & Self::MAX_VALUE
    }
    pub fn next_generation(self) -> FutId {
        let generation = self.generation().wrapping_add(1);
        let id = self.id();
        FutId(id | ((generation as usize) << Self::ID_BIT_LENGTH))
    }

    pub fn waker(&self) -> Waker {
        unsafe { Waker::from_raw(RawWaker::new(self.0 as *const (), &Self::RAW_WAKER_VTABLE)) }
    }
}

unsafe fn fut_id_waker_clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &FutId::RAW_WAKER_VTABLE)
}
unsafe fn fut_id_waker_wake(data: *const ()) {
    let futid = FutId(data as usize);
    RUNTIME.notify_awake(futid);
}
unsafe fn fut_id_waker_wake_by_ref(data: *const ()) {
    let futid = FutId(data as usize);
    RUNTIME.notify_awake(futid);
}
unsafe fn fut_id_waker_drop(_: *const ()) {}
