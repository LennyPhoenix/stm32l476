#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<Cfgr1Spec>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<Cfgr1Spec>;
#[doc = "Field `FWDIS` reader - Firewall disable"]
pub type FwdisR = crate::BitReader;
#[doc = "Field `FWDIS` writer - Firewall disable"]
pub type FwdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOSTEN` reader - I/O analog switch voltage booster enable"]
pub type BoostenR = crate::BitReader;
#[doc = "Field `BOOSTEN` writer - I/O analog switch voltage booster enable"]
pub type BoostenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB6_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB6"]
pub type I2cPb6FmpR = crate::BitReader;
#[doc = "Field `I2C_PB6_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB6"]
pub type I2cPb6FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB7_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB7"]
pub type I2cPb7FmpR = crate::BitReader;
#[doc = "Field `I2C_PB7_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB7"]
pub type I2cPb7FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB8_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB8"]
pub type I2cPb8FmpR = crate::BitReader;
#[doc = "Field `I2C_PB8_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB8"]
pub type I2cPb8FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB9_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB9"]
pub type I2cPb9FmpR = crate::BitReader;
#[doc = "Field `I2C_PB9_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB9"]
pub type I2cPb9FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_FMP` reader - I2C1 Fast-mode Plus driving capability activation"]
pub type I2c1FmpR = crate::BitReader;
#[doc = "Field `I2C1_FMP` writer - I2C1 Fast-mode Plus driving capability activation"]
pub type I2c1FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_FMP` reader - I2C2 Fast-mode Plus driving capability activation"]
pub type I2c2FmpR = crate::BitReader;
#[doc = "Field `I2C2_FMP` writer - I2C2 Fast-mode Plus driving capability activation"]
pub type I2c2FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_FMP` reader - I2C3 Fast-mode Plus driving capability activation"]
pub type I2c3FmpR = crate::BitReader;
#[doc = "Field `I2C3_FMP` writer - I2C3 Fast-mode Plus driving capability activation"]
pub type I2c3FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPU_IE` reader - Floating Point Unit interrupts enable bits"]
pub type FpuIeR = crate::FieldReader;
#[doc = "Field `FPU_IE` writer - Floating Point Unit interrupts enable bits"]
pub type FpuIeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Firewall disable"]
    #[inline(always)]
    pub fn fwdis(&self) -> FwdisR {
        FwdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    pub fn boosten(&self) -> BoostenR {
        BoostenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2cPb6FmpR {
        I2cPb6FmpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2cPb7FmpR {
        I2cPb7FmpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2cPb8FmpR {
        I2cPb8FmpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2cPb9FmpR {
        I2cPb9FmpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2c1FmpR {
        I2c1FmpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2c2FmpR {
        I2c2FmpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2c3FmpR {
        I2c3FmpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Floating Point Unit interrupts enable bits"]
    #[inline(always)]
    pub fn fpu_ie(&self) -> FpuIeR {
        FpuIeR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Firewall disable"]
    #[inline(always)]
    pub fn fwdis(&mut self) -> FwdisW<Cfgr1Spec> {
        FwdisW::new(self, 0)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    pub fn boosten(&mut self) -> BoostenW<Cfgr1Spec> {
        BoostenW::new(self, 8)
    }
    #[doc = "Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2cPb6FmpW<Cfgr1Spec> {
        I2cPb6FmpW::new(self, 16)
    }
    #[doc = "Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2cPb7FmpW<Cfgr1Spec> {
        I2cPb7FmpW::new(self, 17)
    }
    #[doc = "Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2cPb8FmpW<Cfgr1Spec> {
        I2cPb8FmpW::new(self, 18)
    }
    #[doc = "Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2cPb9FmpW<Cfgr1Spec> {
        I2cPb9FmpW::new(self, 19)
    }
    #[doc = "Bit 20 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2c1FmpW<Cfgr1Spec> {
        I2c1FmpW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2c2FmpW<Cfgr1Spec> {
        I2c2FmpW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2c3FmpW<Cfgr1Spec> {
        I2c3FmpW::new(self, 22)
    }
    #[doc = "Bits 26:31 - Floating Point Unit interrupts enable bits"]
    #[inline(always)]
    pub fn fpu_ie(&mut self) -> FpuIeW<Cfgr1Spec> {
        FpuIeW::new(self, 26)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr1Spec;
impl crate::RegisterSpec for Cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for Cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for Cfgr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR1 to value 0x7c00_0001"]
impl crate::Resettable for Cfgr1Spec {
    const RESET_VALUE: u32 = 0x7c00_0001;
}
