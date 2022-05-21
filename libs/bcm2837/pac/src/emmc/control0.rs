#[doc = "Register `CONTROL0` reader"]
pub struct R(crate::R<CONTROL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL0` writer"]
pub struct W(crate::W<CONTROL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL0_SPEC>;
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
impl From<crate::W<CONTROL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALT_BOOT_EN` reader - Enable alternate boot mode"]
pub type ALT_BOOT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALT_BOOT_EN` writer - Enable alternate boot mode"]
pub type ALT_BOOT_EN_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 22>;
#[doc = "Field `BOOT_EN` reader - Boot mode enabled"]
pub type BOOT_EN_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_EN` writer - Boot mode enabled"]
pub type BOOT_EN_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 21>;
#[doc = "Field `SPI_MODE` reader - Enable SPI mode"]
pub type SPI_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MODE` writer - Enable SPI mode"]
pub type SPI_MODE_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 20>;
#[doc = "Field `GAP_IEN` reader - Enable interrupt on block gap"]
pub type GAP_IEN_R = crate::BitReader<bool>;
#[doc = "Field `GAP_IEN` writer - Enable interrupt on block gap"]
pub type GAP_IEN_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 19>;
#[doc = "Field `READWAIT_EN` reader - Use DAT2 read/wait protocol"]
pub type READWAIT_EN_R = crate::BitReader<bool>;
#[doc = "Field `READWAIT_EN` writer - Use DAT2 read/wait protocol"]
pub type READWAIT_EN_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 18>;
#[doc = "Field `GAP_RESTART` reader - Restart a transaction stopped by GAP_STOP"]
pub type GAP_RESTART_R = crate::BitReader<bool>;
#[doc = "Field `GAP_RESTART` writer - Restart a transaction stopped by GAP_STOP"]
pub type GAP_RESTART_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 17>;
#[doc = "Field `GAP_STOP` reader - Stop the current transaction at the next block gap"]
pub type GAP_STOP_R = crate::BitReader<bool>;
#[doc = "Field `GAP_STOP` writer - Stop the current transaction at the next block gap"]
pub type GAP_STOP_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 16>;
#[doc = "Busvoltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BUS_VOLTAGE_A {
    #[doc = "5: `101`"]
    V1_8 = 5,
    #[doc = "6: `110`"]
    V3_0 = 6,
    #[doc = "7: `111`"]
    V3_3 = 7,
}
impl From<BUS_VOLTAGE_A> for u8 {
    #[inline(always)]
    fn from(variant: BUS_VOLTAGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BUS_VOLTAGE` reader - Busvoltage"]
pub type BUS_VOLTAGE_R = crate::FieldReader<u8, BUS_VOLTAGE_A>;
impl BUS_VOLTAGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUS_VOLTAGE_A> {
        match self.bits {
            5 => Some(BUS_VOLTAGE_A::V1_8),
            6 => Some(BUS_VOLTAGE_A::V3_0),
            7 => Some(BUS_VOLTAGE_A::V3_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `V1_8`"]
    #[inline(always)]
    pub fn is_v1_8(&self) -> bool {
        *self == BUS_VOLTAGE_A::V1_8
    }
    #[doc = "Checks if the value of the field is `V3_0`"]
    #[inline(always)]
    pub fn is_v3_0(&self) -> bool {
        *self == BUS_VOLTAGE_A::V3_0
    }
    #[doc = "Checks if the value of the field is `V3_3`"]
    #[inline(always)]
    pub fn is_v3_3(&self) -> bool {
        *self == BUS_VOLTAGE_A::V3_3
    }
}
#[doc = "Field `BUS_VOLTAGE` writer - Busvoltage"]
pub type BUS_VOLTAGE_W<'a> = crate::FieldWriter<'a, u32, CONTROL0_SPEC, u8, BUS_VOLTAGE_A, 3, 9>;
impl<'a> BUS_VOLTAGE_W<'a> {
    #[doc = "`101`"]
    #[inline(always)]
    pub fn v1_8(self) -> &'a mut W {
        self.variant(BUS_VOLTAGE_A::V1_8)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn v3_0(self) -> &'a mut W {
        self.variant(BUS_VOLTAGE_A::V3_0)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn v3_3(self) -> &'a mut W {
        self.variant(BUS_VOLTAGE_A::V3_3)
    }
}
#[doc = "Field `BUS_POWER` reader - Buspower"]
pub type BUS_POWER_R = crate::BitReader<bool>;
#[doc = "Field `BUS_POWER` writer - Buspower"]
pub type BUS_POWER_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 8>;
#[doc = "Field `HCTL_8BIT` reader - Use 8 data lines"]
pub type HCTL_8BIT_R = crate::BitReader<bool>;
#[doc = "Field `HCTL_8BIT` writer - Use 8 data lines"]
pub type HCTL_8BIT_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 5>;
#[doc = "Field `HCTL_HS_EN` reader - Enable high speed mode"]
pub type HCTL_HS_EN_R = crate::BitReader<bool>;
#[doc = "Field `HCTL_HS_EN` writer - Enable high speed mode"]
pub type HCTL_HS_EN_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 2>;
#[doc = "Field `HCTL_DWIDTH` reader - Use 4 data lines"]
pub type HCTL_DWIDTH_R = crate::BitReader<bool>;
#[doc = "Field `HCTL_DWIDTH` writer - Use 4 data lines"]
pub type HCTL_DWIDTH_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 1>;
#[doc = "Field `LED` reader - LED"]
pub type LED_R = crate::BitReader<bool>;
#[doc = "Field `LED` writer - LED"]
pub type LED_W<'a> = crate::BitWriter<'a, u32, CONTROL0_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 22 - Enable alternate boot mode"]
    #[inline(always)]
    pub fn alt_boot_en(&self) -> ALT_BOOT_EN_R {
        ALT_BOOT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Boot mode enabled"]
    #[inline(always)]
    pub fn boot_en(&self) -> BOOT_EN_R {
        BOOT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable SPI mode"]
    #[inline(always)]
    pub fn spi_mode(&self) -> SPI_MODE_R {
        SPI_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable interrupt on block gap"]
    #[inline(always)]
    pub fn gap_ien(&self) -> GAP_IEN_R {
        GAP_IEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Use DAT2 read/wait protocol"]
    #[inline(always)]
    pub fn readwait_en(&self) -> READWAIT_EN_R {
        READWAIT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Restart a transaction stopped by GAP_STOP"]
    #[inline(always)]
    pub fn gap_restart(&self) -> GAP_RESTART_R {
        GAP_RESTART_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Stop the current transaction at the next block gap"]
    #[inline(always)]
    pub fn gap_stop(&self) -> GAP_STOP_R {
        GAP_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Busvoltage"]
    #[inline(always)]
    pub fn bus_voltage(&self) -> BUS_VOLTAGE_R {
        BUS_VOLTAGE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 8 - Buspower"]
    #[inline(always)]
    pub fn bus_power(&self) -> BUS_POWER_R {
        BUS_POWER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 5 - Use 8 data lines"]
    #[inline(always)]
    pub fn hctl_8bit(&self) -> HCTL_8BIT_R {
        HCTL_8BIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable high speed mode"]
    #[inline(always)]
    pub fn hctl_hs_en(&self) -> HCTL_HS_EN_R {
        HCTL_HS_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Use 4 data lines"]
    #[inline(always)]
    pub fn hctl_dwidth(&self) -> HCTL_DWIDTH_R {
        HCTL_DWIDTH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - LED"]
    #[inline(always)]
    pub fn led(&self) -> LED_R {
        LED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - Enable alternate boot mode"]
    #[inline(always)]
    pub fn alt_boot_en(&mut self) -> ALT_BOOT_EN_W {
        ALT_BOOT_EN_W::new(self)
    }
    #[doc = "Bit 21 - Boot mode enabled"]
    #[inline(always)]
    pub fn boot_en(&mut self) -> BOOT_EN_W {
        BOOT_EN_W::new(self)
    }
    #[doc = "Bit 20 - Enable SPI mode"]
    #[inline(always)]
    pub fn spi_mode(&mut self) -> SPI_MODE_W {
        SPI_MODE_W::new(self)
    }
    #[doc = "Bit 19 - Enable interrupt on block gap"]
    #[inline(always)]
    pub fn gap_ien(&mut self) -> GAP_IEN_W {
        GAP_IEN_W::new(self)
    }
    #[doc = "Bit 18 - Use DAT2 read/wait protocol"]
    #[inline(always)]
    pub fn readwait_en(&mut self) -> READWAIT_EN_W {
        READWAIT_EN_W::new(self)
    }
    #[doc = "Bit 17 - Restart a transaction stopped by GAP_STOP"]
    #[inline(always)]
    pub fn gap_restart(&mut self) -> GAP_RESTART_W {
        GAP_RESTART_W::new(self)
    }
    #[doc = "Bit 16 - Stop the current transaction at the next block gap"]
    #[inline(always)]
    pub fn gap_stop(&mut self) -> GAP_STOP_W {
        GAP_STOP_W::new(self)
    }
    #[doc = "Bits 9:11 - Busvoltage"]
    #[inline(always)]
    pub fn bus_voltage(&mut self) -> BUS_VOLTAGE_W {
        BUS_VOLTAGE_W::new(self)
    }
    #[doc = "Bit 8 - Buspower"]
    #[inline(always)]
    pub fn bus_power(&mut self) -> BUS_POWER_W {
        BUS_POWER_W::new(self)
    }
    #[doc = "Bit 5 - Use 8 data lines"]
    #[inline(always)]
    pub fn hctl_8bit(&mut self) -> HCTL_8BIT_W {
        HCTL_8BIT_W::new(self)
    }
    #[doc = "Bit 2 - Enable high speed mode"]
    #[inline(always)]
    pub fn hctl_hs_en(&mut self) -> HCTL_HS_EN_W {
        HCTL_HS_EN_W::new(self)
    }
    #[doc = "Bit 1 - Use 4 data lines"]
    #[inline(always)]
    pub fn hctl_dwidth(&mut self) -> HCTL_DWIDTH_W {
        HCTL_DWIDTH_W::new(self)
    }
    #[doc = "Bit 0 - LED"]
    #[inline(always)]
    pub fn led(&mut self) -> LED_W {
        LED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control0](index.html) module"]
pub struct CONTROL0_SPEC;
impl crate::RegisterSpec for CONTROL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control0::R](R) reader structure"]
impl crate::Readable for CONTROL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control0::W](W) writer structure"]
impl crate::Writable for CONTROL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONTROL0 to value 0"]
impl crate::Resettable for CONTROL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
