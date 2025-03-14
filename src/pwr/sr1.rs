#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Field `CWUF1` reader - Wakeup flag 1"]
pub type Cwuf1R = crate::BitReader;
#[doc = "Field `CWUF2` reader - Wakeup flag 2"]
pub type Cwuf2R = crate::BitReader;
#[doc = "Field `CWUF3` reader - Wakeup flag 3"]
pub type Cwuf3R = crate::BitReader;
#[doc = "Field `CWUF4` reader - Wakeup flag 4"]
pub type Cwuf4R = crate::BitReader;
#[doc = "Field `CWUF5` reader - Wakeup flag 5"]
pub type Cwuf5R = crate::BitReader;
#[doc = "Field `CSBF` reader - Standby flag"]
pub type CsbfR = crate::BitReader;
#[doc = "Field `WUFI` reader - Wakeup flag internal"]
pub type WufiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Wakeup flag 1"]
    #[inline(always)]
    pub fn cwuf1(&self) -> Cwuf1R {
        Cwuf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2"]
    #[inline(always)]
    pub fn cwuf2(&self) -> Cwuf2R {
        Cwuf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3"]
    #[inline(always)]
    pub fn cwuf3(&self) -> Cwuf3R {
        Cwuf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4"]
    #[inline(always)]
    pub fn cwuf4(&self) -> Cwuf4R {
        Cwuf4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5"]
    #[inline(always)]
    pub fn cwuf5(&self) -> Cwuf5R {
        Cwuf5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CsbfR {
        CsbfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Wakeup flag internal"]
    #[inline(always)]
    pub fn wufi(&self) -> WufiR {
        WufiR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for Sr1Spec {}
