#[doc = "Register `CH3CFGR1` reader"]
pub type R = crate::R<Ch3cfgr1Spec>;
#[doc = "Register `CH3CFGR1` writer"]
pub type W = crate::W<Ch3cfgr1Spec>;
#[doc = "Field `SITP` reader - SITP"]
pub type SitpR = crate::FieldReader;
#[doc = "Field `SITP` writer - SITP"]
pub type SitpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPICKSEL` reader - SPICKSEL"]
pub type SpickselR = crate::FieldReader;
#[doc = "Field `SPICKSEL` writer - SPICKSEL"]
pub type SpickselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCDEN` reader - SCDEN"]
pub type ScdenR = crate::BitReader;
#[doc = "Field `SCDEN` writer - SCDEN"]
pub type ScdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKABEN` reader - CKABEN"]
pub type CkabenR = crate::BitReader;
#[doc = "Field `CKABEN` writer - CKABEN"]
pub type CkabenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN` reader - CHEN"]
pub type ChenR = crate::BitReader;
#[doc = "Field `CHEN` writer - CHEN"]
pub type ChenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHINSEL` reader - CHINSEL"]
pub type ChinselR = crate::BitReader;
#[doc = "Field `CHINSEL` writer - CHINSEL"]
pub type ChinselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATMPX` reader - DATMPX"]
pub type DatmpxR = crate::FieldReader;
#[doc = "Field `DATMPX` writer - DATMPX"]
pub type DatmpxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATPACK` reader - DATPACK"]
pub type DatpackR = crate::FieldReader;
#[doc = "Field `DATPACK` writer - DATPACK"]
pub type DatpackW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&self) -> SitpR {
        SitpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&self) -> SpickselR {
        SpickselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&self) -> ScdenR {
        ScdenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&self) -> CkabenR {
        CkabenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&self) -> ChinselR {
        ChinselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&self) -> DatmpxR {
        DatmpxR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&self) -> DatpackR {
        DatpackR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&mut self) -> SitpW<Ch3cfgr1Spec> {
        SitpW::new(self, 0)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&mut self) -> SpickselW<Ch3cfgr1Spec> {
        SpickselW::new(self, 2)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&mut self) -> ScdenW<Ch3cfgr1Spec> {
        ScdenW::new(self, 5)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&mut self) -> CkabenW<Ch3cfgr1Spec> {
        CkabenW::new(self, 6)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&mut self) -> ChenW<Ch3cfgr1Spec> {
        ChenW::new(self, 7)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&mut self) -> ChinselW<Ch3cfgr1Spec> {
        ChinselW::new(self, 8)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&mut self) -> DatmpxW<Ch3cfgr1Spec> {
        DatmpxW::new(self, 12)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&mut self) -> DatpackW<Ch3cfgr1Spec> {
        DatpackW::new(self, 14)
    }
}
#[doc = "CH3CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3cfgr1Spec;
impl crate::RegisterSpec for Ch3cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3cfgr1::R`](R) reader structure"]
impl crate::Readable for Ch3cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ch3cfgr1::W`](W) writer structure"]
impl crate::Writable for Ch3cfgr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3CFGR1 to value 0"]
impl crate::Resettable for Ch3cfgr1Spec {}
