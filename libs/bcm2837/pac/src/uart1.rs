#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_io: [u8; 0x04],
    _reserved_1_ier: [u8; 0x04],
    #[doc = "0x08 - Interrupt Identify"]
    pub iir: crate::Reg<iir::IIR_SPEC>,
    #[doc = "0x0c - Line control"]
    pub lcr: crate::Reg<lcr::LCR_SPEC>,
    #[doc = "0x10 - Modem Control"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x14 - Line Status"]
    pub lsr: crate::Reg<lsr::LSR_SPEC>,
    #[doc = "0x18 - Modem Status"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
    #[doc = "0x1c - Scratch"]
    pub scratch: crate::Reg<scratch::SCRATCH_SPEC>,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - Control"]
    pub cntl: crate::Reg<cntl::CNTL_SPEC>,
    #[doc = "0x24 - Status"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x28 - Baudrate"]
    pub baud: crate::Reg<baud::BAUD_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - Lower bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub fn baudl(&self) -> &crate::Reg<baudl::BAUDL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<baudl::BAUDL_SPEC>)
        }
    }
    #[doc = "0x00 - I/O Data"]
    #[inline(always)]
    pub fn io(&self) -> &crate::Reg<io::IO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<io::IO_SPEC>)
        }
    }
    #[doc = "0x04 - High bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub fn baudh(&self) -> &crate::Reg<baudh::BAUDH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<baudh::BAUDH_SPEC>)
        }
    }
    #[doc = "0x04 - Interrupt Enable"]
    #[inline(always)]
    pub fn ier(&self) -> &crate::Reg<ier::IER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<ier::IER_SPEC>)
        }
    }
}
#[doc = "IO register accessor: an alias for `Reg<IO_SPEC>`"]
pub type IO = crate::Reg<io::IO_SPEC>;
#[doc = "I/O Data"]
pub mod io;
#[doc = "BAUDL register accessor: an alias for `Reg<BAUDL_SPEC>`"]
pub type BAUDL = crate::Reg<baudl::BAUDL_SPEC>;
#[doc = "Lower bits of baudrate when DLAB is set"]
pub mod baudl;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "BAUDH register accessor: an alias for `Reg<BAUDH_SPEC>`"]
pub type BAUDH = crate::Reg<baudh::BAUDH_SPEC>;
#[doc = "High bits of baudrate when DLAB is set"]
pub mod baudh;
#[doc = "IIR register accessor: an alias for `Reg<IIR_SPEC>`"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "Interrupt Identify"]
pub mod iir;
#[doc = "LCR register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "Line control"]
pub mod lcr;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Modem Control"]
pub mod mcr;
#[doc = "LSR register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status"]
pub mod lsr;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Modem Status"]
pub mod msr;
#[doc = "SCRATCH register accessor: an alias for `Reg<SCRATCH_SPEC>`"]
pub type SCRATCH = crate::Reg<scratch::SCRATCH_SPEC>;
#[doc = "Scratch"]
pub mod scratch;
#[doc = "CNTL register accessor: an alias for `Reg<CNTL_SPEC>`"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "Control"]
pub mod cntl;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status"]
pub mod stat;
#[doc = "BAUD register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baudrate"]
pub mod baud;
