#[doc = "Register `VDSSA` reader"]
pub type R = crate::R<VdssaSpec>;
#[doc = "Register `VDSSA` writer"]
pub type W = crate::W<VdssaSpec>;
#[doc = "Field `ADD` reader - Volatile data segment start address"]
pub type AddR = crate::FieldReader<u16>;
#[doc = "Field `ADD` writer - Volatile data segment start address"]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 6:15 - Volatile data segment start address"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - Volatile data segment start address"]
    #[inline(always)]
    pub fn add(&mut self) -> AddW<VdssaSpec> {
        AddW::new(self, 6)
    }
}
#[doc = "Volatile data segment start address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdssa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdssa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdssaSpec;
impl crate::RegisterSpec for VdssaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdssa::R`](R) reader structure"]
impl crate::Readable for VdssaSpec {}
#[doc = "`write(|w| ..)` method takes [`vdssa::W`](W) writer structure"]
impl crate::Writable for VdssaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDSSA to value 0"]
impl crate::Resettable for VdssaSpec {}
