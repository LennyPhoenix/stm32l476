#[doc = "Register `MEMRMP` reader"]
pub type R = crate::R<MemrmpSpec>;
#[doc = "Register `MEMRMP` writer"]
pub type W = crate::W<MemrmpSpec>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection"]
pub type MemModeR = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection"]
pub type MemModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `QFS` reader - QUADSPI memory mapping swap"]
pub type QfsR = crate::BitReader;
#[doc = "Field `QFS` writer - QUADSPI memory mapping swap"]
pub type QfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB_MODE` reader - Flash Bank mode selection"]
pub type FbModeR = crate::BitReader;
#[doc = "Field `FB_MODE` writer - Flash Bank mode selection"]
pub type FbModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MemModeR {
        MemModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - QUADSPI memory mapping swap"]
    #[inline(always)]
    pub fn qfs(&self) -> QfsR {
        QfsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&self) -> FbModeR {
        FbModeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MemModeW<MemrmpSpec> {
        MemModeW::new(self, 0)
    }
    #[doc = "Bit 3 - QUADSPI memory mapping swap"]
    #[inline(always)]
    pub fn qfs(&mut self) -> QfsW<MemrmpSpec> {
        QfsW::new(self, 3)
    }
    #[doc = "Bit 8 - Flash Bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&mut self) -> FbModeW<MemrmpSpec> {
        FbModeW::new(self, 8)
    }
}
#[doc = "memory remap register\n\nYou can [`read`](crate::Reg::read) this register and get [`memrmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemrmpSpec;
impl crate::RegisterSpec for MemrmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memrmp::R`](R) reader structure"]
impl crate::Readable for MemrmpSpec {}
#[doc = "`write(|w| ..)` method takes [`memrmp::W`](W) writer structure"]
impl crate::Writable for MemrmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEMRMP to value 0"]
impl crate::Resettable for MemrmpSpec {}
