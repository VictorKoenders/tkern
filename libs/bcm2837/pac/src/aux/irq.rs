#[doc = "Register `IRQ` reader"]
pub struct R(crate::R<IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ` writer"]
pub struct W(crate::W<IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_SPEC>;
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
impl From<crate::W<IRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_2` reader - SPI2 interrupt active"]
pub type SPI_2_R = crate::BitReader<bool>;
#[doc = "Field `SPI_2` writer - SPI2 interrupt active"]
pub type SPI_2_W<'a> = crate::BitWriter<'a, u32, IRQ_SPEC, bool, 2>;
#[doc = "Field `SPI_1` reader - SPI1 interrupt active"]
pub type SPI_1_R = crate::BitReader<bool>;
#[doc = "Field `SPI_1` writer - SPI1 interrupt active"]
pub type SPI_1_W<'a> = crate::BitWriter<'a, u32, IRQ_SPEC, bool, 1>;
#[doc = "Field `UART_1` reader - UART1 interrupt active"]
pub type UART_1_R = crate::BitReader<bool>;
#[doc = "Field `UART_1` writer - UART1 interrupt active"]
pub type UART_1_W<'a> = crate::BitWriter<'a, u32, IRQ_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 2 - SPI2 interrupt active"]
    #[inline(always)]
    pub fn spi_2(&self) -> SPI_2_R {
        SPI_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SPI1 interrupt active"]
    #[inline(always)]
    pub fn spi_1(&self) -> SPI_1_R {
        SPI_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - UART1 interrupt active"]
    #[inline(always)]
    pub fn uart_1(&self) -> UART_1_R {
        UART_1_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SPI2 interrupt active"]
    #[inline(always)]
    pub fn spi_2(&mut self) -> SPI_2_W {
        SPI_2_W::new(self)
    }
    #[doc = "Bit 1 - SPI1 interrupt active"]
    #[inline(always)]
    pub fn spi_1(&mut self) -> SPI_1_W {
        SPI_1_W::new(self)
    }
    #[doc = "Bit 0 - UART1 interrupt active"]
    #[inline(always)]
    pub fn uart_1(&mut self) -> UART_1_W {
        UART_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq](index.html) module"]
pub struct IRQ_SPEC;
impl crate::RegisterSpec for IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq::R](R) reader structure"]
impl crate::Readable for IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq::W](W) writer structure"]
impl crate::Writable for IRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ to value 0"]
impl crate::Resettable for IRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
