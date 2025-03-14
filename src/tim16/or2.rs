#[doc = "Register `OR2` reader"]
pub type R = crate::R<Or2Spec>;
#[doc = "Register `OR2` writer"]
pub type W = crate::W<Or2Spec>;
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BkineR = crate::BitReader;
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BkineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable"]
pub type Bkcmp1eR = crate::BitReader;
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable"]
pub type Bkcmp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable"]
pub type Bkcmp2eR = crate::BitReader;
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable"]
pub type Bkcmp2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKDFBK1E` reader - BRK DFSDM_BREAK1 enable"]
pub type Bkdfbk1eR = crate::BitReader;
#[doc = "Field `BKDFBK1E` writer - BRK DFSDM_BREAK1 enable"]
pub type Bkdfbk1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKINP` reader - BRK BKIN input polarity"]
pub type BkinpR = crate::BitReader;
#[doc = "Field `BKINP` writer - BRK BKIN input polarity"]
pub type BkinpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity"]
pub type Bkcmp1pR = crate::BitReader;
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity"]
pub type Bkcmp1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarit"]
pub type Bkcmp2pR = crate::BitReader;
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarit"]
pub type Bkcmp2pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BkineR {
        BkineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> Bkcmp1eR {
        Bkcmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> Bkcmp2eR {
        Bkcmp2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - BRK DFSDM_BREAK1 enable"]
    #[inline(always)]
    pub fn bkdfbk1e(&self) -> Bkdfbk1eR {
        Bkdfbk1eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&self) -> BkinpR {
        BkinpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> Bkcmp1pR {
        Bkcmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarit"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> Bkcmp2pR {
        Bkcmp2pR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&mut self) -> BkineW<Or2Spec> {
        BkineW::new(self, 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> Bkcmp1eW<Or2Spec> {
        Bkcmp1eW::new(self, 1)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> Bkcmp2eW<Or2Spec> {
        Bkcmp2eW::new(self, 2)
    }
    #[doc = "Bit 8 - BRK DFSDM_BREAK1 enable"]
    #[inline(always)]
    pub fn bkdfbk1e(&mut self) -> Bkdfbk1eW<Or2Spec> {
        Bkdfbk1eW::new(self, 8)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&mut self) -> BkinpW<Or2Spec> {
        BkinpW::new(self, 9)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> Bkcmp1pW<Or2Spec> {
        Bkcmp1pW::new(self, 10)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarit"]
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> Bkcmp2pW<Or2Spec> {
        Bkcmp2pW::new(self, 11)
    }
}
#[doc = "TIM17 option register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`or2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Or2Spec;
impl crate::RegisterSpec for Or2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or2::R`](R) reader structure"]
impl crate::Readable for Or2Spec {}
#[doc = "`write(|w| ..)` method takes [`or2::W`](W) writer structure"]
impl crate::Writable for Or2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR2 to value 0"]
impl crate::Resettable for Or2Spec {}
