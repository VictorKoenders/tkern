#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    _reserved_1_ecr: [u8; 0x04],
    _reserved2: [u8; 0x10],
    #[doc = "0x18 - Flag Register"]
    pub fr: crate::Reg<fr::FR_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x24 - Integer Baud Rate Register"]
    pub ibrd: crate::Reg<ibrd::IBRD_SPEC>,
    #[doc = "0x28 - Fractional Baud Rate Register"]
    pub fbrd: crate::Reg<fbrd::FBRD_SPEC>,
    #[doc = "0x2c - Line Control Register"]
    pub lcr_h: crate::Reg<lcr_h::LCR_H_SPEC>,
    #[doc = "0x30 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x34 - Interrupt FIFO Level Select Register"]
    pub ifls: crate::Reg<ifls::IFLS_SPEC>,
    #[doc = "0x38 - Interrupt Mask set_Clear Register"]
    pub imsc: crate::Reg<imsc::IMSC_SPEC>,
    #[doc = "0x3c - Raw Interrupt Status Register"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x40 - Masked Interrupt Status Register"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x44 - Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x48 - DMA Control Register"]
    pub dmacr: crate::Reg<dmacr::DMACR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x04 - Error Clear Register"]
    #[inline(always)]
    pub fn ecr(&self) -> &crate::Reg<ecr::ECR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<ecr::ECR_SPEC>)
        }
    }
    #[doc = "0x04 - Receive Status Register"]
    #[inline(always)]
    pub fn rsr(&self) -> &crate::Reg<rsr::RSR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<rsr::RSR_SPEC>)
        }
    }
}
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data Register"]
pub mod dr;
#[doc = "RSR register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ECR register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Clear Register"]
pub mod ecr;
#[doc = "FR register accessor: an alias for `Reg<FR_SPEC>`"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Flag Register"]
pub mod fr;
#[doc = "IBRD register accessor: an alias for `Reg<IBRD_SPEC>`"]
pub type IBRD = crate::Reg<ibrd::IBRD_SPEC>;
#[doc = "Integer Baud Rate Register"]
pub mod ibrd;
#[doc = "FBRD register accessor: an alias for `Reg<FBRD_SPEC>`"]
pub type FBRD = crate::Reg<fbrd::FBRD_SPEC>;
#[doc = "Fractional Baud Rate Register"]
pub mod fbrd;
#[doc = "LCR_H register accessor: an alias for `Reg<LCR_H_SPEC>`"]
pub type LCR_H = crate::Reg<lcr_h::LCR_H_SPEC>;
#[doc = "Line Control Register"]
pub mod lcr_h;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "IFLS register accessor: an alias for `Reg<IFLS_SPEC>`"]
pub type IFLS = crate::Reg<ifls::IFLS_SPEC>;
#[doc = "Interrupt FIFO Level Select Register"]
pub mod ifls;
#[doc = "IMSC register accessor: an alias for `Reg<IMSC_SPEC>`"]
pub type IMSC = crate::Reg<imsc::IMSC_SPEC>;
#[doc = "Interrupt Mask set_Clear Register"]
pub mod imsc;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status Register"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "DMACR register accessor: an alias for `Reg<DMACR_SPEC>`"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "DMA Control Register"]
pub mod dmacr;
