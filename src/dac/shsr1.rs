#[doc = "Register `SHSR1` reader"]
pub type R = crate::R<Shsr1Spec>;
#[doc = "Register `SHSR1` writer"]
pub type W = crate::W<Shsr1Spec>;
#[doc = "Field `TSAMPLE1` reader - DAC Channel 1 sample Time"]
pub type Tsample1R = crate::FieldReader<u16>;
#[doc = "Field `TSAMPLE1` writer - DAC Channel 1 sample Time"]
pub type Tsample1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC Channel 1 sample Time"]
    #[inline(always)]
    pub fn tsample1(&self) -> Tsample1R {
        Tsample1R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 1 sample Time"]
    #[inline(always)]
    pub fn tsample1(&mut self) -> Tsample1W<Shsr1Spec> {
        Tsample1W::new(self, 0)
    }
}
#[doc = "Sample and Hold sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`shsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shsr1Spec;
impl crate::RegisterSpec for Shsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shsr1::R`](R) reader structure"]
impl crate::Readable for Shsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`shsr1::W`](W) writer structure"]
impl crate::Writable for Shsr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHSR1 to value 0"]
impl crate::Resettable for Shsr1Spec {}
