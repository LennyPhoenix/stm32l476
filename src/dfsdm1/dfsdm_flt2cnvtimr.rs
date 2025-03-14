#[doc = "Register `DFSDM_FLT2CNVTIMR` reader"]
pub type R = crate::R<DfsdmFlt2cnvtimrSpec>;
#[doc = "Field `CNVCNT` reader - 28-bit timer counting conversion time t = CNVCNT\\[27:0\\] / fDFSDM_CKIN"]
pub type CnvcntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT\\[27:0\\] / fDFSDM_CKIN"]
    #[inline(always)]
    pub fn cnvcnt(&self) -> CnvcntR {
        CnvcntR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "conversion timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cnvtimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt2cnvtimrSpec;
impl crate::RegisterSpec for DfsdmFlt2cnvtimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt2cnvtimr::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt2cnvtimrSpec {}
#[doc = "`reset()` method sets DFSDM_FLT2CNVTIMR to value 0"]
impl crate::Resettable for DfsdmFlt2cnvtimrSpec {}
