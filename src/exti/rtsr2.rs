#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<Rtsr2Spec>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<Rtsr2Spec>;
#[doc = "Field `RT35` reader - Rising trigger event configuration bit of line 35"]
pub type Rt35R = crate::BitReader;
#[doc = "Field `RT35` writer - Rising trigger event configuration bit of line 35"]
pub type Rt35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT36` reader - Rising trigger event configuration bit of line 36"]
pub type Rt36R = crate::BitReader;
#[doc = "Field `RT36` writer - Rising trigger event configuration bit of line 36"]
pub type Rt36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT37` reader - Rising trigger event configuration bit of line 37"]
pub type Rt37R = crate::BitReader;
#[doc = "Field `RT37` writer - Rising trigger event configuration bit of line 37"]
pub type Rt37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT38` reader - Rising trigger event configuration bit of line 38"]
pub type Rt38R = crate::BitReader;
#[doc = "Field `RT38` writer - Rising trigger event configuration bit of line 38"]
pub type Rt38W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Rising trigger event configuration bit of line 35"]
    #[inline(always)]
    pub fn rt35(&self) -> Rt35R {
        Rt35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of line 36"]
    #[inline(always)]
    pub fn rt36(&self) -> Rt36R {
        Rt36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of line 37"]
    #[inline(always)]
    pub fn rt37(&self) -> Rt37R {
        Rt37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn rt38(&self) -> Rt38R {
        Rt38R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Rising trigger event configuration bit of line 35"]
    #[inline(always)]
    pub fn rt35(&mut self) -> Rt35W<Rtsr2Spec> {
        Rt35W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of line 36"]
    #[inline(always)]
    pub fn rt36(&mut self) -> Rt36W<Rtsr2Spec> {
        Rt36W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of line 37"]
    #[inline(always)]
    pub fn rt37(&mut self) -> Rt37W<Rtsr2Spec> {
        Rt37W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn rt38(&mut self) -> Rt38W<Rtsr2Spec> {
        Rt38W::new(self, 6)
    }
}
#[doc = "Rising Trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtsr2Spec;
impl crate::RegisterSpec for Rtsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr2::R`](R) reader structure"]
impl crate::Readable for Rtsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure"]
impl crate::Writable for Rtsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for Rtsr2Spec {}
