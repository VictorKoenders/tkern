#[doc = "Register `IIR` reader"]
pub struct R(crate::R<IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIR` writer"]
pub struct W(crate::W<IIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIR_SPEC>;
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
impl From<crate::W<IIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_READY` reader - Transmit FIFO is empty"]
pub type TX_READY_R = crate::BitReader<bool>;
#[doc = "Field `TX_READY` writer - Transmit FIFO is empty"]
pub type TX_READY_W<'a> = crate::BitWriter<'a, u32, IIR_SPEC, bool, 2>;
#[doc = "Field `DATA_READY` reader - Receive FIFO has at least 1 byte"]
pub type DATA_READY_R = crate::BitReader<bool>;
#[doc = "Field `DATA_READY` writer - Receive FIFO has at least 1 byte"]
pub type DATA_READY_W<'a> = crate::BitWriter<'a, u32, IIR_SPEC, bool, 1>;
#[doc = "Field `nPENDING` reader - No pending interrupt"]
pub type NPENDING_R = crate::BitReader<bool>;
#[doc = "Field `nPENDING` writer - No pending interrupt"]
pub type NPENDING_W<'a> = crate::BitWriter<'a, u32, IIR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 2 - Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO has at least 1 byte"]
    #[inline(always)]
    pub fn data_ready(&self) -> DATA_READY_R {
        DATA_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - No pending interrupt"]
    #[inline(always)]
    pub fn n_pending(&self) -> NPENDING_R {
        NPENDING_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tx_ready(&mut self) -> TX_READY_W {
        TX_READY_W::new(self)
    }
    #[doc = "Bit 1 - Receive FIFO has at least 1 byte"]
    #[inline(always)]
    pub fn data_ready(&mut self) -> DATA_READY_W {
        DATA_READY_W::new(self)
    }
    #[doc = "Bit 0 - No pending interrupt"]
    #[inline(always)]
    pub fn n_pending(&mut self) -> NPENDING_W {
        NPENDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Identify\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir](index.html) module"]
pub struct IIR_SPEC;
impl crate::RegisterSpec for IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iir::R](R) reader structure"]
impl crate::Readable for IIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iir::W](W) writer structure"]
impl crate::Writable for IIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IIR to value 0xb001"]
impl crate::Resettable for IIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb001
    }
}
