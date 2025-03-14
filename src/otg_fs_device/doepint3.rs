#[doc = "Register `DOEPINT3` reader"]
pub type R = crate::R<Doepint3Spec>;
#[doc = "Register `DOEPINT3` writer"]
pub type W = crate::W<Doepint3Spec>;
#[doc = "Field `XFRC` reader - XFRC"]
pub type XfrcR = crate::BitReader;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - EPDISD"]
pub type EpdisdR = crate::BitReader;
#[doc = "Field `EPDISD` writer - EPDISD"]
pub type EpdisdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUP` reader - STUP"]
pub type StupR = crate::BitReader;
#[doc = "Field `STUP` writer - STUP"]
pub type StupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDIS` reader - OTEPDIS"]
pub type OtepdisR = crate::BitReader;
#[doc = "Field `OTEPDIS` writer - OTEPDIS"]
pub type OtepdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSTUP` reader - B2BSTUP"]
pub type B2bstupR = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - B2BSTUP"]
pub type B2bstupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XfrcR {
        XfrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EpdisdR {
        EpdisdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STUP"]
    #[inline(always)]
    pub fn stup(&self) -> StupR {
        StupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&self) -> OtepdisR {
        OtepdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2bstupR {
        B2bstupR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XfrcW<Doepint3Spec> {
        XfrcW::new(self, 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EpdisdW<Doepint3Spec> {
        EpdisdW::new(self, 1)
    }
    #[doc = "Bit 3 - STUP"]
    #[inline(always)]
    pub fn stup(&mut self) -> StupW<Doepint3Spec> {
        StupW::new(self, 3)
    }
    #[doc = "Bit 4 - OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&mut self) -> OtepdisW<Doepint3Spec> {
        OtepdisW::new(self, 4)
    }
    #[doc = "Bit 6 - B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2bstupW<Doepint3Spec> {
        B2bstupW::new(self, 6)
    }
}
#[doc = "device endpoint-3 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepint3Spec;
impl crate::RegisterSpec for Doepint3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint3::R`](R) reader structure"]
impl crate::Readable for Doepint3Spec {}
#[doc = "`write(|w| ..)` method takes [`doepint3::W`](W) writer structure"]
impl crate::Writable for Doepint3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPINT3 to value 0x80"]
impl crate::Resettable for Doepint3Spec {
    const RESET_VALUE: u32 = 0x80;
}
