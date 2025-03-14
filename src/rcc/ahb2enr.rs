#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<Ahb2enrSpec>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<Ahb2enrSpec>;
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub type GpioaenR = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub type GpiobenR = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub type GpiocenR = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub type GpiocenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub type GpiodenR = crate::BitReader;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub type GpiodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub type GpioeenR = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub type GpioeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - IO port F clock enable"]
pub type GpiofenR = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - IO port F clock enable"]
pub type GpiofenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGEN` reader - IO port G clock enable"]
pub type GpiogenR = crate::BitReader;
#[doc = "Field `GPIOGEN` writer - IO port G clock enable"]
pub type GpiogenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHEN` reader - IO port H clock enable"]
pub type GpiohenR = crate::BitReader;
#[doc = "Field `GPIOHEN` writer - IO port H clock enable"]
pub type GpiohenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOIEN` reader - IO port I clock enable"]
pub type GpioienR = crate::BitReader;
#[doc = "Field `GPIOIEN` writer - IO port I clock enable"]
pub type GpioienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFSEN` reader - OTG full speed clock enable"]
pub type OtgfsenR = crate::BitReader;
#[doc = "Field `OTGFSEN` writer - OTG full speed clock enable"]
pub type OtgfsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMIEN` reader - DCMI clock enable"]
pub type DcmienR = crate::BitReader;
#[doc = "Field `DCMIEN` writer - DCMI clock enable"]
pub type DcmienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESEN` reader - AES accelerator clock enable"]
pub type AesenR = crate::BitReader;
#[doc = "Field `AESEN` writer - AES accelerator clock enable"]
pub type AesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH1EN` reader - HASH clock enable"]
pub type Hash1enR = crate::BitReader;
#[doc = "Field `HASH1EN` writer - HASH clock enable"]
pub type Hash1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGEN` reader - Random Number Generator clock enable"]
pub type RngenR = crate::BitReader;
#[doc = "Field `RNGEN` writer - Random Number Generator clock enable"]
pub type RngenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GpiocenR {
        GpiocenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GpiodenR {
        GpiodenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GpioeenR {
        GpioeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GpiofenR {
        GpiofenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GpiogenR {
        GpiogenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GpiohenR {
        GpiohenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    pub fn gpioien(&self) -> GpioienR {
        GpioienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG full speed clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OtgfsenR {
        OtgfsenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    pub fn dcmien(&self) -> DcmienR {
        DcmienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AesenR {
        AesenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn hash1en(&self) -> Hash1enR {
        Hash1enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RngenR {
        RngenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GpioaenW<Ahb2enrSpec> {
        GpioaenW::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GpiobenW<Ahb2enrSpec> {
        GpiobenW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GpiocenW<Ahb2enrSpec> {
        GpiocenW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GpiodenW<Ahb2enrSpec> {
        GpiodenW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GpioeenW<Ahb2enrSpec> {
        GpioeenW::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GpiofenW<Ahb2enrSpec> {
        GpiofenW::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GpiogenW<Ahb2enrSpec> {
        GpiogenW::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GpiohenW<Ahb2enrSpec> {
        GpiohenW::new(self, 7)
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    pub fn gpioien(&mut self) -> GpioienW<Ahb2enrSpec> {
        GpioienW::new(self, 8)
    }
    #[doc = "Bit 12 - OTG full speed clock enable"]
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OtgfsenW<Ahb2enrSpec> {
        OtgfsenW::new(self, 12)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> AdcenW<Ahb2enrSpec> {
        AdcenW::new(self, 13)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    pub fn dcmien(&mut self) -> DcmienW<Ahb2enrSpec> {
        DcmienW::new(self, 14)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&mut self) -> AesenW<Ahb2enrSpec> {
        AesenW::new(self, 16)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn hash1en(&mut self) -> Hash1enW<Ahb2enrSpec> {
        Hash1enW::new(self, 17)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RngenW<Ahb2enrSpec> {
        RngenW::new(self, 18)
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2enrSpec;
impl crate::RegisterSpec for Ahb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for Ahb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for Ahb2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for Ahb2enrSpec {}
