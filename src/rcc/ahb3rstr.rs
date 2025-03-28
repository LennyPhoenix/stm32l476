#[doc = "Register `AHB3RSTR` reader"]
pub type R = crate::R<Ahb3rstrSpec>;
#[doc = "Register `AHB3RSTR` writer"]
pub type W = crate::W<Ahb3rstrSpec>;
#[doc = "Field `FMCRST` reader - Flexible memory controller reset"]
pub type FmcrstR = crate::BitReader;
#[doc = "Field `FMCRST` writer - Flexible memory controller reset"]
pub type FmcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPIRST` reader - Quad SPI memory interface reset"]
pub type QspirstR = crate::BitReader;
#[doc = "Field `QSPIRST` writer - Quad SPI memory interface reset"]
pub type QspirstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flexible memory controller reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FmcrstR {
        FmcrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Quad SPI memory interface reset"]
    #[inline(always)]
    pub fn qspirst(&self) -> QspirstR {
        QspirstR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller reset"]
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FmcrstW<Ahb3rstrSpec> {
        FmcrstW::new(self, 0)
    }
    #[doc = "Bit 8 - Quad SPI memory interface reset"]
    #[inline(always)]
    pub fn qspirst(&mut self) -> QspirstW<Ahb3rstrSpec> {
        QspirstW::new(self, 8)
    }
}
#[doc = "AHB3 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3rstrSpec;
impl crate::RegisterSpec for Ahb3rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3rstr::R`](R) reader structure"]
impl crate::Readable for Ahb3rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure"]
impl crate::Writable for Ahb3rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for Ahb3rstrSpec {}
