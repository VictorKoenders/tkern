#[doc = "Register `GPHEN0` reader"]
pub struct R(crate::R<GPHEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPHEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPHEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPHEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPHEN0` writer"]
pub struct W(crate::W<GPHEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPHEN0_SPEC>;
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
impl From<crate::W<GPHEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPHEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HEN0` reader - High detect enabled 0"]
pub type HEN0_R = crate::BitReader<bool>;
#[doc = "Field `HEN0` writer - High detect enabled 0"]
pub type HEN0_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 0>;
#[doc = "Field `HEN1` reader - High detect enabled 1"]
pub type HEN1_R = crate::BitReader<bool>;
#[doc = "Field `HEN1` writer - High detect enabled 1"]
pub type HEN1_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 1>;
#[doc = "Field `HEN2` reader - High detect enabled 2"]
pub type HEN2_R = crate::BitReader<bool>;
#[doc = "Field `HEN2` writer - High detect enabled 2"]
pub type HEN2_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 2>;
#[doc = "Field `HEN3` reader - High detect enabled 3"]
pub type HEN3_R = crate::BitReader<bool>;
#[doc = "Field `HEN3` writer - High detect enabled 3"]
pub type HEN3_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 3>;
#[doc = "Field `HEN4` reader - High detect enabled 4"]
pub type HEN4_R = crate::BitReader<bool>;
#[doc = "Field `HEN4` writer - High detect enabled 4"]
pub type HEN4_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 4>;
#[doc = "Field `HEN5` reader - High detect enabled 5"]
pub type HEN5_R = crate::BitReader<bool>;
#[doc = "Field `HEN5` writer - High detect enabled 5"]
pub type HEN5_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 5>;
#[doc = "Field `HEN6` reader - High detect enabled 6"]
pub type HEN6_R = crate::BitReader<bool>;
#[doc = "Field `HEN6` writer - High detect enabled 6"]
pub type HEN6_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 6>;
#[doc = "Field `HEN7` reader - High detect enabled 7"]
pub type HEN7_R = crate::BitReader<bool>;
#[doc = "Field `HEN7` writer - High detect enabled 7"]
pub type HEN7_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 7>;
#[doc = "Field `HEN8` reader - High detect enabled 8"]
pub type HEN8_R = crate::BitReader<bool>;
#[doc = "Field `HEN8` writer - High detect enabled 8"]
pub type HEN8_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 8>;
#[doc = "Field `HEN9` reader - High detect enabled 9"]
pub type HEN9_R = crate::BitReader<bool>;
#[doc = "Field `HEN9` writer - High detect enabled 9"]
pub type HEN9_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 9>;
#[doc = "Field `HEN10` reader - High detect enabled 10"]
pub type HEN10_R = crate::BitReader<bool>;
#[doc = "Field `HEN10` writer - High detect enabled 10"]
pub type HEN10_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 10>;
#[doc = "Field `HEN11` reader - High detect enabled 11"]
pub type HEN11_R = crate::BitReader<bool>;
#[doc = "Field `HEN11` writer - High detect enabled 11"]
pub type HEN11_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 11>;
#[doc = "Field `HEN12` reader - High detect enabled 12"]
pub type HEN12_R = crate::BitReader<bool>;
#[doc = "Field `HEN12` writer - High detect enabled 12"]
pub type HEN12_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 12>;
#[doc = "Field `HEN13` reader - High detect enabled 13"]
pub type HEN13_R = crate::BitReader<bool>;
#[doc = "Field `HEN13` writer - High detect enabled 13"]
pub type HEN13_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 13>;
#[doc = "Field `HEN14` reader - High detect enabled 14"]
pub type HEN14_R = crate::BitReader<bool>;
#[doc = "Field `HEN14` writer - High detect enabled 14"]
pub type HEN14_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 14>;
#[doc = "Field `HEN15` reader - High detect enabled 15"]
pub type HEN15_R = crate::BitReader<bool>;
#[doc = "Field `HEN15` writer - High detect enabled 15"]
pub type HEN15_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 15>;
#[doc = "Field `HEN16` reader - High detect enabled 16"]
pub type HEN16_R = crate::BitReader<bool>;
#[doc = "Field `HEN16` writer - High detect enabled 16"]
pub type HEN16_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 16>;
#[doc = "Field `HEN17` reader - High detect enabled 17"]
pub type HEN17_R = crate::BitReader<bool>;
#[doc = "Field `HEN17` writer - High detect enabled 17"]
pub type HEN17_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 17>;
#[doc = "Field `HEN18` reader - High detect enabled 18"]
pub type HEN18_R = crate::BitReader<bool>;
#[doc = "Field `HEN18` writer - High detect enabled 18"]
pub type HEN18_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 18>;
#[doc = "Field `HEN19` reader - High detect enabled 19"]
pub type HEN19_R = crate::BitReader<bool>;
#[doc = "Field `HEN19` writer - High detect enabled 19"]
pub type HEN19_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 19>;
#[doc = "Field `HEN20` reader - High detect enabled 20"]
pub type HEN20_R = crate::BitReader<bool>;
#[doc = "Field `HEN20` writer - High detect enabled 20"]
pub type HEN20_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 20>;
#[doc = "Field `HEN21` reader - High detect enabled 21"]
pub type HEN21_R = crate::BitReader<bool>;
#[doc = "Field `HEN21` writer - High detect enabled 21"]
pub type HEN21_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 21>;
#[doc = "Field `HEN22` reader - High detect enabled 22"]
pub type HEN22_R = crate::BitReader<bool>;
#[doc = "Field `HEN22` writer - High detect enabled 22"]
pub type HEN22_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 22>;
#[doc = "Field `HEN23` reader - High detect enabled 23"]
pub type HEN23_R = crate::BitReader<bool>;
#[doc = "Field `HEN23` writer - High detect enabled 23"]
pub type HEN23_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 23>;
#[doc = "Field `HEN24` reader - High detect enabled 24"]
pub type HEN24_R = crate::BitReader<bool>;
#[doc = "Field `HEN24` writer - High detect enabled 24"]
pub type HEN24_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 24>;
#[doc = "Field `HEN25` reader - High detect enabled 25"]
pub type HEN25_R = crate::BitReader<bool>;
#[doc = "Field `HEN25` writer - High detect enabled 25"]
pub type HEN25_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 25>;
#[doc = "Field `HEN26` reader - High detect enabled 26"]
pub type HEN26_R = crate::BitReader<bool>;
#[doc = "Field `HEN26` writer - High detect enabled 26"]
pub type HEN26_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 26>;
#[doc = "Field `HEN27` reader - High detect enabled 27"]
pub type HEN27_R = crate::BitReader<bool>;
#[doc = "Field `HEN27` writer - High detect enabled 27"]
pub type HEN27_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 27>;
#[doc = "Field `HEN28` reader - High detect enabled 28"]
pub type HEN28_R = crate::BitReader<bool>;
#[doc = "Field `HEN28` writer - High detect enabled 28"]
pub type HEN28_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 28>;
#[doc = "Field `HEN29` reader - High detect enabled 29"]
pub type HEN29_R = crate::BitReader<bool>;
#[doc = "Field `HEN29` writer - High detect enabled 29"]
pub type HEN29_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 29>;
#[doc = "Field `HEN30` reader - High detect enabled 30"]
pub type HEN30_R = crate::BitReader<bool>;
#[doc = "Field `HEN30` writer - High detect enabled 30"]
pub type HEN30_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 30>;
#[doc = "Field `HEN31` reader - High detect enabled 31"]
pub type HEN31_R = crate::BitReader<bool>;
#[doc = "Field `HEN31` writer - High detect enabled 31"]
pub type HEN31_W<'a> = crate::BitWriter<'a, u32, GPHEN0_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - High detect enabled 0"]
    #[inline(always)]
    pub fn hen0(&self) -> HEN0_R {
        HEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High detect enabled 1"]
    #[inline(always)]
    pub fn hen1(&self) -> HEN1_R {
        HEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High detect enabled 2"]
    #[inline(always)]
    pub fn hen2(&self) -> HEN2_R {
        HEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High detect enabled 3"]
    #[inline(always)]
    pub fn hen3(&self) -> HEN3_R {
        HEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High detect enabled 4"]
    #[inline(always)]
    pub fn hen4(&self) -> HEN4_R {
        HEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High detect enabled 5"]
    #[inline(always)]
    pub fn hen5(&self) -> HEN5_R {
        HEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High detect enabled 6"]
    #[inline(always)]
    pub fn hen6(&self) -> HEN6_R {
        HEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - High detect enabled 7"]
    #[inline(always)]
    pub fn hen7(&self) -> HEN7_R {
        HEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High detect enabled 8"]
    #[inline(always)]
    pub fn hen8(&self) -> HEN8_R {
        HEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - High detect enabled 9"]
    #[inline(always)]
    pub fn hen9(&self) -> HEN9_R {
        HEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - High detect enabled 10"]
    #[inline(always)]
    pub fn hen10(&self) -> HEN10_R {
        HEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - High detect enabled 11"]
    #[inline(always)]
    pub fn hen11(&self) -> HEN11_R {
        HEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - High detect enabled 12"]
    #[inline(always)]
    pub fn hen12(&self) -> HEN12_R {
        HEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - High detect enabled 13"]
    #[inline(always)]
    pub fn hen13(&self) -> HEN13_R {
        HEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - High detect enabled 14"]
    #[inline(always)]
    pub fn hen14(&self) -> HEN14_R {
        HEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - High detect enabled 15"]
    #[inline(always)]
    pub fn hen15(&self) -> HEN15_R {
        HEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - High detect enabled 16"]
    #[inline(always)]
    pub fn hen16(&self) -> HEN16_R {
        HEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High detect enabled 17"]
    #[inline(always)]
    pub fn hen17(&self) -> HEN17_R {
        HEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - High detect enabled 18"]
    #[inline(always)]
    pub fn hen18(&self) -> HEN18_R {
        HEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - High detect enabled 19"]
    #[inline(always)]
    pub fn hen19(&self) -> HEN19_R {
        HEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - High detect enabled 20"]
    #[inline(always)]
    pub fn hen20(&self) -> HEN20_R {
        HEN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - High detect enabled 21"]
    #[inline(always)]
    pub fn hen21(&self) -> HEN21_R {
        HEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - High detect enabled 22"]
    #[inline(always)]
    pub fn hen22(&self) -> HEN22_R {
        HEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - High detect enabled 23"]
    #[inline(always)]
    pub fn hen23(&self) -> HEN23_R {
        HEN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - High detect enabled 24"]
    #[inline(always)]
    pub fn hen24(&self) -> HEN24_R {
        HEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - High detect enabled 25"]
    #[inline(always)]
    pub fn hen25(&self) -> HEN25_R {
        HEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - High detect enabled 26"]
    #[inline(always)]
    pub fn hen26(&self) -> HEN26_R {
        HEN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - High detect enabled 27"]
    #[inline(always)]
    pub fn hen27(&self) -> HEN27_R {
        HEN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - High detect enabled 28"]
    #[inline(always)]
    pub fn hen28(&self) -> HEN28_R {
        HEN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - High detect enabled 29"]
    #[inline(always)]
    pub fn hen29(&self) -> HEN29_R {
        HEN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - High detect enabled 30"]
    #[inline(always)]
    pub fn hen30(&self) -> HEN30_R {
        HEN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - High detect enabled 31"]
    #[inline(always)]
    pub fn hen31(&self) -> HEN31_R {
        HEN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High detect enabled 0"]
    #[inline(always)]
    pub fn hen0(&mut self) -> HEN0_W {
        HEN0_W::new(self)
    }
    #[doc = "Bit 1 - High detect enabled 1"]
    #[inline(always)]
    pub fn hen1(&mut self) -> HEN1_W {
        HEN1_W::new(self)
    }
    #[doc = "Bit 2 - High detect enabled 2"]
    #[inline(always)]
    pub fn hen2(&mut self) -> HEN2_W {
        HEN2_W::new(self)
    }
    #[doc = "Bit 3 - High detect enabled 3"]
    #[inline(always)]
    pub fn hen3(&mut self) -> HEN3_W {
        HEN3_W::new(self)
    }
    #[doc = "Bit 4 - High detect enabled 4"]
    #[inline(always)]
    pub fn hen4(&mut self) -> HEN4_W {
        HEN4_W::new(self)
    }
    #[doc = "Bit 5 - High detect enabled 5"]
    #[inline(always)]
    pub fn hen5(&mut self) -> HEN5_W {
        HEN5_W::new(self)
    }
    #[doc = "Bit 6 - High detect enabled 6"]
    #[inline(always)]
    pub fn hen6(&mut self) -> HEN6_W {
        HEN6_W::new(self)
    }
    #[doc = "Bit 7 - High detect enabled 7"]
    #[inline(always)]
    pub fn hen7(&mut self) -> HEN7_W {
        HEN7_W::new(self)
    }
    #[doc = "Bit 8 - High detect enabled 8"]
    #[inline(always)]
    pub fn hen8(&mut self) -> HEN8_W {
        HEN8_W::new(self)
    }
    #[doc = "Bit 9 - High detect enabled 9"]
    #[inline(always)]
    pub fn hen9(&mut self) -> HEN9_W {
        HEN9_W::new(self)
    }
    #[doc = "Bit 10 - High detect enabled 10"]
    #[inline(always)]
    pub fn hen10(&mut self) -> HEN10_W {
        HEN10_W::new(self)
    }
    #[doc = "Bit 11 - High detect enabled 11"]
    #[inline(always)]
    pub fn hen11(&mut self) -> HEN11_W {
        HEN11_W::new(self)
    }
    #[doc = "Bit 12 - High detect enabled 12"]
    #[inline(always)]
    pub fn hen12(&mut self) -> HEN12_W {
        HEN12_W::new(self)
    }
    #[doc = "Bit 13 - High detect enabled 13"]
    #[inline(always)]
    pub fn hen13(&mut self) -> HEN13_W {
        HEN13_W::new(self)
    }
    #[doc = "Bit 14 - High detect enabled 14"]
    #[inline(always)]
    pub fn hen14(&mut self) -> HEN14_W {
        HEN14_W::new(self)
    }
    #[doc = "Bit 15 - High detect enabled 15"]
    #[inline(always)]
    pub fn hen15(&mut self) -> HEN15_W {
        HEN15_W::new(self)
    }
    #[doc = "Bit 16 - High detect enabled 16"]
    #[inline(always)]
    pub fn hen16(&mut self) -> HEN16_W {
        HEN16_W::new(self)
    }
    #[doc = "Bit 17 - High detect enabled 17"]
    #[inline(always)]
    pub fn hen17(&mut self) -> HEN17_W {
        HEN17_W::new(self)
    }
    #[doc = "Bit 18 - High detect enabled 18"]
    #[inline(always)]
    pub fn hen18(&mut self) -> HEN18_W {
        HEN18_W::new(self)
    }
    #[doc = "Bit 19 - High detect enabled 19"]
    #[inline(always)]
    pub fn hen19(&mut self) -> HEN19_W {
        HEN19_W::new(self)
    }
    #[doc = "Bit 20 - High detect enabled 20"]
    #[inline(always)]
    pub fn hen20(&mut self) -> HEN20_W {
        HEN20_W::new(self)
    }
    #[doc = "Bit 21 - High detect enabled 21"]
    #[inline(always)]
    pub fn hen21(&mut self) -> HEN21_W {
        HEN21_W::new(self)
    }
    #[doc = "Bit 22 - High detect enabled 22"]
    #[inline(always)]
    pub fn hen22(&mut self) -> HEN22_W {
        HEN22_W::new(self)
    }
    #[doc = "Bit 23 - High detect enabled 23"]
    #[inline(always)]
    pub fn hen23(&mut self) -> HEN23_W {
        HEN23_W::new(self)
    }
    #[doc = "Bit 24 - High detect enabled 24"]
    #[inline(always)]
    pub fn hen24(&mut self) -> HEN24_W {
        HEN24_W::new(self)
    }
    #[doc = "Bit 25 - High detect enabled 25"]
    #[inline(always)]
    pub fn hen25(&mut self) -> HEN25_W {
        HEN25_W::new(self)
    }
    #[doc = "Bit 26 - High detect enabled 26"]
    #[inline(always)]
    pub fn hen26(&mut self) -> HEN26_W {
        HEN26_W::new(self)
    }
    #[doc = "Bit 27 - High detect enabled 27"]
    #[inline(always)]
    pub fn hen27(&mut self) -> HEN27_W {
        HEN27_W::new(self)
    }
    #[doc = "Bit 28 - High detect enabled 28"]
    #[inline(always)]
    pub fn hen28(&mut self) -> HEN28_W {
        HEN28_W::new(self)
    }
    #[doc = "Bit 29 - High detect enabled 29"]
    #[inline(always)]
    pub fn hen29(&mut self) -> HEN29_W {
        HEN29_W::new(self)
    }
    #[doc = "Bit 30 - High detect enabled 30"]
    #[inline(always)]
    pub fn hen30(&mut self) -> HEN30_W {
        HEN30_W::new(self)
    }
    #[doc = "Bit 31 - High detect enabled 31"]
    #[inline(always)]
    pub fn hen31(&mut self) -> HEN31_W {
        HEN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin High Detect Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gphen0](index.html) module"]
pub struct GPHEN0_SPEC;
impl crate::RegisterSpec for GPHEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gphen0::R](R) reader structure"]
impl crate::Readable for GPHEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gphen0::W](W) writer structure"]
impl crate::Writable for GPHEN0_SPEC {
    type Writer = W;
}
