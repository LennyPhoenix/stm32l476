#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `FRAME_ISC` writer - Capture complete interrupt status clear"]
pub type FrameIscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR_ISC` writer - Overrun interrupt status clear"]
pub type OvrIscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_ISC` writer - Synchronization error interrupt status clear"]
pub type ErrIscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_ISC` writer - Vertical synch interrupt status clear"]
pub type VsyncIscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_ISC` writer - line interrupt status clear"]
pub type LineIscW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Capture complete interrupt status clear"]
    #[inline(always)]
    pub fn frame_isc(&mut self) -> FrameIscW<IcrSpec> {
        FrameIscW::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun interrupt status clear"]
    #[inline(always)]
    pub fn ovr_isc(&mut self) -> OvrIscW<IcrSpec> {
        OvrIscW::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization error interrupt status clear"]
    #[inline(always)]
    pub fn err_isc(&mut self) -> ErrIscW<IcrSpec> {
        ErrIscW::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical synch interrupt status clear"]
    #[inline(always)]
    pub fn vsync_isc(&mut self) -> VsyncIscW<IcrSpec> {
        VsyncIscW::new(self, 3)
    }
    #[doc = "Bit 4 - line interrupt status clear"]
    #[inline(always)]
    pub fn line_isc(&mut self) -> LineIscW<IcrSpec> {
        LineIscW::new(self, 4)
    }
}
#[doc = "interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
