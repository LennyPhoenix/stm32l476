#[doc = "Register `ACR1` reader"]
pub type R = crate::R<Acr1Spec>;
#[doc = "Register `ACR1` writer"]
pub type W = crate::W<Acr1Spec>;
#[doc = "Field `MODE` reader - Audio block mode"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Audio block mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRTCFG` reader - Protocol configuration"]
pub type PrtcfgR = crate::FieldReader;
#[doc = "Field `PRTCFG` writer - Protocol configuration"]
pub type PrtcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DS` reader - Data size"]
pub type DsR = crate::FieldReader;
#[doc = "Field `DS` writer - Data size"]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LSBFIRST` reader - Least significant bit first"]
pub type LsbfirstR = crate::BitReader;
#[doc = "Field `LSBFIRST` writer - Least significant bit first"]
pub type LsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSTR` reader - Clock strobing edge"]
pub type CkstrR = crate::BitReader;
#[doc = "Field `CKSTR` writer - Clock strobing edge"]
pub type CkstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN` reader - Synchronization enable"]
pub type SyncenR = crate::FieldReader;
#[doc = "Field `SYNCEN` writer - Synchronization enable"]
pub type SyncenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONO` reader - Mono mode"]
pub type MonoR = crate::BitReader;
#[doc = "Field `MONO` writer - Mono mode"]
pub type MonoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OutDri` reader - Output drive"]
pub type OutDriR = crate::BitReader;
#[doc = "Field `OutDri` writer - Output drive"]
pub type OutDriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAIAEN` reader - Audio block A enable"]
pub type SaiaenR = crate::BitReader;
#[doc = "Field `SAIAEN` writer - Audio block A enable"]
pub type SaiaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODIV` reader - No divider"]
pub type NodivR = crate::BitReader;
#[doc = "Field `NODIV` writer - No divider"]
pub type NodivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKDIV` reader - Master clock divider"]
pub type MckdivR = crate::FieldReader;
#[doc = "Field `MCKDIV` writer - Master clock divider"]
pub type MckdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Audio block mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Protocol configuration"]
    #[inline(always)]
    pub fn prtcfg(&self) -> PrtcfgR {
        PrtcfgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Data size"]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LsbfirstR {
        LsbfirstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock strobing edge"]
    #[inline(always)]
    pub fn ckstr(&self) -> CkstrR {
        CkstrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&self) -> SyncenR {
        SyncenR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Mono mode"]
    #[inline(always)]
    pub fn mono(&self) -> MonoR {
        MonoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output drive"]
    #[inline(always)]
    pub fn out_dri(&self) -> OutDriR {
        OutDriR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Audio block A enable"]
    #[inline(always)]
    pub fn saiaen(&self) -> SaiaenR {
        SaiaenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nodiv(&self) -> NodivR {
        NodivR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Master clock divider"]
    #[inline(always)]
    pub fn mckdiv(&self) -> MckdivR {
        MckdivR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Audio block mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<Acr1Spec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Protocol configuration"]
    #[inline(always)]
    pub fn prtcfg(&mut self) -> PrtcfgW<Acr1Spec> {
        PrtcfgW::new(self, 2)
    }
    #[doc = "Bits 5:7 - Data size"]
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<Acr1Spec> {
        DsW::new(self, 5)
    }
    #[doc = "Bit 8 - Least significant bit first"]
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LsbfirstW<Acr1Spec> {
        LsbfirstW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock strobing edge"]
    #[inline(always)]
    pub fn ckstr(&mut self) -> CkstrW<Acr1Spec> {
        CkstrW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&mut self) -> SyncenW<Acr1Spec> {
        SyncenW::new(self, 10)
    }
    #[doc = "Bit 12 - Mono mode"]
    #[inline(always)]
    pub fn mono(&mut self) -> MonoW<Acr1Spec> {
        MonoW::new(self, 12)
    }
    #[doc = "Bit 13 - Output drive"]
    #[inline(always)]
    pub fn out_dri(&mut self) -> OutDriW<Acr1Spec> {
        OutDriW::new(self, 13)
    }
    #[doc = "Bit 16 - Audio block A enable"]
    #[inline(always)]
    pub fn saiaen(&mut self) -> SaiaenW<Acr1Spec> {
        SaiaenW::new(self, 16)
    }
    #[doc = "Bit 17 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<Acr1Spec> {
        DmaenW::new(self, 17)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nodiv(&mut self) -> NodivW<Acr1Spec> {
        NodivW::new(self, 19)
    }
    #[doc = "Bits 20:23 - Master clock divider"]
    #[inline(always)]
    pub fn mckdiv(&mut self) -> MckdivW<Acr1Spec> {
        MckdivW::new(self, 20)
    }
}
#[doc = "AConfiguration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`acr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Acr1Spec;
impl crate::RegisterSpec for Acr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr1::R`](R) reader structure"]
impl crate::Readable for Acr1Spec {}
#[doc = "`write(|w| ..)` method takes [`acr1::W`](W) writer structure"]
impl crate::Writable for Acr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACR1 to value 0x40"]
impl crate::Resettable for Acr1Spec {
    const RESET_VALUE: u32 = 0x40;
}
