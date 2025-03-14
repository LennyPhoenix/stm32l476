#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<Ahb2rstrSpec>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<Ahb2rstrSpec>;
#[doc = "Field `GPIOARST` reader - IO port A reset"]
pub type GpioarstR = crate::BitReader;
#[doc = "Field `GPIOARST` writer - IO port A reset"]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub type GpiobrstR = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub type GpiocrstR = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub type GpiocrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub type GpiodrstR = crate::BitReader;
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub type GpiodrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - IO port E reset"]
pub type GpioerstR = crate::BitReader;
#[doc = "Field `GPIOERST` writer - IO port E reset"]
pub type GpioerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFRST` reader - IO port F reset"]
pub type GpiofrstR = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - IO port F reset"]
pub type GpiofrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGRST` reader - IO port G reset"]
pub type GpiogrstR = crate::BitReader;
#[doc = "Field `GPIOGRST` writer - IO port G reset"]
pub type GpiogrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHRST` reader - IO port H reset"]
pub type GpiohrstR = crate::BitReader;
#[doc = "Field `GPIOHRST` writer - IO port H reset"]
pub type GpiohrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOIRST` reader - IO port I reset"]
pub type GpioirstR = crate::BitReader;
#[doc = "Field `GPIOIRST` writer - IO port I reset"]
pub type GpioirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFSRST` reader - USB OTG FS reset"]
pub type OtgfsrstR = crate::BitReader;
#[doc = "Field `OTGFSRST` writer - USB OTG FS reset"]
pub type OtgfsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type AdcrstR = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMIRST` reader - Digital Camera Interface reset"]
pub type DcmirstR = crate::BitReader;
#[doc = "Field `DCMIRST` writer - Digital Camera Interface reset"]
pub type DcmirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESRST` reader - AES hardware accelerator reset"]
pub type AesrstR = crate::BitReader;
#[doc = "Field `AESRST` writer - AES hardware accelerator reset"]
pub type AesrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH1RST` reader - Hash reset"]
pub type Hash1rstR = crate::BitReader;
#[doc = "Field `HASH1RST` writer - Hash reset"]
pub type Hash1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGRST` reader - Random number generator reset"]
pub type RngrstR = crate::BitReader;
#[doc = "Field `RNGRST` writer - Random number generator reset"]
pub type RngrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GpiocrstR {
        GpiocrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GpiodrstR {
        GpiodrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GpioerstR {
        GpioerstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GpiofrstR {
        GpiofrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GpiogrstR {
        GpiogrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GpiohrstR {
        GpiohrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I reset"]
    #[inline(always)]
    pub fn gpioirst(&self) -> GpioirstR {
        GpioirstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - USB OTG FS reset"]
    #[inline(always)]
    pub fn otgfsrst(&self) -> OtgfsrstR {
        OtgfsrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Digital Camera Interface reset"]
    #[inline(always)]
    pub fn dcmirst(&self) -> DcmirstR {
        DcmirstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset"]
    #[inline(always)]
    pub fn aesrst(&self) -> AesrstR {
        AesrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Hash reset"]
    #[inline(always)]
    pub fn hash1rst(&self) -> Hash1rstR {
        Hash1rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random number generator reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RngrstR {
        RngrstR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GpioarstW<Ahb2rstrSpec> {
        GpioarstW::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GpiobrstW<Ahb2rstrSpec> {
        GpiobrstW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GpiocrstW<Ahb2rstrSpec> {
        GpiocrstW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GpiodrstW<Ahb2rstrSpec> {
        GpiodrstW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GpioerstW<Ahb2rstrSpec> {
        GpioerstW::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GpiofrstW<Ahb2rstrSpec> {
        GpiofrstW::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GpiogrstW<Ahb2rstrSpec> {
        GpiogrstW::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GpiohrstW<Ahb2rstrSpec> {
        GpiohrstW::new(self, 7)
    }
    #[doc = "Bit 8 - IO port I reset"]
    #[inline(always)]
    pub fn gpioirst(&mut self) -> GpioirstW<Ahb2rstrSpec> {
        GpioirstW::new(self, 8)
    }
    #[doc = "Bit 12 - USB OTG FS reset"]
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OtgfsrstW<Ahb2rstrSpec> {
        OtgfsrstW::new(self, 12)
    }
    #[doc = "Bit 13 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> AdcrstW<Ahb2rstrSpec> {
        AdcrstW::new(self, 13)
    }
    #[doc = "Bit 14 - Digital Camera Interface reset"]
    #[inline(always)]
    pub fn dcmirst(&mut self) -> DcmirstW<Ahb2rstrSpec> {
        DcmirstW::new(self, 14)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset"]
    #[inline(always)]
    pub fn aesrst(&mut self) -> AesrstW<Ahb2rstrSpec> {
        AesrstW::new(self, 16)
    }
    #[doc = "Bit 17 - Hash reset"]
    #[inline(always)]
    pub fn hash1rst(&mut self) -> Hash1rstW<Ahb2rstrSpec> {
        Hash1rstW::new(self, 17)
    }
    #[doc = "Bit 18 - Random number generator reset"]
    #[inline(always)]
    pub fn rngrst(&mut self) -> RngrstW<Ahb2rstrSpec> {
        RngrstW::new(self, 18)
    }
}
#[doc = "AHB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2rstrSpec;
impl crate::RegisterSpec for Ahb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for Ahb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for Ahb2rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for Ahb2rstrSpec {}
