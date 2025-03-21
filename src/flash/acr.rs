#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Field `LATENCY` reader - Latency"]
pub type LatencyR = crate::FieldReader;
#[doc = "Field `LATENCY` writer - Latency"]
pub type LatencyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PrftenR = crate::BitReader;
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PrftenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEN` reader - Instruction cache enable"]
pub type IcenR = crate::BitReader;
#[doc = "Field `ICEN` writer - Instruction cache enable"]
pub type IcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEN` reader - Data cache enable"]
pub type DcenR = crate::BitReader;
#[doc = "Field `DCEN` writer - Data cache enable"]
pub type DcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICRST` reader - Instruction cache reset"]
pub type IcrstR = crate::BitReader;
#[doc = "Field `ICRST` writer - Instruction cache reset"]
pub type IcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRST` reader - Data cache reset"]
pub type DcrstR = crate::BitReader;
#[doc = "Field `DCRST` writer - Data cache reset"]
pub type DcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN_PD` reader - Flash Power-down mode during Low-power run mode"]
pub type RunPdR = crate::BitReader;
#[doc = "Field `RUN_PD` writer - Flash Power-down mode during Low-power run mode"]
pub type RunPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_PD` reader - Flash Power-down mode during Low-power sleep mode"]
pub type SleepPdR = crate::BitReader;
#[doc = "Field `SLEEP_PD` writer - Flash Power-down mode during Low-power sleep mode"]
pub type SleepPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LatencyR {
        LatencyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PrftenR {
        PrftenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> IcenR {
        IcenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    pub fn dcen(&self) -> DcenR {
        DcenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> IcrstR {
        IcrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    pub fn dcrst(&self) -> DcrstR {
        DcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    pub fn run_pd(&self) -> RunPdR {
        RunPdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    pub fn sleep_pd(&self) -> SleepPdR {
        SleepPdR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    pub fn latency(&mut self) -> LatencyW<AcrSpec> {
        LatencyW::new(self, 0)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&mut self) -> PrftenW<AcrSpec> {
        PrftenW::new(self, 8)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&mut self) -> IcenW<AcrSpec> {
        IcenW::new(self, 9)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    pub fn dcen(&mut self) -> DcenW<AcrSpec> {
        DcenW::new(self, 10)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&mut self) -> IcrstW<AcrSpec> {
        IcrstW::new(self, 11)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    pub fn dcrst(&mut self) -> DcrstW<AcrSpec> {
        DcrstW::new(self, 12)
    }
    #[doc = "Bit 13 - Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    pub fn run_pd(&mut self) -> RunPdW<AcrSpec> {
        RunPdW::new(self, 13)
    }
    #[doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SleepPdW<AcrSpec> {
        SleepPdW::new(self, 14)
    }
}
#[doc = "Access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACR to value 0x0600"]
impl crate::Resettable for AcrSpec {
    const RESET_VALUE: u32 = 0x0600;
}
