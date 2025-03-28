#[doc = "Register `ECCR` reader"]
pub type R = crate::R<EccrSpec>;
#[doc = "Field `ECCx` reader - ECCx"]
pub type EccxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECCx"]
    #[inline(always)]
    pub fn eccx(&self) -> EccxR {
        EccxR::new(self.bits)
    }
}
#[doc = "ECC result register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccrSpec;
impl crate::RegisterSpec for EccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr::R`](R) reader structure"]
impl crate::Readable for EccrSpec {}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for EccrSpec {}
