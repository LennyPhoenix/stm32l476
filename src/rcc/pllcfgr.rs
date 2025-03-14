#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PllcfgrSpec>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PllcfgrSpec>;
#[doc = "Field `PLLSRC` reader - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub type PllsrcR = crate::FieldReader;
#[doc = "Field `PLLSRC` writer - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub type PllsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLM` reader - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub type PllmR = crate::FieldReader;
#[doc = "Field `PLLM` writer - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub type PllmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLN` reader - Main PLL multiplication factor for VCO"]
pub type PllnR = crate::FieldReader;
#[doc = "Field `PLLN` writer - Main PLL multiplication factor for VCO"]
pub type PllnW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLLPEN` reader - Main PLL PLLSAI3CLK output enable"]
pub type PllpenR = crate::BitReader;
#[doc = "Field `PLLPEN` writer - Main PLL PLLSAI3CLK output enable"]
pub type PllpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLP` reader - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
pub type PllpR = crate::BitReader;
#[doc = "Field `PLLP` writer - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
pub type PllpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLQEN` reader - Main PLL PLLUSB1CLK output enable"]
pub type PllqenR = crate::BitReader;
#[doc = "Field `PLLQEN` writer - Main PLL PLLUSB1CLK output enable"]
pub type PllqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLQ` reader - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
pub type PllqR = crate::FieldReader;
#[doc = "Field `PLLQ` writer - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
pub type PllqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLREN` reader - Main PLL PLLCLK output enable"]
pub type PllrenR = crate::BitReader;
#[doc = "Field `PLLREN` writer - Main PLL PLLCLK output enable"]
pub type PllrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLR` reader - Main PLL division factor for PLLCLK (system clock)"]
pub type PllrR = crate::FieldReader;
#[doc = "Field `PLLR` writer - Main PLL division factor for PLLCLK (system clock)"]
pub type PllrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLPDIV` reader - Main PLL division factor for PLLSAI2CLK"]
pub type PllpdivR = crate::FieldReader;
#[doc = "Field `PLLPDIV` writer - Main PLL division factor for PLLSAI2CLK"]
pub type PllpdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    pub fn pllm(&self) -> PllmR {
        PllmR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&self) -> PllnR {
        PllnR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Main PLL PLLSAI3CLK output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PllpenR {
        PllpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
    #[inline(always)]
    pub fn pllp(&self) -> PllpR {
        PllpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Main PLL PLLUSB1CLK output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PllqenR {
        PllqenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
    #[inline(always)]
    pub fn pllq(&self) -> PllqR {
        PllqR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - Main PLL PLLCLK output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PllrenR {
        PllrenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLLCLK (system clock)"]
    #[inline(always)]
    pub fn pllr(&self) -> PllrR {
        PllrR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:31 - Main PLL division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllpdiv(&self) -> PllpdivR {
        PllpdivR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PllsrcW<PllcfgrSpec> {
        PllsrcW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    pub fn pllm(&mut self) -> PllmW<PllcfgrSpec> {
        PllmW::new(self, 4)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&mut self) -> PllnW<PllcfgrSpec> {
        PllnW::new(self, 8)
    }
    #[doc = "Bit 16 - Main PLL PLLSAI3CLK output enable"]
    #[inline(always)]
    pub fn pllpen(&mut self) -> PllpenW<PllcfgrSpec> {
        PllpenW::new(self, 16)
    }
    #[doc = "Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
    #[inline(always)]
    pub fn pllp(&mut self) -> PllpW<PllcfgrSpec> {
        PllpW::new(self, 17)
    }
    #[doc = "Bit 20 - Main PLL PLLUSB1CLK output enable"]
    #[inline(always)]
    pub fn pllqen(&mut self) -> PllqenW<PllcfgrSpec> {
        PllqenW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
    #[inline(always)]
    pub fn pllq(&mut self) -> PllqW<PllcfgrSpec> {
        PllqW::new(self, 21)
    }
    #[doc = "Bit 24 - Main PLL PLLCLK output enable"]
    #[inline(always)]
    pub fn pllren(&mut self) -> PllrenW<PllcfgrSpec> {
        PllrenW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLLCLK (system clock)"]
    #[inline(always)]
    pub fn pllr(&mut self) -> PllrW<PllcfgrSpec> {
        PllrW::new(self, 25)
    }
    #[doc = "Bits 27:31 - Main PLL division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllpdiv(&mut self) -> PllpdivW<PllcfgrSpec> {
        PllpdivW::new(self, 27)
    }
}
#[doc = "PLL configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcfgrSpec;
impl crate::RegisterSpec for PllcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PllcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PllcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x1000"]
impl crate::Resettable for PllcfgrSpec {
    const RESET_VALUE: u32 = 0x1000;
}
