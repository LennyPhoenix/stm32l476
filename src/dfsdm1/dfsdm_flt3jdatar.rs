#[doc = "Register `DFSDM_FLT3JDATAR` reader"]
pub type R = crate::R<DfsdmFlt3jdatarSpec>;
#[doc = "Field `JDATACH` reader - Injected channel most recently converted"]
pub type JdatachR = crate::FieldReader;
#[doc = "Field `JDATA` reader - Injected group conversion data"]
pub type JdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Injected channel most recently converted"]
    #[inline(always)]
    pub fn jdatach(&self) -> JdatachR {
        JdatachR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - Injected group conversion data"]
    #[inline(always)]
    pub fn jdata(&self) -> JdataR {
        JdataR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "data register for injected group\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3jdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt3jdatarSpec;
impl crate::RegisterSpec for DfsdmFlt3jdatarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt3jdatar::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt3jdatarSpec {}
#[doc = "`reset()` method sets DFSDM_FLT3JDATAR to value 0"]
impl crate::Resettable for DfsdmFlt3jdatarSpec {}
