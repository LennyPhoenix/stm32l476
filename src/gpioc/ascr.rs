#[doc = "Register `ASCR` reader"]
pub type R = crate::R<AscrSpec>;
#[doc = "Register `ASCR` writer"]
pub type W = crate::W<AscrSpec>;
#[doc = "Field `ASC0` reader - Port analog switch control"]
pub type Asc0R = crate::BitReader;
#[doc = "Field `ASC0` writer - Port analog switch control"]
pub type Asc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC1` reader - Port analog switch control"]
pub type Asc1R = crate::BitReader;
#[doc = "Field `ASC1` writer - Port analog switch control"]
pub type Asc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC2` reader - Port analog switch control"]
pub type Asc2R = crate::BitReader;
#[doc = "Field `ASC2` writer - Port analog switch control"]
pub type Asc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC3` reader - Port analog switch control"]
pub type Asc3R = crate::BitReader;
#[doc = "Field `ASC3` writer - Port analog switch control"]
pub type Asc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC4` reader - Port analog switch control"]
pub type Asc4R = crate::BitReader;
#[doc = "Field `ASC4` writer - Port analog switch control"]
pub type Asc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC5` reader - Port analog switch control"]
pub type Asc5R = crate::BitReader;
#[doc = "Field `ASC5` writer - Port analog switch control"]
pub type Asc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC6` reader - Port analog switch control"]
pub type Asc6R = crate::BitReader;
#[doc = "Field `ASC6` writer - Port analog switch control"]
pub type Asc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC7` reader - Port analog switch control"]
pub type Asc7R = crate::BitReader;
#[doc = "Field `ASC7` writer - Port analog switch control"]
pub type Asc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC8` reader - Port analog switch control"]
pub type Asc8R = crate::BitReader;
#[doc = "Field `ASC8` writer - Port analog switch control"]
pub type Asc8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC9` reader - Port analog switch control"]
pub type Asc9R = crate::BitReader;
#[doc = "Field `ASC9` writer - Port analog switch control"]
pub type Asc9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC10` reader - Port analog switch control"]
pub type Asc10R = crate::BitReader;
#[doc = "Field `ASC10` writer - Port analog switch control"]
pub type Asc10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC11` reader - Port analog switch control"]
pub type Asc11R = crate::BitReader;
#[doc = "Field `ASC11` writer - Port analog switch control"]
pub type Asc11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC12` reader - Port analog switch control"]
pub type Asc12R = crate::BitReader;
#[doc = "Field `ASC12` writer - Port analog switch control"]
pub type Asc12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC13` reader - Port analog switch control"]
pub type Asc13R = crate::BitReader;
#[doc = "Field `ASC13` writer - Port analog switch control"]
pub type Asc13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC14` reader - Port analog switch control"]
pub type Asc14R = crate::BitReader;
#[doc = "Field `ASC14` writer - Port analog switch control"]
pub type Asc14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC15` reader - Port analog switch control"]
pub type Asc15R = crate::BitReader;
#[doc = "Field `ASC15` writer - Port analog switch control"]
pub type Asc15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port analog switch control"]
    #[inline(always)]
    pub fn asc0(&self) -> Asc0R {
        Asc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port analog switch control"]
    #[inline(always)]
    pub fn asc1(&self) -> Asc1R {
        Asc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port analog switch control"]
    #[inline(always)]
    pub fn asc2(&self) -> Asc2R {
        Asc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port analog switch control"]
    #[inline(always)]
    pub fn asc3(&self) -> Asc3R {
        Asc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port analog switch control"]
    #[inline(always)]
    pub fn asc4(&self) -> Asc4R {
        Asc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port analog switch control"]
    #[inline(always)]
    pub fn asc5(&self) -> Asc5R {
        Asc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port analog switch control"]
    #[inline(always)]
    pub fn asc6(&self) -> Asc6R {
        Asc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port analog switch control"]
    #[inline(always)]
    pub fn asc7(&self) -> Asc7R {
        Asc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port analog switch control"]
    #[inline(always)]
    pub fn asc8(&self) -> Asc8R {
        Asc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port analog switch control"]
    #[inline(always)]
    pub fn asc9(&self) -> Asc9R {
        Asc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port analog switch control"]
    #[inline(always)]
    pub fn asc10(&self) -> Asc10R {
        Asc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port analog switch control"]
    #[inline(always)]
    pub fn asc11(&self) -> Asc11R {
        Asc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port analog switch control"]
    #[inline(always)]
    pub fn asc12(&self) -> Asc12R {
        Asc12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port analog switch control"]
    #[inline(always)]
    pub fn asc13(&self) -> Asc13R {
        Asc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port analog switch control"]
    #[inline(always)]
    pub fn asc14(&self) -> Asc14R {
        Asc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port analog switch control"]
    #[inline(always)]
    pub fn asc15(&self) -> Asc15R {
        Asc15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port analog switch control"]
    #[inline(always)]
    pub fn asc0(&mut self) -> Asc0W<AscrSpec> {
        Asc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port analog switch control"]
    #[inline(always)]
    pub fn asc1(&mut self) -> Asc1W<AscrSpec> {
        Asc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port analog switch control"]
    #[inline(always)]
    pub fn asc2(&mut self) -> Asc2W<AscrSpec> {
        Asc2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port analog switch control"]
    #[inline(always)]
    pub fn asc3(&mut self) -> Asc3W<AscrSpec> {
        Asc3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port analog switch control"]
    #[inline(always)]
    pub fn asc4(&mut self) -> Asc4W<AscrSpec> {
        Asc4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port analog switch control"]
    #[inline(always)]
    pub fn asc5(&mut self) -> Asc5W<AscrSpec> {
        Asc5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port analog switch control"]
    #[inline(always)]
    pub fn asc6(&mut self) -> Asc6W<AscrSpec> {
        Asc6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port analog switch control"]
    #[inline(always)]
    pub fn asc7(&mut self) -> Asc7W<AscrSpec> {
        Asc7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port analog switch control"]
    #[inline(always)]
    pub fn asc8(&mut self) -> Asc8W<AscrSpec> {
        Asc8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port analog switch control"]
    #[inline(always)]
    pub fn asc9(&mut self) -> Asc9W<AscrSpec> {
        Asc9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port analog switch control"]
    #[inline(always)]
    pub fn asc10(&mut self) -> Asc10W<AscrSpec> {
        Asc10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port analog switch control"]
    #[inline(always)]
    pub fn asc11(&mut self) -> Asc11W<AscrSpec> {
        Asc11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port analog switch control"]
    #[inline(always)]
    pub fn asc12(&mut self) -> Asc12W<AscrSpec> {
        Asc12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port analog switch control"]
    #[inline(always)]
    pub fn asc13(&mut self) -> Asc13W<AscrSpec> {
        Asc13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port analog switch control"]
    #[inline(always)]
    pub fn asc14(&mut self) -> Asc14W<AscrSpec> {
        Asc14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port analog switch control"]
    #[inline(always)]
    pub fn asc15(&mut self) -> Asc15W<AscrSpec> {
        Asc15W::new(self, 15)
    }
}
#[doc = "GPIO port analog switch control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ascr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AscrSpec;
impl crate::RegisterSpec for AscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ascr::R`](R) reader structure"]
impl crate::Readable for AscrSpec {}
#[doc = "`write(|w| ..)` method takes [`ascr::W`](W) writer structure"]
impl crate::Writable for AscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASCR to value 0"]
impl crate::Resettable for AscrSpec {}
