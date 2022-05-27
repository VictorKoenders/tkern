#[doc = "Register `STATUS1` reader"]
pub struct R(crate::R<STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS1` writer"]
pub struct W(crate::W<STATUS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS1_SPEC>;
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
impl From<crate::W<STATUS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS1_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status1](index.html) module"]
pub struct STATUS1_SPEC;
impl crate::RegisterSpec for STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status1::R](R) reader structure"]
impl crate::Readable for STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status1::W](W) writer structure"]
impl crate::Writable for STATUS1_SPEC {
    type Writer = W;
}