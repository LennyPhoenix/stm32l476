#[doc = "Register `CWSTRT` reader"]
pub type R = crate::R<CwstrtSpec>;
#[doc = "Register `CWSTRT` writer"]
pub type W = crate::W<CwstrtSpec>;
#[doc = "Field `HOFFCNT` reader - Horizontal offset count"]
pub type HoffcntR = crate::FieldReader<u16>;
#[doc = "Field `HOFFCNT` writer - Horizontal offset count"]
pub type HoffcntW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `VST` reader - Vertical start line count"]
pub type VstR = crate::FieldReader<u16>;
#[doc = "Field `VST` writer - Vertical start line count"]
pub type VstW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:13 - Horizontal offset count"]
    #[inline(always)]
    pub fn hoffcnt(&self) -> HoffcntR {
        HoffcntR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - Vertical start line count"]
    #[inline(always)]
    pub fn vst(&self) -> VstR {
        VstR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Horizontal offset count"]
    #[inline(always)]
    pub fn hoffcnt(&mut self) -> HoffcntW<CwstrtSpec> {
        HoffcntW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Vertical start line count"]
    #[inline(always)]
    pub fn vst(&mut self) -> VstW<CwstrtSpec> {
        VstW::new(self, 16)
    }
}
#[doc = "crop window start\n\nYou can [`read`](crate::Reg::read) this register and get [`cwstrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwstrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwstrtSpec;
impl crate::RegisterSpec for CwstrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwstrt::R`](R) reader structure"]
impl crate::Readable for CwstrtSpec {}
#[doc = "`write(|w| ..)` method takes [`cwstrt::W`](W) writer structure"]
impl crate::Writable for CwstrtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CWSTRT to value 0"]
impl crate::Resettable for CwstrtSpec {}
