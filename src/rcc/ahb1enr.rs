#[doc = "Register `AHB1ENR` reader"]
pub type R = crate::R<Ahb1enrSpec>;
#[doc = "Register `AHB1ENR` writer"]
pub type W = crate::W<Ahb1enrSpec>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type Dma1enR = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type Dma2enR = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type Dma2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHEN` reader - Flash memory interface clock enable"]
pub type FlashenR = crate::BitReader;
#[doc = "Field `FLASHEN` writer - Flash memory interface clock enable"]
pub type FlashenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCEN` reader - Touch Sensing Controller clock enable"]
pub type TscenR = crate::BitReader;
#[doc = "Field `TSCEN` writer - Touch Sensing Controller clock enable"]
pub type TscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DEN` reader - DMA2D clock enable"]
pub type Dma2denR = crate::BitReader;
#[doc = "Field `DMA2DEN` writer - DMA2D clock enable"]
pub type Dma2denW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FlashenR {
        FlashenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clock enable"]
    #[inline(always)]
    pub fn tscen(&self) -> TscenR {
        TscenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D clock enable"]
    #[inline(always)]
    pub fn dma2den(&self) -> Dma2denR {
        Dma2denR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> Dma1enW<Ahb1enrSpec> {
        Dma1enW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> Dma2enW<Ahb1enrSpec> {
        Dma2enW::new(self, 1)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    pub fn flashen(&mut self) -> FlashenW<Ahb1enrSpec> {
        FlashenW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<Ahb1enrSpec> {
        CrcenW::new(self, 12)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clock enable"]
    #[inline(always)]
    pub fn tscen(&mut self) -> TscenW<Ahb1enrSpec> {
        TscenW::new(self, 16)
    }
    #[doc = "Bit 17 - DMA2D clock enable"]
    #[inline(always)]
    pub fn dma2den(&mut self) -> Dma2denW<Ahb1enrSpec> {
        Dma2denW::new(self, 17)
    }
}
#[doc = "AHB1 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1enrSpec;
impl crate::RegisterSpec for Ahb1enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1enr::R`](R) reader structure"]
impl crate::Readable for Ahb1enrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure"]
impl crate::Writable for Ahb1enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1ENR to value 0x0100"]
impl crate::Resettable for Ahb1enrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
