#[doc = "Register `CWSIZE` reader"]
pub type R = crate::R<CwsizeSpec>;
#[doc = "Register `CWSIZE` writer"]
pub type W = crate::W<CwsizeSpec>;
#[doc = "Field `CAPCNT` reader - Capture count"]
pub type CapcntR = crate::FieldReader<u16>;
#[doc = "Field `CAPCNT` writer - Capture count"]
pub type CapcntW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `VLINE` reader - Vertical line count"]
pub type VlineR = crate::FieldReader<u16>;
#[doc = "Field `VLINE` writer - Vertical line count"]
pub type VlineW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    pub fn capcnt(&self) -> CapcntR {
        CapcntR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    pub fn vline(&self) -> VlineR {
        VlineR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    pub fn capcnt(&mut self) -> CapcntW<CwsizeSpec> {
        CapcntW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    pub fn vline(&mut self) -> VlineW<CwsizeSpec> {
        VlineW::new(self, 16)
    }
}
#[doc = "crop window size\n\nYou can [`read`](crate::Reg::read) this register and get [`cwsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwsizeSpec;
impl crate::RegisterSpec for CwsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwsize::R`](R) reader structure"]
impl crate::Readable for CwsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`cwsize::W`](W) writer structure"]
impl crate::Writable for CwsizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CWSIZE to value 0"]
impl crate::Resettable for CwsizeSpec {}
