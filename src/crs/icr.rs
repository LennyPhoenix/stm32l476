#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `SYNCOKC` reader - SYNC event OK clear flag"]
pub type SyncokcR = crate::BitReader;
#[doc = "Field `SYNCOKC` writer - SYNC event OK clear flag"]
pub type SyncokcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCWARNC` reader - SYNC warning clear flag"]
pub type SyncwarncR = crate::BitReader;
#[doc = "Field `SYNCWARNC` writer - SYNC warning clear flag"]
pub type SyncwarncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRC` reader - Error clear flag"]
pub type ErrcR = crate::BitReader;
#[doc = "Field `ERRC` writer - Error clear flag"]
pub type ErrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESYNCC` reader - Expected SYNC clear flag"]
pub type EsynccR = crate::BitReader;
#[doc = "Field `ESYNCC` writer - Expected SYNC clear flag"]
pub type EsynccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYNC event OK clear flag"]
    #[inline(always)]
    pub fn syncokc(&self) -> SyncokcR {
        SyncokcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning clear flag"]
    #[inline(always)]
    pub fn syncwarnc(&self) -> SyncwarncR {
        SyncwarncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error clear flag"]
    #[inline(always)]
    pub fn errc(&self) -> ErrcR {
        ErrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC clear flag"]
    #[inline(always)]
    pub fn esyncc(&self) -> EsynccR {
        EsynccR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK clear flag"]
    #[inline(always)]
    pub fn syncokc(&mut self) -> SyncokcW<IcrSpec> {
        SyncokcW::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC warning clear flag"]
    #[inline(always)]
    pub fn syncwarnc(&mut self) -> SyncwarncW<IcrSpec> {
        SyncwarncW::new(self, 1)
    }
    #[doc = "Bit 2 - Error clear flag"]
    #[inline(always)]
    pub fn errc(&mut self) -> ErrcW<IcrSpec> {
        ErrcW::new(self, 2)
    }
    #[doc = "Bit 3 - Expected SYNC clear flag"]
    #[inline(always)]
    pub fn esyncc(&mut self) -> EsynccW<IcrSpec> {
        EsynccW::new(self, 3)
    }
}
#[doc = "interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
