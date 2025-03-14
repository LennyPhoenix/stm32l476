#[doc = "Register `ESUR` reader"]
pub type R = crate::R<EsurSpec>;
#[doc = "Register `ESUR` writer"]
pub type W = crate::W<EsurSpec>;
#[doc = "Field `FSU` reader - Frame start delimiter unmask"]
pub type FsuR = crate::FieldReader;
#[doc = "Field `FSU` writer - Frame start delimiter unmask"]
pub type FsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LSU` reader - Line start delimiter unmask"]
pub type LsuR = crate::FieldReader;
#[doc = "Field `LSU` writer - Line start delimiter unmask"]
pub type LsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LEU` reader - Line end delimiter unmask"]
pub type LeuR = crate::FieldReader;
#[doc = "Field `LEU` writer - Line end delimiter unmask"]
pub type LeuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FEU` reader - Frame end delimiter unmask"]
pub type FeuR = crate::FieldReader;
#[doc = "Field `FEU` writer - Frame end delimiter unmask"]
pub type FeuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame start delimiter unmask"]
    #[inline(always)]
    pub fn fsu(&self) -> FsuR {
        FsuR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start delimiter unmask"]
    #[inline(always)]
    pub fn lsu(&self) -> LsuR {
        LsuR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end delimiter unmask"]
    #[inline(always)]
    pub fn leu(&self) -> LeuR {
        LeuR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end delimiter unmask"]
    #[inline(always)]
    pub fn feu(&self) -> FeuR {
        FeuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start delimiter unmask"]
    #[inline(always)]
    pub fn fsu(&mut self) -> FsuW<EsurSpec> {
        FsuW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Line start delimiter unmask"]
    #[inline(always)]
    pub fn lsu(&mut self) -> LsuW<EsurSpec> {
        LsuW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Line end delimiter unmask"]
    #[inline(always)]
    pub fn leu(&mut self) -> LeuW<EsurSpec> {
        LeuW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Frame end delimiter unmask"]
    #[inline(always)]
    pub fn feu(&mut self) -> FeuW<EsurSpec> {
        FeuW::new(self, 24)
    }
}
#[doc = "embedded synchronization unmask register\n\nYou can [`read`](crate::Reg::read) this register and get [`esur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsurSpec;
impl crate::RegisterSpec for EsurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esur::R`](R) reader structure"]
impl crate::Readable for EsurSpec {}
#[doc = "`write(|w| ..)` method takes [`esur::W`](W) writer structure"]
impl crate::Writable for EsurSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ESUR to value 0"]
impl crate::Resettable for EsurSpec {}
