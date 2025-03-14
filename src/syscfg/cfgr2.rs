#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<Cfgr2Spec>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<Cfgr2Spec>;
#[doc = "Field `CLL` writer - Cortex-M4 LOCKUP (Hardfault) output enable bit"]
pub type CllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPL` writer - SRAM2 parity lock bit"]
pub type SplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDL` writer - PVD lock enable bit"]
pub type PvdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCL` writer - ECC Lock"]
pub type EcclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPF` reader - SRAM2 parity error flag"]
pub type SpfR = crate::BitReader;
#[doc = "Field `SPF` writer - SRAM2 parity error flag"]
pub type SpfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&self) -> SpfR {
        SpfR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M4 LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    pub fn cll(&mut self) -> CllW<Cfgr2Spec> {
        CllW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    pub fn spl(&mut self) -> SplW<Cfgr2Spec> {
        SplW::new(self, 1)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvdl(&mut self) -> PvdlW<Cfgr2Spec> {
        PvdlW::new(self, 2)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    pub fn eccl(&mut self) -> EcclW<Cfgr2Spec> {
        EcclW::new(self, 3)
    }
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&mut self) -> SpfW<Cfgr2Spec> {
        SpfW::new(self, 8)
    }
}
#[doc = "CFGR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr2Spec;
impl crate::RegisterSpec for Cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for Cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for Cfgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for Cfgr2Spec {}
