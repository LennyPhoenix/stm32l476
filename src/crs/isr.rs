#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `SYNCOKF` reader - SYNC event OK flag"]
pub type SyncokfR = crate::BitReader;
#[doc = "Field `SYNCWARNF` reader - SYNC warning flag"]
pub type SyncwarnfR = crate::BitReader;
#[doc = "Field `ERRF` reader - Error flag"]
pub type ErrfR = crate::BitReader;
#[doc = "Field `ESYNCF` reader - Expected SYNC flag"]
pub type EsyncfR = crate::BitReader;
#[doc = "Field `SYNCERR` reader - SYNC error"]
pub type SyncerrR = crate::BitReader;
#[doc = "Field `SYNCMISS` reader - SYNC missed"]
pub type SyncmissR = crate::BitReader;
#[doc = "Field `TRIMOVF` reader - Trimming overflow or underflow"]
pub type TrimovfR = crate::BitReader;
#[doc = "Field `FEDIR` reader - Frequency error direction"]
pub type FedirR = crate::BitReader;
#[doc = "Field `FECAP` reader - Frequency error capture"]
pub type FecapR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - SYNC event OK flag"]
    #[inline(always)]
    pub fn syncokf(&self) -> SyncokfR {
        SyncokfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning flag"]
    #[inline(always)]
    pub fn syncwarnf(&self) -> SyncwarnfR {
        SyncwarnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error flag"]
    #[inline(always)]
    pub fn errf(&self) -> ErrfR {
        ErrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC flag"]
    #[inline(always)]
    pub fn esyncf(&self) -> EsyncfR {
        EsyncfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SYNC error"]
    #[inline(always)]
    pub fn syncerr(&self) -> SyncerrR {
        SyncerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SYNC missed"]
    #[inline(always)]
    pub fn syncmiss(&self) -> SyncmissR {
        SyncmissR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Trimming overflow or underflow"]
    #[inline(always)]
    pub fn trimovf(&self) -> TrimovfR {
        TrimovfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Frequency error direction"]
    #[inline(always)]
    pub fn fedir(&self) -> FedirR {
        FedirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Frequency error capture"]
    #[inline(always)]
    pub fn fecap(&self) -> FecapR {
        FecapR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
