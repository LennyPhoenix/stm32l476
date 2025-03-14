#[doc = "Register `SR2` reader"]
pub type R = crate::R<Sr2Spec>;
#[doc = "Field `REGLPS` reader - Low-power regulator started"]
pub type ReglpsR = crate::BitReader;
#[doc = "Field `REGLPF` reader - Low-power regulator flag"]
pub type ReglpfR = crate::BitReader;
#[doc = "Field `VOSF` reader - Voltage scaling flag"]
pub type VosfR = crate::BitReader;
#[doc = "Field `PVDO` reader - Power voltage detector output"]
pub type PvdoR = crate::BitReader;
#[doc = "Field `PVMO1` reader - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
pub type Pvmo1R = crate::BitReader;
#[doc = "Field `PVMO2` reader - Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V"]
pub type Pvmo2R = crate::BitReader;
#[doc = "Field `PVMO3` reader - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
pub type Pvmo3R = crate::BitReader;
#[doc = "Field `PVMO4` reader - Peripheral voltage monitoring output: VDDA vs. 2.2 V"]
pub type Pvmo4R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - Low-power regulator started"]
    #[inline(always)]
    pub fn reglps(&self) -> ReglpsR {
        ReglpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power regulator flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> ReglpfR {
        ReglpfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VosfR {
        VosfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power voltage detector output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PvdoR {
        PvdoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
    #[inline(always)]
    pub fn pvmo1(&self) -> Pvmo1R {
        Pvmo1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V"]
    #[inline(always)]
    pub fn pvmo2(&self) -> Pvmo2R {
        Pvmo2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
    #[inline(always)]
    pub fn pvmo3(&self) -> Pvmo3R {
        Pvmo3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral voltage monitoring output: VDDA vs. 2.2 V"]
    #[inline(always)]
    pub fn pvmo4(&self) -> Pvmo4R {
        Pvmo4R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr2Spec;
impl crate::RegisterSpec for Sr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for Sr2Spec {}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for Sr2Spec {}
