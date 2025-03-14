#[doc = "Register `NVDSL` reader"]
pub type R = crate::R<NvdslSpec>;
#[doc = "Register `NVDSL` writer"]
pub type W = crate::W<NvdslSpec>;
#[doc = "Field `LENG` reader - Non-volatile data segment length"]
pub type LengR = crate::FieldReader<u16>;
#[doc = "Field `LENG` writer - Non-volatile data segment length"]
pub type LengW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 8:21 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&self) -> LengR {
        LengR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:21 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&mut self) -> LengW<NvdslSpec> {
        LengW::new(self, 8)
    }
}
#[doc = "Non-volatile data segment length\n\nYou can [`read`](crate::Reg::read) this register and get [`nvdsl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvdsl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvdslSpec;
impl crate::RegisterSpec for NvdslSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvdsl::R`](R) reader structure"]
impl crate::Readable for NvdslSpec {}
#[doc = "`write(|w| ..)` method takes [`nvdsl::W`](W) writer structure"]
impl crate::Writable for NvdslSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NVDSL to value 0"]
impl crate::Resettable for NvdslSpec {}
