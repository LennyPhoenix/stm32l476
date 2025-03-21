#[doc = "Register `OPAMP2_LPOTR` reader"]
pub type R = crate::R<Opamp2LpotrSpec>;
#[doc = "Register `OPAMP2_LPOTR` writer"]
pub type W = crate::W<Opamp2LpotrSpec>;
#[doc = "Field `TRIMLPOFFSETN` reader - Trim for NMOS differential pairs"]
pub type TrimlpoffsetnR = crate::FieldReader;
#[doc = "Field `TRIMLPOFFSETN` writer - Trim for NMOS differential pairs"]
pub type TrimlpoffsetnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMLPOFFSETP` reader - Trim for PMOS differential pairs"]
pub type TrimlpoffsetpR = crate::FieldReader;
#[doc = "Field `TRIMLPOFFSETP` writer - Trim for PMOS differential pairs"]
pub type TrimlpoffsetpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetn(&self) -> TrimlpoffsetnR {
        TrimlpoffsetnR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetp(&self) -> TrimlpoffsetpR {
        TrimlpoffsetpR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetn(&mut self) -> TrimlpoffsetnW<Opamp2LpotrSpec> {
        TrimlpoffsetnW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetp(&mut self) -> TrimlpoffsetpW<Opamp2LpotrSpec> {
        TrimlpoffsetpW::new(self, 8)
    }
}
#[doc = "OPAMP2 offset trimming register in low-power mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_lpotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_lpotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opamp2LpotrSpec;
impl crate::RegisterSpec for Opamp2LpotrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp2_lpotr::R`](R) reader structure"]
impl crate::Readable for Opamp2LpotrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp2_lpotr::W`](W) writer structure"]
impl crate::Writable for Opamp2LpotrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPAMP2_LPOTR to value 0"]
impl crate::Resettable for Opamp2LpotrSpec {}
