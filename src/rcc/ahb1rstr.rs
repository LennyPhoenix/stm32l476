#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<Ahb1rstrSpec>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<Ahb1rstrSpec>;
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type Dma1rstR = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type Dma1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2RST` reader - DMA2 reset"]
pub type Dma2rstR = crate::BitReader;
#[doc = "Field `DMA2RST` writer - DMA2 reset"]
pub type Dma2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHRST` reader - Flash memory interface reset"]
pub type FlashrstR = crate::BitReader;
#[doc = "Field `FLASHRST` writer - Flash memory interface reset"]
pub type FlashrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCRST` reader - Touch Sensing Controller reset"]
pub type TscrstR = crate::BitReader;
#[doc = "Field `TSCRST` writer - Touch Sensing Controller reset"]
pub type TscrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DRST` reader - DMA2D reset"]
pub type Dma2drstR = crate::BitReader;
#[doc = "Field `DMA2DRST` writer - DMA2D reset"]
pub type Dma2drstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> Dma1rstR {
        Dma1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> Dma2rstR {
        Dma2rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FlashrstR {
        FlashrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller reset"]
    #[inline(always)]
    pub fn tscrst(&self) -> TscrstR {
        TscrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D reset"]
    #[inline(always)]
    pub fn dma2drst(&self) -> Dma2drstR {
        Dma2drstR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> Dma1rstW<Ahb1rstrSpec> {
        Dma1rstW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&mut self) -> Dma2rstW<Ahb1rstrSpec> {
        Dma2rstW::new(self, 1)
    }
    #[doc = "Bit 8 - Flash memory interface reset"]
    #[inline(always)]
    pub fn flashrst(&mut self) -> FlashrstW<Ahb1rstrSpec> {
        FlashrstW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CrcrstW<Ahb1rstrSpec> {
        CrcrstW::new(self, 12)
    }
    #[doc = "Bit 16 - Touch Sensing Controller reset"]
    #[inline(always)]
    pub fn tscrst(&mut self) -> TscrstW<Ahb1rstrSpec> {
        TscrstW::new(self, 16)
    }
    #[doc = "Bit 17 - DMA2D reset"]
    #[inline(always)]
    pub fn dma2drst(&mut self) -> Dma2drstW<Ahb1rstrSpec> {
        Dma2drstW::new(self, 17)
    }
}
#[doc = "AHB1 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1rstrSpec;
impl crate::RegisterSpec for Ahb1rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rstr::R`](R) reader structure"]
impl crate::Readable for Ahb1rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure"]
impl crate::Writable for Ahb1rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for Ahb1rstrSpec {}
