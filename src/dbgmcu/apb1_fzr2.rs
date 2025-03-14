#[doc = "Register `APB1_FZR2` reader"]
pub type R = crate::R<Apb1Fzr2Spec>;
#[doc = "Register `APB1_FZR2` writer"]
pub type W = crate::W<Apb1Fzr2Spec>;
#[doc = "Field `DBG_LPTIM2_STOP` reader - LPTIM2 counter stopped when core is halted"]
pub type DbgLptim2StopR = crate::BitReader;
#[doc = "Field `DBG_LPTIM2_STOP` writer - LPTIM2 counter stopped when core is halted"]
pub type DbgLptim2StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - LPTIM2 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DbgLptim2StopR {
        DbgLptim2StopR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - LPTIM2 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DbgLptim2StopW<Apb1Fzr2Spec> {
        DbgLptim2StopW::new(self, 5)
    }
}
#[doc = "APB Low Freeze Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1_fzr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1_fzr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1Fzr2Spec;
impl crate::RegisterSpec for Apb1Fzr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1_fzr2::R`](R) reader structure"]
impl crate::Readable for Apb1Fzr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apb1_fzr2::W`](W) writer structure"]
impl crate::Writable for Apb1Fzr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1_FZR2 to value 0"]
impl crate::Resettable for Apb1Fzr2Spec {}
