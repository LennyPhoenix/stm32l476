#[doc = "Register `CICR` writer"]
pub type W = crate::W<CicrSpec>;
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub type LserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRDYC` writer - MSI ready interrupt clear"]
pub type MsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYC` writer - PLL ready interrupt clear"]
pub type PllrdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1RDYC` writer - PLLSAI1 ready interrupt clear"]
pub type Pllsai1rdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2RDYC` writer - PLLSAI2 ready interrupt clear"]
pub type Pllsai2rdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CsscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSC` writer - LSE Clock security system interrupt clear"]
pub type LsecsscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48RDYC` writer - HSI48 oscillator ready interrupt clear"]
pub type Hsi48rdycW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LsirdycW<CicrSpec> {
        LsirdycW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LserdycW<CicrSpec> {
        LserdycW::new(self, 1)
    }
    #[doc = "Bit 2 - MSI ready interrupt clear"]
    #[inline(always)]
    pub fn msirdyc(&mut self) -> MsirdycW<CicrSpec> {
        MsirdycW::new(self, 2)
    }
    #[doc = "Bit 3 - HSI ready interrupt clear"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HsirdycW<CicrSpec> {
        HsirdycW::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HserdycW<CicrSpec> {
        HserdycW::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt clear"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PllrdycW<CicrSpec> {
        PllrdycW::new(self, 5)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt clear"]
    #[inline(always)]
    pub fn pllsai1rdyc(&mut self) -> Pllsai1rdycW<CicrSpec> {
        Pllsai1rdycW::new(self, 6)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt clear"]
    #[inline(always)]
    pub fn pllsai2rdyc(&mut self) -> Pllsai2rdycW<CicrSpec> {
        Pllsai2rdycW::new(self, 7)
    }
    #[doc = "Bit 8 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn cssc(&mut self) -> CsscW<CicrSpec> {
        CsscW::new(self, 8)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt clear"]
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LsecsscW<CicrSpec> {
        LsecsscW::new(self, 9)
    }
    #[doc = "Bit 10 - HSI48 oscillator ready interrupt clear"]
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> Hsi48rdycW<CicrSpec> {
        Hsi48rdycW::new(self, 10)
    }
}
#[doc = "Clock interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicrSpec;
impl crate::RegisterSpec for CicrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cicr::W`](W) writer structure"]
impl crate::Writable for CicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CicrSpec {}
