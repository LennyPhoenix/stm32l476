#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `WUF1` writer - Clear wakeup flag 1"]
pub type Wuf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUF2` writer - Clear wakeup flag 2"]
pub type Wuf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUF3` writer - Clear wakeup flag 3"]
pub type Wuf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUF4` writer - Clear wakeup flag 4"]
pub type Wuf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUF5` writer - Clear wakeup flag 5"]
pub type Wuf5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBF` writer - Clear standby flag"]
pub type SbfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1"]
    #[inline(always)]
    pub fn wuf1(&mut self) -> Wuf1W<ScrSpec> {
        Wuf1W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2"]
    #[inline(always)]
    pub fn wuf2(&mut self) -> Wuf2W<ScrSpec> {
        Wuf2W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3"]
    #[inline(always)]
    pub fn wuf3(&mut self) -> Wuf3W<ScrSpec> {
        Wuf3W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear wakeup flag 4"]
    #[inline(always)]
    pub fn wuf4(&mut self) -> Wuf4W<ScrSpec> {
        Wuf4W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear wakeup flag 5"]
    #[inline(always)]
    pub fn wuf5(&mut self) -> Wuf5W<ScrSpec> {
        Wuf5W::new(self, 4)
    }
    #[doc = "Bit 8 - Clear standby flag"]
    #[inline(always)]
    pub fn sbf(&mut self) -> SbfW<ScrSpec> {
        SbfW::new(self, 8)
    }
}
#[doc = "Power status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {}
