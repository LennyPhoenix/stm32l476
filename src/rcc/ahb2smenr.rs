#[doc = "Register `AHB2SMENR` reader"]
pub type R = crate::R<Ahb2smenrSpec>;
#[doc = "Register `AHB2SMENR` writer"]
pub type W = crate::W<Ahb2smenrSpec>;
#[doc = "Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes"]
pub type GpioasmenR = crate::BitReader;
#[doc = "Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes"]
pub type GpioasmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes"]
pub type GpiobsmenR = crate::BitReader;
#[doc = "Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes"]
pub type GpiobsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes"]
pub type GpiocsmenR = crate::BitReader;
#[doc = "Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes"]
pub type GpiocsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes"]
pub type GpiodsmenR = crate::BitReader;
#[doc = "Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes"]
pub type GpiodsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes"]
pub type GpioesmenR = crate::BitReader;
#[doc = "Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes"]
pub type GpioesmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFSMEN` reader - IO port F clocks enable during Sleep and Stop modes"]
pub type GpiofsmenR = crate::BitReader;
#[doc = "Field `GPIOFSMEN` writer - IO port F clocks enable during Sleep and Stop modes"]
pub type GpiofsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGSMEN` reader - IO port G clocks enable during Sleep and Stop modes"]
pub type GpiogsmenR = crate::BitReader;
#[doc = "Field `GPIOGSMEN` writer - IO port G clocks enable during Sleep and Stop modes"]
pub type GpiogsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHSMEN` reader - IO port H clocks enable during Sleep and Stop modes"]
pub type GpiohsmenR = crate::BitReader;
#[doc = "Field `GPIOHSMEN` writer - IO port H clocks enable during Sleep and Stop modes"]
pub type GpiohsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOISMEN` reader - IO port I clocks enable during Sleep and Stop modes"]
pub type GpioismenR = crate::BitReader;
#[doc = "Field `GPIOISMEN` writer - IO port I clocks enable during Sleep and Stop modes"]
pub type GpioismenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type Sram2smenR = crate::BitReader;
#[doc = "Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type Sram2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFSSMEN` reader - OTG full speed clocks enable during Sleep and Stop modes"]
pub type OtgfssmenR = crate::BitReader;
#[doc = "Field `OTGFSSMEN` writer - OTG full speed clocks enable during Sleep and Stop modes"]
pub type OtgfssmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCFSSMEN` reader - ADC clocks enable during Sleep and Stop modes"]
pub type AdcfssmenR = crate::BitReader;
#[doc = "Field `ADCFSSMEN` writer - ADC clocks enable during Sleep and Stop modes"]
pub type AdcfssmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMISMEN` reader - DCMI clock enable during Sleep and Stop modes"]
pub type DcmismenR = crate::BitReader;
#[doc = "Field `DCMISMEN` writer - DCMI clock enable during Sleep and Stop modes"]
pub type DcmismenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESSMEN` reader - AES accelerator clocks enable during Sleep and Stop modes"]
pub type AessmenR = crate::BitReader;
#[doc = "Field `AESSMEN` writer - AES accelerator clocks enable during Sleep and Stop modes"]
pub type AessmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH1SMEN` reader - HASH clock enable during Sleep and Stop modes"]
pub type Hash1smenR = crate::BitReader;
#[doc = "Field `HASH1SMEN` writer - HASH clock enable during Sleep and Stop modes"]
pub type Hash1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSMEN` reader - Random Number Generator clocks enable during Sleep and Stop modes"]
pub type RngsmenR = crate::BitReader;
#[doc = "Field `RNGSMEN` writer - Random Number Generator clocks enable during Sleep and Stop modes"]
pub type RngsmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GpioasmenR {
        GpioasmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GpiobsmenR {
        GpiobsmenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GpiocsmenR {
        GpiocsmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GpiodsmenR {
        GpiodsmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GpioesmenR {
        GpioesmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GpiofsmenR {
        GpiofsmenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GpiogsmenR {
        GpiogsmenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GpiohsmenR {
        GpiohsmenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioismen(&self) -> GpioismenR {
        GpioismenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram2smen(&self) -> Sram2smenR {
        Sram2smenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG full speed clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn otgfssmen(&self) -> OtgfssmenR {
        OtgfssmenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adcfssmen(&self) -> AdcfssmenR {
        AdcfssmenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dcmismen(&self) -> DcmismenR {
        DcmismenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn aessmen(&self) -> AessmenR {
        AessmenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn hash1smen(&self) -> Hash1smenR {
        Hash1smenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RngsmenR {
        RngsmenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GpioasmenW<Ahb2smenrSpec> {
        GpioasmenW::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GpiobsmenW<Ahb2smenrSpec> {
        GpiobsmenW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GpiocsmenW<Ahb2smenrSpec> {
        GpiocsmenW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GpiodsmenW<Ahb2smenrSpec> {
        GpiodsmenW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GpioesmenW<Ahb2smenrSpec> {
        GpioesmenW::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiofsmen(&mut self) -> GpiofsmenW<Ahb2smenrSpec> {
        GpiofsmenW::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiogsmen(&mut self) -> GpiogsmenW<Ahb2smenrSpec> {
        GpiogsmenW::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GpiohsmenW<Ahb2smenrSpec> {
        GpiohsmenW::new(self, 7)
    }
    #[doc = "Bit 8 - IO port I clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioismen(&mut self) -> GpioismenW<Ahb2smenrSpec> {
        GpioismenW::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram2smen(&mut self) -> Sram2smenW<Ahb2smenrSpec> {
        Sram2smenW::new(self, 9)
    }
    #[doc = "Bit 12 - OTG full speed clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn otgfssmen(&mut self) -> OtgfssmenW<Ahb2smenrSpec> {
        OtgfssmenW::new(self, 12)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adcfssmen(&mut self) -> AdcfssmenW<Ahb2smenrSpec> {
        AdcfssmenW::new(self, 13)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dcmismen(&mut self) -> DcmismenW<Ahb2smenrSpec> {
        DcmismenW::new(self, 14)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn aessmen(&mut self) -> AessmenW<Ahb2smenrSpec> {
        AessmenW::new(self, 16)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn hash1smen(&mut self) -> Hash1smenW<Ahb2smenrSpec> {
        Hash1smenW::new(self, 17)
    }
    #[doc = "Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RngsmenW<Ahb2smenrSpec> {
        RngsmenW::new(self, 18)
    }
}
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2smenrSpec;
impl crate::RegisterSpec for Ahb2smenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2smenr::R`](R) reader structure"]
impl crate::Readable for Ahb2smenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2smenr::W`](W) writer structure"]
impl crate::Writable for Ahb2smenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2SMENR to value 0x0005_32ff"]
impl crate::Resettable for Ahb2smenrSpec {
    const RESET_VALUE: u32 = 0x0005_32ff;
}
