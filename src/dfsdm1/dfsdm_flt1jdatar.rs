#[doc = "Register `DFSDM_FLT1JDATAR` reader"]
pub type R = crate::R<DfsdmFlt1jdatarSpec>;
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
#[doc = "data register for injected group\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1jdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt1jdatarSpec;
impl crate::RegisterSpec for DfsdmFlt1jdatarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt1jdatar::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt1jdatarSpec {}
#[doc = "`reset()` method sets DFSDM_FLT1JDATAR to value 0"]
impl crate::Resettable for DfsdmFlt1jdatarSpec {}
