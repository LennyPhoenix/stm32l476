#[doc = "Register `DFSDM_FLT1AWSR` reader"]
pub type R = crate::R<DfsdmFlt1awsrSpec>;
#[doc = "Field `AWLTF` reader - Analog watchdog low threshold flag"]
pub type AwltfR = crate::FieldReader;
#[doc = "Field `AWHTF` reader - Analog watchdog high threshold flag"]
pub type AwhtfR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn awltf(&self) -> AwltfR {
        AwltfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn awhtf(&self) -> AwhtfR {
        AwhtfR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "analog watchdog status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt1awsrSpec;
impl crate::RegisterSpec for DfsdmFlt1awsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt1awsr::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt1awsrSpec {}
#[doc = "`reset()` method sets DFSDM_FLT1AWSR to value 0"]
impl crate::Resettable for DfsdmFlt1awsrSpec {}
