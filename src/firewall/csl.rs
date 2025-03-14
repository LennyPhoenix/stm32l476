#[doc = "Register `CSL` reader"]
pub type R = crate::R<CslSpec>;
#[doc = "Register `CSL` writer"]
pub type W = crate::W<CslSpec>;
#[doc = "Field `LENG` reader - code segment length"]
pub type LengR = crate::FieldReader<u16>;
#[doc = "Field `LENG` writer - code segment length"]
pub type LengW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 8:21 - code segment length"]
    #[inline(always)]
    pub fn leng(&self) -> LengR {
        LengR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:21 - code segment length"]
    #[inline(always)]
    pub fn leng(&mut self) -> LengW<CslSpec> {
        LengW::new(self, 8)
    }
}
#[doc = "Code segment length\n\nYou can [`read`](crate::Reg::read) this register and get [`csl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CslSpec;
impl crate::RegisterSpec for CslSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csl::R`](R) reader structure"]
impl crate::Readable for CslSpec {}
#[doc = "`write(|w| ..)` method takes [`csl::W`](W) writer structure"]
impl crate::Writable for CslSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSL to value 0"]
impl crate::Resettable for CslSpec {}
