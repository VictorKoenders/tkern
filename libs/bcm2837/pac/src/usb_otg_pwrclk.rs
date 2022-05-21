#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power and clock gating control"]
    pub pcgcctl: crate::Reg<pcgcctl::PCGCCTL_SPEC>,
}
#[doc = "PCGCCTL register accessor: an alias for `Reg<PCGCCTL_SPEC>`"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "power and clock gating control"]
pub mod pcgcctl;
