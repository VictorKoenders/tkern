#[doc = "Register `DIV` reader"]
pub struct R(crate::R<DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV` writer"]
pub struct W(crate::W<DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_SPEC>;
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
impl From<crate::W<DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Password. Always 0x5a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PASSWD_AW {
    #[doc = "90: `1011010`"]
    PASSWD = 90,
}
impl From<PASSWD_AW> for u8 {
    #[inline(always)]
    fn from(variant: PASSWD_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PASSWD` writer - Password. Always 0x5a"]
pub type PASSWD_W<'a> = crate::FieldWriter<'a, u32, DIV_SPEC, u8, PASSWD_AW, 8, 24>;
impl<'a> PASSWD_W<'a> {
    #[doc = "`1011010`"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(PASSWD_AW::PASSWD)
    }
}
#[doc = "Field `DIVI` reader - Integer part of divisor"]
pub type DIVI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIVI` writer - Integer part of divisor"]
pub type DIVI_W<'a> = crate::FieldWriter<'a, u32, DIV_SPEC, u16, u16, 12, 12>;
#[doc = "Field `DIVF` reader - Fractional part of divisor"]
pub type DIVF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIVF` writer - Fractional part of divisor"]
pub type DIVF_W<'a> = crate::FieldWriter<'a, u32, DIV_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 12:23 - Integer part of divisor"]
    #[inline(always)]
    pub fn divi(&self) -> DIVI_R {
        DIVI_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - Fractional part of divisor"]
    #[inline(always)]
    pub fn divf(&self) -> DIVF_R {
        DIVF_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    pub fn passwd(&mut self) -> PASSWD_W {
        PASSWD_W::new(self)
    }
    #[doc = "Bits 12:23 - Integer part of divisor"]
    #[inline(always)]
    pub fn divi(&mut self) -> DIVI_W {
        DIVI_W::new(self)
    }
    #[doc = "Bits 0:11 - Fractional part of divisor"]
    #[inline(always)]
    pub fn divf(&mut self) -> DIVF_W {
        DIVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div](index.html) module"]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div::R](R) reader structure"]
impl crate::Readable for DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div::W](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
