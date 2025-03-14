#[doc = "Register `PUCRH` reader"]
pub type R = crate::R<PucrhSpec>;
#[doc = "Register `PUCRH` writer"]
pub type W = crate::W<PucrhSpec>;
#[doc = "Field `PU0` reader - Port H pull-up bit y (y=0..1)"]
pub type Pu0R = crate::BitReader;
#[doc = "Field `PU0` writer - Port H pull-up bit y (y=0..1)"]
pub type Pu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU1` reader - Port H pull-up bit y (y=0..1)"]
pub type Pu1R = crate::BitReader;
#[doc = "Field `PU1` writer - Port H pull-up bit y (y=0..1)"]
pub type Pu1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    pub fn pu0(&self) -> Pu0R {
        Pu0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    pub fn pu1(&self) -> Pu1R {
        Pu1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    pub fn pu0(&mut self) -> Pu0W<PucrhSpec> {
        Pu0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    pub fn pu1(&mut self) -> Pu1W<PucrhSpec> {
        Pu1W::new(self, 1)
    }
}
#[doc = "Power Port H pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PucrhSpec;
impl crate::RegisterSpec for PucrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucrh::R`](R) reader structure"]
impl crate::Readable for PucrhSpec {}
#[doc = "`write(|w| ..)` method takes [`pucrh::W`](W) writer structure"]
impl crate::Writable for PucrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUCRH to value 0"]
impl crate::Resettable for PucrhSpec {}
