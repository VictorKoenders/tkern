#[doc = "Register `HCSPLT` reader"]
pub struct R(crate::R<HCSPLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCSPLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCSPLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCSPLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCSPLT` writer"]
pub struct W(crate::W<HCSPLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCSPLT_SPEC>;
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
impl From<crate::W<HCSPLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCSPLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRTADDR` reader - Port address"]
pub type PRTADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRTADDR` writer - Port address"]
pub type PRTADDR_W<'a> = crate::FieldWriter<'a, u32, HCSPLT_SPEC, u8, u8, 7, 0>;
#[doc = "Field `HUBADDR` reader - Hub address"]
pub type HUBADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HUBADDR` writer - Hub address"]
pub type HUBADDR_W<'a> = crate::FieldWriter<'a, u32, HCSPLT_SPEC, u8, u8, 7, 7>;
#[doc = "Field `XACTPOS` reader - XACTPOS"]
pub type XACTPOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XACTPOS` writer - XACTPOS"]
pub type XACTPOS_W<'a> = crate::FieldWriter<'a, u32, HCSPLT_SPEC, u8, u8, 2, 14>;
#[doc = "Field `COMPLSPLT` reader - Do complete split"]
pub type COMPLSPLT_R = crate::BitReader<bool>;
#[doc = "Field `COMPLSPLT` writer - Do complete split"]
pub type COMPLSPLT_W<'a> = crate::BitWriter<'a, u32, HCSPLT_SPEC, bool, 16>;
#[doc = "Field `SPLITEN` reader - Split enable"]
pub type SPLITEN_R = crate::BitReader<bool>;
#[doc = "Field `SPLITEN` writer - Split enable"]
pub type SPLITEN_W<'a> = crate::BitWriter<'a, u32, HCSPLT_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PRTADDR_W {
        PRTADDR_W::new(self)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HUBADDR_W {
        HUBADDR_W::new(self)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&mut self) -> XACTPOS_W {
        XACTPOS_W::new(self)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&mut self) -> COMPLSPLT_W {
        COMPLSPLT_W::new(self)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&mut self) -> SPLITEN_W {
        SPLITEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt](index.html) module"]
pub struct HCSPLT_SPEC;
impl crate::RegisterSpec for HCSPLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcsplt::R](R) reader structure"]
impl crate::Readable for HCSPLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcsplt::W](W) writer structure"]
impl crate::Writable for HCSPLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCSPLT to value 0"]
impl crate::Resettable for HCSPLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}