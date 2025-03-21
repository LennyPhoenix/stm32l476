#[doc = "Register `AHB3SMENR` reader"]
pub type R = crate::R<Ahb3smenrSpec>;
#[doc = "Register `AHB3SMENR` writer"]
pub type W = crate::W<Ahb3smenrSpec>;
#[doc = "Field `FMCSMEN` reader - Flexible memory controller clocks enable during Sleep and Stop modes"]
pub type FmcsmenR = crate::BitReader;
#[doc = "Field `FMCSMEN` writer - Flexible memory controller clocks enable during Sleep and Stop modes"]
pub type FmcsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPISMEN` reader - QSPISMEN"]
pub type QspismenR = crate::BitReader;
#[doc = "Field `QSPISMEN` writer - QSPISMEN"]
pub type QspismenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn fmcsmen(&self) -> FmcsmenR {
        FmcsmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - QSPISMEN"]
    #[inline(always)]
    pub fn qspismen(&self) -> QspismenR {
        QspismenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn fmcsmen(&mut self) -> FmcsmenW<Ahb3smenrSpec> {
        FmcsmenW::new(self, 0)
    }
    #[doc = "Bit 8 - QSPISMEN"]
    #[inline(always)]
    pub fn qspismen(&mut self) -> QspismenW<Ahb3smenrSpec> {
        QspismenW::new(self, 8)
    }
}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3smenrSpec;
impl crate::RegisterSpec for Ahb3smenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3smenr::R`](R) reader structure"]
impl crate::Readable for Ahb3smenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3smenr::W`](W) writer structure"]
impl crate::Writable for Ahb3smenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB3SMENR to value 0x0101"]
impl crate::Resettable for Ahb3smenrSpec {
    const RESET_VALUE: u32 = 0x0101;
}
