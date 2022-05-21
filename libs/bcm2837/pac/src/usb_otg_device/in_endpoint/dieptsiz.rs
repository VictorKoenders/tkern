#[doc = "Register `DIEPTSIZ` reader"]
pub struct R(crate::R<DIEPTSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTSIZ` writer"]
pub struct W(crate::W<DIEPTSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTSIZ_SPEC>;
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
impl From<crate::W<DIEPTSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XFRSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XFRSIZ_W<'a> = crate::FieldWriter<'a, u32, DIEPTSIZ_SPEC, u8, u8, 7, 0>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a> = crate::FieldWriter<'a, u32, DIEPTSIZ_SPEC, u8, u8, 2, 19>;
impl R {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W {
        XFRSIZ_W::new(self)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz](index.html) module"]
pub struct DIEPTSIZ_SPEC;
impl crate::RegisterSpec for DIEPTSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptsiz::R](R) reader structure"]
impl crate::Readable for DIEPTSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptsiz::W](W) writer structure"]
impl crate::Writable for DIEPTSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPTSIZ to value 0"]
impl crate::Resettable for DIEPTSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
