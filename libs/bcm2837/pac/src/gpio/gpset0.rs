#[doc = "Register `GPSET0` writer"]
pub struct W(crate::W<GPSET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPSET0_SPEC>;
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
impl From<crate::W<GPSET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPSET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET0` writer - Set 0"]
pub type SET0_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 0>;
#[doc = "Field `SET1` writer - Set 1"]
pub type SET1_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 1>;
#[doc = "Field `SET2` writer - Set 2"]
pub type SET2_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 2>;
#[doc = "Field `SET3` writer - Set 3"]
pub type SET3_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 3>;
#[doc = "Field `SET4` writer - Set 4"]
pub type SET4_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 4>;
#[doc = "Field `SET5` writer - Set 5"]
pub type SET5_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 5>;
#[doc = "Field `SET6` writer - Set 6"]
pub type SET6_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 6>;
#[doc = "Field `SET7` writer - Set 7"]
pub type SET7_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 7>;
#[doc = "Field `SET8` writer - Set 8"]
pub type SET8_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 8>;
#[doc = "Field `SET9` writer - Set 9"]
pub type SET9_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 9>;
#[doc = "Field `SET10` writer - Set 10"]
pub type SET10_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 10>;
#[doc = "Field `SET11` writer - Set 11"]
pub type SET11_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 11>;
#[doc = "Field `SET12` writer - Set 12"]
pub type SET12_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 12>;
#[doc = "Field `SET13` writer - Set 13"]
pub type SET13_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 13>;
#[doc = "Field `SET14` writer - Set 14"]
pub type SET14_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 14>;
#[doc = "Field `SET15` writer - Set 15"]
pub type SET15_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 15>;
#[doc = "Field `SET16` writer - Set 16"]
pub type SET16_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 16>;
#[doc = "Field `SET17` writer - Set 17"]
pub type SET17_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 17>;
#[doc = "Field `SET18` writer - Set 18"]
pub type SET18_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 18>;
#[doc = "Field `SET19` writer - Set 19"]
pub type SET19_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 19>;
#[doc = "Field `SET20` writer - Set 20"]
pub type SET20_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 20>;
#[doc = "Field `SET21` writer - Set 21"]
pub type SET21_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 21>;
#[doc = "Field `SET22` writer - Set 22"]
pub type SET22_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 22>;
#[doc = "Field `SET23` writer - Set 23"]
pub type SET23_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 23>;
#[doc = "Field `SET24` writer - Set 24"]
pub type SET24_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 24>;
#[doc = "Field `SET25` writer - Set 25"]
pub type SET25_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 25>;
#[doc = "Field `SET26` writer - Set 26"]
pub type SET26_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 26>;
#[doc = "Field `SET27` writer - Set 27"]
pub type SET27_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 27>;
#[doc = "Field `SET28` writer - Set 28"]
pub type SET28_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 28>;
#[doc = "Field `SET29` writer - Set 29"]
pub type SET29_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 29>;
#[doc = "Field `SET30` writer - Set 30"]
pub type SET30_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 30>;
#[doc = "Field `SET31` writer - Set 31"]
pub type SET31_W<'a> = crate::BitWriter1S<'a, u32, GPSET0_SPEC, bool, 31>;
impl W {
    #[doc = "Bit 0 - Set 0"]
    #[inline(always)]
    pub fn set0(&mut self) -> SET0_W {
        SET0_W::new(self)
    }
    #[doc = "Bit 1 - Set 1"]
    #[inline(always)]
    pub fn set1(&mut self) -> SET1_W {
        SET1_W::new(self)
    }
    #[doc = "Bit 2 - Set 2"]
    #[inline(always)]
    pub fn set2(&mut self) -> SET2_W {
        SET2_W::new(self)
    }
    #[doc = "Bit 3 - Set 3"]
    #[inline(always)]
    pub fn set3(&mut self) -> SET3_W {
        SET3_W::new(self)
    }
    #[doc = "Bit 4 - Set 4"]
    #[inline(always)]
    pub fn set4(&mut self) -> SET4_W {
        SET4_W::new(self)
    }
    #[doc = "Bit 5 - Set 5"]
    #[inline(always)]
    pub fn set5(&mut self) -> SET5_W {
        SET5_W::new(self)
    }
    #[doc = "Bit 6 - Set 6"]
    #[inline(always)]
    pub fn set6(&mut self) -> SET6_W {
        SET6_W::new(self)
    }
    #[doc = "Bit 7 - Set 7"]
    #[inline(always)]
    pub fn set7(&mut self) -> SET7_W {
        SET7_W::new(self)
    }
    #[doc = "Bit 8 - Set 8"]
    #[inline(always)]
    pub fn set8(&mut self) -> SET8_W {
        SET8_W::new(self)
    }
    #[doc = "Bit 9 - Set 9"]
    #[inline(always)]
    pub fn set9(&mut self) -> SET9_W {
        SET9_W::new(self)
    }
    #[doc = "Bit 10 - Set 10"]
    #[inline(always)]
    pub fn set10(&mut self) -> SET10_W {
        SET10_W::new(self)
    }
    #[doc = "Bit 11 - Set 11"]
    #[inline(always)]
    pub fn set11(&mut self) -> SET11_W {
        SET11_W::new(self)
    }
    #[doc = "Bit 12 - Set 12"]
    #[inline(always)]
    pub fn set12(&mut self) -> SET12_W {
        SET12_W::new(self)
    }
    #[doc = "Bit 13 - Set 13"]
    #[inline(always)]
    pub fn set13(&mut self) -> SET13_W {
        SET13_W::new(self)
    }
    #[doc = "Bit 14 - Set 14"]
    #[inline(always)]
    pub fn set14(&mut self) -> SET14_W {
        SET14_W::new(self)
    }
    #[doc = "Bit 15 - Set 15"]
    #[inline(always)]
    pub fn set15(&mut self) -> SET15_W {
        SET15_W::new(self)
    }
    #[doc = "Bit 16 - Set 16"]
    #[inline(always)]
    pub fn set16(&mut self) -> SET16_W {
        SET16_W::new(self)
    }
    #[doc = "Bit 17 - Set 17"]
    #[inline(always)]
    pub fn set17(&mut self) -> SET17_W {
        SET17_W::new(self)
    }
    #[doc = "Bit 18 - Set 18"]
    #[inline(always)]
    pub fn set18(&mut self) -> SET18_W {
        SET18_W::new(self)
    }
    #[doc = "Bit 19 - Set 19"]
    #[inline(always)]
    pub fn set19(&mut self) -> SET19_W {
        SET19_W::new(self)
    }
    #[doc = "Bit 20 - Set 20"]
    #[inline(always)]
    pub fn set20(&mut self) -> SET20_W {
        SET20_W::new(self)
    }
    #[doc = "Bit 21 - Set 21"]
    #[inline(always)]
    pub fn set21(&mut self) -> SET21_W {
        SET21_W::new(self)
    }
    #[doc = "Bit 22 - Set 22"]
    #[inline(always)]
    pub fn set22(&mut self) -> SET22_W {
        SET22_W::new(self)
    }
    #[doc = "Bit 23 - Set 23"]
    #[inline(always)]
    pub fn set23(&mut self) -> SET23_W {
        SET23_W::new(self)
    }
    #[doc = "Bit 24 - Set 24"]
    #[inline(always)]
    pub fn set24(&mut self) -> SET24_W {
        SET24_W::new(self)
    }
    #[doc = "Bit 25 - Set 25"]
    #[inline(always)]
    pub fn set25(&mut self) -> SET25_W {
        SET25_W::new(self)
    }
    #[doc = "Bit 26 - Set 26"]
    #[inline(always)]
    pub fn set26(&mut self) -> SET26_W {
        SET26_W::new(self)
    }
    #[doc = "Bit 27 - Set 27"]
    #[inline(always)]
    pub fn set27(&mut self) -> SET27_W {
        SET27_W::new(self)
    }
    #[doc = "Bit 28 - Set 28"]
    #[inline(always)]
    pub fn set28(&mut self) -> SET28_W {
        SET28_W::new(self)
    }
    #[doc = "Bit 29 - Set 29"]
    #[inline(always)]
    pub fn set29(&mut self) -> SET29_W {
        SET29_W::new(self)
    }
    #[doc = "Bit 30 - Set 30"]
    #[inline(always)]
    pub fn set30(&mut self) -> SET30_W {
        SET30_W::new(self)
    }
    #[doc = "Bit 31 - Set 31"]
    #[inline(always)]
    pub fn set31(&mut self) -> SET31_W {
        SET31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Output Set 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpset0](index.html) module"]
pub struct GPSET0_SPEC;
impl crate::RegisterSpec for GPSET0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpset0::W](W) writer structure"]
impl crate::Writable for GPSET0_SPEC {
    type Writer = W;
}
