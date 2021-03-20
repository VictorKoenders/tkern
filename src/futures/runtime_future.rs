use super::FutId;
use alloc::boxed::Box;
use core::{future::Future, pin::Pin};

pub enum MaybeFut {
    Present { future: RuntimeFuture },
    Vacant { next_id: FutId },
}

pub struct RuntimeFuture {
    pub id: FutId,
    pub future: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    pub woken: bool,
}
