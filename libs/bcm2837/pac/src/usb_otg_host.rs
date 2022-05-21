#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    pub hcfg: crate::Reg<hcfg::HCFG_SPEC>,
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    pub hfir: crate::Reg<hfir::HFIR_SPEC>,
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    pub hfnum: crate::Reg<hfnum::HFNUM_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Host periodic transmit FIFO/queue status register"]
    pub hptxsts: crate::Reg<hptxsts::HPTXSTS_SPEC>,
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    pub haint: crate::Reg<haint::HAINT_SPEC>,
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    pub haintmsk: crate::Reg<haintmsk::HAINTMSK_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - OTG_HS host port control and status register"]
    pub hprt: crate::Reg<hprt::HPRT_SPEC>,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100 - Host channel %s"]
    pub host_channel: crate::ArrayProxy<HOST_CHANNEL, 12, 0x20>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct HOST_CHANNEL {
    #[doc = "0x00 - Characteristics register"]
    pub hcchar: crate::Reg<self::host_channel::hcchar::HCCHAR_SPEC>,
    #[doc = "0x04 - Split control register"]
    pub hcsplt: crate::Reg<self::host_channel::hcsplt::HCSPLT_SPEC>,
    #[doc = "0x08 - Interrupt register"]
    pub hcint: crate::Reg<self::host_channel::hcint::HCINT_SPEC>,
    #[doc = "0x0c - Interrupt mask"]
    pub hcintmsk: crate::Reg<self::host_channel::hcintmsk::HCINTMSK_SPEC>,
    #[doc = "0x10 - Transfer size"]
    pub hctsiz: crate::Reg<self::host_channel::hctsiz::HCTSIZ_SPEC>,
    #[doc = "0x14 - DMA address"]
    pub hcdma: crate::Reg<self::host_channel::hcdma::HCDMA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Host channel %s"]
pub mod host_channel;
#[doc = "HCFG register accessor: an alias for `Reg<HCFG_SPEC>`"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "OTG_HS host configuration register"]
pub mod hcfg;
#[doc = "HFIR register accessor: an alias for `Reg<HFIR_SPEC>`"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "OTG_HS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM register accessor: an alias for `Reg<HFNUM_SPEC>`"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod hfnum;
#[doc = "HPTXSTS register accessor: an alias for `Reg<HPTXSTS_SPEC>`"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "Host periodic transmit FIFO/queue status register"]
pub mod hptxsts;
#[doc = "HAINT register accessor: an alias for `Reg<HAINT_SPEC>`"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK register accessor: an alias for `Reg<HAINTMSK_SPEC>`"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT register accessor: an alias for `Reg<HPRT_SPEC>`"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "OTG_HS host port control and status register"]
pub mod hprt;
