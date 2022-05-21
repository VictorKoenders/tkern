#[doc = "Register `CNTL0` reader"]
pub struct R(crate::R<CNTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTL0` writer"]
pub struct W(crate::W<CNTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTL0_SPEC>;
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
impl From<crate::W<CNTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPEED` reader - SPI clock speed. clk = sys / 2 * (SPEED + 1)"]
pub type SPEED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPEED` writer - SPI clock speed. clk = sys / 2 * (SPEED + 1)"]
pub type SPEED_W<'a> = crate::FieldWriter<'a, u32, CNTL0_SPEC, u16, u16, 12, 20>;
#[doc = "Field `CHIP_SELECTS` reader - The CS pattern when active"]
pub type CHIP_SELECTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHIP_SELECTS` writer - The CS pattern when active"]
pub type CHIP_SELECTS_W<'a> = crate::FieldWriter<'a, u32, CNTL0_SPEC, u8, u8, 3, 17>;
#[doc = "Field `POST_INPUT` reader - Post input mode"]
pub type POST_INPUT_R = crate::BitReader<bool>;
#[doc = "Field `POST_INPUT` writer - Post input mode"]
pub type POST_INPUT_W<'a> = crate::BitWriter<'a, u32, CNTL0_SPEC, bool, 16>;
#[doc = "Field `VARIABLE_CS` reader - Take CS pattern and data from TX FIFO (along with VARIABLE_WIDTH)"]
pub type VARIABLE_CS_R = crate::BitReader<bool>;
#[doc = "Field `VARIABLE_CS` writer - Take CS pattern and data from TX FIFO (along with VARIABLE_WIDTH)"]
pub type VARIABLE_CS_W<'a> = crate::BitWriter<'a, u32, CNTL0_SPEC, bool, 15>;
#[doc = "Field `VARIABLE_WIDTH` reader - Take shift length and data from FIFO"]
pub type VARIABLE_WIDTH_R = crate::BitReader<bool>;
#[doc = "Field `VARIABLE_WIDTH` writer - Take shift length and data from FIFO"]
pub type VARIABLE_WIDTH_W<'a> = crate::BitWriter<'a, u32, CNTL0_SPEC, bool, 14>;
#[doc = "Controls extra DOUT hold time in system clock cycles\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DOUT_HOLD_TIME_A {
    #[doc = "0: `0`"]
    _0 = 0,
    #[doc = "1: `1`"]
    _1 = 1,
    #[doc = "2: `10`"]
    _4 = 2,
    #[doc = "3: `11`"]
    _7 = 3,
}
impl From<DOUT_HOLD_TIME_A> for u8 {
    #[inline(always)]
    fn from(variant: DOUT_HOLD_TIME_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DOUT_HOLD_TIME` reader - Controls extra DOUT hold time in system clock cycles"]
pub type DOUT_HOLD_TIME_R = crate::FieldReader<u8, DOUT_HOLD_TIME_A>;
impl DOUT_HOLD_TIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT_HOLD_TIME_A {
        match self.bits {
            0 => DOUT_HOLD_TIME_A::_0,
            1 => DOUT_HOLD_TIME_A::_1,
            2 => DOUT_HOLD_TIME_A::_4,
            3 => DOUT_HOLD_TIME_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOUT_HOLD_TIME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOUT_HOLD_TIME_A::_1
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == DOUT_HOLD_TIME_A::_4
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == DOUT_HOLD_TIME_A::_7
    }
}
#[doc = "Field `DOUT_HOLD_TIME` writer - Controls extra DOUT hold time in system clock cycles"]
pub type DOUT_HOLD_TIME_W<'a> =
    crate::FieldWriterSafe<'a, u32, CNTL0_SPEC, u8, DOUT_HOLD_TIME_A, 2, 12>;
impl<'a> DOUT_HOLD_TIME_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT_HOLD_TIME_A::_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT_HOLD_TIME_A::_1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(DOUT_HOLD_TIME_A::_4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(DOUT_HOLD_TIME_A::_7)
    }
}
#[doc = "Field `ENABLE` reader - Enable the interface"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable the interface"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, CNTL0_SPEC, bool, 11>;
#[doc = "Field `IN_RISING` reader - Data is clocked in on rising edge of CLK"]
pub type IN_RISING_R = crate::BitReader<bool>;
#[doc = "Field `IN_RISING` writer - Data is clocked in on rising edge of CLK"]
pub type IN_RISING_W<'a> = crate::BitWriter<'a, u32, CNTL0_SPEC, bool, 10>;
#[doc = "Field `CLEAR_FIFOS` reader - Clear FIFOs"]
pub type CLEAR_FIFOS_R = crate::BitReader<bool>;
#[doc = "Field `CLEAR_FIFOS` writer - Clear FIFOs"]
pub type CLEAR_FIFOS_W<'a> = crate::BitWriter<'a, u32, CNTL0_SPEC, bool, 9>;
#[doc = "Field `OUT_RISING` reader - Data is clocked out on rising edge of CLK"]
pub type OUT_RISING_R = crate::BitReader<bool>;
#[doc = "Field `OUT_RISING` writer - Data is clocked out on rising edge of CLK"]
pub type OUT_RISING_W<'a> = crate::BitWriter<'a, u32, CNTL0_SPEC, bool, 8>;
#[doc = "Field `INVERT_CLK` reader - Idle clock high"]
pub type INVERT_CLK_R = crate::BitReader<bool>;
#[doc = "Field `INVERT_CLK` writer - Idle clock high"]
pub type INVERT_CLK_W<'a> = crate::BitWriter<'a, u32, CNTL0_SPEC, bool, 7>;
#[doc = "Field `MSB_FIRST` reader - Shift out the most significant bit (MSB) first"]
pub type MSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `MSB_FIRST` writer - Shift out the most significant bit (MSB) first"]
pub type MSB_FIRST_W<'a> = crate::BitWriter<'a, u32, CNTL0_SPEC, bool, 6>;
#[doc = "Field `SHIFT_LENGTH` reader - Number of bits to shift"]
pub type SHIFT_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHIFT_LENGTH` writer - Number of bits to shift"]
pub type SHIFT_LENGTH_W<'a> = crate::FieldWriter<'a, u32, CNTL0_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 20:31 - SPI clock speed. clk = sys / 2 * (SPEED + 1)"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 17:19 - The CS pattern when active"]
    #[inline(always)]
    pub fn chip_selects(&self) -> CHIP_SELECTS_R {
        CHIP_SELECTS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 16 - Post input mode"]
    #[inline(always)]
    pub fn post_input(&self) -> POST_INPUT_R {
        POST_INPUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Take CS pattern and data from TX FIFO (along with VARIABLE_WIDTH)"]
    #[inline(always)]
    pub fn variable_cs(&self) -> VARIABLE_CS_R {
        VARIABLE_CS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Take shift length and data from FIFO"]
    #[inline(always)]
    pub fn variable_width(&self) -> VARIABLE_WIDTH_R {
        VARIABLE_WIDTH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Controls extra DOUT hold time in system clock cycles"]
    #[inline(always)]
    pub fn dout_hold_time(&self) -> DOUT_HOLD_TIME_R {
        DOUT_HOLD_TIME_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 11 - Enable the interface"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Data is clocked in on rising edge of CLK"]
    #[inline(always)]
    pub fn in_rising(&self) -> IN_RISING_R {
        IN_RISING_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear FIFOs"]
    #[inline(always)]
    pub fn clear_fifos(&self) -> CLEAR_FIFOS_R {
        CLEAR_FIFOS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Data is clocked out on rising edge of CLK"]
    #[inline(always)]
    pub fn out_rising(&self) -> OUT_RISING_R {
        OUT_RISING_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Idle clock high"]
    #[inline(always)]
    pub fn invert_clk(&self) -> INVERT_CLK_R {
        INVERT_CLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Shift out the most significant bit (MSB) first"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 0:5 - Number of bits to shift"]
    #[inline(always)]
    pub fn shift_length(&self) -> SHIFT_LENGTH_R {
        SHIFT_LENGTH_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:31 - SPI clock speed. clk = sys / 2 * (SPEED + 1)"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W::new(self)
    }
    #[doc = "Bits 17:19 - The CS pattern when active"]
    #[inline(always)]
    pub fn chip_selects(&mut self) -> CHIP_SELECTS_W {
        CHIP_SELECTS_W::new(self)
    }
    #[doc = "Bit 16 - Post input mode"]
    #[inline(always)]
    pub fn post_input(&mut self) -> POST_INPUT_W {
        POST_INPUT_W::new(self)
    }
    #[doc = "Bit 15 - Take CS pattern and data from TX FIFO (along with VARIABLE_WIDTH)"]
    #[inline(always)]
    pub fn variable_cs(&mut self) -> VARIABLE_CS_W {
        VARIABLE_CS_W::new(self)
    }
    #[doc = "Bit 14 - Take shift length and data from FIFO"]
    #[inline(always)]
    pub fn variable_width(&mut self) -> VARIABLE_WIDTH_W {
        VARIABLE_WIDTH_W::new(self)
    }
    #[doc = "Bits 12:13 - Controls extra DOUT hold time in system clock cycles"]
    #[inline(always)]
    pub fn dout_hold_time(&mut self) -> DOUT_HOLD_TIME_W {
        DOUT_HOLD_TIME_W::new(self)
    }
    #[doc = "Bit 11 - Enable the interface"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 10 - Data is clocked in on rising edge of CLK"]
    #[inline(always)]
    pub fn in_rising(&mut self) -> IN_RISING_W {
        IN_RISING_W::new(self)
    }
    #[doc = "Bit 9 - Clear FIFOs"]
    #[inline(always)]
    pub fn clear_fifos(&mut self) -> CLEAR_FIFOS_W {
        CLEAR_FIFOS_W::new(self)
    }
    #[doc = "Bit 8 - Data is clocked out on rising edge of CLK"]
    #[inline(always)]
    pub fn out_rising(&mut self) -> OUT_RISING_W {
        OUT_RISING_W::new(self)
    }
    #[doc = "Bit 7 - Idle clock high"]
    #[inline(always)]
    pub fn invert_clk(&mut self) -> INVERT_CLK_W {
        INVERT_CLK_W::new(self)
    }
    #[doc = "Bit 6 - Shift out the most significant bit (MSB) first"]
    #[inline(always)]
    pub fn msb_first(&mut self) -> MSB_FIRST_W {
        MSB_FIRST_W::new(self)
    }
    #[doc = "Bits 0:5 - Number of bits to shift"]
    #[inline(always)]
    pub fn shift_length(&mut self) -> SHIFT_LENGTH_W {
        SHIFT_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl0](index.html) module"]
pub struct CNTL0_SPEC;
impl crate::RegisterSpec for CNTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntl0::R](R) reader structure"]
impl crate::Readable for CNTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntl0::W](W) writer structure"]
impl crate::Writable for CNTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTL0 to value 0x000e_0000"]
impl crate::Resettable for CNTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000e_0000
    }
}
