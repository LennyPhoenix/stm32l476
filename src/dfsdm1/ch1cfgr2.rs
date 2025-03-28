#[doc = "Register `CH1CFGR2` reader"]
pub type R = crate::R<Ch1cfgr2Spec>;
#[doc = "Register `CH1CFGR2` writer"]
pub type W = crate::W<Ch1cfgr2Spec>;
#[doc = "Field `DTRBS` reader - DTRBS"]
pub type DtrbsR = crate::FieldReader;
#[doc = "Field `DTRBS` writer - DTRBS"]
pub type DtrbsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSET` reader - OFFSET"]
pub type OffsetR = crate::FieldReader<u32>;
#[doc = "Field `OFFSET` writer - OFFSET"]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    pub fn dtrbs(&self) -> DtrbsR {
        DtrbsR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    pub fn dtrbs(&mut self) -> DtrbsW<Ch1cfgr2Spec> {
        DtrbsW::new(self, 3)
    }
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<Ch1cfgr2Spec> {
        OffsetW::new(self, 8)
    }
}
#[doc = "CH1CFGR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1cfgr2Spec;
impl crate::RegisterSpec for Ch1cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1cfgr2::R`](R) reader structure"]
impl crate::Readable for Ch1cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ch1cfgr2::W`](W) writer structure"]
impl crate::Writable for Ch1cfgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1CFGR2 to value 0"]
impl crate::Resettable for Ch1cfgr2Spec {}
