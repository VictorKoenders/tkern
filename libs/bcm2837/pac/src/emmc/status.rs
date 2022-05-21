#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT_LEVEL1` reader - Value of DAT\\[7:4\\]"]
pub type DAT_LEVEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAT_LEVEL1` writer - Value of DAT\\[7:4\\]"]
pub type DAT_LEVEL1_W<'a> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 4, 25>;
#[doc = "Field `CMD_LEVEL` reader - Value of CMD"]
pub type CMD_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CMD_LEVEL` writer - Value of CMD"]
pub type CMD_LEVEL_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 24>;
#[doc = "Field `DAT_LEVEL0` reader - Value of DAT\\[3:0\\]"]
pub type DAT_LEVEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAT_LEVEL0` writer - Value of DAT\\[3:0\\]"]
pub type DAT_LEVEL0_W<'a> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 4, 20>;
#[doc = "Field `BUFFER_READ_ENABLE` reader - New data is available to read"]
pub type BUFFER_READ_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `BUFFER_READ_ENABLE` writer - New data is available to read"]
pub type BUFFER_READ_ENABLE_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 11>;
#[doc = "Field `BUFFER_WRITE_ENABLE` reader - The buffer has space for new data"]
pub type BUFFER_WRITE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `BUFFER_WRITE_ENABLE` writer - The buffer has space for new data"]
pub type BUFFER_WRITE_ENABLE_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 10>;
#[doc = "Field `READ_TRANSFER` reader - Read transfer is active"]
pub type READ_TRANSFER_R = crate::BitReader<bool>;
#[doc = "Field `READ_TRANSFER` writer - Read transfer is active"]
pub type READ_TRANSFER_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 9>;
#[doc = "Field `WRITE_TRANSFER` reader - Write transfer is active"]
pub type WRITE_TRANSFER_R = crate::BitReader<bool>;
#[doc = "Field `WRITE_TRANSFER` writer - Write transfer is active"]
pub type WRITE_TRANSFER_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 8>;
#[doc = "Field `DAT_ACTIVE` reader - At least one data line is active"]
pub type DAT_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `DAT_ACTIVE` writer - At least one data line is active"]
pub type DAT_ACTIVE_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 2>;
#[doc = "Field `DAT_INHIBIT` reader - Data lines still in use"]
pub type DAT_INHIBIT_R = crate::BitReader<bool>;
#[doc = "Field `DAT_INHIBIT` writer - Data lines still in use"]
pub type DAT_INHIBIT_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 1>;
#[doc = "Field `CMD_INHIBIT` reader - Command line still in use"]
pub type CMD_INHIBIT_R = crate::BitReader<bool>;
#[doc = "Field `CMD_INHIBIT` writer - Command line still in use"]
pub type CMD_INHIBIT_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 0>;
impl R {
    #[doc = "Bits 25:28 - Value of DAT\\[7:4\\]"]
    #[inline(always)]
    pub fn dat_level1(&self) -> DAT_LEVEL1_R {
        DAT_LEVEL1_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Value of CMD"]
    #[inline(always)]
    pub fn cmd_level(&self) -> CMD_LEVEL_R {
        CMD_LEVEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Value of DAT\\[3:0\\]"]
    #[inline(always)]
    pub fn dat_level0(&self) -> DAT_LEVEL0_R {
        DAT_LEVEL0_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - New data is available to read"]
    #[inline(always)]
    pub fn buffer_read_enable(&self) -> BUFFER_READ_ENABLE_R {
        BUFFER_READ_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - The buffer has space for new data"]
    #[inline(always)]
    pub fn buffer_write_enable(&self) -> BUFFER_WRITE_ENABLE_R {
        BUFFER_WRITE_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Read transfer is active"]
    #[inline(always)]
    pub fn read_transfer(&self) -> READ_TRANSFER_R {
        READ_TRANSFER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Write transfer is active"]
    #[inline(always)]
    pub fn write_transfer(&self) -> WRITE_TRANSFER_R {
        WRITE_TRANSFER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 2 - At least one data line is active"]
    #[inline(always)]
    pub fn dat_active(&self) -> DAT_ACTIVE_R {
        DAT_ACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Data lines still in use"]
    #[inline(always)]
    pub fn dat_inhibit(&self) -> DAT_INHIBIT_R {
        DAT_INHIBIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Command line still in use"]
    #[inline(always)]
    pub fn cmd_inhibit(&self) -> CMD_INHIBIT_R {
        CMD_INHIBIT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 25:28 - Value of DAT\\[7:4\\]"]
    #[inline(always)]
    pub fn dat_level1(&mut self) -> DAT_LEVEL1_W {
        DAT_LEVEL1_W::new(self)
    }
    #[doc = "Bit 24 - Value of CMD"]
    #[inline(always)]
    pub fn cmd_level(&mut self) -> CMD_LEVEL_W {
        CMD_LEVEL_W::new(self)
    }
    #[doc = "Bits 20:23 - Value of DAT\\[3:0\\]"]
    #[inline(always)]
    pub fn dat_level0(&mut self) -> DAT_LEVEL0_W {
        DAT_LEVEL0_W::new(self)
    }
    #[doc = "Bit 11 - New data is available to read"]
    #[inline(always)]
    pub fn buffer_read_enable(&mut self) -> BUFFER_READ_ENABLE_W {
        BUFFER_READ_ENABLE_W::new(self)
    }
    #[doc = "Bit 10 - The buffer has space for new data"]
    #[inline(always)]
    pub fn buffer_write_enable(&mut self) -> BUFFER_WRITE_ENABLE_W {
        BUFFER_WRITE_ENABLE_W::new(self)
    }
    #[doc = "Bit 9 - Read transfer is active"]
    #[inline(always)]
    pub fn read_transfer(&mut self) -> READ_TRANSFER_W {
        READ_TRANSFER_W::new(self)
    }
    #[doc = "Bit 8 - Write transfer is active"]
    #[inline(always)]
    pub fn write_transfer(&mut self) -> WRITE_TRANSFER_W {
        WRITE_TRANSFER_W::new(self)
    }
    #[doc = "Bit 2 - At least one data line is active"]
    #[inline(always)]
    pub fn dat_active(&mut self) -> DAT_ACTIVE_W {
        DAT_ACTIVE_W::new(self)
    }
    #[doc = "Bit 1 - Data lines still in use"]
    #[inline(always)]
    pub fn dat_inhibit(&mut self) -> DAT_INHIBIT_W {
        DAT_INHIBIT_W::new(self)
    }
    #[doc = "Bit 0 - Command line still in use"]
    #[inline(always)]
    pub fn cmd_inhibit(&mut self) -> CMD_INHIBIT_W {
        CMD_INHIBIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status info for debugging\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
