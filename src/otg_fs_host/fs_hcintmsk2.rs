#[doc = "Register `FS_HCINTMSK2` reader"]
pub type R = crate::R<FsHcintmsk2Spec>;
#[doc = "Register `FS_HCINTMSK2` writer"]
pub type W = crate::W<FsHcintmsk2Spec>;
#[doc = "Field `XFRCM` reader - Transfer completed mask"]
pub type XfrcmR = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed mask"]
pub type XfrcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHM` reader - Channel halted mask"]
pub type ChhmR = crate::BitReader;
#[doc = "Field `CHHM` writer - Channel halted mask"]
pub type ChhmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLM` reader - STALL response received interrupt mask"]
pub type StallmR = crate::BitReader;
#[doc = "Field `STALLM` writer - STALL response received interrupt mask"]
pub type StallmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - NAK response received interrupt mask"]
pub type NakmR = crate::BitReader;
#[doc = "Field `NAKM` writer - NAK response received interrupt mask"]
pub type NakmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKM` reader - ACK response received/transmitted interrupt mask"]
pub type AckmR = crate::BitReader;
#[doc = "Field `ACKM` writer - ACK response received/transmitted interrupt mask"]
pub type AckmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - response received interrupt mask"]
pub type NyetR = crate::BitReader;
#[doc = "Field `NYET` writer - response received interrupt mask"]
pub type NyetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRM` reader - Transaction error mask"]
pub type TxerrmR = crate::BitReader;
#[doc = "Field `TXERRM` writer - Transaction error mask"]
pub type TxerrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERRM` reader - Babble error mask"]
pub type BberrmR = crate::BitReader;
#[doc = "Field `BBERRM` writer - Babble error mask"]
pub type BberrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMORM` reader - Frame overrun mask"]
pub type FrmormR = crate::BitReader;
#[doc = "Field `FRMORM` writer - Frame overrun mask"]
pub type FrmormW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRM` reader - Data toggle error mask"]
pub type DterrmR = crate::BitReader;
#[doc = "Field `DTERRM` writer - Data toggle error mask"]
pub type DterrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XfrcmR {
        XfrcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&self) -> ChhmR {
        ChhmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&self) -> StallmR {
        StallmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NakmR {
        NakmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&self) -> AckmR {
        AckmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    pub fn nyet(&self) -> NyetR {
        NyetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn txerrm(&self) -> TxerrmR {
        TxerrmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bberrm(&self) -> BberrmR {
        BberrmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&self) -> FrmormR {
        FrmormR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&self) -> DterrmR {
        DterrmR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XfrcmW<FsHcintmsk2Spec> {
        XfrcmW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&mut self) -> ChhmW<FsHcintmsk2Spec> {
        ChhmW::new(self, 1)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&mut self) -> StallmW<FsHcintmsk2Spec> {
        StallmW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&mut self) -> NakmW<FsHcintmsk2Spec> {
        NakmW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&mut self) -> AckmW<FsHcintmsk2Spec> {
        AckmW::new(self, 5)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NyetW<FsHcintmsk2Spec> {
        NyetW::new(self, 6)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn txerrm(&mut self) -> TxerrmW<FsHcintmsk2Spec> {
        TxerrmW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bberrm(&mut self) -> BberrmW<FsHcintmsk2Spec> {
        BberrmW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&mut self) -> FrmormW<FsHcintmsk2Spec> {
        FrmormW::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&mut self) -> DterrmW<FsHcintmsk2Spec> {
        DterrmW::new(self, 10)
    }
}
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hcintmsk2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hcintmsk2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsHcintmsk2Spec;
impl crate::RegisterSpec for FsHcintmsk2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hcintmsk2::R`](R) reader structure"]
impl crate::Readable for FsHcintmsk2Spec {}
#[doc = "`write(|w| ..)` method takes [`fs_hcintmsk2::W`](W) writer structure"]
impl crate::Writable for FsHcintmsk2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_HCINTMSK2 to value 0"]
impl crate::Resettable for FsHcintmsk2Spec {}
