#[doc = "Register `IMR2` reader"]
pub type R = crate::R<Imr2Spec>;
#[doc = "Register `IMR2` writer"]
pub type W = crate::W<Imr2Spec>;
#[doc = "Field `MR32` reader - Interrupt Mask on external/internal line 32"]
pub type Mr32R = crate::BitReader;
#[doc = "Field `MR32` writer - Interrupt Mask on external/internal line 32"]
pub type Mr32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR33` reader - Interrupt Mask on external/internal line 33"]
pub type Mr33R = crate::BitReader;
#[doc = "Field `MR33` writer - Interrupt Mask on external/internal line 33"]
pub type Mr33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR34` reader - Interrupt Mask on external/internal line 34"]
pub type Mr34R = crate::BitReader;
#[doc = "Field `MR34` writer - Interrupt Mask on external/internal line 34"]
pub type Mr34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR35` reader - Interrupt Mask on external/internal line 35"]
pub type Mr35R = crate::BitReader;
#[doc = "Field `MR35` writer - Interrupt Mask on external/internal line 35"]
pub type Mr35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR36` reader - Interrupt Mask on external/internal line 36"]
pub type Mr36R = crate::BitReader;
#[doc = "Field `MR36` writer - Interrupt Mask on external/internal line 36"]
pub type Mr36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR37` reader - Interrupt Mask on external/internal line 37"]
pub type Mr37R = crate::BitReader;
#[doc = "Field `MR37` writer - Interrupt Mask on external/internal line 37"]
pub type Mr37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR38` reader - Interrupt Mask on external/internal line 38"]
pub type Mr38R = crate::BitReader;
#[doc = "Field `MR38` writer - Interrupt Mask on external/internal line 38"]
pub type Mr38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR39` reader - Interrupt Mask on external/internal line 39"]
pub type Mr39R = crate::BitReader;
#[doc = "Field `MR39` writer - Interrupt Mask on external/internal line 39"]
pub type Mr39W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    pub fn mr32(&self) -> Mr32R {
        Mr32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    pub fn mr33(&self) -> Mr33R {
        Mr33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    pub fn mr34(&self) -> Mr34R {
        Mr34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    pub fn mr35(&self) -> Mr35R {
        Mr35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Mask on external/internal line 36"]
    #[inline(always)]
    pub fn mr36(&self) -> Mr36R {
        Mr36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask on external/internal line 37"]
    #[inline(always)]
    pub fn mr37(&self) -> Mr37R {
        Mr37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Mask on external/internal line 38"]
    #[inline(always)]
    pub fn mr38(&self) -> Mr38R {
        Mr38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Mask on external/internal line 39"]
    #[inline(always)]
    pub fn mr39(&self) -> Mr39R {
        Mr39R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    pub fn mr32(&mut self) -> Mr32W<Imr2Spec> {
        Mr32W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    pub fn mr33(&mut self) -> Mr33W<Imr2Spec> {
        Mr33W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    pub fn mr34(&mut self) -> Mr34W<Imr2Spec> {
        Mr34W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    pub fn mr35(&mut self) -> Mr35W<Imr2Spec> {
        Mr35W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Mask on external/internal line 36"]
    #[inline(always)]
    pub fn mr36(&mut self) -> Mr36W<Imr2Spec> {
        Mr36W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Mask on external/internal line 37"]
    #[inline(always)]
    pub fn mr37(&mut self) -> Mr37W<Imr2Spec> {
        Mr37W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Mask on external/internal line 38"]
    #[inline(always)]
    pub fn mr38(&mut self) -> Mr38W<Imr2Spec> {
        Mr38W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Mask on external/internal line 39"]
    #[inline(always)]
    pub fn mr39(&mut self) -> Mr39W<Imr2Spec> {
        Mr39W::new(self, 7)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imr2Spec;
impl crate::RegisterSpec for Imr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for Imr2Spec {}
#[doc = "`write(|w| ..)` method takes [`imr2::W`](W) writer structure"]
impl crate::Writable for Imr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR2 to value 0xffff_ff87"]
impl crate::Resettable for Imr2Spec {
    const RESET_VALUE: u32 = 0xffff_ff87;
}
