#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIMIC` writer - RIMIC"]
pub type RIMIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 0>;
#[doc = "Field `CTSMIC` writer - CTSMIC"]
pub type CTSMIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 1>;
#[doc = "Field `DCDMIC` writer - DCDMIC"]
pub type DCDMIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 2>;
#[doc = "Field `DSRMIC` writer - DSRMIC"]
pub type DSRMIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 3>;
#[doc = "Field `RXIC` writer - RXIC"]
pub type RXIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 4>;
#[doc = "Field `TXIC` writer - TXIC"]
pub type TXIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 5>;
#[doc = "Field `RTIC` writer - RTIC"]
pub type RTIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 6>;
#[doc = "Field `FEIC` writer - FEIC"]
pub type FEIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 7>;
#[doc = "Field `PEIC` writer - PEIC"]
pub type PEIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 8>;
#[doc = "Field `BEIC` writer - BEIC"]
pub type BEIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 9>;
#[doc = "Field `OEIC` writer - OEIC"]
pub type OEIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 10>;
impl W {
    #[doc = "Bit 0 - RIMIC"]
    #[inline(always)]
    pub fn rimic(&mut self) -> RIMIC_W {
        RIMIC_W::new(self)
    }
    #[doc = "Bit 1 - CTSMIC"]
    #[inline(always)]
    pub fn ctsmic(&mut self) -> CTSMIC_W {
        CTSMIC_W::new(self)
    }
    #[doc = "Bit 2 - DCDMIC"]
    #[inline(always)]
    pub fn dcdmic(&mut self) -> DCDMIC_W {
        DCDMIC_W::new(self)
    }
    #[doc = "Bit 3 - DSRMIC"]
    #[inline(always)]
    pub fn dsrmic(&mut self) -> DSRMIC_W {
        DSRMIC_W::new(self)
    }
    #[doc = "Bit 4 - RXIC"]
    #[inline(always)]
    pub fn rxic(&mut self) -> RXIC_W {
        RXIC_W::new(self)
    }
    #[doc = "Bit 5 - TXIC"]
    #[inline(always)]
    pub fn txic(&mut self) -> TXIC_W {
        TXIC_W::new(self)
    }
    #[doc = "Bit 6 - RTIC"]
    #[inline(always)]
    pub fn rtic(&mut self) -> RTIC_W {
        RTIC_W::new(self)
    }
    #[doc = "Bit 7 - FEIC"]
    #[inline(always)]
    pub fn feic(&mut self) -> FEIC_W {
        FEIC_W::new(self)
    }
    #[doc = "Bit 8 - PEIC"]
    #[inline(always)]
    pub fn peic(&mut self) -> PEIC_W {
        PEIC_W::new(self)
    }
    #[doc = "Bit 9 - BEIC"]
    #[inline(always)]
    pub fn beic(&mut self) -> BEIC_W {
        BEIC_W::new(self)
    }
    #[doc = "Bit 10 - OEIC"]
    #[inline(always)]
    pub fn oeic(&mut self) -> OEIC_W {
        OEIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
