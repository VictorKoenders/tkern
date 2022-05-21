#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG0` reader"]
pub struct R(crate::R<GPIO_PUP_PDN_CNTRL_REG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_PUP_PDN_CNTRL_REG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_PUP_PDN_CNTRL_REG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_PUP_PDN_CNTRL_REG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG0` writer"]
pub struct W(crate::W<GPIO_PUP_PDN_CNTRL_REG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_PUP_PDN_CNTRL_REG0_SPEC>;
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
impl From<crate::W<GPIO_PUP_PDN_CNTRL_REG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_PUP_PDN_CNTRL_REG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Resistor select for 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO_PUP_PDN_CNTRL0_A {
    #[doc = "0: No pull"]
    NONE = 0,
    #[doc = "1: Pull up"]
    UP = 1,
    #[doc = "2: Pull down"]
    DOWN = 2,
}
impl From<GPIO_PUP_PDN_CNTRL0_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO_PUP_PDN_CNTRL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_PUP_PDN_CNTRL0` reader - Resistor select for 0"]
pub type GPIO_PUP_PDN_CNTRL0_R = crate::FieldReader<u8, GPIO_PUP_PDN_CNTRL0_A>;
impl GPIO_PUP_PDN_CNTRL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_PUP_PDN_CNTRL0_A> {
        match self.bits {
            0 => Some(GPIO_PUP_PDN_CNTRL0_A::NONE),
            1 => Some(GPIO_PUP_PDN_CNTRL0_A::UP),
            2 => Some(GPIO_PUP_PDN_CNTRL0_A::DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == GPIO_PUP_PDN_CNTRL0_A::NONE
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == GPIO_PUP_PDN_CNTRL0_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == GPIO_PUP_PDN_CNTRL0_A::DOWN
    }
}
#[doc = "Field `GPIO_PUP_PDN_CNTRL0` writer - Resistor select for 0"]
pub type GPIO_PUP_PDN_CNTRL0_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL0_A, 2, 0>;
impl<'a> GPIO_PUP_PDN_CNTRL0_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL0_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL0_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL0_A::DOWN)
    }
}
#[doc = "Resistor select for 1"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL1_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL1` reader - Resistor select for 1"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL1_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL1` writer - Resistor select for 1"]
pub type GPIO_PUP_PDN_CNTRL1_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL1_A, 2, 2>;
impl<'a> GPIO_PUP_PDN_CNTRL1_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL1_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL1_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL1_A::DOWN)
    }
}
#[doc = "Resistor select for 2"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL2_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL2` reader - Resistor select for 2"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL2_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL2` writer - Resistor select for 2"]
pub type GPIO_PUP_PDN_CNTRL2_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL2_A, 2, 4>;
impl<'a> GPIO_PUP_PDN_CNTRL2_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL2_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL2_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL2_A::DOWN)
    }
}
#[doc = "Resistor select for 3"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL3_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL3` reader - Resistor select for 3"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL3_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL3` writer - Resistor select for 3"]
pub type GPIO_PUP_PDN_CNTRL3_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL3_A, 2, 6>;
impl<'a> GPIO_PUP_PDN_CNTRL3_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL3_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL3_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL3_A::DOWN)
    }
}
#[doc = "Resistor select for 4"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL4_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL4` reader - Resistor select for 4"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL4_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL4` writer - Resistor select for 4"]
pub type GPIO_PUP_PDN_CNTRL4_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL4_A, 2, 8>;
impl<'a> GPIO_PUP_PDN_CNTRL4_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL4_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL4_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL4_A::DOWN)
    }
}
#[doc = "Resistor select for 5"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL5_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL5` reader - Resistor select for 5"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL5_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL5` writer - Resistor select for 5"]
pub type GPIO_PUP_PDN_CNTRL5_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL5_A, 2, 10>;
impl<'a> GPIO_PUP_PDN_CNTRL5_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL5_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL5_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL5_A::DOWN)
    }
}
#[doc = "Resistor select for 6"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL6_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL6` reader - Resistor select for 6"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL6_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL6` writer - Resistor select for 6"]
pub type GPIO_PUP_PDN_CNTRL6_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL6_A, 2, 12>;
impl<'a> GPIO_PUP_PDN_CNTRL6_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL6_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL6_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL6_A::DOWN)
    }
}
#[doc = "Resistor select for 7"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL7_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL7` reader - Resistor select for 7"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL7_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL7` writer - Resistor select for 7"]
pub type GPIO_PUP_PDN_CNTRL7_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL7_A, 2, 14>;
impl<'a> GPIO_PUP_PDN_CNTRL7_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL7_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL7_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL7_A::DOWN)
    }
}
#[doc = "Resistor select for 8"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL8_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL8` reader - Resistor select for 8"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL8_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL8` writer - Resistor select for 8"]
pub type GPIO_PUP_PDN_CNTRL8_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL8_A, 2, 16>;
impl<'a> GPIO_PUP_PDN_CNTRL8_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL8_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL8_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL8_A::DOWN)
    }
}
#[doc = "Resistor select for 9"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL9_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL9` reader - Resistor select for 9"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL9_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL9` writer - Resistor select for 9"]
pub type GPIO_PUP_PDN_CNTRL9_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL9_A, 2, 18>;
impl<'a> GPIO_PUP_PDN_CNTRL9_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL9_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL9_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL9_A::DOWN)
    }
}
#[doc = "Resistor select for 10"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL10_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL10` reader - Resistor select for 10"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL10_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL10` writer - Resistor select for 10"]
pub type GPIO_PUP_PDN_CNTRL10_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL10_A, 2, 20>;
impl<'a> GPIO_PUP_PDN_CNTRL10_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL10_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL10_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL10_A::DOWN)
    }
}
#[doc = "Resistor select for 11"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL11_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL11` reader - Resistor select for 11"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL11_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL11` writer - Resistor select for 11"]
pub type GPIO_PUP_PDN_CNTRL11_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL11_A, 2, 22>;
impl<'a> GPIO_PUP_PDN_CNTRL11_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL11_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL11_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL11_A::DOWN)
    }
}
#[doc = "Resistor select for 12"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL12_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL12` reader - Resistor select for 12"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL12_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL12` writer - Resistor select for 12"]
pub type GPIO_PUP_PDN_CNTRL12_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL12_A, 2, 24>;
impl<'a> GPIO_PUP_PDN_CNTRL12_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL12_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL12_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL12_A::DOWN)
    }
}
#[doc = "Resistor select for 13"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL13_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL13` reader - Resistor select for 13"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL13_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL13` writer - Resistor select for 13"]
pub type GPIO_PUP_PDN_CNTRL13_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL13_A, 2, 26>;
impl<'a> GPIO_PUP_PDN_CNTRL13_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL13_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL13_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL13_A::DOWN)
    }
}
#[doc = "Resistor select for 14"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL14_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL14` reader - Resistor select for 14"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL14_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL14` writer - Resistor select for 14"]
pub type GPIO_PUP_PDN_CNTRL14_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL14_A, 2, 28>;
impl<'a> GPIO_PUP_PDN_CNTRL14_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL14_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL14_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL14_A::DOWN)
    }
}
#[doc = "Resistor select for 15"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_A as GPIO_PUP_PDN_CNTRL15_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL15` reader - Resistor select for 15"]
pub use crate::gpio::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL15_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL15` writer - Resistor select for 15"]
pub type GPIO_PUP_PDN_CNTRL15_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, GPIO_PUP_PDN_CNTRL15_A, 2, 30>;
impl<'a> GPIO_PUP_PDN_CNTRL15_W<'a> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL15_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL15_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(GPIO_PUP_PDN_CNTRL15_A::DOWN)
    }
}
impl R {
    #[doc = "Bits 0:1 - Resistor select for 0"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl0(&self) -> GPIO_PUP_PDN_CNTRL0_R {
        GPIO_PUP_PDN_CNTRL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Resistor select for 1"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl1(&self) -> GPIO_PUP_PDN_CNTRL1_R {
        GPIO_PUP_PDN_CNTRL1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Resistor select for 2"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl2(&self) -> GPIO_PUP_PDN_CNTRL2_R {
        GPIO_PUP_PDN_CNTRL2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Resistor select for 3"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl3(&self) -> GPIO_PUP_PDN_CNTRL3_R {
        GPIO_PUP_PDN_CNTRL3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resistor select for 4"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl4(&self) -> GPIO_PUP_PDN_CNTRL4_R {
        GPIO_PUP_PDN_CNTRL4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Resistor select for 5"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl5(&self) -> GPIO_PUP_PDN_CNTRL5_R {
        GPIO_PUP_PDN_CNTRL5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Resistor select for 6"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl6(&self) -> GPIO_PUP_PDN_CNTRL6_R {
        GPIO_PUP_PDN_CNTRL6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Resistor select for 7"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl7(&self) -> GPIO_PUP_PDN_CNTRL7_R {
        GPIO_PUP_PDN_CNTRL7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Resistor select for 8"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl8(&self) -> GPIO_PUP_PDN_CNTRL8_R {
        GPIO_PUP_PDN_CNTRL8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Resistor select for 9"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl9(&self) -> GPIO_PUP_PDN_CNTRL9_R {
        GPIO_PUP_PDN_CNTRL9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Resistor select for 10"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl10(&self) -> GPIO_PUP_PDN_CNTRL10_R {
        GPIO_PUP_PDN_CNTRL10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Resistor select for 11"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl11(&self) -> GPIO_PUP_PDN_CNTRL11_R {
        GPIO_PUP_PDN_CNTRL11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Resistor select for 12"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl12(&self) -> GPIO_PUP_PDN_CNTRL12_R {
        GPIO_PUP_PDN_CNTRL12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Resistor select for 13"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl13(&self) -> GPIO_PUP_PDN_CNTRL13_R {
        GPIO_PUP_PDN_CNTRL13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Resistor select for 14"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl14(&self) -> GPIO_PUP_PDN_CNTRL14_R {
        GPIO_PUP_PDN_CNTRL14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Resistor select for 15"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl15(&self) -> GPIO_PUP_PDN_CNTRL15_R {
        GPIO_PUP_PDN_CNTRL15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor select for 0"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl0(&mut self) -> GPIO_PUP_PDN_CNTRL0_W {
        GPIO_PUP_PDN_CNTRL0_W::new(self)
    }
    #[doc = "Bits 2:3 - Resistor select for 1"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl1(&mut self) -> GPIO_PUP_PDN_CNTRL1_W {
        GPIO_PUP_PDN_CNTRL1_W::new(self)
    }
    #[doc = "Bits 4:5 - Resistor select for 2"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl2(&mut self) -> GPIO_PUP_PDN_CNTRL2_W {
        GPIO_PUP_PDN_CNTRL2_W::new(self)
    }
    #[doc = "Bits 6:7 - Resistor select for 3"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl3(&mut self) -> GPIO_PUP_PDN_CNTRL3_W {
        GPIO_PUP_PDN_CNTRL3_W::new(self)
    }
    #[doc = "Bits 8:9 - Resistor select for 4"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl4(&mut self) -> GPIO_PUP_PDN_CNTRL4_W {
        GPIO_PUP_PDN_CNTRL4_W::new(self)
    }
    #[doc = "Bits 10:11 - Resistor select for 5"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl5(&mut self) -> GPIO_PUP_PDN_CNTRL5_W {
        GPIO_PUP_PDN_CNTRL5_W::new(self)
    }
    #[doc = "Bits 12:13 - Resistor select for 6"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl6(&mut self) -> GPIO_PUP_PDN_CNTRL6_W {
        GPIO_PUP_PDN_CNTRL6_W::new(self)
    }
    #[doc = "Bits 14:15 - Resistor select for 7"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl7(&mut self) -> GPIO_PUP_PDN_CNTRL7_W {
        GPIO_PUP_PDN_CNTRL7_W::new(self)
    }
    #[doc = "Bits 16:17 - Resistor select for 8"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl8(&mut self) -> GPIO_PUP_PDN_CNTRL8_W {
        GPIO_PUP_PDN_CNTRL8_W::new(self)
    }
    #[doc = "Bits 18:19 - Resistor select for 9"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl9(&mut self) -> GPIO_PUP_PDN_CNTRL9_W {
        GPIO_PUP_PDN_CNTRL9_W::new(self)
    }
    #[doc = "Bits 20:21 - Resistor select for 10"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl10(&mut self) -> GPIO_PUP_PDN_CNTRL10_W {
        GPIO_PUP_PDN_CNTRL10_W::new(self)
    }
    #[doc = "Bits 22:23 - Resistor select for 11"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl11(&mut self) -> GPIO_PUP_PDN_CNTRL11_W {
        GPIO_PUP_PDN_CNTRL11_W::new(self)
    }
    #[doc = "Bits 24:25 - Resistor select for 12"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl12(&mut self) -> GPIO_PUP_PDN_CNTRL12_W {
        GPIO_PUP_PDN_CNTRL12_W::new(self)
    }
    #[doc = "Bits 26:27 - Resistor select for 13"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl13(&mut self) -> GPIO_PUP_PDN_CNTRL13_W {
        GPIO_PUP_PDN_CNTRL13_W::new(self)
    }
    #[doc = "Bits 28:29 - Resistor select for 14"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl14(&mut self) -> GPIO_PUP_PDN_CNTRL14_W {
        GPIO_PUP_PDN_CNTRL14_W::new(self)
    }
    #[doc = "Bits 30:31 - Resistor select for 15"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl15(&mut self) -> GPIO_PUP_PDN_CNTRL15_W {
        GPIO_PUP_PDN_CNTRL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pull-up / Pull-down Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pup_pdn_cntrl_reg0](index.html) module"]
pub struct GPIO_PUP_PDN_CNTRL_REG0_SPEC;
impl crate::RegisterSpec for GPIO_PUP_PDN_CNTRL_REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_pup_pdn_cntrl_reg0::R](R) reader structure"]
impl crate::Readable for GPIO_PUP_PDN_CNTRL_REG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_pup_pdn_cntrl_reg0::W](W) writer structure"]
impl crate::Writable for GPIO_PUP_PDN_CNTRL_REG0_SPEC {
    type Writer = W;
}
