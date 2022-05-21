#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - Status"]
    pub sta: crate::Reg<sta::STA_SPEC>,
    #[doc = "0x08 - DMA control"]
    pub dmac: crate::Reg<dmac::DMAC_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Range for channel 1"]
    pub rng1: crate::Reg<rng1::RNG1_SPEC>,
    #[doc = "0x14 - Channel 1 data"]
    pub dat1: crate::Reg<dat1::DAT1_SPEC>,
    #[doc = "0x18 - FIFO input"]
    pub fif1: crate::Reg<fif1::FIF1_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Range for channel 2"]
    pub rng2: crate::Reg<rng2::RNG2_SPEC>,
    #[doc = "0x24 - Channel 2 data"]
    pub dat2: crate::Reg<dat2::DAT2_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STA register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Status"]
pub mod sta;
#[doc = "DMAC register accessor: an alias for `Reg<DMAC_SPEC>`"]
pub type DMAC = crate::Reg<dmac::DMAC_SPEC>;
#[doc = "DMA control"]
pub mod dmac;
#[doc = "RNG1 register accessor: an alias for `Reg<RNG1_SPEC>`"]
pub type RNG1 = crate::Reg<rng1::RNG1_SPEC>;
#[doc = "Range for channel 1"]
pub mod rng1;
#[doc = "DAT1 register accessor: an alias for `Reg<DAT1_SPEC>`"]
pub type DAT1 = crate::Reg<dat1::DAT1_SPEC>;
#[doc = "Channel 1 data"]
pub mod dat1;
#[doc = "FIF1 register accessor: an alias for `Reg<FIF1_SPEC>`"]
pub type FIF1 = crate::Reg<fif1::FIF1_SPEC>;
#[doc = "FIFO input"]
pub mod fif1;
#[doc = "RNG2 register accessor: an alias for `Reg<RNG2_SPEC>`"]
pub type RNG2 = crate::Reg<rng2::RNG2_SPEC>;
#[doc = "Range for channel 2"]
pub mod rng2;
#[doc = "DAT2 register accessor: an alias for `Reg<DAT2_SPEC>`"]
pub type DAT2 = crate::Reg<dat2::DAT2_SPEC>;
#[doc = "Channel 2 data"]
pub mod dat2;
