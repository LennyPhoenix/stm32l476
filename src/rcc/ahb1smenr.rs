#[doc = "Register `AHB1SMENR` reader"]
pub type R = crate::R<Ahb1smenrSpec>;
#[doc = "Register `AHB1SMENR` writer"]
pub type W = crate::W<Ahb1smenrSpec>;
#[doc = "Field `DMA1SMEN` reader - DMA1 clocks enable during Sleep and Stop modes"]
pub type Dma1smenR = crate::BitReader;
#[doc = "Field `DMA1SMEN` writer - DMA1 clocks enable during Sleep and Stop modes"]
pub type Dma1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2SMEN` reader - DMA2 clocks enable during Sleep and Stop modes"]
pub type Dma2smenR = crate::BitReader;
#[doc = "Field `DMA2SMEN` writer - DMA2 clocks enable during Sleep and Stop modes"]
pub type Dma2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHSMEN` reader - Flash memory interface clocks enable during Sleep and Stop modes"]
pub type FlashsmenR = crate::BitReader;
#[doc = "Field `FLASHSMEN` writer - Flash memory interface clocks enable during Sleep and Stop modes"]
pub type FlashsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1SMEN` reader - SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type Sram1smenR = crate::BitReader;
#[doc = "Field `SRAM1SMEN` writer - SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type Sram1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSMEN` reader - CRCSMEN"]
pub type CrcsmenR = crate::BitReader;
#[doc = "Field `CRCSMEN` writer - CRCSMEN"]
pub type CrcsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCSMEN` reader - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TscsmenR = crate::BitReader;
#[doc = "Field `TSCSMEN` writer - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TscsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DSMEN` reader - DMA2D clock enable during Sleep and Stop modes"]
pub type Dma2dsmenR = crate::BitReader;
#[doc = "Field `DMA2DSMEN` writer - DMA2D clock enable during Sleep and Stop modes"]
pub type Dma2dsmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma1smen(&self) -> Dma1smenR {
        Dma1smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2smen(&self) -> Dma2smenR {
        Dma2smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FlashsmenR {
        FlashsmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram1smen(&self) -> Sram1smenR {
        Sram1smenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRCSMEN"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CrcsmenR {
        CrcsmenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tscsmen(&self) -> TscsmenR {
        TscsmenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2dsmen(&self) -> Dma2dsmenR {
        Dma2dsmenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma1smen(&mut self) -> Dma1smenW<Ahb1smenrSpec> {
        Dma1smenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2smen(&mut self) -> Dma2smenW<Ahb1smenrSpec> {
        Dma2smenW::new(self, 1)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FlashsmenW<Ahb1smenrSpec> {
        FlashsmenW::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram1smen(&mut self) -> Sram1smenW<Ahb1smenrSpec> {
        Sram1smenW::new(self, 9)
    }
    #[doc = "Bit 12 - CRCSMEN"]
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CrcsmenW<Ahb1smenrSpec> {
        CrcsmenW::new(self, 12)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tscsmen(&mut self) -> TscsmenW<Ahb1smenrSpec> {
        TscsmenW::new(self, 16)
    }
    #[doc = "Bit 17 - DMA2D clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2dsmen(&mut self) -> Dma2dsmenW<Ahb1smenrSpec> {
        Dma2dsmenW::new(self, 17)
    }
}
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1smenrSpec;
impl crate::RegisterSpec for Ahb1smenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1smenr::R`](R) reader structure"]
impl crate::Readable for Ahb1smenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1smenr::W`](W) writer structure"]
impl crate::Writable for Ahb1smenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1SMENR to value 0x0001_1303"]
impl crate::Resettable for Ahb1smenrSpec {
    const RESET_VALUE: u32 = 0x0001_1303;
}
