#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control / Status"]
    pub cs: crate::Reg<cs::CS_SPEC>,
    #[doc = "0x04 - Clock divisor"]
    pub div: crate::Reg<div::DIV_SPEC>,
}
#[doc = "CS register accessor: an alias for `Reg<CS_SPEC>`"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "Control / Status"]
pub mod cs;
#[doc = "DIV register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Clock divisor"]
pub mod div;
