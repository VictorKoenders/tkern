#[doc = "Register `CNTL` reader"]
pub struct R(crate::R<CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTL` writer"]
pub struct W(crate::W<CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTL_SPEC>;
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
impl From<crate::W<CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CTS assert level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_ASSERT_A {
    #[doc = "0: Assert high"]
    HIGH = 0,
    #[doc = "1: Assert low"]
    LOW = 1,
}
impl From<CTS_ASSERT_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_ASSERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS_ASSERT` reader - CTS assert level"]
pub type CTS_ASSERT_R = crate::BitReader<CTS_ASSERT_A>;
impl CTS_ASSERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTS_ASSERT_A {
        match self.bits {
            false => CTS_ASSERT_A::HIGH,
            true => CTS_ASSERT_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CTS_ASSERT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CTS_ASSERT_A::LOW
    }
}
#[doc = "Field `CTS_ASSERT` writer - CTS assert level"]
pub type CTS_ASSERT_W<'a> = crate::BitWriter<'a, u32, CNTL_SPEC, CTS_ASSERT_A, 7>;
impl<'a> CTS_ASSERT_W<'a> {
    #[doc = "Assert high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CTS_ASSERT_A::HIGH)
    }
    #[doc = "Assert low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CTS_ASSERT_A::LOW)
    }
}
#[doc = "RTS assert level"]
pub use crate::uart1::cntl::CTS_ASSERT_A as RTS_ASSERT_A;
#[doc = "Field `RTS_ASSERT` reader - RTS assert level"]
pub use crate::uart1::cntl::CTS_ASSERT_R as RTS_ASSERT_R;
#[doc = "Field `RTS_ASSERT` writer - RTS assert level"]
pub type RTS_ASSERT_W<'a> = crate::BitWriter<'a, u32, CNTL_SPEC, RTS_ASSERT_A, 6>;
impl<'a> RTS_ASSERT_W<'a> {
    #[doc = "Assert high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RTS_ASSERT_A::HIGH)
    }
    #[doc = "Assert low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RTS_ASSERT_A::LOW)
    }
}
#[doc = "FIFO level to de-assert RTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTS_FIFO_LEVEL_A {
    #[doc = "0: 3 empty spaces"]
    _3EMPTY = 0,
    #[doc = "1: 2 empty spaces"]
    _2EMPTY = 1,
    #[doc = "2: 1 empty spaces"]
    _1EMPTY = 2,
    #[doc = "3: 4 empty spaces"]
    _4EMPTY = 3,
}
impl From<RTS_FIFO_LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTS_FIFO_LEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTS_FIFO_LEVEL` reader - FIFO level to de-assert RTS"]
pub type RTS_FIFO_LEVEL_R = crate::FieldReader<u8, RTS_FIFO_LEVEL_A>;
impl RTS_FIFO_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTS_FIFO_LEVEL_A {
        match self.bits {
            0 => RTS_FIFO_LEVEL_A::_3EMPTY,
            1 => RTS_FIFO_LEVEL_A::_2EMPTY,
            2 => RTS_FIFO_LEVEL_A::_1EMPTY,
            3 => RTS_FIFO_LEVEL_A::_4EMPTY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_3EMPTY`"]
    #[inline(always)]
    pub fn is_3empty(&self) -> bool {
        *self == RTS_FIFO_LEVEL_A::_3EMPTY
    }
    #[doc = "Checks if the value of the field is `_2EMPTY`"]
    #[inline(always)]
    pub fn is_2empty(&self) -> bool {
        *self == RTS_FIFO_LEVEL_A::_2EMPTY
    }
    #[doc = "Checks if the value of the field is `_1EMPTY`"]
    #[inline(always)]
    pub fn is_1empty(&self) -> bool {
        *self == RTS_FIFO_LEVEL_A::_1EMPTY
    }
    #[doc = "Checks if the value of the field is `_4EMPTY`"]
    #[inline(always)]
    pub fn is_4empty(&self) -> bool {
        *self == RTS_FIFO_LEVEL_A::_4EMPTY
    }
}
#[doc = "Field `RTS_FIFO_LEVEL` writer - FIFO level to de-assert RTS"]
pub type RTS_FIFO_LEVEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, CNTL_SPEC, u8, RTS_FIFO_LEVEL_A, 2, 4>;
impl<'a> RTS_FIFO_LEVEL_W<'a> {
    #[doc = "3 empty spaces"]
    #[inline(always)]
    pub fn _3empty(self) -> &'a mut W {
        self.variant(RTS_FIFO_LEVEL_A::_3EMPTY)
    }
    #[doc = "2 empty spaces"]
    #[inline(always)]
    pub fn _2empty(self) -> &'a mut W {
        self.variant(RTS_FIFO_LEVEL_A::_2EMPTY)
    }
    #[doc = "1 empty spaces"]
    #[inline(always)]
    pub fn _1empty(self) -> &'a mut W {
        self.variant(RTS_FIFO_LEVEL_A::_1EMPTY)
    }
    #[doc = "4 empty spaces"]
    #[inline(always)]
    pub fn _4empty(self) -> &'a mut W {
        self.variant(RTS_FIFO_LEVEL_A::_4EMPTY)
    }
}
#[doc = "Field `CTS_ENABLE` reader - Enable auto transmit flow control with CTS"]
pub type CTS_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CTS_ENABLE` writer - Enable auto transmit flow control with CTS"]
pub type CTS_ENABLE_W<'a> = crate::BitWriter<'a, u32, CNTL_SPEC, bool, 3>;
#[doc = "Field `RTS_ENABLE` reader - Enable auto receive flow control with RTS"]
pub type RTS_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RTS_ENABLE` writer - Enable auto receive flow control with RTS"]
pub type RTS_ENABLE_W<'a> = crate::BitWriter<'a, u32, CNTL_SPEC, bool, 2>;
#[doc = "Field `TX_ENABLE` reader - Enable transmit"]
pub type TX_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `TX_ENABLE` writer - Enable transmit"]
pub type TX_ENABLE_W<'a> = crate::BitWriter<'a, u32, CNTL_SPEC, bool, 1>;
#[doc = "Field `RX_ENABLE` reader - Enable receive"]
pub type RX_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RX_ENABLE` writer - Enable receive"]
pub type RX_ENABLE_W<'a> = crate::BitWriter<'a, u32, CNTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 7 - CTS assert level"]
    #[inline(always)]
    pub fn cts_assert(&self) -> CTS_ASSERT_R {
        CTS_ASSERT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - RTS assert level"]
    #[inline(always)]
    pub fn rts_assert(&self) -> RTS_ASSERT_R {
        RTS_ASSERT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5 - FIFO level to de-assert RTS"]
    #[inline(always)]
    pub fn rts_fifo_level(&self) -> RTS_FIFO_LEVEL_R {
        RTS_FIFO_LEVEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - Enable auto transmit flow control with CTS"]
    #[inline(always)]
    pub fn cts_enable(&self) -> CTS_ENABLE_R {
        CTS_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable auto receive flow control with RTS"]
    #[inline(always)]
    pub fn rts_enable(&self) -> RTS_ENABLE_R {
        RTS_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Enable transmit"]
    #[inline(always)]
    pub fn tx_enable(&self) -> TX_ENABLE_R {
        TX_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable receive"]
    #[inline(always)]
    pub fn rx_enable(&self) -> RX_ENABLE_R {
        RX_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - CTS assert level"]
    #[inline(always)]
    pub fn cts_assert(&mut self) -> CTS_ASSERT_W {
        CTS_ASSERT_W::new(self)
    }
    #[doc = "Bit 6 - RTS assert level"]
    #[inline(always)]
    pub fn rts_assert(&mut self) -> RTS_ASSERT_W {
        RTS_ASSERT_W::new(self)
    }
    #[doc = "Bits 4:5 - FIFO level to de-assert RTS"]
    #[inline(always)]
    pub fn rts_fifo_level(&mut self) -> RTS_FIFO_LEVEL_W {
        RTS_FIFO_LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Enable auto transmit flow control with CTS"]
    #[inline(always)]
    pub fn cts_enable(&mut self) -> CTS_ENABLE_W {
        CTS_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Enable auto receive flow control with RTS"]
    #[inline(always)]
    pub fn rts_enable(&mut self) -> RTS_ENABLE_W {
        RTS_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enable transmit"]
    #[inline(always)]
    pub fn tx_enable(&mut self) -> TX_ENABLE_W {
        TX_ENABLE_W::new(self)
    }
    #[doc = "Bit 0 - Enable receive"]
    #[inline(always)]
    pub fn rx_enable(&mut self) -> RX_ENABLE_W {
        RX_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl](index.html) module"]
pub struct CNTL_SPEC;
impl crate::RegisterSpec for CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntl::R](R) reader structure"]
impl crate::Readable for CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntl::W](W) writer structure"]
impl crate::Writable for CNTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTL to value 0"]
impl crate::Resettable for CNTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
