#[doc = "Register `DOUTR` reader"]
pub type R = crate::R<DoutrSpec>;
#[doc = "Field `AES_DOUTR` reader - Data output register"]
pub type AesDoutrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data output register"]
    #[inline(always)]
    pub fn aes_doutr(&self) -> AesDoutrR {
        AesDoutrR::new(self.bits)
    }
}
#[doc = "data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`doutr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutrSpec;
impl crate::RegisterSpec for DoutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr::R`](R) reader structure"]
impl crate::Readable for DoutrSpec {}
#[doc = "`reset()` method sets DOUTR to value 0"]
impl crate::Resettable for DoutrSpec {}
