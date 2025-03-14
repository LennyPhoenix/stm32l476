#[doc = "Register `FMR` reader"]
pub type R = crate::R<FmrSpec>;
#[doc = "Register `FMR` writer"]
pub type W = crate::W<FmrSpec>;
#[doc = "Field `FINIT` reader - Filter initialization mode"]
pub type FinitR = crate::BitReader;
#[doc = "Field `FINIT` writer - Filter initialization mode"]
pub type FinitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANSB` reader - CAN start bank"]
pub type CansbR = crate::FieldReader;
#[doc = "Field `CANSB` writer - CAN start bank"]
pub type CansbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Filter initialization mode"]
    #[inline(always)]
    pub fn finit(&self) -> FinitR {
        FinitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - CAN start bank"]
    #[inline(always)]
    pub fn cansb(&self) -> CansbR {
        CansbR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Filter initialization mode"]
    #[inline(always)]
    pub fn finit(&mut self) -> FinitW<FmrSpec> {
        FinitW::new(self, 0)
    }
    #[doc = "Bits 8:13 - CAN start bank"]
    #[inline(always)]
    pub fn cansb(&mut self) -> CansbW<FmrSpec> {
        CansbW::new(self, 8)
    }
}
#[doc = "filter master register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmrSpec;
impl crate::RegisterSpec for FmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmr::R`](R) reader structure"]
impl crate::Readable for FmrSpec {}
#[doc = "`write(|w| ..)` method takes [`fmr::W`](W) writer structure"]
impl crate::Writable for FmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMR to value 0x2a1c_0e01"]
impl crate::Resettable for FmrSpec {
    const RESET_VALUE: u32 = 0x2a1c_0e01;
}
