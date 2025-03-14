#[doc = "Register `OR1` reader"]
pub type R = crate::R<Or1Spec>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<Or1Spec>;
#[doc = "Field `ETR_ADC2_RMP` reader - External trigger remap on ADC2 analog watchdog"]
pub type EtrAdc2RmpR = crate::FieldReader;
#[doc = "Field `ETR_ADC2_RMP` writer - External trigger remap on ADC2 analog watchdog"]
pub type EtrAdc2RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ETR_ADC3_RMP` reader - External trigger remap on ADC3 analog watchdog"]
pub type EtrAdc3RmpR = crate::FieldReader;
#[doc = "Field `ETR_ADC3_RMP` writer - External trigger remap on ADC3 analog watchdog"]
pub type EtrAdc3RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TI1_RMP` reader - Input Capture 1 remap"]
pub type Ti1RmpR = crate::BitReader;
#[doc = "Field `TI1_RMP` writer - Input Capture 1 remap"]
pub type Ti1RmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - External trigger remap on ADC2 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc2_rmp(&self) -> EtrAdc2RmpR {
        EtrAdc2RmpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External trigger remap on ADC3 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc3_rmp(&self) -> EtrAdc3RmpR {
        EtrAdc3RmpR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> Ti1RmpR {
        Ti1RmpR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - External trigger remap on ADC2 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc2_rmp(&mut self) -> EtrAdc2RmpW<Or1Spec> {
        EtrAdc2RmpW::new(self, 0)
    }
    #[doc = "Bits 2:3 - External trigger remap on ADC3 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc3_rmp(&mut self) -> EtrAdc3RmpW<Or1Spec> {
        EtrAdc3RmpW::new(self, 2)
    }
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> Ti1RmpW<Or1Spec> {
        Ti1RmpW::new(self, 4)
    }
}
#[doc = "DMA address for full transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Or1Spec;
impl crate::RegisterSpec for Or1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for Or1Spec {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for Or1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for Or1Spec {}
