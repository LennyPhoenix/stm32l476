#[doc = "Register `POWER` reader"]
pub type R = crate::R<PowerSpec>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<PowerSpec>;
#[doc = "Field `PWRCTRL` reader - PWRCTRL"]
pub type PwrctrlR = crate::FieldReader;
#[doc = "Field `PWRCTRL` writer - PWRCTRL"]
pub type PwrctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PwrctrlR {
        PwrctrlR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    pub fn pwrctrl(&mut self) -> PwrctrlW<PowerSpec> {
        PwrctrlW::new(self, 0)
    }
}
#[doc = "power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerSpec;
impl crate::RegisterSpec for PowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for PowerSpec {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for PowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for PowerSpec {}
