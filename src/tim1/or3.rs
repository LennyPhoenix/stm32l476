#[doc = "Register `OR3` reader"]
pub type R = crate::R<Or3Spec>;
#[doc = "Register `OR3` writer"]
pub type W = crate::W<Or3Spec>;
#[doc = "Field `BK2INE` reader - BRK2 BKIN input enable"]
pub type Bk2ineR = crate::BitReader;
#[doc = "Field `BK2INE` writer - BRK2 BKIN input enable"]
pub type Bk2ineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable"]
pub type Bk2cmp1eR = crate::BitReader;
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable"]
pub type Bk2cmp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable"]
pub type Bk2cmp2eR = crate::BitReader;
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable"]
pub type Bk2cmp2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2DFBK0E` reader - BRK2 DFSDM_BREAK0 enable"]
pub type Bk2dfbk0eR = crate::BitReader;
#[doc = "Field `BK2DFBK0E` writer - BRK2 DFSDM_BREAK0 enable"]
pub type Bk2dfbk0eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2INP` reader - BRK2 BKIN input polarity"]
pub type Bk2inpR = crate::BitReader;
#[doc = "Field `BK2INP` writer - BRK2 BKIN input polarity"]
pub type Bk2inpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity"]
pub type Bk2cmp1pR = crate::BitReader;
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity"]
pub type Bk2cmp1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity"]
pub type Bk2cmp2pR = crate::BitReader;
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity"]
pub type Bk2cmp2pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&self) -> Bk2ineR {
        Bk2ineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> Bk2cmp1eR {
        Bk2cmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> Bk2cmp2eR {
        Bk2cmp2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - BRK2 DFSDM_BREAK0 enable"]
    #[inline(always)]
    pub fn bk2dfbk0e(&self) -> Bk2dfbk0eR {
        Bk2dfbk0eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> Bk2inpR {
        Bk2inpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> Bk2cmp1pR {
        Bk2cmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> Bk2cmp2pR {
        Bk2cmp2pR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&mut self) -> Bk2ineW<Or3Spec> {
        Bk2ineW::new(self, 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> Bk2cmp1eW<Or3Spec> {
        Bk2cmp1eW::new(self, 1)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> Bk2cmp2eW<Or3Spec> {
        Bk2cmp2eW::new(self, 2)
    }
    #[doc = "Bit 8 - BRK2 DFSDM_BREAK0 enable"]
    #[inline(always)]
    pub fn bk2dfbk0e(&mut self) -> Bk2dfbk0eW<Or3Spec> {
        Bk2dfbk0eW::new(self, 8)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> Bk2inpW<Or3Spec> {
        Bk2inpW::new(self, 9)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> Bk2cmp1pW<Or3Spec> {
        Bk2cmp1pW::new(self, 10)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> Bk2cmp2pW<Or3Spec> {
        Bk2cmp2pW::new(self, 11)
    }
}
#[doc = "DMA address for full transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`or3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Or3Spec;
impl crate::RegisterSpec for Or3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or3::R`](R) reader structure"]
impl crate::Readable for Or3Spec {}
#[doc = "`write(|w| ..)` method takes [`or3::W`](W) writer structure"]
impl crate::Writable for Or3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR3 to value 0x01"]
impl crate::Resettable for Or3Spec {
    const RESET_VALUE: u32 = 0x01;
}
