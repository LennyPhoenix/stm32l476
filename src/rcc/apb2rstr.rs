#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<Apb2rstrSpec>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<Apb2rstrSpec>;
#[doc = "Field `SYSCFGRST` reader - System configuration (SYSCFG) reset"]
pub type SyscfgrstR = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - System configuration (SYSCFG) reset"]
pub type SyscfgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMCRST` reader - SDMMC reset"]
pub type SdmmcrstR = crate::BitReader;
#[doc = "Field `SDMMCRST` writer - SDMMC reset"]
pub type SdmmcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type Tim1rstR = crate::BitReader;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type Tim1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8RST` reader - TIM8 timer reset"]
pub type Tim8rstR = crate::BitReader;
#[doc = "Field `TIM8RST` writer - TIM8 timer reset"]
pub type Tim8rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15RST` reader - TIM15 timer reset"]
pub type Tim15rstR = crate::BitReader;
#[doc = "Field `TIM15RST` writer - TIM15 timer reset"]
pub type Tim15rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub type Tim16rstR = crate::BitReader;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub type Tim16rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17RST` reader - TIM17 timer reset"]
pub type Tim17rstR = crate::BitReader;
#[doc = "Field `TIM17RST` writer - TIM17 timer reset"]
pub type Tim17rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1RST` reader - Serial audio interface 1 (SAI1) reset"]
pub type Sai1rstR = crate::BitReader;
#[doc = "Field `SAI1RST` writer - Serial audio interface 1 (SAI1) reset"]
pub type Sai1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2RST` reader - Serial audio interface 2 (SAI2) reset"]
pub type Sai2rstR = crate::BitReader;
#[doc = "Field `SAI2RST` writer - Serial audio interface 2 (SAI2) reset"]
pub type Sai2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDMRST` reader - Digital filters for sigma-delata modulators (DFSDM) reset"]
pub type DfsdmrstR = crate::BitReader;
#[doc = "Field `DFSDMRST` writer - Digital filters for sigma-delata modulators (DFSDM) reset"]
pub type DfsdmrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System configuration (SYSCFG) reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - SDMMC reset"]
    #[inline(always)]
    pub fn sdmmcrst(&self) -> SdmmcrstR {
        SdmmcrstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> Tim1rstR {
        Tim1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> Tim8rstR {
        Tim8rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> Tim15rstR {
        Tim15rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> Tim16rstR {
        Tim16rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> Tim17rstR {
        Tim17rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset"]
    #[inline(always)]
    pub fn sai1rst(&self) -> Sai1rstR {
        Sai1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Serial audio interface 2 (SAI2) reset"]
    #[inline(always)]
    pub fn sai2rst(&self) -> Sai2rstR {
        Sai2rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset"]
    #[inline(always)]
    pub fn dfsdmrst(&self) -> DfsdmrstR {
        DfsdmrstR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration (SYSCFG) reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<Apb2rstrSpec> {
        SyscfgrstW::new(self, 0)
    }
    #[doc = "Bit 10 - SDMMC reset"]
    #[inline(always)]
    pub fn sdmmcrst(&mut self) -> SdmmcrstW<Apb2rstrSpec> {
        SdmmcrstW::new(self, 10)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> Tim1rstW<Apb2rstrSpec> {
        Tim1rstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> Spi1rstW<Apb2rstrSpec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> Tim8rstW<Apb2rstrSpec> {
        Tim8rstW::new(self, 13)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> Usart1rstW<Apb2rstrSpec> {
        Usart1rstW::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&mut self) -> Tim15rstW<Apb2rstrSpec> {
        Tim15rstW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> Tim16rstW<Apb2rstrSpec> {
        Tim16rstW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&mut self) -> Tim17rstW<Apb2rstrSpec> {
        Tim17rstW::new(self, 18)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset"]
    #[inline(always)]
    pub fn sai1rst(&mut self) -> Sai1rstW<Apb2rstrSpec> {
        Sai1rstW::new(self, 21)
    }
    #[doc = "Bit 22 - Serial audio interface 2 (SAI2) reset"]
    #[inline(always)]
    pub fn sai2rst(&mut self) -> Sai2rstW<Apb2rstrSpec> {
        Sai2rstW::new(self, 22)
    }
    #[doc = "Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset"]
    #[inline(always)]
    pub fn dfsdmrst(&mut self) -> DfsdmrstW<Apb2rstrSpec> {
        DfsdmrstW::new(self, 24)
    }
}
#[doc = "APB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstrSpec;
impl crate::RegisterSpec for Apb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for Apb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for Apb2rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for Apb2rstrSpec {}
