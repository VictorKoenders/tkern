#[doc = "Register `DISABLE_BASIC` reader"]
pub struct R(crate::R<DISABLE_BASIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISABLE_BASIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISABLE_BASIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISABLE_BASIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISABLE_BASIC` writer"]
pub struct W(crate::W<DISABLE_BASIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISABLE_BASIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DISABLE_BASIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISABLE_BASIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TIMER_R = crate::BitReader<bool>;
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TIMER_W<'a> = crate::BitWriter1C<'a, u32, DISABLE_BASIC_SPEC, bool, 0>;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MAILBOX_R = crate::BitReader<bool>;
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MAILBOX_W<'a> = crate::BitWriter1C<'a, u32, DISABLE_BASIC_SPEC, bool, 1>;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type DOORBELL0_R = crate::BitReader<bool>;
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type DOORBELL0_W<'a> = crate::BitWriter1C<'a, u32, DISABLE_BASIC_SPEC, bool, 2>;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type DOORBELL1_R = crate::BitReader<bool>;
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type DOORBELL1_W<'a> = crate::BitWriter1C<'a, u32, DISABLE_BASIC_SPEC, bool, 3>;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type VPU0_HALTED_R = crate::BitReader<bool>;
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type VPU0_HALTED_W<'a> = crate::BitWriter1C<'a, u32, DISABLE_BASIC_SPEC, bool, 4>;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type VPU1_HALTED_R = crate::BitReader<bool>;
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type VPU1_HALTED_W<'a> = crate::BitWriter1C<'a, u32, DISABLE_BASIC_SPEC, bool, 5>;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ARM_ADDRESS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ARM_ADDRESS_ERROR_W<'a> = crate::BitWriter1C<'a, u32, DISABLE_BASIC_SPEC, bool, 6>;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ARM_AXI_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ARM_AXI_ERROR_W<'a> = crate::BitWriter1C<'a, u32, DISABLE_BASIC_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&self) -> MAILBOX_R {
        MAILBOX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&self) -> DOORBELL0_R {
        DOORBELL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&self) -> DOORBELL1_R {
        DOORBELL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&self) -> VPU0_HALTED_R {
        VPU0_HALTED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&self) -> VPU1_HALTED_R {
        VPU1_HALTED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&self) -> ARM_ADDRESS_ERROR_R {
        ARM_ADDRESS_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&self) -> ARM_AXI_ERROR_R {
        ARM_AXI_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W {
        TIMER_W::new(self)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&mut self) -> MAILBOX_W {
        MAILBOX_W::new(self)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&mut self) -> DOORBELL0_W {
        DOORBELL0_W::new(self)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&mut self) -> DOORBELL1_W {
        DOORBELL1_W::new(self)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&mut self) -> VPU0_HALTED_W {
        VPU0_HALTED_W::new(self)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&mut self) -> VPU1_HALTED_W {
        VPU1_HALTED_W::new(self)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&mut self) -> ARM_ADDRESS_ERROR_W {
        ARM_ADDRESS_ERROR_W::new(self)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&mut self) -> ARM_AXI_ERROR_W {
        ARM_AXI_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable basic interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disable_basic](index.html) module"]
pub struct DISABLE_BASIC_SPEC;
impl crate::RegisterSpec for DISABLE_BASIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [disable_basic::R](R) reader structure"]
impl crate::Readable for DISABLE_BASIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [disable_basic::W](W) writer structure"]
impl crate::Writable for DISABLE_BASIC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DISABLE_BASIC to value 0"]
impl crate::Resettable for DISABLE_BASIC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
