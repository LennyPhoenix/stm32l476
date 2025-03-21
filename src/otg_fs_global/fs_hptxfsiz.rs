#[doc = "Register `FS_HPTXFSIZ` reader"]
pub type R = crate::R<FsHptxfsizSpec>;
#[doc = "Register `FS_HPTXFSIZ` writer"]
pub type W = crate::W<FsHptxfsizSpec>;
#[doc = "Field `PTXSA` reader - Host periodic TxFIFO start address"]
pub type PtxsaR = crate::FieldReader<u16>;
#[doc = "Field `PTXSA` writer - Host periodic TxFIFO start address"]
pub type PtxsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXFSIZ` reader - Host periodic TxFIFO depth"]
pub type PtxfsizR = crate::FieldReader<u16>;
#[doc = "Field `PTXFSIZ` writer - Host periodic TxFIFO depth"]
pub type PtxfsizW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PtxsaR {
        PtxsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfsiz(&self) -> PtxfsizR {
        PtxfsizR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PtxsaW<FsHptxfsizSpec> {
        PtxsaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfsiz(&mut self) -> PtxfsizW<FsHptxfsizSpec> {
        PtxfsizW::new(self, 16)
    }
}
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsHptxfsizSpec;
impl crate::RegisterSpec for FsHptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hptxfsiz::R`](R) reader structure"]
impl crate::Readable for FsHptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_hptxfsiz::W`](W) writer structure"]
impl crate::Writable for FsHptxfsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_HPTXFSIZ to value 0x0200_0600"]
impl crate::Resettable for FsHptxfsizSpec {
    const RESET_VALUE: u32 = 0x0200_0600;
}
