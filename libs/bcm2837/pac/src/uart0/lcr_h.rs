#[doc = "Register `LCR_H` reader"]
pub struct R(crate::R<LCR_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR_H` writer"]
pub struct W(crate::W<LCR_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_H_SPEC>;
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
impl From<crate::W<LCR_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRK` reader - BRK"]
pub type BRK_R = crate::BitReader<bool>;
#[doc = "Field `BRK` writer - BRK"]
pub type BRK_W<'a> = crate::BitWriter<'a, u32, LCR_H_SPEC, bool, 0>;
#[doc = "Field `PEN` reader - PEN"]
pub type PEN_R = crate::BitReader<bool>;
#[doc = "Field `PEN` writer - PEN"]
pub type PEN_W<'a> = crate::BitWriter<'a, u32, LCR_H_SPEC, bool, 1>;
#[doc = "Field `EPS` reader - EPS"]
pub type EPS_R = crate::BitReader<bool>;
#[doc = "Field `EPS` writer - EPS"]
pub type EPS_W<'a> = crate::BitWriter<'a, u32, LCR_H_SPEC, bool, 2>;
#[doc = "Field `STP2` reader - STP2"]
pub type STP2_R = crate::BitReader<bool>;
#[doc = "Field `STP2` writer - STP2"]
pub type STP2_W<'a> = crate::BitWriter<'a, u32, LCR_H_SPEC, bool, 3>;
#[doc = "Field `FEN` reader - FEN"]
pub type FEN_R = crate::BitReader<bool>;
#[doc = "Field `FEN` writer - FEN"]
pub type FEN_W<'a> = crate::BitWriter<'a, u32, LCR_H_SPEC, bool, 4>;
#[doc = "Field `WLEN` reader - WLEN"]
pub type WLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WLEN` writer - WLEN"]
pub type WLEN_W<'a> = crate::FieldWriter<'a, u32, LCR_H_SPEC, u8, u8, 2, 5>;
#[doc = "Field `SPS` reader - SPS"]
pub type SPS_R = crate::BitReader<bool>;
#[doc = "Field `SPS` writer - SPS"]
pub type SPS_W<'a> = crate::BitWriter<'a, u32, LCR_H_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - BRK"]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEN"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EPS"]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STP2"]
    #[inline(always)]
    pub fn stp2(&self) -> STP2_R {
        STP2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FEN"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - WLEN"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - SPS"]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK"]
    #[inline(always)]
    pub fn brk(&mut self) -> BRK_W {
        BRK_W::new(self)
    }
    #[doc = "Bit 1 - PEN"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W::new(self)
    }
    #[doc = "Bit 2 - EPS"]
    #[inline(always)]
    pub fn eps(&mut self) -> EPS_W {
        EPS_W::new(self)
    }
    #[doc = "Bit 3 - STP2"]
    #[inline(always)]
    pub fn stp2(&mut self) -> STP2_W {
        STP2_W::new(self)
    }
    #[doc = "Bit 4 - FEN"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W::new(self)
    }
    #[doc = "Bits 5:6 - WLEN"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W::new(self)
    }
    #[doc = "Bit 7 - SPS"]
    #[inline(always)]
    pub fn sps(&mut self) -> SPS_W {
        SPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr_h](index.html) module"]
pub struct LCR_H_SPEC;
impl crate::RegisterSpec for LCR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcr_h::R](R) reader structure"]
impl crate::Readable for LCR_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr_h::W](W) writer structure"]
impl crate::Writable for LCR_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCR_H to value 0"]
impl crate::Resettable for LCR_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
