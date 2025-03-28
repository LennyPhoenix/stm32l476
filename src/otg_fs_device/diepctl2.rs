#[doc = "Register `DIEPCTL2` reader"]
pub type R = crate::R<Diepctl2Spec>;
#[doc = "Register `DIEPCTL2` writer"]
pub type W = crate::W<Diepctl2Spec>;
#[doc = "Field `MPSIZ` reader - MPSIZ"]
pub type MpsizR = crate::FieldReader<u16>;
#[doc = "Field `MPSIZ` writer - MPSIZ"]
pub type MpsizW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBAEP` reader - USBAEP"]
pub type UsbaepR = crate::BitReader;
#[doc = "Field `USBAEP` writer - USBAEP"]
pub type UsbaepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EONUM_DPID` reader - EONUM/DPID"]
pub type EonumDpidR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAKSTS"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYP` reader - EPTYP"]
pub type EptypR = crate::FieldReader;
#[doc = "Field `EPTYP` writer - EPTYP"]
pub type EptypW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Stall` reader - Stall"]
pub type StallR = crate::BitReader;
#[doc = "Field `Stall` writer - Stall"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TXFNUM"]
pub type TxfnumR = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TXFNUM"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - CNAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - SNAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD0PID_SEVNFRM` writer - SD0PID/SEVNFRM"]
pub type Sd0pidSevnfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SODDFRM` writer - SODDFRM"]
pub type SoddfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - EPDIS"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDIS` writer - EPDIS"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - EPENA"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPENA` writer - EPENA"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MpsizR {
        MpsizR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USBAEP"]
    #[inline(always)]
    pub fn usbaep(&self) -> UsbaepR {
        UsbaepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - EONUM/DPID"]
    #[inline(always)]
    pub fn eonum_dpid(&self) -> EonumDpidR {
        EonumDpidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAKSTS"]
    #[inline(always)]
    pub fn naksts(&self) -> NakstsR {
        NakstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EptypR {
        EptypR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Stall"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TXFNUM"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - EPDIS"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EPENA"]
    #[inline(always)]
    pub fn epena(&self) -> EpenaR {
        EpenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MpsizW<Diepctl2Spec> {
        MpsizW::new(self, 0)
    }
    #[doc = "Bit 15 - USBAEP"]
    #[inline(always)]
    pub fn usbaep(&mut self) -> UsbaepW<Diepctl2Spec> {
        UsbaepW::new(self, 15)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    pub fn eptyp(&mut self) -> EptypW<Diepctl2Spec> {
        EptypW::new(self, 18)
    }
    #[doc = "Bit 21 - Stall"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<Diepctl2Spec> {
        StallW::new(self, 21)
    }
    #[doc = "Bits 22:25 - TXFNUM"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TxfnumW<Diepctl2Spec> {
        TxfnumW::new(self, 22)
    }
    #[doc = "Bit 26 - CNAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<Diepctl2Spec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - SNAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<Diepctl2Spec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 28 - SD0PID/SEVNFRM"]
    #[inline(always)]
    pub fn sd0pid_sevnfrm(&mut self) -> Sd0pidSevnfrmW<Diepctl2Spec> {
        Sd0pidSevnfrmW::new(self, 28)
    }
    #[doc = "Bit 29 - SODDFRM"]
    #[inline(always)]
    pub fn soddfrm(&mut self) -> SoddfrmW<Diepctl2Spec> {
        SoddfrmW::new(self, 29)
    }
    #[doc = "Bit 30 - EPDIS"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EpdisW<Diepctl2Spec> {
        EpdisW::new(self, 30)
    }
    #[doc = "Bit 31 - EPENA"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<Diepctl2Spec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "OTG device endpoint-2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepctl2Spec;
impl crate::RegisterSpec for Diepctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl2::R`](R) reader structure"]
impl crate::Readable for Diepctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`diepctl2::W`](W) writer structure"]
impl crate::Writable for Diepctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPCTL2 to value 0"]
impl crate::Resettable for Diepctl2Spec {}
