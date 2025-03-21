#[doc = "Register `CSSA` reader"]
pub type R = crate::R<CssaSpec>;
#[doc = "Register `CSSA` writer"]
pub type W = crate::W<CssaSpec>;
#[doc = "Field `ADD` reader - code segment start address"]
pub type AddR = crate::FieldReader<u16>;
#[doc = "Field `ADD` writer - code segment start address"]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:23 - code segment start address"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23 - code segment start address"]
    #[inline(always)]
    pub fn add(&mut self) -> AddW<CssaSpec> {
        AddW::new(self, 8)
    }
}
#[doc = "Code segment start address\n\nYou can [`read`](crate::Reg::read) this register and get [`cssa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cssa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CssaSpec;
impl crate::RegisterSpec for CssaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cssa::R`](R) reader structure"]
impl crate::Readable for CssaSpec {}
#[doc = "`write(|w| ..)` method takes [`cssa::W`](W) writer structure"]
impl crate::Writable for CssaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSSA to value 0"]
impl crate::Resettable for CssaSpec {}
