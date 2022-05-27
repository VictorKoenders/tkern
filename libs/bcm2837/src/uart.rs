use crate::peripherals::AUX;
use bcm2837_pac::UART1;
use core::sync::atomic::{AtomicBool, Ordering};

static UART1_TAKEN: AtomicBool = AtomicBool::new(false);

pub struct Uart1<STATE> {
    state: STATE,
}

impl Uart1<Uninit> {
    pub fn take() -> Option<Uart1<Uninit>> {
        if UART1_TAKEN
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_ok()
        {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    pub unsafe fn steal() -> Uart1<Uninit> {
        Uart1 { state: Uninit(()) }
    }

    fn reg(&self) -> UART1 {
        // Safety: We're the only struct that should access the UART1 peripheral,
        // and the typesystem should guarantee this struct is not used mutably in 2 places.
        unsafe { bcm2837_pac::Peripherals::steal() }.UART1
    }

    pub fn init(self) -> Result<Uart1<Init>, Uart1<Uninit>> {
        AUX::enables(|e| e.modify(|_r, w| w.uart_1().set_bit()));
        let reg = self.reg();
        reg.cntl.write(|w| unsafe { w.bits(0) });
        reg.lcr.write(|w| w.data_size()._8bit());
        reg.mcr.write(|w| unsafe { w.bits(0) });
        reg.ier().write(|w| unsafe { w.bits(0) });
        reg.iir.write(|w| unsafe { w.bits(0xc6) });
        reg.baud.write(|w| unsafe { w.bits(270) });

        unimplemented!()
    }
}

pub struct Uninit(());
pub struct Init(());

impl<T> Drop for Uart1<T> {
    fn drop(&mut self) {
        AUX::enables(|e| e.modify(|_r, w| w.uart_1().clear_bit()));
        // Release the uart1 lock
        UART1_TAKEN.store(false, Ordering::Release);
    }
}
