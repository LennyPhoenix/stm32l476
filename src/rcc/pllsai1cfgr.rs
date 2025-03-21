#[doc = "Register `PLLSAI1CFGR` reader"]
pub type R = crate::R<Pllsai1cfgrSpec>;
#[doc = "Register `PLLSAI1CFGR` writer"]
pub type W = crate::W<Pllsai1cfgrSpec>;
#[doc = "Field `PLLSAI1N` reader - SAI1PLL multiplication factor for VCO"]
pub type Pllsai1nR = crate::FieldReader;
#[doc = "Field `PLLSAI1N` writer - SAI1PLL multiplication factor for VCO"]
pub type Pllsai1nW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLLSAI1PEN` reader - SAI1PLL PLLSAI1CLK output enable"]
pub type Pllsai1penR = crate::BitReader;
#[doc = "Field `PLLSAI1PEN` writer - SAI1PLL PLLSAI1CLK output enable"]
pub type Pllsai1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1P` reader - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
pub type Pllsai1pR = crate::BitReader;
#[doc = "Field `PLLSAI1P` writer - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
pub type Pllsai1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1QEN` reader - SAI1PLL PLLUSB2CLK output enable"]
pub type Pllsai1qenR = crate::BitReader;
#[doc = "Field `PLLSAI1QEN` writer - SAI1PLL PLLUSB2CLK output enable"]
pub type Pllsai1qenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1Q` reader - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
pub type Pllsai1qR = crate::FieldReader;
#[doc = "Field `PLLSAI1Q` writer - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
pub type Pllsai1qW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLSAI1REN` reader - PLLSAI1 PLLADC1CLK output enable"]
pub type Pllsai1renR = crate::BitReader;
#[doc = "Field `PLLSAI1REN` writer - PLLSAI1 PLLADC1CLK output enable"]
pub type Pllsai1renW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1R` reader - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
pub type Pllsai1rR = crate::FieldReader;
#[doc = "Field `PLLSAI1R` writer - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
pub type Pllsai1rW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLSAI1PDIV` reader - PLLSAI1 division factor for PLLSAI1CLK"]
pub type Pllsai1pdivR = crate::FieldReader;
#[doc = "Field `PLLSAI1PDIV` writer - PLLSAI1 division factor for PLLSAI1CLK"]
pub type Pllsai1pdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 8:14 - SAI1PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai1n(&self) -> Pllsai1nR {
        Pllsai1nR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - SAI1PLL PLLSAI1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1pen(&self) -> Pllsai1penR {
        Pllsai1penR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai1p(&self) -> Pllsai1pR {
        Pllsai1pR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - SAI1PLL PLLUSB2CLK output enable"]
    #[inline(always)]
    pub fn pllsai1qen(&self) -> Pllsai1qenR {
        Pllsai1qenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
    #[inline(always)]
    pub fn pllsai1q(&self) -> Pllsai1qR {
        Pllsai1qR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - PLLSAI1 PLLADC1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1ren(&self) -> Pllsai1renR {
        Pllsai1renR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai1r(&self) -> Pllsai1rR {
        Pllsai1rR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK"]
    #[inline(always)]
    pub fn pllsai1pdiv(&self) -> Pllsai1pdivR {
        Pllsai1pdivR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - SAI1PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai1n(&mut self) -> Pllsai1nW<Pllsai1cfgrSpec> {
        Pllsai1nW::new(self, 8)
    }
    #[doc = "Bit 16 - SAI1PLL PLLSAI1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1pen(&mut self) -> Pllsai1penW<Pllsai1cfgrSpec> {
        Pllsai1penW::new(self, 16)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai1p(&mut self) -> Pllsai1pW<Pllsai1cfgrSpec> {
        Pllsai1pW::new(self, 17)
    }
    #[doc = "Bit 20 - SAI1PLL PLLUSB2CLK output enable"]
    #[inline(always)]
    pub fn pllsai1qen(&mut self) -> Pllsai1qenW<Pllsai1cfgrSpec> {
        Pllsai1qenW::new(self, 20)
    }
    #[doc = "Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
    #[inline(always)]
    pub fn pllsai1q(&mut self) -> Pllsai1qW<Pllsai1cfgrSpec> {
        Pllsai1qW::new(self, 21)
    }
    #[doc = "Bit 24 - PLLSAI1 PLLADC1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1ren(&mut self) -> Pllsai1renW<Pllsai1cfgrSpec> {
        Pllsai1renW::new(self, 24)
    }
    #[doc = "Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai1r(&mut self) -> Pllsai1rW<Pllsai1cfgrSpec> {
        Pllsai1rW::new(self, 25)
    }
    #[doc = "Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK"]
    #[inline(always)]
    pub fn pllsai1pdiv(&mut self) -> Pllsai1pdivW<Pllsai1cfgrSpec> {
        Pllsai1pdivW::new(self, 27)
    }
}
#[doc = "PLLSAI1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllsai1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pllsai1cfgrSpec;
impl crate::RegisterSpec for Pllsai1cfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsai1cfgr::R`](R) reader structure"]
impl crate::Readable for Pllsai1cfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllsai1cfgr::W`](W) writer structure"]
impl crate::Writable for Pllsai1cfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLSAI1CFGR to value 0x1000"]
impl crate::Resettable for Pllsai1cfgrSpec {
    const RESET_VALUE: u32 = 0x1000;
}
