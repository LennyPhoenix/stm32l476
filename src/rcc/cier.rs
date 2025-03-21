#[doc = "Register `CIER` reader"]
pub type R = crate::R<CierSpec>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CierSpec>;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LsirdyieR = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
pub type LserdyieR = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub type LserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt enable"]
pub type MsirdyieR = crate::BitReader;
#[doc = "Field `MSIRDYIE` writer - MSI ready interrupt enable"]
pub type MsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub type HsirdyieR = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub type HsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub type HserdyieR = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub type HserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt enable"]
pub type PllrdyieR = crate::BitReader;
#[doc = "Field `PLLRDYIE` writer - PLL ready interrupt enable"]
pub type PllrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1RDYIE` reader - PLLSAI1 ready interrupt enable"]
pub type Pllsai1rdyieR = crate::BitReader;
#[doc = "Field `PLLSAI1RDYIE` writer - PLLSAI1 ready interrupt enable"]
pub type Pllsai1rdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2RDYIE` reader - PLLSAI2 ready interrupt enable"]
pub type Pllsai2rdyieR = crate::BitReader;
#[doc = "Field `PLLSAI2RDYIE` writer - PLLSAI2 ready interrupt enable"]
pub type Pllsai2rdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSIE` reader - LSE clock security system interrupt enable"]
pub type LsecssieR = crate::BitReader;
#[doc = "Field `LSECSSIE` writer - LSE clock security system interrupt enable"]
pub type LsecssieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable"]
pub type Hsi48rdyieR = crate::BitReader;
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable"]
pub type Hsi48rdyieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LsirdyieR {
        LsirdyieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LserdyieR {
        LserdyieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MsirdyieR {
        MsirdyieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HsirdyieR {
        HsirdyieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HserdyieR {
        HserdyieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PllrdyieR {
        PllrdyieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai1rdyie(&self) -> Pllsai1rdyieR {
        Pllsai1rdyieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai2rdyie(&self) -> Pllsai2rdyieR {
        Pllsai2rdyieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LsecssieR {
        LsecssieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> Hsi48rdyieR {
        Hsi48rdyieR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LsirdyieW<CierSpec> {
        LsirdyieW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LserdyieW<CierSpec> {
        LserdyieW::new(self, 1)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MsirdyieW<CierSpec> {
        MsirdyieW::new(self, 2)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HsirdyieW<CierSpec> {
        HsirdyieW::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HserdyieW<CierSpec> {
        HserdyieW::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PllrdyieW<CierSpec> {
        PllrdyieW::new(self, 5)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai1rdyie(&mut self) -> Pllsai1rdyieW<CierSpec> {
        Pllsai1rdyieW::new(self, 6)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai2rdyie(&mut self) -> Pllsai2rdyieW<CierSpec> {
        Pllsai2rdyieW::new(self, 7)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LsecssieW<CierSpec> {
        LsecssieW::new(self, 9)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> Hsi48rdyieW<CierSpec> {
        Hsi48rdyieW::new(self, 10)
    }
}
#[doc = "Clock interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CierSpec;
impl crate::RegisterSpec for CierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CierSpec {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CierSpec {}
