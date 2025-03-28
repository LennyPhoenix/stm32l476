#[doc = "Register `DLR` reader"]
pub type R = crate::R<DlrSpec>;
#[doc = "Register `DLR` writer"]
pub type W = crate::W<DlrSpec>;
#[doc = "Field `DL` reader - Data length"]
pub type DlR = crate::FieldReader<u32>;
#[doc = "Field `DL` writer - Data length"]
pub type DlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data length"]
    #[inline(always)]
    pub fn dl(&self) -> DlR {
        DlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data length"]
    #[inline(always)]
    pub fn dl(&mut self) -> DlW<DlrSpec> {
        DlW::new(self, 0)
    }
}
#[doc = "data length register\n\nYou can [`read`](crate::Reg::read) this register and get [`dlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlrSpec;
impl crate::RegisterSpec for DlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlr::R`](R) reader structure"]
impl crate::Readable for DlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dlr::W`](W) writer structure"]
impl crate::Writable for DlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLR to value 0"]
impl crate::Resettable for DlrSpec {}
