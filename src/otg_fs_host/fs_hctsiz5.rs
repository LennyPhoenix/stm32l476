#[doc = "Register `FS_HCTSIZ5` reader"]
pub type R = crate::R<FsHctsiz5Spec>;
#[doc = "Register `FS_HCTSIZ5` writer"]
pub type W = crate::W<FsHctsiz5Spec>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XfrsizR = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XfrsizW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `DPID` writer - Data PID"]
pub type DpidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XfrsizR {
        XfrsizR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XfrsizW<FsHctsiz5Spec> {
        XfrsizW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<FsHctsiz5Spec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&mut self) -> DpidW<FsHctsiz5Spec> {
        DpidW::new(self, 29)
    }
}
#[doc = "OTG_FS host channel-5 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hctsiz5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hctsiz5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsHctsiz5Spec;
impl crate::RegisterSpec for FsHctsiz5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hctsiz5::R`](R) reader structure"]
impl crate::Readable for FsHctsiz5Spec {}
#[doc = "`write(|w| ..)` method takes [`fs_hctsiz5::W`](W) writer structure"]
impl crate::Writable for FsHctsiz5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_HCTSIZ5 to value 0"]
impl crate::Resettable for FsHctsiz5Spec {}
