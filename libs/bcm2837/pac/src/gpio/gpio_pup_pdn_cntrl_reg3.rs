#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG3` reader"]
pub struct R(crate::R<GPIO_PUP_PDN_CNTRL_REG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_PUP_PDN_CNTRL_REG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_PUP_PDN_CNTRL_REG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_PUP_PDN_CNTRL_REG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG3` writer"]
pub struct W(crate::W<GPIO_PUP_PDN_CNTRL_REG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_PUP_PDN_CNTRL_REG3_SPEC>;
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
impl From<crate::W<GPIO_PUP_PDN_CNTRL_REG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_PUP_PDN_CNTRL_REG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Resistor select for 48"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL48_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL48` reader - Resistor select for 48"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL48_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL48` writer - Resistor select for 48"]
pub type GPIO_PUP_PDN_CNTRL48_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG3_SPEC, u8, GPIO_PUP_PDN_CNTRL48_A, 2, 0>;
impl<'a> GPIO_PUP_PDN_CNTRL48_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL48_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL48_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL48_A::DOWN)
    }
}
#[doc = "Resistor select for 49"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL49_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL49` reader - Resistor select for 49"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL49_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL49` writer - Resistor select for 49"]
pub type GPIO_PUP_PDN_CNTRL49_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG3_SPEC, u8, GPIO_PUP_PDN_CNTRL49_A, 2, 2>;
impl<'a> GPIO_PUP_PDN_CNTRL49_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL49_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL49_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL49_A::DOWN)
    }
}
#[doc = "Resistor select for 50"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL50_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL50` reader - Resistor select for 50"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL50_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL50` writer - Resistor select for 50"]
pub type GPIO_PUP_PDN_CNTRL50_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG3_SPEC, u8, GPIO_PUP_PDN_CNTRL50_A, 2, 4>;
impl<'a> GPIO_PUP_PDN_CNTRL50_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL50_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL50_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL50_A::DOWN)
    }
}
#[doc = "Resistor select for 51"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL51_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL51` reader - Resistor select for 51"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL51_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL51` writer - Resistor select for 51"]
pub type GPIO_PUP_PDN_CNTRL51_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG3_SPEC, u8, GPIO_PUP_PDN_CNTRL51_A, 2, 6>;
impl<'a> GPIO_PUP_PDN_CNTRL51_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL51_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL51_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL51_A::DOWN)
    }
}
#[doc = "Resistor select for 52"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL52_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL52` reader - Resistor select for 52"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL52_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL52` writer - Resistor select for 52"]
pub type GPIO_PUP_PDN_CNTRL52_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG3_SPEC, u8, GPIO_PUP_PDN_CNTRL52_A, 2, 8>;
impl<'a> GPIO_PUP_PDN_CNTRL52_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL52_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL52_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL52_A::DOWN)
    }
}
#[doc = "Resistor select for 53"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL53_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL53` reader - Resistor select for 53"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL53_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL53` writer - Resistor select for 53"]
pub type GPIO_PUP_PDN_CNTRL53_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG3_SPEC, u8, GPIO_PUP_PDN_CNTRL53_A, 2, 10>;
impl<'a> GPIO_PUP_PDN_CNTRL53_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL53_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL53_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL53_A::DOWN)
    }
}
impl R {
    #[doc = "Bits 0:1 - Resistor select for 48"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl48(&self) -> GPIO_PUP_PDN_CNTRL48_R {
        GPIO_PUP_PDN_CNTRL48_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Resistor select for 49"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl49(&self) -> GPIO_PUP_PDN_CNTRL49_R {
        GPIO_PUP_PDN_CNTRL49_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Resistor select for 50"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl50(&self) -> GPIO_PUP_PDN_CNTRL50_R {
        GPIO_PUP_PDN_CNTRL50_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Resistor select for 51"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl51(&self) -> GPIO_PUP_PDN_CNTRL51_R {
        GPIO_PUP_PDN_CNTRL51_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resistor select for 52"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl52(&self) -> GPIO_PUP_PDN_CNTRL52_R {
        GPIO_PUP_PDN_CNTRL52_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Resistor select for 53"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl53(&self) -> GPIO_PUP_PDN_CNTRL53_R {
        GPIO_PUP_PDN_CNTRL53_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor select for 48"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl48(&mut self) -> GPIO_PUP_PDN_CNTRL48_W {
        GPIO_PUP_PDN_CNTRL48_W::new(self)
    }
    #[doc = "Bits 2:3 - Resistor select for 49"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl49(&mut self) -> GPIO_PUP_PDN_CNTRL49_W {
        GPIO_PUP_PDN_CNTRL49_W::new(self)
    }
    #[doc = "Bits 4:5 - Resistor select for 50"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl50(&mut self) -> GPIO_PUP_PDN_CNTRL50_W {
        GPIO_PUP_PDN_CNTRL50_W::new(self)
    }
    #[doc = "Bits 6:7 - Resistor select for 51"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl51(&mut self) -> GPIO_PUP_PDN_CNTRL51_W {
        GPIO_PUP_PDN_CNTRL51_W::new(self)
    }
    #[doc = "Bits 8:9 - Resistor select for 52"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl52(&mut self) -> GPIO_PUP_PDN_CNTRL52_W {
        GPIO_PUP_PDN_CNTRL52_W::new(self)
    }
    #[doc = "Bits 10:11 - Resistor select for 53"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl53(&mut self) -> GPIO_PUP_PDN_CNTRL53_W {
        GPIO_PUP_PDN_CNTRL53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pull-up / Pull-down Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pup_pdn_cntrl_reg3](index.html) module"]
pub struct GPIO_PUP_PDN_CNTRL_REG3_SPEC;
impl crate::RegisterSpec for GPIO_PUP_PDN_CNTRL_REG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_pup_pdn_cntrl_reg3::R](R) reader structure"]
impl crate::Readable for GPIO_PUP_PDN_CNTRL_REG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_pup_pdn_cntrl_reg3::W](W) writer structure"]
impl crate::Writable for GPIO_PUP_PDN_CNTRL_REG3_SPEC {
    type Writer = W;
}
