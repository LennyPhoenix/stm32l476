#[doc = "Register `DOEPTSIZ2` reader"]
pub type R = crate::R<Doeptsiz2Spec>;
#[doc = "Register `DOEPTSIZ2` writer"]
pub type W = crate::W<Doeptsiz2Spec>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XfrsizR = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XfrsizW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RXDPID_STUPCNT` reader - Received data PID/SETUP packet count"]
pub type RxdpidStupcntR = crate::FieldReader;
#[doc = "Field `RXDPID_STUPCNT` writer - Received data PID/SETUP packet count"]
pub type RxdpidStupcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 29:30 - Received data PID/SETUP packet count"]
    #[inline(always)]
    pub fn rxdpid_stupcnt(&self) -> RxdpidStupcntR {
        RxdpidStupcntR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XfrsizW<Doeptsiz2Spec> {
        XfrsizW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<Doeptsiz2Spec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - Received data PID/SETUP packet count"]
    #[inline(always)]
    pub fn rxdpid_stupcnt(&mut self) -> RxdpidStupcntW<Doeptsiz2Spec> {
        RxdpidStupcntW::new(self, 29)
    }
}
#[doc = "device OUT endpoint-2 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doeptsiz2Spec;
impl crate::RegisterSpec for Doeptsiz2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz2::R`](R) reader structure"]
impl crate::Readable for Doeptsiz2Spec {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz2::W`](W) writer structure"]
impl crate::Writable for Doeptsiz2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPTSIZ2 to value 0"]
impl crate::Resettable for Doeptsiz2Spec {}
