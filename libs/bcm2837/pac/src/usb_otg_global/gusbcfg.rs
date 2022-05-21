#[doc = "Register `GUSBCFG` reader"]
pub struct R(crate::R<GUSBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GUSBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GUSBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GUSBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GUSBCFG` writer"]
pub struct W(crate::W<GUSBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GUSBCFG_SPEC>;
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
impl From<crate::W<GUSBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GUSBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOCAL` reader - FS timeout calibration"]
pub type TOCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCAL` writer - FS timeout calibration"]
pub type TOCAL_W<'a> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 3, 0>;
#[doc = "PHY Interface width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYIF_A {
    #[doc = "0: `0`"]
    _8BIT = 0,
    #[doc = "1: `1`"]
    _16BIT = 1,
}
impl From<PHYIF_A> for bool {
    #[inline(always)]
    fn from(variant: PHYIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYIF` reader - PHY Interface width"]
pub type PHYIF_R = crate::BitReader<PHYIF_A>;
impl PHYIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYIF_A {
        match self.bits {
            false => PHYIF_A::_8BIT,
            true => PHYIF_A::_16BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == PHYIF_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == PHYIF_A::_16BIT
    }
}
#[doc = "Field `PHYIF` writer - PHY Interface width"]
pub type PHYIF_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, PHYIF_A, 3>;
impl<'a> PHYIF_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(PHYIF_A::_8BIT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(PHYIF_A::_16BIT)
    }
}
#[doc = "PHY Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYTYPE_A {
    #[doc = "0: `0`"]
    UTMI = 0,
    #[doc = "1: `1`"]
    ULPI = 1,
}
impl From<PHYTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: PHYTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYTYPE` reader - PHY Type"]
pub type PHYTYPE_R = crate::BitReader<PHYTYPE_A>;
impl PHYTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYTYPE_A {
        match self.bits {
            false => PHYTYPE_A::UTMI,
            true => PHYTYPE_A::ULPI,
        }
    }
    #[doc = "Checks if the value of the field is `UTMI`"]
    #[inline(always)]
    pub fn is_utmi(&self) -> bool {
        *self == PHYTYPE_A::UTMI
    }
    #[doc = "Checks if the value of the field is `ULPI`"]
    #[inline(always)]
    pub fn is_ulpi(&self) -> bool {
        *self == PHYTYPE_A::ULPI
    }
}
#[doc = "Field `PHYTYPE` writer - PHY Type"]
pub type PHYTYPE_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, PHYTYPE_A, 4>;
impl<'a> PHYTYPE_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn utmi(self) -> &'a mut W {
        self.variant(PHYTYPE_A::UTMI)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ulpi(self) -> &'a mut W {
        self.variant(PHYTYPE_A::ULPI)
    }
}
#[doc = "Full speed interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSIF_A {
    #[doc = "0: `0`"]
    _6PIN = 0,
    #[doc = "1: `1`"]
    _3PIN = 1,
}
impl From<FSIF_A> for bool {
    #[inline(always)]
    fn from(variant: FSIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSIF` reader - Full speed interface"]
pub type FSIF_R = crate::BitReader<FSIF_A>;
impl FSIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSIF_A {
        match self.bits {
            false => FSIF_A::_6PIN,
            true => FSIF_A::_3PIN,
        }
    }
    #[doc = "Checks if the value of the field is `_6PIN`"]
    #[inline(always)]
    pub fn is_6pin(&self) -> bool {
        *self == FSIF_A::_6PIN
    }
    #[doc = "Checks if the value of the field is `_3PIN`"]
    #[inline(always)]
    pub fn is_3pin(&self) -> bool {
        *self == FSIF_A::_3PIN
    }
}
#[doc = "Field `FSIF` writer - Full speed interface"]
pub type FSIF_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, FSIF_A, 5>;
impl<'a> FSIF_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _6pin(self) -> &'a mut W {
        self.variant(FSIF_A::_6PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _3pin(self) -> &'a mut W {
        self.variant(FSIF_A::_3PIN)
    }
}
#[doc = "Transceiver select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYSEL_A {
    #[doc = "0: `0`"]
    USB20 = 0,
    #[doc = "1: `1`"]
    USB11 = 1,
}
impl From<PHYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PHYSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYSEL` reader - Transceiver select"]
pub type PHYSEL_R = crate::BitReader<PHYSEL_A>;
impl PHYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYSEL_A {
        match self.bits {
            false => PHYSEL_A::USB20,
            true => PHYSEL_A::USB11,
        }
    }
    #[doc = "Checks if the value of the field is `USB20`"]
    #[inline(always)]
    pub fn is_usb20(&self) -> bool {
        *self == PHYSEL_A::USB20
    }
    #[doc = "Checks if the value of the field is `USB11`"]
    #[inline(always)]
    pub fn is_usb11(&self) -> bool {
        *self == PHYSEL_A::USB11
    }
}
#[doc = "Field `PHYSEL` writer - Transceiver select"]
pub type PHYSEL_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, PHYSEL_A, 6>;
impl<'a> PHYSEL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn usb20(self) -> &'a mut W {
        self.variant(PHYSEL_A::USB20)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn usb11(self) -> &'a mut W {
        self.variant(PHYSEL_A::USB11)
    }
}
#[doc = "ULPI data rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRSEL_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    DOUBLE = 1,
}
impl From<DDRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DDRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRSEL` reader - ULPI data rate"]
pub type DDRSEL_R = crate::BitReader<DDRSEL_A>;
impl DDRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRSEL_A {
        match self.bits {
            false => DDRSEL_A::SINGLE,
            true => DDRSEL_A::DOUBLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DDRSEL_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == DDRSEL_A::DOUBLE
    }
}
#[doc = "Field `DDRSEL` writer - ULPI data rate"]
pub type DDRSEL_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, DDRSEL_A, 7>;
impl<'a> DDRSEL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DDRSEL_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(DDRSEL_A::DOUBLE)
    }
}
#[doc = "Field `SRPCAP` reader - SRP-capable"]
pub type SRPCAP_R = crate::BitReader<bool>;
#[doc = "Field `SRPCAP` writer - SRP-capable"]
pub type SRPCAP_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 8>;
#[doc = "Field `HNPCAP` reader - HNP-capable"]
pub type HNPCAP_R = crate::BitReader<bool>;
#[doc = "Field `HNPCAP` writer - HNP-capable"]
pub type HNPCAP_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 9>;
#[doc = "Field `TRDT` reader - USB turnaround time"]
pub type TRDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRDT` writer - USB turnaround time"]
pub type TRDT_W<'a> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 4, 10>;
#[doc = "Field `PHYLPCS` reader - PHY Low-power clock select"]
pub type PHYLPCS_R = crate::BitReader<bool>;
#[doc = "Field `PHYLPCS` writer - PHY Low-power clock select"]
pub type PHYLPCS_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 15>;
#[doc = "Field `ULPIFSLS` reader - ULPI FS/LS select"]
pub type ULPIFSLS_R = crate::BitReader<bool>;
#[doc = "Field `ULPIFSLS` writer - ULPI FS/LS select"]
pub type ULPIFSLS_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 17>;
#[doc = "Field `ULPIAR` reader - ULPI Auto-resume"]
pub type ULPIAR_R = crate::BitReader<bool>;
#[doc = "Field `ULPIAR` writer - ULPI Auto-resume"]
pub type ULPIAR_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 18>;
#[doc = "Field `ULPICSM` reader - ULPI Clock SuspendM"]
pub type ULPICSM_R = crate::BitReader<bool>;
#[doc = "Field `ULPICSM` writer - ULPI Clock SuspendM"]
pub type ULPICSM_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 19>;
#[doc = "Field `ULPIEVBUSD` reader - ULPI External VBUS Drive"]
pub type ULPIEVBUSD_R = crate::BitReader<bool>;
#[doc = "Field `ULPIEVBUSD` writer - ULPI External VBUS Drive"]
pub type ULPIEVBUSD_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 20>;
#[doc = "Field `ULPIEVBUSI` reader - ULPI external VBUS indicator"]
pub type ULPIEVBUSI_R = crate::BitReader<bool>;
#[doc = "Field `ULPIEVBUSI` writer - ULPI external VBUS indicator"]
pub type ULPIEVBUSI_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 21>;
#[doc = "Field `TSDPS` reader - TermSel DLine pulsing selection"]
pub type TSDPS_R = crate::BitReader<bool>;
#[doc = "Field `TSDPS` writer - TermSel DLine pulsing selection"]
pub type TSDPS_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 22>;
#[doc = "Field `PCCI` reader - Indicator complement"]
pub type PCCI_R = crate::BitReader<bool>;
#[doc = "Field `PCCI` writer - Indicator complement"]
pub type PCCI_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 23>;
#[doc = "Field `PTCI` reader - Indicator pass through"]
pub type PTCI_R = crate::BitReader<bool>;
#[doc = "Field `PTCI` writer - Indicator pass through"]
pub type PTCI_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 24>;
#[doc = "Field `ULPIIPD` reader - ULPI interface protect disable"]
pub type ULPIIPD_R = crate::BitReader<bool>;
#[doc = "Field `ULPIIPD` writer - ULPI interface protect disable"]
pub type ULPIIPD_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 25>;
#[doc = "Field `FHMOD` reader - Forced host mode"]
pub type FHMOD_R = crate::BitReader<bool>;
#[doc = "Field `FHMOD` writer - Forced host mode"]
pub type FHMOD_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 29>;
#[doc = "Field `FDMOD` reader - Forced peripheral mode"]
pub type FDMOD_R = crate::BitReader<bool>;
#[doc = "Field `FDMOD` writer - Forced peripheral mode"]
pub type FDMOD_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 30>;
#[doc = "Field `CTXPKT` reader - Corrupt Tx packet"]
pub type CTXPKT_R = crate::BitReader<bool>;
#[doc = "Field `CTXPKT` writer - Corrupt Tx packet"]
pub type CTXPKT_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - PHY Interface width"]
    #[inline(always)]
    pub fn phyif(&self) -> PHYIF_R {
        PHYIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Type"]
    #[inline(always)]
    pub fn phytype(&self) -> PHYTYPE_R {
        PHYTYPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full speed interface"]
    #[inline(always)]
    pub fn fsif(&self) -> FSIF_R {
        FSIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transceiver select"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ULPI data rate"]
    #[inline(always)]
    pub fn ddrsel(&self) -> DDRSEL_R {
        DDRSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PHY Low-power clock select"]
    #[inline(always)]
    pub fn phylpcs(&self) -> PHYLPCS_R {
        PHYLPCS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - ULPI FS/LS select"]
    #[inline(always)]
    pub fn ulpifsls(&self) -> ULPIFSLS_R {
        ULPIFSLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ULPI Auto-resume"]
    #[inline(always)]
    pub fn ulpiar(&self) -> ULPIAR_R {
        ULPIAR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ULPI Clock SuspendM"]
    #[inline(always)]
    pub fn ulpicsm(&self) -> ULPICSM_R {
        ULPICSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ULPI External VBUS Drive"]
    #[inline(always)]
    pub fn ulpievbusd(&self) -> ULPIEVBUSD_R {
        ULPIEVBUSD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ULPI external VBUS indicator"]
    #[inline(always)]
    pub fn ulpievbusi(&self) -> ULPIEVBUSI_R {
        ULPIEVBUSI_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TermSel DLine pulsing selection"]
    #[inline(always)]
    pub fn tsdps(&self) -> TSDPS_R {
        TSDPS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicator complement"]
    #[inline(always)]
    pub fn pcci(&self) -> PCCI_R {
        PCCI_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicator pass through"]
    #[inline(always)]
    pub fn ptci(&self) -> PTCI_R {
        PTCI_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ULPI interface protect disable"]
    #[inline(always)]
    pub fn ulpiipd(&self) -> ULPIIPD_R {
        ULPIIPD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Forced host mode"]
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Forced peripheral mode"]
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctxpkt(&self) -> CTXPKT_R {
        CTXPKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(&mut self) -> TOCAL_W {
        TOCAL_W::new(self)
    }
    #[doc = "Bit 3 - PHY Interface width"]
    #[inline(always)]
    pub fn phyif(&mut self) -> PHYIF_W {
        PHYIF_W::new(self)
    }
    #[doc = "Bit 4 - PHY Type"]
    #[inline(always)]
    pub fn phytype(&mut self) -> PHYTYPE_W {
        PHYTYPE_W::new(self)
    }
    #[doc = "Bit 5 - Full speed interface"]
    #[inline(always)]
    pub fn fsif(&mut self) -> FSIF_W {
        FSIF_W::new(self)
    }
    #[doc = "Bit 6 - Transceiver select"]
    #[inline(always)]
    pub fn physel(&mut self) -> PHYSEL_W {
        PHYSEL_W::new(self)
    }
    #[doc = "Bit 7 - ULPI data rate"]
    #[inline(always)]
    pub fn ddrsel(&mut self) -> DDRSEL_W {
        DDRSEL_W::new(self)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    pub fn srpcap(&mut self) -> SRPCAP_W {
        SRPCAP_W::new(self)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(&mut self) -> HNPCAP_W {
        HNPCAP_W::new(self)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn trdt(&mut self) -> TRDT_W {
        TRDT_W::new(self)
    }
    #[doc = "Bit 15 - PHY Low-power clock select"]
    #[inline(always)]
    pub fn phylpcs(&mut self) -> PHYLPCS_W {
        PHYLPCS_W::new(self)
    }
    #[doc = "Bit 17 - ULPI FS/LS select"]
    #[inline(always)]
    pub fn ulpifsls(&mut self) -> ULPIFSLS_W {
        ULPIFSLS_W::new(self)
    }
    #[doc = "Bit 18 - ULPI Auto-resume"]
    #[inline(always)]
    pub fn ulpiar(&mut self) -> ULPIAR_W {
        ULPIAR_W::new(self)
    }
    #[doc = "Bit 19 - ULPI Clock SuspendM"]
    #[inline(always)]
    pub fn ulpicsm(&mut self) -> ULPICSM_W {
        ULPICSM_W::new(self)
    }
    #[doc = "Bit 20 - ULPI External VBUS Drive"]
    #[inline(always)]
    pub fn ulpievbusd(&mut self) -> ULPIEVBUSD_W {
        ULPIEVBUSD_W::new(self)
    }
    #[doc = "Bit 21 - ULPI external VBUS indicator"]
    #[inline(always)]
    pub fn ulpievbusi(&mut self) -> ULPIEVBUSI_W {
        ULPIEVBUSI_W::new(self)
    }
    #[doc = "Bit 22 - TermSel DLine pulsing selection"]
    #[inline(always)]
    pub fn tsdps(&mut self) -> TSDPS_W {
        TSDPS_W::new(self)
    }
    #[doc = "Bit 23 - Indicator complement"]
    #[inline(always)]
    pub fn pcci(&mut self) -> PCCI_W {
        PCCI_W::new(self)
    }
    #[doc = "Bit 24 - Indicator pass through"]
    #[inline(always)]
    pub fn ptci(&mut self) -> PTCI_W {
        PTCI_W::new(self)
    }
    #[doc = "Bit 25 - ULPI interface protect disable"]
    #[inline(always)]
    pub fn ulpiipd(&mut self) -> ULPIIPD_W {
        ULPIIPD_W::new(self)
    }
    #[doc = "Bit 29 - Forced host mode"]
    #[inline(always)]
    pub fn fhmod(&mut self) -> FHMOD_W {
        FHMOD_W::new(self)
    }
    #[doc = "Bit 30 - Forced peripheral mode"]
    #[inline(always)]
    pub fn fdmod(&mut self) -> FDMOD_W {
        FDMOD_W::new(self)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctxpkt(&mut self) -> CTXPKT_W {
        CTXPKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS USB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcfg](index.html) module"]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gusbcfg::R](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x0a00"]
impl crate::Resettable for GUSBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a00
    }
}
