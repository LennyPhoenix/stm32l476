#[doc = "Register `HAINTMSK` reader"]
pub type R = crate::R<HaintmskSpec>;
#[doc = "Register `HAINTMSK` writer"]
pub type W = crate::W<HaintmskSpec>;
#[doc = "Field `HAINTM` reader - Channel interrupt mask"]
pub type HaintmR = crate::FieldReader<u16>;
#[doc = "Field `HAINTM` writer - Channel interrupt mask"]
pub type HaintmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel interrupt mask"]
    #[inline(always)]
    pub fn haintm(&self) -> HaintmR {
        HaintmR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel interrupt mask"]
    #[inline(always)]
    pub fn haintm(&mut self) -> HaintmW<HaintmskSpec> {
        HaintmW::new(self, 0)
    }
}
#[doc = "OTG_FS host all channels interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`haintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HaintmskSpec;
impl crate::RegisterSpec for HaintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haintmsk::R`](R) reader structure"]
impl crate::Readable for HaintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`haintmsk::W`](W) writer structure"]
impl crate::Writable for HaintmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HAINTMSK to value 0"]
impl crate::Resettable for HaintmskSpec {}
