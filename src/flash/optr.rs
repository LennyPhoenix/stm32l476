#[doc = "Register `OPTR` reader"]
pub type R = crate::R<OptrSpec>;
#[doc = "Register `OPTR` writer"]
pub type W = crate::W<OptrSpec>;
#[doc = "Field `RDP` reader - Read protection level"]
pub type RdpR = crate::FieldReader;
#[doc = "Field `RDP` writer - Read protection level"]
pub type RdpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BOR_LEV` reader - BOR reset Level"]
pub type BorLevR = crate::FieldReader;
#[doc = "Field `BOR_LEV` writer - BOR reset Level"]
pub type BorLevW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type NRstStopR = crate::BitReader;
#[doc = "Field `nRST_STOP` writer - nRST_STOP"]
pub type NRstStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type NRstStdbyR = crate::BitReader;
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY"]
pub type NRstStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_SHDW` reader - nRST_SHDW"]
pub type NRstShdwR = crate::BitReader;
#[doc = "Field `nRST_SHDW` writer - nRST_SHDW"]
pub type NRstShdwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDWG_SW` reader - Independent watchdog selection"]
pub type IdwgSwR = crate::BitReader;
#[doc = "Field `IDWG_SW` writer - Independent watchdog selection"]
pub type IdwgSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode"]
pub type IwdgStopR = crate::BitReader;
#[doc = "Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode"]
pub type IwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode"]
pub type IwdgStdbyR = crate::BitReader;
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode"]
pub type IwdgStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub type WwdgSwR = crate::BitReader;
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub type WwdgSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFB2` reader - Dual-bank boot"]
pub type Bfb2R = crate::BitReader;
#[doc = "Field `BFB2` writer - Dual-bank boot"]
pub type Bfb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALBANK` reader - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
pub type DualbankR = crate::BitReader;
#[doc = "Field `DUALBANK` writer - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
pub type DualbankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT1` reader - Boot configuration"]
pub type NBoot1R = crate::BitReader;
#[doc = "Field `nBOOT1` writer - Boot configuration"]
pub type NBoot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2_PE` reader - SRAM2 parity check enable"]
pub type Sram2PeR = crate::BitReader;
#[doc = "Field `SRAM2_PE` writer - SRAM2 parity check enable"]
pub type Sram2PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2_RST` reader - SRAM2 Erase when system reset"]
pub type Sram2RstR = crate::BitReader;
#[doc = "Field `SRAM2_RST` writer - SRAM2 Erase when system reset"]
pub type Sram2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nSWBOOT0` reader - Software BOOT0"]
pub type NSwboot0R = crate::BitReader;
#[doc = "Field `nSWBOOT0` writer - Software BOOT0"]
pub type NSwboot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT0` reader - nBOOT0 option bit"]
pub type NBoot0R = crate::BitReader;
#[doc = "Field `nBOOT0` writer - nBOOT0 option bit"]
pub type NBoot0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BorLevR {
        BorLevR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRstStopR {
        NRstStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRstStdbyR {
        NRstStdbyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - nRST_SHDW"]
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> NRstShdwR {
        NRstShdwR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&self) -> IdwgSwR {
        IdwgSwR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IwdgStopR {
        IwdgStopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IwdgStdbyR {
        IwdgStdbyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WwdgSwR {
        WwdgSwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Dual-bank boot"]
    #[inline(always)]
    pub fn bfb2(&self) -> Bfb2R {
        Bfb2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
    #[inline(always)]
    pub fn dualbank(&self) -> DualbankR {
        DualbankR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBoot1R {
        NBoot1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM2 parity check enable"]
    #[inline(always)]
    pub fn sram2_pe(&self) -> Sram2PeR {
        Sram2PeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SRAM2 Erase when system reset"]
    #[inline(always)]
    pub fn sram2_rst(&self) -> Sram2RstR {
        Sram2RstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software BOOT0"]
    #[inline(always)]
    pub fn n_swboot0(&self) -> NSwboot0R {
        NSwboot0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&self) -> NBoot0R {
        NBoot0R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RdpW<OptrSpec> {
        RdpW::new(self, 0)
    }
    #[doc = "Bits 8:10 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BorLevW<OptrSpec> {
        BorLevW::new(self, 8)
    }
    #[doc = "Bit 12 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> NRstStopW<OptrSpec> {
        NRstStopW::new(self, 12)
    }
    #[doc = "Bit 13 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> NRstStdbyW<OptrSpec> {
        NRstStdbyW::new(self, 13)
    }
    #[doc = "Bit 14 - nRST_SHDW"]
    #[inline(always)]
    pub fn n_rst_shdw(&mut self) -> NRstShdwW<OptrSpec> {
        NRstShdwW::new(self, 14)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&mut self) -> IdwgSwW<OptrSpec> {
        IdwgSwW::new(self, 16)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IwdgStopW<OptrSpec> {
        IwdgStopW::new(self, 17)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IwdgStdbyW<OptrSpec> {
        IwdgStdbyW::new(self, 18)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WwdgSwW<OptrSpec> {
        WwdgSwW::new(self, 19)
    }
    #[doc = "Bit 20 - Dual-bank boot"]
    #[inline(always)]
    pub fn bfb2(&mut self) -> Bfb2W<OptrSpec> {
        Bfb2W::new(self, 20)
    }
    #[doc = "Bit 21 - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
    #[inline(always)]
    pub fn dualbank(&mut self) -> DualbankW<OptrSpec> {
        DualbankW::new(self, 21)
    }
    #[doc = "Bit 23 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&mut self) -> NBoot1W<OptrSpec> {
        NBoot1W::new(self, 23)
    }
    #[doc = "Bit 24 - SRAM2 parity check enable"]
    #[inline(always)]
    pub fn sram2_pe(&mut self) -> Sram2PeW<OptrSpec> {
        Sram2PeW::new(self, 24)
    }
    #[doc = "Bit 25 - SRAM2 Erase when system reset"]
    #[inline(always)]
    pub fn sram2_rst(&mut self) -> Sram2RstW<OptrSpec> {
        Sram2RstW::new(self, 25)
    }
    #[doc = "Bit 26 - Software BOOT0"]
    #[inline(always)]
    pub fn n_swboot0(&mut self) -> NSwboot0W<OptrSpec> {
        NSwboot0W::new(self, 26)
    }
    #[doc = "Bit 27 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&mut self) -> NBoot0W<OptrSpec> {
        NBoot0W::new(self, 27)
    }
}
#[doc = "Flash option register\n\nYou can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptrSpec;
impl crate::RegisterSpec for OptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr::R`](R) reader structure"]
impl crate::Readable for OptrSpec {}
#[doc = "`write(|w| ..)` method takes [`optr::W`](W) writer structure"]
impl crate::Writable for OptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPTR to value 0xf000_0000"]
impl crate::Resettable for OptrSpec {
    const RESET_VALUE: u32 = 0xf000_0000;
}
