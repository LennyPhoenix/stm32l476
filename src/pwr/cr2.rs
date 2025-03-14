#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PvdeR = crate::BitReader;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PvdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - Power voltage detector level selection"]
pub type PlsR = crate::FieldReader;
#[doc = "Field `PLS` writer - Power voltage detector level selection"]
pub type PlsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PVME1` reader - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
pub type Pvme1R = crate::BitReader;
#[doc = "Field `PVME1` writer - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
pub type Pvme1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVME2` reader - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
pub type Pvme2R = crate::BitReader;
#[doc = "Field `PVME2` writer - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
pub type Pvme2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVME3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type Pvme3R = crate::BitReader;
#[doc = "Field `PVME3` writer - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type Pvme3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVME4` reader - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
pub type Pvme4R = crate::BitReader;
#[doc = "Field `PVME4` writer - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
pub type Pvme4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSV` reader - VDDIO2 Independent I/Os supply valid"]
pub type IosvR = crate::BitReader;
#[doc = "Field `IOSV` writer - VDDIO2 Independent I/Os supply valid"]
pub type IosvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USV` reader - VDDUSB USB supply valid"]
pub type UsvR = crate::BitReader;
#[doc = "Field `USV` writer - VDDUSB USB supply valid"]
pub type UsvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PvdeR {
        PvdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PlsR {
        PlsR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline(always)]
    pub fn pvme1(&self) -> Pvme1R {
        Pvme1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
    #[inline(always)]
    pub fn pvme2(&self) -> Pvme2R {
        Pvme2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&self) -> Pvme3R {
        Pvme3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
    #[inline(always)]
    pub fn pvme4(&self) -> Pvme4R {
        Pvme4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDIO2 Independent I/Os supply valid"]
    #[inline(always)]
    pub fn iosv(&self) -> IosvR {
        IosvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDUSB USB supply valid"]
    #[inline(always)]
    pub fn usv(&self) -> UsvR {
        UsvR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PvdeW<Cr2Spec> {
        PvdeW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    pub fn pls(&mut self) -> PlsW<Cr2Spec> {
        PlsW::new(self, 1)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline(always)]
    pub fn pvme1(&mut self) -> Pvme1W<Cr2Spec> {
        Pvme1W::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
    #[inline(always)]
    pub fn pvme2(&mut self) -> Pvme2W<Cr2Spec> {
        Pvme2W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&mut self) -> Pvme3W<Cr2Spec> {
        Pvme3W::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
    #[inline(always)]
    pub fn pvme4(&mut self) -> Pvme4W<Cr2Spec> {
        Pvme4W::new(self, 7)
    }
    #[doc = "Bit 9 - VDDIO2 Independent I/Os supply valid"]
    #[inline(always)]
    pub fn iosv(&mut self) -> IosvW<Cr2Spec> {
        IosvW::new(self, 9)
    }
    #[doc = "Bit 10 - VDDUSB USB supply valid"]
    #[inline(always)]
    pub fn usv(&mut self) -> UsvW<Cr2Spec> {
        UsvW::new(self, 10)
    }
}
#[doc = "Power control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
