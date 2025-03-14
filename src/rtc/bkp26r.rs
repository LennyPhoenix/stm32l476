#[doc = "Register `BKP26R` reader"]
pub type R = crate::R<Bkp26rSpec>;
#[doc = "Register `BKP26R` writer"]
pub type W = crate::W<Bkp26rSpec>;
#[doc = "Field `BKP` reader - BKP"]
pub type BkpR = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - BKP"]
pub type BkpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<Bkp26rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp26r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp26r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp26rSpec;
impl crate::RegisterSpec for Bkp26rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp26r::R`](R) reader structure"]
impl crate::Readable for Bkp26rSpec {}
#[doc = "`write(|w| ..)` method takes [`bkp26r::W`](W) writer structure"]
impl crate::Writable for Bkp26rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP26R to value 0"]
impl crate::Resettable for Bkp26rSpec {}
