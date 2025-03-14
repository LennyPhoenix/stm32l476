#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `FRAME_RIS` reader - Capture complete raw interrupt status"]
pub type FrameRisR = crate::BitReader;
#[doc = "Field `OVR_RIS` reader - Overrun raw interrupt status"]
pub type OvrRisR = crate::BitReader;
#[doc = "Field `ERR_RIS` reader - Synchronization error raw interrupt status"]
pub type ErrRisR = crate::BitReader;
#[doc = "Field `VSYNC_RIS` reader - VSYNC raw interrupt status"]
pub type VsyncRisR = crate::BitReader;
#[doc = "Field `LINE_RIS` reader - Line raw interrupt status"]
pub type LineRisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Capture complete raw interrupt status"]
    #[inline(always)]
    pub fn frame_ris(&self) -> FrameRisR {
        FrameRisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun raw interrupt status"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OvrRisR {
        OvrRisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error raw interrupt status"]
    #[inline(always)]
    pub fn err_ris(&self) -> ErrRisR {
        ErrRisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC raw interrupt status"]
    #[inline(always)]
    pub fn vsync_ris(&self) -> VsyncRisR {
        VsyncRisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line raw interrupt status"]
    #[inline(always)]
    pub fn line_ris(&self) -> LineRisR {
        LineRisR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {}
