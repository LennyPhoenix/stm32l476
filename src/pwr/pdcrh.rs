#[doc = "Register `PDCRH` reader"]
pub type R = crate::R<PdcrhSpec>;
#[doc = "Register `PDCRH` writer"]
pub type W = crate::W<PdcrhSpec>;
#[doc = "Field `PD0` reader - Port H pull-down bit y (y=0..1)"]
pub type Pd0R = crate::BitReader;
#[doc = "Field `PD0` writer - Port H pull-down bit y (y=0..1)"]
pub type Pd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port H pull-down bit y (y=0..1)"]
pub type Pd1R = crate::BitReader;
#[doc = "Field `PD1` writer - Port H pull-down bit y (y=0..1)"]
pub type Pd1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port H pull-down bit y (y=0..1)"]
    #[inline(always)]
    pub fn pd0(&self) -> Pd0R {
        Pd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port H pull-down bit y (y=0..1)"]
    #[inline(always)]
    pub fn pd1(&self) -> Pd1R {
        Pd1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port H pull-down bit y (y=0..1)"]
    #[inline(always)]
    pub fn pd0(&mut self) -> Pd0W<PdcrhSpec> {
        Pd0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port H pull-down bit y (y=0..1)"]
    #[inline(always)]
    pub fn pd1(&mut self) -> Pd1W<PdcrhSpec> {
        Pd1W::new(self, 1)
    }
}
#[doc = "Power Port H pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdcrhSpec;
impl crate::RegisterSpec for PdcrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrh::R`](R) reader structure"]
impl crate::Readable for PdcrhSpec {}
#[doc = "`write(|w| ..)` method takes [`pdcrh::W`](W) writer structure"]
impl crate::Writable for PdcrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDCRH to value 0"]
impl crate::Resettable for PdcrhSpec {}
