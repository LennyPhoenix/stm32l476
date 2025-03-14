#[doc = "Register `PDCRC` reader"]
pub type R = crate::R<PdcrcSpec>;
#[doc = "Register `PDCRC` writer"]
pub type W = crate::W<PdcrcSpec>;
#[doc = "Field `PD0` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd0R = crate::BitReader;
#[doc = "Field `PD0` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd1R = crate::BitReader;
#[doc = "Field `PD1` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd2R = crate::BitReader;
#[doc = "Field `PD2` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd3R = crate::BitReader;
#[doc = "Field `PD3` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd4R = crate::BitReader;
#[doc = "Field `PD4` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd5R = crate::BitReader;
#[doc = "Field `PD5` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd6R = crate::BitReader;
#[doc = "Field `PD6` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd7R = crate::BitReader;
#[doc = "Field `PD7` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd8R = crate::BitReader;
#[doc = "Field `PD8` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd9R = crate::BitReader;
#[doc = "Field `PD9` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd10R = crate::BitReader;
#[doc = "Field `PD10` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD11` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd11R = crate::BitReader;
#[doc = "Field `PD11` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD12` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd12R = crate::BitReader;
#[doc = "Field `PD12` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD13` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd13R = crate::BitReader;
#[doc = "Field `PD13` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD14` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd14R = crate::BitReader;
#[doc = "Field `PD14` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD15` reader - Port C pull-down bit y (y=0..15)"]
pub type Pd15R = crate::BitReader;
#[doc = "Field `PD15` writer - Port C pull-down bit y (y=0..15)"]
pub type Pd15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd0(&self) -> Pd0R {
        Pd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd1(&self) -> Pd1R {
        Pd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd2(&self) -> Pd2R {
        Pd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd3(&self) -> Pd3R {
        Pd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd4(&self) -> Pd4R {
        Pd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd5(&self) -> Pd5R {
        Pd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd6(&self) -> Pd6R {
        Pd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd7(&self) -> Pd7R {
        Pd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd8(&self) -> Pd8R {
        Pd8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd9(&self) -> Pd9R {
        Pd9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd10(&self) -> Pd10R {
        Pd10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd11(&self) -> Pd11R {
        Pd11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd12(&self) -> Pd12R {
        Pd12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd13(&self) -> Pd13R {
        Pd13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd14(&self) -> Pd14R {
        Pd14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd15(&self) -> Pd15R {
        Pd15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd0(&mut self) -> Pd0W<PdcrcSpec> {
        Pd0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd1(&mut self) -> Pd1W<PdcrcSpec> {
        Pd1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd2(&mut self) -> Pd2W<PdcrcSpec> {
        Pd2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd3(&mut self) -> Pd3W<PdcrcSpec> {
        Pd3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd4(&mut self) -> Pd4W<PdcrcSpec> {
        Pd4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd5(&mut self) -> Pd5W<PdcrcSpec> {
        Pd5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd6(&mut self) -> Pd6W<PdcrcSpec> {
        Pd6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd7(&mut self) -> Pd7W<PdcrcSpec> {
        Pd7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd8(&mut self) -> Pd8W<PdcrcSpec> {
        Pd8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd9(&mut self) -> Pd9W<PdcrcSpec> {
        Pd9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd10(&mut self) -> Pd10W<PdcrcSpec> {
        Pd10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd11(&mut self) -> Pd11W<PdcrcSpec> {
        Pd11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd12(&mut self) -> Pd12W<PdcrcSpec> {
        Pd12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd13(&mut self) -> Pd13W<PdcrcSpec> {
        Pd13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd14(&mut self) -> Pd14W<PdcrcSpec> {
        Pd14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port C pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd15(&mut self) -> Pd15W<PdcrcSpec> {
        Pd15W::new(self, 15)
    }
}
#[doc = "Power Port C pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdcrcSpec;
impl crate::RegisterSpec for PdcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrc::R`](R) reader structure"]
impl crate::Readable for PdcrcSpec {}
#[doc = "`write(|w| ..)` method takes [`pdcrc::W`](W) writer structure"]
impl crate::Writable for PdcrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDCRC to value 0"]
impl crate::Resettable for PdcrcSpec {}
