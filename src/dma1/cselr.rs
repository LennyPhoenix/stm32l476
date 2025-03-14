#[doc = "Register `CSELR` reader"]
pub type R = crate::R<CselrSpec>;
#[doc = "Register `CSELR` writer"]
pub type W = crate::W<CselrSpec>;
#[doc = "Field `C1S` reader - DMA channel 1 selection"]
pub type C1sR = crate::FieldReader;
#[doc = "Field `C1S` writer - DMA channel 1 selection"]
pub type C1sW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C2S` reader - DMA channel 2 selection"]
pub type C2sR = crate::FieldReader;
#[doc = "Field `C2S` writer - DMA channel 2 selection"]
pub type C2sW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C3S` reader - DMA channel 3 selection"]
pub type C3sR = crate::FieldReader;
#[doc = "Field `C3S` writer - DMA channel 3 selection"]
pub type C3sW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C4S` reader - DMA channel 4 selection"]
pub type C4sR = crate::FieldReader;
#[doc = "Field `C4S` writer - DMA channel 4 selection"]
pub type C4sW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C5S` reader - DMA channel 5 selection"]
pub type C5sR = crate::FieldReader;
#[doc = "Field `C5S` writer - DMA channel 5 selection"]
pub type C5sW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C6S` reader - DMA channel 6 selection"]
pub type C6sR = crate::FieldReader;
#[doc = "Field `C6S` writer - DMA channel 6 selection"]
pub type C6sW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C7S` reader - DMA channel 7 selection"]
pub type C7sR = crate::FieldReader;
#[doc = "Field `C7S` writer - DMA channel 7 selection"]
pub type C7sW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline(always)]
    pub fn c1s(&self) -> C1sR {
        C1sR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline(always)]
    pub fn c2s(&self) -> C2sR {
        C2sR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline(always)]
    pub fn c3s(&self) -> C3sR {
        C3sR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline(always)]
    pub fn c4s(&self) -> C4sR {
        C4sR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline(always)]
    pub fn c5s(&self) -> C5sR {
        C5sR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline(always)]
    pub fn c6s(&self) -> C6sR {
        C6sR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline(always)]
    pub fn c7s(&self) -> C7sR {
        C7sR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline(always)]
    pub fn c1s(&mut self) -> C1sW<CselrSpec> {
        C1sW::new(self, 0)
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline(always)]
    pub fn c2s(&mut self) -> C2sW<CselrSpec> {
        C2sW::new(self, 4)
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline(always)]
    pub fn c3s(&mut self) -> C3sW<CselrSpec> {
        C3sW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline(always)]
    pub fn c4s(&mut self) -> C4sW<CselrSpec> {
        C4sW::new(self, 12)
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline(always)]
    pub fn c5s(&mut self) -> C5sW<CselrSpec> {
        C5sW::new(self, 16)
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline(always)]
    pub fn c6s(&mut self) -> C6sW<CselrSpec> {
        C6sW::new(self, 20)
    }
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline(always)]
    pub fn c7s(&mut self) -> C7sW<CselrSpec> {
        C7sW::new(self, 24)
    }
}
#[doc = "channel selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`cselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CselrSpec;
impl crate::RegisterSpec for CselrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cselr::R`](R) reader structure"]
impl crate::Readable for CselrSpec {}
#[doc = "`write(|w| ..)` method takes [`cselr::W`](W) writer structure"]
impl crate::Writable for CselrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSELR to value 0"]
impl crate::Resettable for CselrSpec {}
