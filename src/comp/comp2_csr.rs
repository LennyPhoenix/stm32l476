#[doc = "Register `COMP2_CSR` reader"]
pub type R = crate::R<Comp2CsrSpec>;
#[doc = "Register `COMP2_CSR` writer"]
pub type W = crate::W<Comp2CsrSpec>;
#[doc = "Field `COMP2_EN` reader - Comparator 2 enable bit"]
pub type Comp2EnR = crate::BitReader;
#[doc = "Field `COMP2_EN` writer - Comparator 2 enable bit"]
pub type Comp2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_PWRMODE` reader - Power Mode of the comparator 2"]
pub type Comp2PwrmodeR = crate::FieldReader;
#[doc = "Field `COMP2_PWRMODE` writer - Power Mode of the comparator 2"]
pub type Comp2PwrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2_INMSEL` reader - Comparator 2 Input Minus connection configuration bit"]
pub type Comp2InmselR = crate::FieldReader;
#[doc = "Field `COMP2_INMSEL` writer - Comparator 2 Input Minus connection configuration bit"]
pub type Comp2InmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP2_INPSEL` reader - Comparator 2 Input Plus connection configuration bit"]
pub type Comp2InpselR = crate::BitReader;
#[doc = "Field `COMP2_INPSEL` writer - Comparator 2 Input Plus connection configuration bit"]
pub type Comp2InpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_WINMODE` reader - Windows mode selection bit"]
pub type Comp2WinmodeR = crate::BitReader;
#[doc = "Field `COMP2_WINMODE` writer - Windows mode selection bit"]
pub type Comp2WinmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_POLARITY` reader - Comparator 2 polarity selection bit"]
pub type Comp2PolarityR = crate::BitReader;
#[doc = "Field `COMP2_POLARITY` writer - Comparator 2 polarity selection bit"]
pub type Comp2PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_HYST` reader - Comparator 2 hysteresis selection bits"]
pub type Comp2HystR = crate::FieldReader;
#[doc = "Field `COMP2_HYST` writer - Comparator 2 hysteresis selection bits"]
pub type Comp2HystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2_BLANKING` reader - Comparator 2 blanking source selection bits"]
pub type Comp2BlankingR = crate::FieldReader;
#[doc = "Field `COMP2_BLANKING` writer - Comparator 2 blanking source selection bits"]
pub type Comp2BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP2_BRGEN` reader - Scaler bridge enable"]
pub type Comp2BrgenR = crate::BitReader;
#[doc = "Field `COMP2_BRGEN` writer - Scaler bridge enable"]
pub type Comp2BrgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_SCALEN` reader - Voltage scaler enable bit"]
pub type Comp2ScalenR = crate::BitReader;
#[doc = "Field `COMP2_SCALEN` writer - Voltage scaler enable bit"]
pub type Comp2ScalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_VALUE` reader - Comparator 2 output status bit"]
pub type Comp2ValueR = crate::BitReader;
#[doc = "Field `COMP2_LOCK` writer - COMP2_CSR register lock bit"]
pub type Comp2LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2_en(&self) -> Comp2EnR {
        Comp2EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn comp2_pwrmode(&self) -> Comp2PwrmodeR {
        Comp2PwrmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inmsel(&self) -> Comp2InmselR {
        Comp2InmselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inpsel(&self) -> Comp2InpselR {
        Comp2InpselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn comp2_winmode(&self) -> Comp2WinmodeR {
        Comp2WinmodeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2_polarity(&self) -> Comp2PolarityR {
        Comp2PolarityR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn comp2_hyst(&self) -> Comp2HystR {
        Comp2HystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> Comp2BlankingR {
        Comp2BlankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn comp2_brgen(&self) -> Comp2BrgenR {
        Comp2BrgenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn comp2_scalen(&self) -> Comp2ScalenR {
        Comp2ScalenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn comp2_value(&self) -> Comp2ValueR {
        Comp2ValueR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2_en(&mut self) -> Comp2EnW<Comp2CsrSpec> {
        Comp2EnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn comp2_pwrmode(&mut self) -> Comp2PwrmodeW<Comp2CsrSpec> {
        Comp2PwrmodeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inmsel(&mut self) -> Comp2InmselW<Comp2CsrSpec> {
        Comp2InmselW::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inpsel(&mut self) -> Comp2InpselW<Comp2CsrSpec> {
        Comp2InpselW::new(self, 7)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn comp2_winmode(&mut self) -> Comp2WinmodeW<Comp2CsrSpec> {
        Comp2WinmodeW::new(self, 9)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2_polarity(&mut self) -> Comp2PolarityW<Comp2CsrSpec> {
        Comp2PolarityW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn comp2_hyst(&mut self) -> Comp2HystW<Comp2CsrSpec> {
        Comp2HystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn comp2_blanking(&mut self) -> Comp2BlankingW<Comp2CsrSpec> {
        Comp2BlankingW::new(self, 18)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn comp2_brgen(&mut self) -> Comp2BrgenW<Comp2CsrSpec> {
        Comp2BrgenW::new(self, 22)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn comp2_scalen(&mut self) -> Comp2ScalenW<Comp2CsrSpec> {
        Comp2ScalenW::new(self, 23)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    pub fn comp2_lock(&mut self) -> Comp2LockW<Comp2CsrSpec> {
        Comp2LockW::new(self, 31)
    }
}
#[doc = "Comparator 2 control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp2CsrSpec;
impl crate::RegisterSpec for Comp2CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp2_csr::R`](R) reader structure"]
impl crate::Readable for Comp2CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp2_csr::W`](W) writer structure"]
impl crate::Writable for Comp2CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for Comp2CsrSpec {}
