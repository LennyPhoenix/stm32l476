#[doc = "Register `HAINT` reader"]
pub type R = crate::R<HaintSpec>;
#[doc = "Field `HAINT` reader - Channel interrupts"]
pub type HaintR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Channel interrupts"]
    #[inline(always)]
    pub fn haint(&self) -> HaintR {
        HaintR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_FS Host all channels interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`haint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HaintSpec;
impl crate::RegisterSpec for HaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haint::R`](R) reader structure"]
impl crate::Readable for HaintSpec {}
#[doc = "`reset()` method sets HAINT to value 0"]
impl crate::Resettable for HaintSpec {}
