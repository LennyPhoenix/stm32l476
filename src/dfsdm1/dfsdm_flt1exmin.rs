#[doc = "Register `DFSDM_FLT1EXMIN` reader"]
pub type R = crate::R<DfsdmFlt1exminSpec>;
#[doc = "Field `EXMINCH` reader - Extremes detector minimum data channel"]
pub type ExminchR = crate::FieldReader;
#[doc = "Field `EXMIN` reader - EXMIN"]
pub type ExminR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Extremes detector minimum data channel"]
    #[inline(always)]
    pub fn exminch(&self) -> ExminchR {
        ExminchR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> ExminR {
        ExminR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Extremes detector minimum register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1exmin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt1exminSpec;
impl crate::RegisterSpec for DfsdmFlt1exminSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt1exmin::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt1exminSpec {}
#[doc = "`reset()` method sets DFSDM_FLT1EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for DfsdmFlt1exminSpec {
    const RESET_VALUE: u32 = 0x7fff_ff00;
}
