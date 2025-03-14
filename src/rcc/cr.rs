#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `MSION` reader - MSI clock enable"]
pub type MsionR = crate::BitReader;
#[doc = "Field `MSION` writer - MSI clock enable"]
pub type MsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRDY` reader - MSI clock ready flag"]
pub type MsirdyR = crate::BitReader;
#[doc = "Field `MSIPLLEN` reader - MSI clock PLL enable"]
pub type MsipllenR = crate::BitReader;
#[doc = "Field `MSIPLLEN` writer - MSI clock PLL enable"]
pub type MsipllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRGSEL` writer - MSI clock range selection"]
pub type MsirgselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub type MsirangeR = crate::FieldReader;
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub type MsirangeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HSION` reader - HSI clock enable"]
pub type HsionR = crate::BitReader;
#[doc = "Field `HSION` writer - HSI clock enable"]
pub type HsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIKERON` reader - HSI always enable for peripheral kernels"]
pub type HsikeronR = crate::BitReader;
#[doc = "Field `HSIKERON` writer - HSI always enable for peripheral kernels"]
pub type HsikeronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - HSI clock ready flag"]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSIASFS` reader - HSI automatic start from Stop"]
pub type HsiasfsR = crate::BitReader;
#[doc = "Field `HSIASFS` writer - HSI automatic start from Stop"]
pub type HsiasfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub type HseonR = crate::BitReader;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub type HseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HserdyR = crate::BitReader;
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass"]
pub type HsebypR = crate::BitReader;
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass"]
pub type HsebypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSON` writer - Clock security system enable"]
pub type CssonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLON` reader - Main PLL enable"]
pub type PllonR = crate::BitReader;
#[doc = "Field `PLLON` writer - Main PLL enable"]
pub type PllonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - Main PLL clock ready flag"]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `PLLSAI1ON` reader - SAI1 PLL enable"]
pub type Pllsai1onR = crate::BitReader;
#[doc = "Field `PLLSAI1ON` writer - SAI1 PLL enable"]
pub type Pllsai1onW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1RDY` reader - SAI1 PLL clock ready flag"]
pub type Pllsai1rdyR = crate::BitReader;
#[doc = "Field `PLLSAI2ON` reader - SAI2 PLL enable"]
pub type Pllsai2onR = crate::BitReader;
#[doc = "Field `PLLSAI2ON` writer - SAI2 PLL enable"]
pub type Pllsai2onW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2RDY` reader - SAI2 PLL clock ready flag"]
pub type Pllsai2rdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&self) -> MsionR {
        MsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSI clock ready flag"]
    #[inline(always)]
    pub fn msirdy(&self) -> MsirdyR {
        MsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&self) -> MsipllenR {
        MsipllenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MsirangeR {
        MsirangeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HSI clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HsionR {
        HsionR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HsikeronR {
        HsikeronR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&self) -> HsiasfsR {
        HsiasfsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HseonR {
        HseonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HsebypR {
        HsebypR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PllonR {
        PllonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Main PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SAI1 PLL enable"]
    #[inline(always)]
    pub fn pllsai1on(&self) -> Pllsai1onR {
        Pllsai1onR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SAI1 PLL clock ready flag"]
    #[inline(always)]
    pub fn pllsai1rdy(&self) -> Pllsai1rdyR {
        Pllsai1rdyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SAI2 PLL enable"]
    #[inline(always)]
    pub fn pllsai2on(&self) -> Pllsai2onR {
        Pllsai2onR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SAI2 PLL clock ready flag"]
    #[inline(always)]
    pub fn pllsai2rdy(&self) -> Pllsai2rdyR {
        Pllsai2rdyR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&mut self) -> MsionW<CrSpec> {
        MsionW::new(self, 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&mut self) -> MsipllenW<CrSpec> {
        MsipllenW::new(self, 2)
    }
    #[doc = "Bit 3 - MSI clock range selection"]
    #[inline(always)]
    pub fn msirgsel(&mut self) -> MsirgselW<CrSpec> {
        MsirgselW::new(self, 3)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&mut self) -> MsirangeW<CrSpec> {
        MsirangeW::new(self, 4)
    }
    #[doc = "Bit 8 - HSI clock enable"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HsionW<CrSpec> {
        HsionW::new(self, 8)
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HsikeronW<CrSpec> {
        HsikeronW::new(self, 9)
    }
    #[doc = "Bit 11 - HSI automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&mut self) -> HsiasfsW<CrSpec> {
        HsiasfsW::new(self, 11)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HseonW<CrSpec> {
        HseonW::new(self, 16)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HsebypW<CrSpec> {
        HsebypW::new(self, 18)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    pub fn csson(&mut self) -> CssonW<CrSpec> {
        CssonW::new(self, 19)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PllonW<CrSpec> {
        PllonW::new(self, 24)
    }
    #[doc = "Bit 26 - SAI1 PLL enable"]
    #[inline(always)]
    pub fn pllsai1on(&mut self) -> Pllsai1onW<CrSpec> {
        Pllsai1onW::new(self, 26)
    }
    #[doc = "Bit 28 - SAI2 PLL enable"]
    #[inline(always)]
    pub fn pllsai2on(&mut self) -> Pllsai2onW<CrSpec> {
        Pllsai2onW::new(self, 28)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x63"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x63;
}
