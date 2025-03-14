#[doc = "Register `FS_HCINT3` reader"]
pub type R = crate::R<FsHcint3Spec>;
#[doc = "Register `FS_HCINT3` writer"]
pub type W = crate::W<FsHcint3Spec>;
#[doc = "Field `XFRC` reader - Transfer completed"]
pub type XfrcR = crate::BitReader;
#[doc = "Field `XFRC` writer - Transfer completed"]
pub type XfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHH` reader - Channel halted"]
pub type ChhR = crate::BitReader;
#[doc = "Field `CHH` writer - Channel halted"]
pub type ChhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERR` reader - Transaction error"]
pub type TxerrR = crate::BitReader;
#[doc = "Field `TXERR` writer - Transaction error"]
pub type TxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERR` reader - Babble error"]
pub type BberrR = crate::BitReader;
#[doc = "Field `BBERR` writer - Babble error"]
pub type BberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOR` reader - Frame overrun"]
pub type FrmorR = crate::BitReader;
#[doc = "Field `FRMOR` writer - Frame overrun"]
pub type FrmorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERR` reader - Data toggle error"]
pub type DterrR = crate::BitReader;
#[doc = "Field `DTERR` writer - Data toggle error"]
pub type DterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    pub fn xfrc(&self) -> XfrcR {
        XfrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn chh(&self) -> ChhR {
        ChhR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bberr(&self) -> BberrR {
        BberrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    pub fn frmor(&self) -> FrmorR {
        FrmorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dterr(&self) -> DterrR {
        DterrR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XfrcW<FsHcint3Spec> {
        XfrcW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn chh(&mut self) -> ChhW<FsHcint3Spec> {
        ChhW::new(self, 1)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<FsHcint3Spec> {
        StallW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&mut self) -> NakW<FsHcint3Spec> {
        NakW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<FsHcint3Spec> {
        AckW::new(self, 5)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    pub fn txerr(&mut self) -> TxerrW<FsHcint3Spec> {
        TxerrW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bberr(&mut self) -> BberrW<FsHcint3Spec> {
        BberrW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    pub fn frmor(&mut self) -> FrmorW<FsHcint3Spec> {
        FrmorW::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dterr(&mut self) -> DterrW<FsHcint3Spec> {
        DterrW::new(self, 10)
    }
}
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hcint3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hcint3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsHcint3Spec;
impl crate::RegisterSpec for FsHcint3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hcint3::R`](R) reader structure"]
impl crate::Readable for FsHcint3Spec {}
#[doc = "`write(|w| ..)` method takes [`fs_hcint3::W`](W) writer structure"]
impl crate::Writable for FsHcint3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_HCINT3 to value 0"]
impl crate::Resettable for FsHcint3Spec {}
