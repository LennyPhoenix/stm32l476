#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CifrSpec>;
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LsirdyfR = crate::BitReader;
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub type LserdyfR = crate::BitReader;
#[doc = "Field `MSIRDYF` reader - MSI ready interrupt flag"]
pub type MsirdyfR = crate::BitReader;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub type HsirdyfR = crate::BitReader;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub type HserdyfR = crate::BitReader;
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag"]
pub type PllrdyfR = crate::BitReader;
#[doc = "Field `PLLSAI1RDYF` reader - PLLSAI1 ready interrupt flag"]
pub type Pllsai1rdyfR = crate::BitReader;
#[doc = "Field `PLLSAI2RDYF` reader - PLLSAI2 ready interrupt flag"]
pub type Pllsai2rdyfR = crate::BitReader;
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CssfR = crate::BitReader;
#[doc = "Field `LSECSSF` reader - LSE Clock security system interrupt flag"]
pub type LsecssfR = crate::BitReader;
#[doc = "Field `HSI48RDYF` reader - HSI48 ready interrupt flag"]
pub type Hsi48rdyfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LsirdyfR {
        LsirdyfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LserdyfR {
        LserdyfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MsirdyfR {
        MsirdyfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HsirdyfR {
        HsirdyfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HserdyfR {
        HserdyfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PllrdyfR {
        PllrdyfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt flag"]
    #[inline(always)]
    pub fn pllsai1rdyf(&self) -> Pllsai1rdyfR {
        Pllsai1rdyfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt flag"]
    #[inline(always)]
    pub fn pllsai2rdyf(&self) -> Pllsai2rdyfR {
        Pllsai2rdyfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CssfR {
        CssfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LsecssfR {
        LsecssfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> Hsi48rdyfR {
        Hsi48rdyfR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Clock interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CifrSpec;
impl crate::RegisterSpec for CifrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cifr::R`](R) reader structure"]
impl crate::Readable for CifrSpec {}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CifrSpec {}
