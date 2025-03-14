#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `FRAME_IE` reader - Capture complete interrupt enable"]
pub type FrameIeR = crate::BitReader;
#[doc = "Field `FRAME_IE` writer - Capture complete interrupt enable"]
pub type FrameIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR_IE` reader - Overrun interrupt enable"]
pub type OvrIeR = crate::BitReader;
#[doc = "Field `OVR_IE` writer - Overrun interrupt enable"]
pub type OvrIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_IE` reader - Synchronization error interrupt enable"]
pub type ErrIeR = crate::BitReader;
#[doc = "Field `ERR_IE` writer - Synchronization error interrupt enable"]
pub type ErrIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_IE` reader - VSYNC interrupt enable"]
pub type VsyncIeR = crate::BitReader;
#[doc = "Field `VSYNC_IE` writer - VSYNC interrupt enable"]
pub type VsyncIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_IE` reader - Line interrupt enable"]
pub type LineIeR = crate::BitReader;
#[doc = "Field `LINE_IE` writer - Line interrupt enable"]
pub type LineIeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    pub fn frame_ie(&self) -> FrameIeR {
        FrameIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OvrIeR {
        OvrIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable"]
    #[inline(always)]
    pub fn err_ie(&self) -> ErrIeR {
        ErrIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC interrupt enable"]
    #[inline(always)]
    pub fn vsync_ie(&self) -> VsyncIeR {
        VsyncIeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    pub fn line_ie(&self) -> LineIeR {
        LineIeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    pub fn frame_ie(&mut self) -> FrameIeW<IerSpec> {
        FrameIeW::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovr_ie(&mut self) -> OvrIeW<IerSpec> {
        OvrIeW::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable"]
    #[inline(always)]
    pub fn err_ie(&mut self) -> ErrIeW<IerSpec> {
        ErrIeW::new(self, 2)
    }
    #[doc = "Bit 3 - VSYNC interrupt enable"]
    #[inline(always)]
    pub fn vsync_ie(&mut self) -> VsyncIeW<IerSpec> {
        VsyncIeW::new(self, 3)
    }
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    pub fn line_ie(&mut self) -> LineIeW<IerSpec> {
        LineIeW::new(self, 4)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
