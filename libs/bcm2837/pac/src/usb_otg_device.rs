#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    pub dcfg: crate::Reg<dcfg::DCFG_SPEC>,
    #[doc = "0x04 - OTG_HS device control register"]
    pub dctl: crate::Reg<dctl::DCTL_SPEC>,
    #[doc = "0x08 - OTG_HS device status register"]
    pub dsts: crate::Reg<dsts::DSTS_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    pub diepmsk: crate::Reg<diepmsk::DIEPMSK_SPEC>,
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    pub doepmsk: crate::Reg<doepmsk::DOEPMSK_SPEC>,
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    pub daint: crate::Reg<daint::DAINT_SPEC>,
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    pub daintmsk: crate::Reg<daintmsk::DAINTMSK_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    pub dvbusdis: crate::Reg<dvbusdis::DVBUSDIS_SPEC>,
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    pub dvbuspulse: crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>,
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    pub dthrctl: crate::Reg<dthrctl::DTHRCTL_SPEC>,
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>,
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    pub deachint: crate::Reg<deachint::DEACHINT_SPEC>,
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    pub deachintmsk: crate::Reg<deachintmsk::DEACHINTMSK_SPEC>,
    #[doc = "0x40 - OTG_HS device each in endpoint-1 interrupt register"]
    pub diepeachmsk1: crate::Reg<diepeachmsk1::DIEPEACHMSK1_SPEC>,
    _reserved14: [u8; 0x3c],
    #[doc = "0x80 - OTG_HS device each OUT endpoint-1 interrupt register"]
    pub doepeachmsk1: crate::Reg<doepeachmsk1::DOEPEACHMSK1_SPEC>,
    _reserved15: [u8; 0x7c],
    #[doc = "0x100 - IN Endpoint %s"]
    pub in_endpoint: crate::ArrayProxy<IN_ENDPOINT, 12, 0x20>,
    _reserved16: [u8; 0x0200],
    #[doc = "0x300 - OUT Endpoint %s"]
    pub out_endpoint: crate::ArrayProxy<OUT_ENDPOINT, 12, 0x20>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct IN_ENDPOINT {
    #[doc = "0x00 - Control"]
    pub diepctl0: crate::Reg<self::in_endpoint::diepctl0::DIEPCTL0_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Interrupt"]
    pub diepint: crate::Reg<self::in_endpoint::diepint::DIEPINT_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Transfer size"]
    pub dieptsiz: crate::Reg<self::in_endpoint::dieptsiz::DIEPTSIZ_SPEC>,
    #[doc = "0x14 - DMA address"]
    pub diepdma: crate::Reg<self::in_endpoint::diepdma::DIEPDMA_SPEC>,
    #[doc = "0x18 - Transmit FIFO status"]
    pub dtxfsts: crate::Reg<self::in_endpoint::dtxfsts::DTXFSTS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "IN Endpoint %s"]
pub mod in_endpoint;
#[doc = r"Register block"]
#[repr(C)]
pub struct OUT_ENDPOINT {
    #[doc = "0x00 - Control"]
    pub doepctl: crate::Reg<self::out_endpoint::doepctl::DOEPCTL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Interrupt"]
    pub doepint: crate::Reg<self::out_endpoint::doepint::DOEPINT_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Transfer size"]
    pub doeptsiz: crate::Reg<self::out_endpoint::doeptsiz::DOEPTSIZ_SPEC>,
    #[doc = "0x14 - DMA address"]
    pub doepdma: crate::Reg<self::out_endpoint::doepdma::DOEPDMA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "OUT Endpoint %s"]
pub mod out_endpoint;
#[doc = "DCFG register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "OTG_HS device configuration register"]
pub mod dcfg;
#[doc = "DCTL register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "OTG_HS device control register"]
pub mod dctl;
#[doc = "DSTS register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "OTG_HS device status register"]
pub mod dsts;
#[doc = "DIEPMSK register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod diepmsk;
#[doc = "DOEPMSK register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod doepmsk;
#[doc = "DAINT register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod daint;
#[doc = "DAINTMSK register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod daintmsk;
#[doc = "DVBUSDIS register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL register accessor: an alias for `Reg<DTHRCTL_SPEC>`"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = "OTG_HS Device threshold control register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DEACHINT register accessor: an alias for `Reg<DEACHINT_SPEC>`"]
pub type DEACHINT = crate::Reg<deachint::DEACHINT_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod deachint;
#[doc = "DEACHINTMSK register accessor: an alias for `Reg<DEACHINTMSK_SPEC>`"]
pub type DEACHINTMSK = crate::Reg<deachintmsk::DEACHINTMSK_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod deachintmsk;
#[doc = "DIEPEACHMSK1 register accessor: an alias for `Reg<DIEPEACHMSK1_SPEC>`"]
pub type DIEPEACHMSK1 = crate::Reg<diepeachmsk1::DIEPEACHMSK1_SPEC>;
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub mod diepeachmsk1;
#[doc = "DOEPEACHMSK1 register accessor: an alias for `Reg<DOEPEACHMSK1_SPEC>`"]
pub type DOEPEACHMSK1 = crate::Reg<doepeachmsk1::DOEPEACHMSK1_SPEC>;
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register"]
pub mod doepeachmsk1;
