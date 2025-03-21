#[doc = "Register `APB2_FZR` reader"]
pub type R = crate::R<Apb2FzrSpec>;
#[doc = "Register `APB2_FZR` writer"]
pub type W = crate::W<Apb2FzrSpec>;
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 counter stopped when core is halted"]
pub type DbgTim1StopR = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 counter stopped when core is halted"]
pub type DbgTim1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM8_STOP` reader - TIM8 counter stopped when core is halted"]
pub type DbgTim8StopR = crate::BitReader;
#[doc = "Field `DBG_TIM8_STOP` writer - TIM8 counter stopped when core is halted"]
pub type DbgTim8StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM15_STOP` reader - TIM15 counter stopped when core is halted"]
pub type DbgTim15StopR = crate::BitReader;
#[doc = "Field `DBG_TIM15_STOP` writer - TIM15 counter stopped when core is halted"]
pub type DbgTim15StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - TIM16 counter stopped when core is halted"]
pub type DbgTim16StopR = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - TIM16 counter stopped when core is halted"]
pub type DbgTim16StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STOP` reader - TIM17 counter stopped when core is halted"]
pub type DbgTim17StopR = crate::BitReader;
#[doc = "Field `DBG_TIM17_STOP` writer - TIM17 counter stopped when core is halted"]
pub type DbgTim17StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DbgTim1StopR {
        DbgTim1StopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DbgTim8StopR {
        DbgTim8StopR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DbgTim15StopR {
        DbgTim15StopR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DbgTim16StopR {
        DbgTim16StopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DbgTim17StopR {
        DbgTim17StopR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DbgTim1StopW<Apb2FzrSpec> {
        DbgTim1StopW::new(self, 11)
    }
    #[doc = "Bit 13 - TIM8 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&mut self) -> DbgTim8StopW<Apb2FzrSpec> {
        DbgTim8StopW::new(self, 13)
    }
    #[doc = "Bit 16 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DbgTim15StopW<Apb2FzrSpec> {
        DbgTim15StopW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DbgTim16StopW<Apb2FzrSpec> {
        DbgTim16StopW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DbgTim17StopW<Apb2FzrSpec> {
        DbgTim17StopW::new(self, 18)
    }
}
#[doc = "APB High Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2_fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2FzrSpec;
impl crate::RegisterSpec for Apb2FzrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2_fzr::R`](R) reader structure"]
impl crate::Readable for Apb2FzrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2_fzr::W`](W) writer structure"]
impl crate::Writable for Apb2FzrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2_FZR to value 0"]
impl crate::Resettable for Apb2FzrSpec {}
