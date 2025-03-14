#[doc = "Register `PUCRA` reader"]
pub type R = crate::R<PucraSpec>;
#[doc = "Register `PUCRA` writer"]
pub type W = crate::W<PucraSpec>;
#[doc = "Field `PU0` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu0R = crate::BitReader;
#[doc = "Field `PU0` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU1` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu1R = crate::BitReader;
#[doc = "Field `PU1` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU2` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu2R = crate::BitReader;
#[doc = "Field `PU2` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU3` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu3R = crate::BitReader;
#[doc = "Field `PU3` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU4` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu4R = crate::BitReader;
#[doc = "Field `PU4` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU5` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu5R = crate::BitReader;
#[doc = "Field `PU5` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU6` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu6R = crate::BitReader;
#[doc = "Field `PU6` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU7` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu7R = crate::BitReader;
#[doc = "Field `PU7` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU8` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu8R = crate::BitReader;
#[doc = "Field `PU8` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU9` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu9R = crate::BitReader;
#[doc = "Field `PU9` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU10` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu10R = crate::BitReader;
#[doc = "Field `PU10` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU11` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu11R = crate::BitReader;
#[doc = "Field `PU11` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU12` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu12R = crate::BitReader;
#[doc = "Field `PU12` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU13` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu13R = crate::BitReader;
#[doc = "Field `PU13` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU14` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu14R = crate::BitReader;
#[doc = "Field `PU14` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU15` reader - Port A pull-up bit y (y=0..15)"]
pub type Pu15R = crate::BitReader;
#[doc = "Field `PU15` writer - Port A pull-up bit y (y=0..15)"]
pub type Pu15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu0(&self) -> Pu0R {
        Pu0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&self) -> Pu1R {
        Pu1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&self) -> Pu2R {
        Pu2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu3(&self) -> Pu3R {
        Pu3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu4(&self) -> Pu4R {
        Pu4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu5(&self) -> Pu5R {
        Pu5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu6(&self) -> Pu6R {
        Pu6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu7(&self) -> Pu7R {
        Pu7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu8(&self) -> Pu8R {
        Pu8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu9(&self) -> Pu9R {
        Pu9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu10(&self) -> Pu10R {
        Pu10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu11(&self) -> Pu11R {
        Pu11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu12(&self) -> Pu12R {
        Pu12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu13(&self) -> Pu13R {
        Pu13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu14(&self) -> Pu14R {
        Pu14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu15(&self) -> Pu15R {
        Pu15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu0(&mut self) -> Pu0W<PucraSpec> {
        Pu0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&mut self) -> Pu1W<PucraSpec> {
        Pu1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&mut self) -> Pu2W<PucraSpec> {
        Pu2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu3(&mut self) -> Pu3W<PucraSpec> {
        Pu3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu4(&mut self) -> Pu4W<PucraSpec> {
        Pu4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu5(&mut self) -> Pu5W<PucraSpec> {
        Pu5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu6(&mut self) -> Pu6W<PucraSpec> {
        Pu6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu7(&mut self) -> Pu7W<PucraSpec> {
        Pu7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu8(&mut self) -> Pu8W<PucraSpec> {
        Pu8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu9(&mut self) -> Pu9W<PucraSpec> {
        Pu9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu10(&mut self) -> Pu10W<PucraSpec> {
        Pu10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu11(&mut self) -> Pu11W<PucraSpec> {
        Pu11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu12(&mut self) -> Pu12W<PucraSpec> {
        Pu12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu13(&mut self) -> Pu13W<PucraSpec> {
        Pu13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu14(&mut self) -> Pu14W<PucraSpec> {
        Pu14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu15(&mut self) -> Pu15W<PucraSpec> {
        Pu15W::new(self, 15)
    }
}
#[doc = "Power Port A pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PucraSpec;
impl crate::RegisterSpec for PucraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucra::R`](R) reader structure"]
impl crate::Readable for PucraSpec {}
#[doc = "`write(|w| ..)` method takes [`pucra::W`](W) writer structure"]
impl crate::Writable for PucraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUCRA to value 0"]
impl crate::Resettable for PucraSpec {}
