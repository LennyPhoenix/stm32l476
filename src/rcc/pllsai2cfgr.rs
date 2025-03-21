#[doc = "Register `PLLSAI2CFGR` reader"]
pub type R = crate::R<Pllsai2cfgrSpec>;
#[doc = "Register `PLLSAI2CFGR` writer"]
pub type W = crate::W<Pllsai2cfgrSpec>;
#[doc = "Field `PLLSAI2N` reader - SAI2PLL multiplication factor for VCO"]
pub type Pllsai2nR = crate::FieldReader;
#[doc = "Field `PLLSAI2N` writer - SAI2PLL multiplication factor for VCO"]
pub type Pllsai2nW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLLSAI2PEN` reader - SAI2PLL PLLSAI2CLK output enable"]
pub type Pllsai2penR = crate::BitReader;
#[doc = "Field `PLLSAI2PEN` writer - SAI2PLL PLLSAI2CLK output enable"]
pub type Pllsai2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2P` reader - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
pub type Pllsai2pR = crate::BitReader;
#[doc = "Field `PLLSAI2P` writer - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
pub type Pllsai2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2REN` reader - PLLSAI2 PLLADC2CLK output enable"]
pub type Pllsai2renR = crate::BitReader;
#[doc = "Field `PLLSAI2REN` writer - PLLSAI2 PLLADC2CLK output enable"]
pub type Pllsai2renW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2R` reader - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
pub type Pllsai2rR = crate::FieldReader;
#[doc = "Field `PLLSAI2R` writer - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
pub type Pllsai2rW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLSAI2PDIV` reader - PLLSAI2 division factor for PLLSAI2CLK"]
pub type Pllsai2pdivR = crate::FieldReader;
#[doc = "Field `PLLSAI2PDIV` writer - PLLSAI2 division factor for PLLSAI2CLK"]
pub type Pllsai2pdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai2n(&self) -> Pllsai2nR {
        Pllsai2nR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2pen(&self) -> Pllsai2penR {
        Pllsai2penR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai2p(&self) -> Pllsai2pR {
        Pllsai2pR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2ren(&self) -> Pllsai2renR {
        Pllsai2renR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai2r(&self) -> Pllsai2rR {
        Pllsai2rR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllsai2pdiv(&self) -> Pllsai2pdivR {
        Pllsai2pdivR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai2n(&mut self) -> Pllsai2nW<Pllsai2cfgrSpec> {
        Pllsai2nW::new(self, 8)
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2pen(&mut self) -> Pllsai2penW<Pllsai2cfgrSpec> {
        Pllsai2penW::new(self, 16)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai2p(&mut self) -> Pllsai2pW<Pllsai2cfgrSpec> {
        Pllsai2pW::new(self, 17)
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2ren(&mut self) -> Pllsai2renW<Pllsai2cfgrSpec> {
        Pllsai2renW::new(self, 24)
    }
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai2r(&mut self) -> Pllsai2rW<Pllsai2cfgrSpec> {
        Pllsai2rW::new(self, 25)
    }
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllsai2pdiv(&mut self) -> Pllsai2pdivW<Pllsai2cfgrSpec> {
        Pllsai2pdivW::new(self, 27)
    }
}
#[doc = "PLLSAI2 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllsai2cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai2cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pllsai2cfgrSpec;
impl crate::RegisterSpec for Pllsai2cfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsai2cfgr::R`](R) reader structure"]
impl crate::Readable for Pllsai2cfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllsai2cfgr::W`](W) writer structure"]
impl crate::Writable for Pllsai2cfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLSAI2CFGR to value 0x1000"]
impl crate::Resettable for Pllsai2cfgrSpec {
    const RESET_VALUE: u32 = 0x1000;
}
