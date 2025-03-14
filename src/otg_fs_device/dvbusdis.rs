#[doc = "Register `DVBUSDIS` reader"]
pub type R = crate::R<DvbusdisSpec>;
#[doc = "Register `DVBUSDIS` writer"]
pub type W = crate::W<DvbusdisSpec>;
#[doc = "Field `VBUSDT` reader - Device VBUS discharge time"]
pub type VbusdtR = crate::FieldReader<u16>;
#[doc = "Field `VBUSDT` writer - Device VBUS discharge time"]
pub type VbusdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Device VBUS discharge time"]
    #[inline(always)]
    pub fn vbusdt(&self) -> VbusdtR {
        VbusdtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device VBUS discharge time"]
    #[inline(always)]
    pub fn vbusdt(&mut self) -> VbusdtW<DvbusdisSpec> {
        VbusdtW::new(self, 0)
    }
}
#[doc = "OTG_FS device VBUS discharge time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbusdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvbusdisSpec;
impl crate::RegisterSpec for DvbusdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvbusdis::R`](R) reader structure"]
impl crate::Readable for DvbusdisSpec {}
#[doc = "`write(|w| ..)` method takes [`dvbusdis::W`](W) writer structure"]
impl crate::Writable for DvbusdisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVBUSDIS to value 0x17d7"]
impl crate::Resettable for DvbusdisSpec {
    const RESET_VALUE: u32 = 0x17d7;
}
