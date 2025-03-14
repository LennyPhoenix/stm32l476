#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `RELOAD` reader - Counter reload value"]
pub type ReloadR = crate::FieldReader<u16>;
#[doc = "Field `RELOAD` writer - Counter reload value"]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FELIM` reader - Frequency error limit"]
pub type FelimR = crate::FieldReader;
#[doc = "Field `FELIM` writer - Frequency error limit"]
pub type FelimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYNCDIV` reader - SYNC divider"]
pub type SyncdivR = crate::FieldReader;
#[doc = "Field `SYNCDIV` writer - SYNC divider"]
pub type SyncdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYNCSRC` reader - SYNC signal source selection"]
pub type SyncsrcR = crate::FieldReader;
#[doc = "Field `SYNCSRC` writer - SYNC signal source selection"]
pub type SyncsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNCPOL` reader - SYNC polarity selection"]
pub type SyncpolR = crate::BitReader;
#[doc = "Field `SYNCPOL` writer - SYNC polarity selection"]
pub type SyncpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Counter reload value"]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Frequency error limit"]
    #[inline(always)]
    pub fn felim(&self) -> FelimR {
        FelimR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - SYNC divider"]
    #[inline(always)]
    pub fn syncdiv(&self) -> SyncdivR {
        SyncdivR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection"]
    #[inline(always)]
    pub fn syncsrc(&self) -> SyncsrcR {
        SyncsrcR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SYNC polarity selection"]
    #[inline(always)]
    pub fn syncpol(&self) -> SyncpolR {
        SyncpolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter reload value"]
    #[inline(always)]
    pub fn reload(&mut self) -> ReloadW<CfgrSpec> {
        ReloadW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Frequency error limit"]
    #[inline(always)]
    pub fn felim(&mut self) -> FelimW<CfgrSpec> {
        FelimW::new(self, 16)
    }
    #[doc = "Bits 24:26 - SYNC divider"]
    #[inline(always)]
    pub fn syncdiv(&mut self) -> SyncdivW<CfgrSpec> {
        SyncdivW::new(self, 24)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection"]
    #[inline(always)]
    pub fn syncsrc(&mut self) -> SyncsrcW<CfgrSpec> {
        SyncsrcW::new(self, 28)
    }
    #[doc = "Bit 31 - SYNC polarity selection"]
    #[inline(always)]
    pub fn syncpol(&mut self) -> SyncpolW<CfgrSpec> {
        SyncpolW::new(self, 31)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0x2022_bb7f"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0x2022_bb7f;
}
