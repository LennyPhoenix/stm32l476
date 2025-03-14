#[doc = "Register `OR1` reader"]
pub type R = crate::R<Or1Spec>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<Or1Spec>;
#[doc = "Field `TI1_RMP` reader - Input capture 1 remap"]
pub type Ti1RmpR = crate::FieldReader;
#[doc = "Field `TI1_RMP` writer - Input capture 1 remap"]
pub type Ti1RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Input capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> Ti1RmpR {
        Ti1RmpR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> Ti1RmpW<Or1Spec> {
        Ti1RmpW::new(self, 0)
    }
}
#[doc = "TIM16 option register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Or1Spec;
impl crate::RegisterSpec for Or1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for Or1Spec {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for Or1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for Or1Spec {}
