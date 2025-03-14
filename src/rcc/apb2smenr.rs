#[doc = "Register `APB2SMENR` reader"]
pub type R = crate::R<Apb2smenrSpec>;
#[doc = "Register `APB2SMENR` writer"]
pub type W = crate::W<Apb2smenrSpec>;
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG clocks enable during Sleep and Stop modes"]
pub type SyscfgsmenR = crate::BitReader;
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG clocks enable during Sleep and Stop modes"]
pub type SyscfgsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMCSMEN` reader - SDMMC clocks enable during Sleep and Stop modes"]
pub type SdmmcsmenR = crate::BitReader;
#[doc = "Field `SDMMCSMEN` writer - SDMMC clocks enable during Sleep and Stop modes"]
pub type SdmmcsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clocks enable during Sleep and Stop modes"]
pub type Tim1smenR = crate::BitReader;
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clocks enable during Sleep and Stop modes"]
pub type Tim1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1SMEN` reader - SPI1 clocks enable during Sleep and Stop modes"]
pub type Spi1smenR = crate::BitReader;
#[doc = "Field `SPI1SMEN` writer - SPI1 clocks enable during Sleep and Stop modes"]
pub type Spi1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8SMEN` reader - TIM8 timer clocks enable during Sleep and Stop modes"]
pub type Tim8smenR = crate::BitReader;
#[doc = "Field `TIM8SMEN` writer - TIM8 timer clocks enable during Sleep and Stop modes"]
pub type Tim8smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SMEN` reader - USART1clocks enable during Sleep and Stop modes"]
pub type Usart1smenR = crate::BitReader;
#[doc = "Field `USART1SMEN` writer - USART1clocks enable during Sleep and Stop modes"]
pub type Usart1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clocks enable during Sleep and Stop modes"]
pub type Tim15smenR = crate::BitReader;
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clocks enable during Sleep and Stop modes"]
pub type Tim15smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clocks enable during Sleep and Stop modes"]
pub type Tim16smenR = crate::BitReader;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clocks enable during Sleep and Stop modes"]
pub type Tim16smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17SMEN` reader - TIM17 timer clocks enable during Sleep and Stop modes"]
pub type Tim17smenR = crate::BitReader;
#[doc = "Field `TIM17SMEN` writer - TIM17 timer clocks enable during Sleep and Stop modes"]
pub type Tim17smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1SMEN` reader - SAI1 clocks enable during Sleep and Stop modes"]
pub type Sai1smenR = crate::BitReader;
#[doc = "Field `SAI1SMEN` writer - SAI1 clocks enable during Sleep and Stop modes"]
pub type Sai1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2SMEN` reader - SAI2 clocks enable during Sleep and Stop modes"]
pub type Sai2smenR = crate::BitReader;
#[doc = "Field `SAI2SMEN` writer - SAI2 clocks enable during Sleep and Stop modes"]
pub type Sai2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDMSMEN` reader - DFSDM timer clocks enable during Sleep and Stop modes"]
pub type DfsdmsmenR = crate::BitReader;
#[doc = "Field `DFSDMSMEN` writer - DFSDM timer clocks enable during Sleep and Stop modes"]
pub type DfsdmsmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SyscfgsmenR {
        SyscfgsmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - SDMMC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sdmmcsmen(&self) -> SdmmcsmenR {
        SdmmcsmenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim1smen(&self) -> Tim1smenR {
        Tim1smenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi1smen(&self) -> Spi1smenR {
        Spi1smenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim8smen(&self) -> Tim8smenR {
        Tim8smenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart1smen(&self) -> Usart1smenR {
        Usart1smenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim15smen(&self) -> Tim15smenR {
        Tim15smenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim16smen(&self) -> Tim16smenR {
        Tim16smenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim17smen(&self) -> Tim17smenR {
        Tim17smenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai1smen(&self) -> Sai1smenR {
        Sai1smenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai2smen(&self) -> Sai2smenR {
        Sai2smenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dfsdmsmen(&self) -> DfsdmsmenR {
        DfsdmsmenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SyscfgsmenW<Apb2smenrSpec> {
        SyscfgsmenW::new(self, 0)
    }
    #[doc = "Bit 10 - SDMMC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sdmmcsmen(&mut self) -> SdmmcsmenW<Apb2smenrSpec> {
        SdmmcsmenW::new(self, 10)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim1smen(&mut self) -> Tim1smenW<Apb2smenrSpec> {
        Tim1smenW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi1smen(&mut self) -> Spi1smenW<Apb2smenrSpec> {
        Spi1smenW::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim8smen(&mut self) -> Tim8smenW<Apb2smenrSpec> {
        Tim8smenW::new(self, 13)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart1smen(&mut self) -> Usart1smenW<Apb2smenrSpec> {
        Usart1smenW::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim15smen(&mut self) -> Tim15smenW<Apb2smenrSpec> {
        Tim15smenW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim16smen(&mut self) -> Tim16smenW<Apb2smenrSpec> {
        Tim16smenW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim17smen(&mut self) -> Tim17smenW<Apb2smenrSpec> {
        Tim17smenW::new(self, 18)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai1smen(&mut self) -> Sai1smenW<Apb2smenrSpec> {
        Sai1smenW::new(self, 21)
    }
    #[doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai2smen(&mut self) -> Sai2smenW<Apb2smenrSpec> {
        Sai2smenW::new(self, 22)
    }
    #[doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dfsdmsmen(&mut self) -> DfsdmsmenW<Apb2smenrSpec> {
        DfsdmsmenW::new(self, 24)
    }
}
#[doc = "APB2SMENR\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2smenrSpec;
impl crate::RegisterSpec for Apb2smenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2smenr::R`](R) reader structure"]
impl crate::Readable for Apb2smenrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2smenr::W`](W) writer structure"]
impl crate::Writable for Apb2smenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2SMENR to value 0x0167_7c01"]
impl crate::Resettable for Apb2smenrSpec {
    const RESET_VALUE: u32 = 0x0167_7c01;
}
