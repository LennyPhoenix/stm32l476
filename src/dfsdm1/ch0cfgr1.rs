#[doc = "Register `CH0CFGR1` reader"]
pub type R = crate::R<Ch0cfgr1Spec>;
#[doc = "Register `CH0CFGR1` writer"]
pub type W = crate::W<Ch0cfgr1Spec>;
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
#[doc = "Field `CKOUTDIV` reader - CKOUTDIV"]
pub type CkoutdivR = crate::FieldReader;
#[doc = "Field `CKOUTDIV` writer - CKOUTDIV"]
pub type CkoutdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKOUTSRC` reader - CKOUTSRC"]
pub type CkoutsrcR = crate::BitReader;
#[doc = "Field `CKOUTSRC` writer - CKOUTSRC"]
pub type CkoutsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDMEN` reader - DFSDMEN"]
pub type DfsdmenR = crate::BitReader;
#[doc = "Field `DFSDMEN` writer - DFSDMEN"]
pub type DfsdmenW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 16:23 - CKOUTDIV"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CkoutdivR {
        CkoutdivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CkoutsrcR {
        CkoutsrcR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DfsdmenR {
        DfsdmenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&mut self) -> SitpW<Ch0cfgr1Spec> {
        SitpW::new(self, 0)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&mut self) -> SpickselW<Ch0cfgr1Spec> {
        SpickselW::new(self, 2)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&mut self) -> ScdenW<Ch0cfgr1Spec> {
        ScdenW::new(self, 5)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&mut self) -> CkabenW<Ch0cfgr1Spec> {
        CkabenW::new(self, 6)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&mut self) -> ChenW<Ch0cfgr1Spec> {
        ChenW::new(self, 7)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&mut self) -> ChinselW<Ch0cfgr1Spec> {
        ChinselW::new(self, 8)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&mut self) -> DatmpxW<Ch0cfgr1Spec> {
        DatmpxW::new(self, 12)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&mut self) -> DatpackW<Ch0cfgr1Spec> {
        DatpackW::new(self, 14)
    }
    #[doc = "Bits 16:23 - CKOUTDIV"]
    #[inline(always)]
    pub fn ckoutdiv(&mut self) -> CkoutdivW<Ch0cfgr1Spec> {
        CkoutdivW::new(self, 16)
    }
    #[doc = "Bit 30 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&mut self) -> CkoutsrcW<Ch0cfgr1Spec> {
        CkoutsrcW::new(self, 30)
    }
    #[doc = "Bit 31 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DfsdmenW<Ch0cfgr1Spec> {
        DfsdmenW::new(self, 31)
    }
}
#[doc = "channel configuration y register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0cfgr1Spec;
impl crate::RegisterSpec for Ch0cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0cfgr1::R`](R) reader structure"]
impl crate::Readable for Ch0cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ch0cfgr1::W`](W) writer structure"]
impl crate::Writable for Ch0cfgr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0CFGR1 to value 0"]
impl crate::Resettable for Ch0cfgr1Spec {}
