#[doc = "Register `INTERRUPT` reader"]
pub struct R(crate::R<INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT` writer"]
pub struct W(crate::W<INTERRUPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_SPEC>;
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
impl From<crate::W<INTERRUPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMD_ERR` reader - Auto command error"]
pub type ACMD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ACMD_ERR` writer - Auto command error"]
pub type ACMD_ERR_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 24>;
#[doc = "Field `DEND_ERR` reader - Data end bit error (not 1)"]
pub type DEND_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DEND_ERR` writer - Data end bit error (not 1)"]
pub type DEND_ERR_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 22>;
#[doc = "Field `DCRC_ERR` reader - Data CRC error"]
pub type DCRC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DCRC_ERR` writer - Data CRC error"]
pub type DCRC_ERR_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 21>;
#[doc = "Field `DTO_ERR` reader - Data timeout"]
pub type DTO_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DTO_ERR` writer - Data timeout"]
pub type DTO_ERR_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 20>;
#[doc = "Field `CBAD_ERR` reader - Incorrect response command index"]
pub type CBAD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CBAD_ERR` writer - Incorrect response command index"]
pub type CBAD_ERR_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 19>;
#[doc = "Field `CEND_ERR` reader - Command end bit error (not 1)"]
pub type CEND_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CEND_ERR` writer - Command end bit error (not 1)"]
pub type CEND_ERR_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 18>;
#[doc = "Field `CCRC_ERR` reader - Command CRC error"]
pub type CCRC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CCRC_ERR` writer - Command CRC error"]
pub type CCRC_ERR_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 17>;
#[doc = "Field `CTO_ERR` reader - Command timeout"]
pub type CTO_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CTO_ERR` writer - Command timeout"]
pub type CTO_ERR_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 16>;
#[doc = "Field `ERR` reader - An error has occured"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ENDBOOT` reader - Boot operation has terminated"]
pub type ENDBOOT_R = crate::BitReader<bool>;
#[doc = "Field `ENDBOOT` writer - Boot operation has terminated"]
pub type ENDBOOT_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 14>;
#[doc = "Field `BOOTACK` reader - Boot has been acknowledged"]
pub type BOOTACK_R = crate::BitReader<bool>;
#[doc = "Field `BOOTACK` writer - Boot has been acknowledged"]
pub type BOOTACK_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 13>;
#[doc = "Field `RETUNE` reader - Clock retune request"]
pub type RETUNE_R = crate::BitReader<bool>;
#[doc = "Field `RETUNE` writer - Clock retune request"]
pub type RETUNE_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 12>;
#[doc = "Field `CARD` reader - Card made interrupt request"]
pub type CARD_R = crate::BitReader<bool>;
#[doc = "Field `CARD` writer - Card made interrupt request"]
pub type CARD_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 8>;
#[doc = "Field `READ_RDY` reader - DATA contains data to be read"]
pub type READ_RDY_R = crate::BitReader<bool>;
#[doc = "Field `READ_RDY` writer - DATA contains data to be read"]
pub type READ_RDY_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 5>;
#[doc = "Field `WRITE_RDY` reader - DATA can be written to"]
pub type WRITE_RDY_R = crate::BitReader<bool>;
#[doc = "Field `WRITE_RDY` writer - DATA can be written to"]
pub type WRITE_RDY_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 4>;
#[doc = "Field `BLOCK_GAP` reader - Data transfer has stopped at block gap"]
pub type BLOCK_GAP_R = crate::BitReader<bool>;
#[doc = "Field `BLOCK_GAP` writer - Data transfer has stopped at block gap"]
pub type BLOCK_GAP_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 2>;
#[doc = "Field `DATA_DONE` reader - Data transfer has finished"]
pub type DATA_DONE_R = crate::BitReader<bool>;
#[doc = "Field `DATA_DONE` writer - Data transfer has finished"]
pub type DATA_DONE_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 1>;
#[doc = "Field `CMD_DONE` reader - Command has finished"]
pub type CMD_DONE_R = crate::BitReader<bool>;
#[doc = "Field `CMD_DONE` writer - Command has finished"]
pub type CMD_DONE_W<'a> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 24 - Auto command error"]
    #[inline(always)]
    pub fn acmd_err(&self) -> ACMD_ERR_R {
        ACMD_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 22 - Data end bit error (not 1)"]
    #[inline(always)]
    pub fn dend_err(&self) -> DEND_ERR_R {
        DEND_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC error"]
    #[inline(always)]
    pub fn dcrc_err(&self) -> DCRC_ERR_R {
        DCRC_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Data timeout"]
    #[inline(always)]
    pub fn dto_err(&self) -> DTO_ERR_R {
        DTO_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Incorrect response command index"]
    #[inline(always)]
    pub fn cbad_err(&self) -> CBAD_ERR_R {
        CBAD_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Command end bit error (not 1)"]
    #[inline(always)]
    pub fn cend_err(&self) -> CEND_ERR_R {
        CEND_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC error"]
    #[inline(always)]
    pub fn ccrc_err(&self) -> CCRC_ERR_R {
        CCRC_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Command timeout"]
    #[inline(always)]
    pub fn cto_err(&self) -> CTO_ERR_R {
        CTO_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - An error has occured"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot operation has terminated"]
    #[inline(always)]
    pub fn endboot(&self) -> ENDBOOT_R {
        ENDBOOT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot has been acknowledged"]
    #[inline(always)]
    pub fn bootack(&self) -> BOOTACK_R {
        BOOTACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Clock retune request"]
    #[inline(always)]
    pub fn retune(&self) -> RETUNE_R {
        RETUNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 8 - Card made interrupt request"]
    #[inline(always)]
    pub fn card(&self) -> CARD_R {
        CARD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 5 - DATA contains data to be read"]
    #[inline(always)]
    pub fn read_rdy(&self) -> READ_RDY_R {
        READ_RDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - DATA can be written to"]
    #[inline(always)]
    pub fn write_rdy(&self) -> WRITE_RDY_R {
        WRITE_RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - Data transfer has stopped at block gap"]
    #[inline(always)]
    pub fn block_gap(&self) -> BLOCK_GAP_R {
        BLOCK_GAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer has finished"]
    #[inline(always)]
    pub fn data_done(&self) -> DATA_DONE_R {
        DATA_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Command has finished"]
    #[inline(always)]
    pub fn cmd_done(&self) -> CMD_DONE_R {
        CMD_DONE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Auto command error"]
    #[inline(always)]
    pub fn acmd_err(&mut self) -> ACMD_ERR_W {
        ACMD_ERR_W::new(self)
    }
    #[doc = "Bit 22 - Data end bit error (not 1)"]
    #[inline(always)]
    pub fn dend_err(&mut self) -> DEND_ERR_W {
        DEND_ERR_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC error"]
    #[inline(always)]
    pub fn dcrc_err(&mut self) -> DCRC_ERR_W {
        DCRC_ERR_W::new(self)
    }
    #[doc = "Bit 20 - Data timeout"]
    #[inline(always)]
    pub fn dto_err(&mut self) -> DTO_ERR_W {
        DTO_ERR_W::new(self)
    }
    #[doc = "Bit 19 - Incorrect response command index"]
    #[inline(always)]
    pub fn cbad_err(&mut self) -> CBAD_ERR_W {
        CBAD_ERR_W::new(self)
    }
    #[doc = "Bit 18 - Command end bit error (not 1)"]
    #[inline(always)]
    pub fn cend_err(&mut self) -> CEND_ERR_W {
        CEND_ERR_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC error"]
    #[inline(always)]
    pub fn ccrc_err(&mut self) -> CCRC_ERR_W {
        CCRC_ERR_W::new(self)
    }
    #[doc = "Bit 16 - Command timeout"]
    #[inline(always)]
    pub fn cto_err(&mut self) -> CTO_ERR_W {
        CTO_ERR_W::new(self)
    }
    #[doc = "Bit 14 - Boot operation has terminated"]
    #[inline(always)]
    pub fn endboot(&mut self) -> ENDBOOT_W {
        ENDBOOT_W::new(self)
    }
    #[doc = "Bit 13 - Boot has been acknowledged"]
    #[inline(always)]
    pub fn bootack(&mut self) -> BOOTACK_W {
        BOOTACK_W::new(self)
    }
    #[doc = "Bit 12 - Clock retune request"]
    #[inline(always)]
    pub fn retune(&mut self) -> RETUNE_W {
        RETUNE_W::new(self)
    }
    #[doc = "Bit 8 - Card made interrupt request"]
    #[inline(always)]
    pub fn card(&mut self) -> CARD_W {
        CARD_W::new(self)
    }
    #[doc = "Bit 5 - DATA contains data to be read"]
    #[inline(always)]
    pub fn read_rdy(&mut self) -> READ_RDY_W {
        READ_RDY_W::new(self)
    }
    #[doc = "Bit 4 - DATA can be written to"]
    #[inline(always)]
    pub fn write_rdy(&mut self) -> WRITE_RDY_W {
        WRITE_RDY_W::new(self)
    }
    #[doc = "Bit 2 - Data transfer has stopped at block gap"]
    #[inline(always)]
    pub fn block_gap(&mut self) -> BLOCK_GAP_W {
        BLOCK_GAP_W::new(self)
    }
    #[doc = "Bit 1 - Data transfer has finished"]
    #[inline(always)]
    pub fn data_done(&mut self) -> DATA_DONE_W {
        DATA_DONE_W::new(self)
    }
    #[doc = "Bit 0 - Command has finished"]
    #[inline(always)]
    pub fn cmd_done(&mut self) -> CMD_DONE_W {
        CMD_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt](index.html) module"]
pub struct INTERRUPT_SPEC;
impl crate::RegisterSpec for INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt::R](R) reader structure"]
impl crate::Readable for INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt::W](W) writer structure"]
impl crate::Writable for INTERRUPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for INTERRUPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
