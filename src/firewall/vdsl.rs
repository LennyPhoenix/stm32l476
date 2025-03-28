#[doc = "Register `VDSL` reader"]
pub type R = crate::R<VdslSpec>;
#[doc = "Register `VDSL` writer"]
pub type W = crate::W<VdslSpec>;
#[doc = "Field `LENG` reader - Non-volatile data segment length"]
pub type LengR = crate::FieldReader<u16>;
#[doc = "Field `LENG` writer - Non-volatile data segment length"]
pub type LengW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 6:15 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&self) -> LengR {
        LengR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&mut self) -> LengW<VdslSpec> {
        LengW::new(self, 6)
    }
}
#[doc = "Volatile data segment length\n\nYou can [`read`](crate::Reg::read) this register and get [`vdsl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdsl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdslSpec;
impl crate::RegisterSpec for VdslSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdsl::R`](R) reader structure"]
impl crate::Readable for VdslSpec {}
#[doc = "`write(|w| ..)` method takes [`vdsl::W`](W) writer structure"]
impl crate::Writable for VdslSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDSL to value 0"]
impl crate::Resettable for VdslSpec {}
