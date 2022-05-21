#[doc = "Register `FR` reader"]
pub struct R(crate::R<FR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FR` writer"]
pub struct W(crate::W<FR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FR_SPEC>;
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
impl From<crate::W<FR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTS` reader - CTS"]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `CTS` writer - CTS"]
pub type CTS_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 0>;
#[doc = "Field `DSR` reader - DSR"]
pub type DSR_R = crate::BitReader<bool>;
#[doc = "Field `DSR` writer - DSR"]
pub type DSR_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 1>;
#[doc = "Field `DCD` reader - DCD"]
pub type DCD_R = crate::BitReader<bool>;
#[doc = "Field `DCD` writer - DCD"]
pub type DCD_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 2>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BUSY_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 3>;
#[doc = "Field `RXFE` reader - RXFE"]
pub type RXFE_R = crate::BitReader<bool>;
#[doc = "Field `RXFE` writer - RXFE"]
pub type RXFE_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 4>;
#[doc = "Field `TXFF` reader - TXFF"]
pub type TXFF_R = crate::BitReader<bool>;
#[doc = "Field `TXFF` writer - TXFF"]
pub type TXFF_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 5>;
#[doc = "Field `RXFF` reader - RXFF"]
pub type RXFF_R = crate::BitReader<bool>;
#[doc = "Field `RXFF` writer - RXFF"]
pub type RXFF_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 6>;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TXFE_R = crate::BitReader<bool>;
#[doc = "Field `TXFE` writer - TXFE"]
pub type TXFE_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 7>;
#[doc = "Field `RI` reader - RI"]
pub type RI_R = crate::BitReader<bool>;
#[doc = "Field `RI` writer - RI"]
pub type RI_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSR"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCD"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFE"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFF"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXFF"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RI"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTS"]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W {
        CTS_W::new(self)
    }
    #[doc = "Bit 1 - DSR"]
    #[inline(always)]
    pub fn dsr(&mut self) -> DSR_W {
        DSR_W::new(self)
    }
    #[doc = "Bit 2 - DCD"]
    #[inline(always)]
    pub fn dcd(&mut self) -> DCD_W {
        DCD_W::new(self)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W::new(self)
    }
    #[doc = "Bit 4 - RXFE"]
    #[inline(always)]
    pub fn rxfe(&mut self) -> RXFE_W {
        RXFE_W::new(self)
    }
    #[doc = "Bit 5 - TXFF"]
    #[inline(always)]
    pub fn txff(&mut self) -> TXFF_W {
        TXFF_W::new(self)
    }
    #[doc = "Bit 6 - RXFF"]
    #[inline(always)]
    pub fn rxff(&mut self) -> RXFF_W {
        RXFF_W::new(self)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&mut self) -> TXFE_W {
        TXFE_W::new(self)
    }
    #[doc = "Bit 8 - RI"]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](index.html) module"]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr::R](R) reader structure"]
impl crate::Readable for FR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fr::W](W) writer structure"]
impl crate::Writable for FR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
