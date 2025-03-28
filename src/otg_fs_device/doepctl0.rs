#[doc = "Register `DOEPCTL0` reader"]
pub type R = crate::R<Doepctl0Spec>;
#[doc = "Register `DOEPCTL0` writer"]
pub type W = crate::W<Doepctl0Spec>;
#[doc = "Field `MPSIZ` reader - MPSIZ"]
pub type MpsizR = crate::FieldReader;
#[doc = "Field `USBAEP` reader - USBAEP"]
pub type UsbaepR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAKSTS"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYP` reader - EPTYP"]
pub type EptypR = crate::FieldReader;
#[doc = "Field `SNPM` reader - SNPM"]
pub type SnpmR = crate::BitReader;
#[doc = "Field `SNPM` writer - SNPM"]
pub type SnpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Stall` reader - Stall"]
pub type StallR = crate::BitReader;
#[doc = "Field `Stall` writer - Stall"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - CNAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - SNAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - EPDIS"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPENA` writer - EPENA"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MpsizR {
        MpsizR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USBAEP"]
    #[inline(always)]
    pub fn usbaep(&self) -> UsbaepR {
        UsbaepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAKSTS"]
    #[inline(always)]
    pub fn naksts(&self) -> NakstsR {
        NakstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EptypR {
        EptypR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - SNPM"]
    #[inline(always)]
    pub fn snpm(&self) -> SnpmR {
        SnpmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stall"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - EPDIS"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - SNPM"]
    #[inline(always)]
    pub fn snpm(&mut self) -> SnpmW<Doepctl0Spec> {
        SnpmW::new(self, 20)
    }
    #[doc = "Bit 21 - Stall"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<Doepctl0Spec> {
        StallW::new(self, 21)
    }
    #[doc = "Bit 26 - CNAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<Doepctl0Spec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - SNAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<Doepctl0Spec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 31 - EPENA"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<Doepctl0Spec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "device endpoint-0 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepctl0Spec;
impl crate::RegisterSpec for Doepctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl0::R`](R) reader structure"]
impl crate::Readable for Doepctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`doepctl0::W`](W) writer structure"]
impl crate::Writable for Doepctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPCTL0 to value 0x8000"]
impl crate::Resettable for Doepctl0Spec {
    const RESET_VALUE: u32 = 0x8000;
}
