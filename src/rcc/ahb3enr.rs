#[doc = "Register `AHB3ENR` reader"]
pub type R = crate::R<Ahb3enrSpec>;
#[doc = "Register `AHB3ENR` writer"]
pub type W = crate::W<Ahb3enrSpec>;
#[doc = "Field `FMCEN` reader - Flexible memory controller clock enable"]
pub type FmcenR = crate::BitReader;
#[doc = "Field `FMCEN` writer - Flexible memory controller clock enable"]
pub type FmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPIEN` reader - QSPIEN"]
pub type QspienR = crate::BitReader;
#[doc = "Field `QSPIEN` writer - QSPIEN"]
pub type QspienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flexible memory controller clock enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FmcenR {
        FmcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - QSPIEN"]
    #[inline(always)]
    pub fn qspien(&self) -> QspienR {
        QspienR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller clock enable"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FmcenW<Ahb3enrSpec> {
        FmcenW::new(self, 0)
    }
    #[doc = "Bit 8 - QSPIEN"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QspienW<Ahb3enrSpec> {
        QspienW::new(self, 8)
    }
}
#[doc = "AHB3 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3enrSpec;
impl crate::RegisterSpec for Ahb3enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3enr::R`](R) reader structure"]
impl crate::Readable for Ahb3enrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure"]
impl crate::Writable for Ahb3enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB3ENR to value 0"]
impl crate::Resettable for Ahb3enrSpec {}
