//! This module exposes each of the peripherals in [`bcm2837_pac`] behind an [`AtomicBool`] flag.
//!
//! This ensures that the access of these peripherals will not conflict with other cores running at the same time.
//!
//! If this overhead is not desired, or if you know for sure that these functions are only being ran on 1 core, please use [`bcm2837_pac::Peripherals`] directly
//!
//! [`AtomicBool`]: core::sync::atomic::AtomicBool

macro_rules! expose_peripherals {
    (
        $($p:ident {
            $($field:ident : $field_ty:ty,)*
        })*
    ) => {
        $(
            #[allow(non_snake_case)]
            pub mod $p {
                $(
                    mod $field {
                        use core::sync::atomic::{AtomicBool, Ordering};
                        use bcm2837_pac::generic::Reg;

                        static LOCKED: AtomicBool = AtomicBool::new(false);
                        #[allow(dead_code)]
                        pub fn $field<F, R>(cb: F) -> R
                            where F: FnOnce(&Reg<$field_ty>) -> R {
                            while LOCKED.compare_exchange(
                                false,
                                true,
                                Ordering::Relaxed,
                                Ordering::Relaxed
                            ).is_err() {
                                cortex_a::asm::nop();
                            }
                            struct Lock;
                            impl Drop for Lock {
                                fn drop(&mut self) {
                                    LOCKED.store(false, Ordering::Relaxed);
                                }
                            }
                            let lock = Lock;
                            let peripherals = unsafe { bcm2837_pac::Peripherals::steal() };
                            let result = cb(&peripherals.$p.$field);
                            drop(lock);
                            result
                        }
                    }
                    pub use $field::$field;
                )*
            }
        )*
    }
}

expose_peripherals! {
    AUX {
        irq: bcm2837_pac::aux::irq::IRQ_SPEC,
        enables: bcm2837_pac::aux::enables::ENABLES_SPEC,
    }
    GPIO {
        gpfsel0: bcm2837_pac::gpio::gpfsel0::GPFSEL0_SPEC,
        gpfsel1: bcm2837_pac::gpio::gpfsel1::GPFSEL1_SPEC,
        gpfsel2: bcm2837_pac::gpio::gpfsel2::GPFSEL2_SPEC,
        gpfsel3: bcm2837_pac::gpio::gpfsel3::GPFSEL3_SPEC,
        gpfsel4: bcm2837_pac::gpio::gpfsel4::GPFSEL4_SPEC,
        gpfsel5: bcm2837_pac::gpio::gpfsel5::GPFSEL5_SPEC,
    }
}
