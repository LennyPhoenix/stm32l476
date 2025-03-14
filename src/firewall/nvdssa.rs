#[doc = "Register `NVDSSA` reader"]
pub type R = crate::R<NvdssaSpec>;
#[doc = "Register `NVDSSA` writer"]
pub type W = crate::W<NvdssaSpec>;
#[doc = "Field `ADD` reader - Non-volatile data segment start address"]
pub type AddR = crate::FieldReader<u16>;
#[doc = "Field `ADD` writer - Non-volatile data segment start address"]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:23 - Non-volatile data segment start address"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23 - Non-volatile data segment start address"]
    #[inline(always)]
    pub fn add(&mut self) -> AddW<NvdssaSpec> {
        AddW::new(self, 8)
    }
}
#[doc = "Non-volatile data segment start address\n\nYou can [`read`](crate::Reg::read) this register and get [`nvdssa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvdssa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvdssaSpec;
impl crate::RegisterSpec for NvdssaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvdssa::R`](R) reader structure"]
impl crate::Readable for NvdssaSpec {}
#[doc = "`write(|w| ..)` method takes [`nvdssa::W`](W) writer structure"]
impl crate::Writable for NvdssaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NVDSSA to value 0"]
impl crate::Resettable for NvdssaSpec {}
