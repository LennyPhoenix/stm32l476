#[doc = "Register `COMP1_CSR` reader"]
pub type R = crate::R<Comp1CsrSpec>;
#[doc = "Register `COMP1_CSR` writer"]
pub type W = crate::W<Comp1CsrSpec>;
#[doc = "Field `COMP1_EN` reader - Comparator 1 enable bit"]
pub type Comp1EnR = crate::BitReader;
#[doc = "Field `COMP1_EN` writer - Comparator 1 enable bit"]
pub type Comp1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_PWRMODE` reader - Power Mode of the comparator 1"]
pub type Comp1PwrmodeR = crate::FieldReader;
#[doc = "Field `COMP1_PWRMODE` writer - Power Mode of the comparator 1"]
pub type Comp1PwrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1_INMSEL` reader - Comparator 1 Input Minus connection configuration bit"]
pub type Comp1InmselR = crate::FieldReader;
#[doc = "Field `COMP1_INMSEL` writer - Comparator 1 Input Minus connection configuration bit"]
pub type Comp1InmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP1_INPSEL` reader - Comparator1 input plus selection bit"]
pub type Comp1InpselR = crate::BitReader;
#[doc = "Field `COMP1_INPSEL` writer - Comparator1 input plus selection bit"]
pub type Comp1InpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_POLARITY` reader - Comparator 1 polarity selection bit"]
pub type Comp1PolarityR = crate::BitReader;
#[doc = "Field `COMP1_POLARITY` writer - Comparator 1 polarity selection bit"]
pub type Comp1PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_HYST` reader - Comparator 1 hysteresis selection bits"]
pub type Comp1HystR = crate::FieldReader;
#[doc = "Field `COMP1_HYST` writer - Comparator 1 hysteresis selection bits"]
pub type Comp1HystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1_BLANKING` reader - Comparator 1 blanking source selection bits"]
pub type Comp1BlankingR = crate::FieldReader;
#[doc = "Field `COMP1_BLANKING` writer - Comparator 1 blanking source selection bits"]
pub type Comp1BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP1_BRGEN` reader - Scaler bridge enable"]
pub type Comp1BrgenR = crate::BitReader;
#[doc = "Field `COMP1_BRGEN` writer - Scaler bridge enable"]
pub type Comp1BrgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_SCALEN` reader - Voltage scaler enable bit"]
pub type Comp1ScalenR = crate::BitReader;
#[doc = "Field `COMP1_SCALEN` writer - Voltage scaler enable bit"]
pub type Comp1ScalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_VALUE` reader - Comparator 1 output status bit"]
pub type Comp1ValueR = crate::BitReader;
#[doc = "Field `COMP1_LOCK` writer - COMP1_CSR register lock bit"]
pub type Comp1LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1_en(&self) -> Comp1EnR {
        Comp1EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 1"]
    #[inline(always)]
    pub fn comp1_pwrmode(&self) -> Comp1PwrmodeR {
        Comp1PwrmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1_inmsel(&self) -> Comp1InmselR {
        Comp1InmselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator1 input plus selection bit"]
    #[inline(always)]
    pub fn comp1_inpsel(&self) -> Comp1InpselR {
        Comp1InpselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1_polarity(&self) -> Comp1PolarityR {
        Comp1PolarityR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis selection bits"]
    #[inline(always)]
    pub fn comp1_hyst(&self) -> Comp1HystR {
        Comp1HystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 1 blanking source selection bits"]
    #[inline(always)]
    pub fn comp1_blanking(&self) -> Comp1BlankingR {
        Comp1BlankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn comp1_brgen(&self) -> Comp1BrgenR {
        Comp1BrgenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn comp1_scalen(&self) -> Comp1ScalenR {
        Comp1ScalenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1_value(&self) -> Comp1ValueR {
        Comp1ValueR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1_en(&mut self) -> Comp1EnW<Comp1CsrSpec> {
        Comp1EnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 1"]
    #[inline(always)]
    pub fn comp1_pwrmode(&mut self) -> Comp1PwrmodeW<Comp1CsrSpec> {
        Comp1PwrmodeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1_inmsel(&mut self) -> Comp1InmselW<Comp1CsrSpec> {
        Comp1InmselW::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator1 input plus selection bit"]
    #[inline(always)]
    pub fn comp1_inpsel(&mut self) -> Comp1InpselW<Comp1CsrSpec> {
        Comp1InpselW::new(self, 7)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1_polarity(&mut self) -> Comp1PolarityW<Comp1CsrSpec> {
        Comp1PolarityW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis selection bits"]
    #[inline(always)]
    pub fn comp1_hyst(&mut self) -> Comp1HystW<Comp1CsrSpec> {
        Comp1HystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 1 blanking source selection bits"]
    #[inline(always)]
    pub fn comp1_blanking(&mut self) -> Comp1BlankingW<Comp1CsrSpec> {
        Comp1BlankingW::new(self, 18)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn comp1_brgen(&mut self) -> Comp1BrgenW<Comp1CsrSpec> {
        Comp1BrgenW::new(self, 22)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn comp1_scalen(&mut self) -> Comp1ScalenW<Comp1CsrSpec> {
        Comp1ScalenW::new(self, 23)
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1_lock(&mut self) -> Comp1LockW<Comp1CsrSpec> {
        Comp1LockW::new(self, 31)
    }
}
#[doc = "Comparator 1 control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1CsrSpec;
impl crate::RegisterSpec for Comp1CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1_csr::R`](R) reader structure"]
impl crate::Readable for Comp1CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp1_csr::W`](W) writer structure"]
impl crate::Writable for Comp1CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for Comp1CsrSpec {}
