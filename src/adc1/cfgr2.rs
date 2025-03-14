#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<Cfgr2Spec>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<Cfgr2Spec>;
#[doc = "Field `ROVSE` reader - DMAEN"]
pub type RovseR = crate::BitReader;
#[doc = "Field `ROVSE` writer - DMAEN"]
pub type RovseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JOVSE` reader - DMACFG"]
pub type JovseR = crate::BitReader;
#[doc = "Field `JOVSE` writer - DMACFG"]
pub type JovseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSR` reader - RES"]
pub type OvsrR = crate::FieldReader;
#[doc = "Field `OVSR` writer - RES"]
pub type OvsrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OVSS` reader - ALIGN"]
pub type OvssR = crate::FieldReader;
#[doc = "Field `OVSS` writer - ALIGN"]
pub type OvssW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TROVS` reader - Triggered Regular Oversampling"]
pub type TrovsR = crate::BitReader;
#[doc = "Field `TROVS` writer - Triggered Regular Oversampling"]
pub type TrovsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVSM` reader - EXTEN"]
pub type RovsmR = crate::BitReader;
#[doc = "Field `ROVSM` writer - EXTEN"]
pub type RovsmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn rovse(&self) -> RovseR {
        RovseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn jovse(&self) -> JovseR {
        JovseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    pub fn ovsr(&self) -> OvsrR {
        OvsrR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - ALIGN"]
    #[inline(always)]
    pub fn ovss(&self) -> OvssR {
        OvssR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    pub fn trovs(&self) -> TrovsR {
        TrovsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTEN"]
    #[inline(always)]
    pub fn rovsm(&self) -> RovsmR {
        RovsmR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn rovse(&mut self) -> RovseW<Cfgr2Spec> {
        RovseW::new(self, 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn jovse(&mut self) -> JovseW<Cfgr2Spec> {
        JovseW::new(self, 1)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    pub fn ovsr(&mut self) -> OvsrW<Cfgr2Spec> {
        OvsrW::new(self, 2)
    }
    #[doc = "Bits 5:8 - ALIGN"]
    #[inline(always)]
    pub fn ovss(&mut self) -> OvssW<Cfgr2Spec> {
        OvssW::new(self, 5)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    pub fn trovs(&mut self) -> TrovsW<Cfgr2Spec> {
        TrovsW::new(self, 9)
    }
    #[doc = "Bit 10 - EXTEN"]
    #[inline(always)]
    pub fn rovsm(&mut self) -> RovsmW<Cfgr2Spec> {
        RovsmW::new(self, 10)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr2Spec;
impl crate::RegisterSpec for Cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for Cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for Cfgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for Cfgr2Spec {}
