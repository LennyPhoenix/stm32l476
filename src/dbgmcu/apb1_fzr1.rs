#[doc = "Register `APB1_FZR1` reader"]
pub type R = crate::R<Apb1Fzr1Spec>;
#[doc = "Register `APB1_FZR1` writer"]
pub type W = crate::W<Apb1Fzr1Spec>;
#[doc = "Field `DBG_TIMER2_STOP` reader - Debug Timer 2 stopped when Core is halted"]
pub type DbgTimer2StopR = crate::BitReader;
#[doc = "Field `DBG_TIMER2_STOP` writer - Debug Timer 2 stopped when Core is halted"]
pub type DbgTimer2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - TIM3 counter stopped when core is halted"]
pub type DbgTim3StopR = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - TIM3 counter stopped when core is halted"]
pub type DbgTim3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4_STOP` reader - TIM4 counter stopped when core is halted"]
pub type DbgTim4StopR = crate::BitReader;
#[doc = "Field `DBG_TIM4_STOP` writer - TIM4 counter stopped when core is halted"]
pub type DbgTim4StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5_STOP` reader - TIM5 counter stopped when core is halted"]
pub type DbgTim5StopR = crate::BitReader;
#[doc = "Field `DBG_TIM5_STOP` writer - TIM5 counter stopped when core is halted"]
pub type DbgTim5StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIMER6_STOP` reader - Debug Timer 6 stopped when Core is halted"]
pub type DbgTimer6StopR = crate::BitReader;
#[doc = "Field `DBG_TIMER6_STOP` writer - Debug Timer 6 stopped when Core is halted"]
pub type DbgTimer6StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM7_STOP` reader - TIM7 counter stopped when core is halted"]
pub type DbgTim7StopR = crate::BitReader;
#[doc = "Field `DBG_TIM7_STOP` writer - TIM7 counter stopped when core is halted"]
pub type DbgTim7StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted"]
pub type DbgRtcStopR = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted"]
pub type DbgRtcStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted"]
pub type DbgWwdgStopR = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted"]
pub type DbgWwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted"]
pub type DbgIwdgStopR = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted"]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout mode stopped when core is halted"]
pub type DbgI2c1StopR = crate::BitReader;
#[doc = "Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout mode stopped when core is halted"]
pub type DbgI2c1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout mode stopped when core is halted"]
pub type DbgI2c2StopR = crate::BitReader;
#[doc = "Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout mode stopped when core is halted"]
pub type DbgI2c2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout counter stopped when core is halted"]
pub type DbgI2c3StopR = crate::BitReader;
#[doc = "Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout counter stopped when core is halted"]
pub type DbgI2c3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CAN_STOP` reader - bxCAN stopped when core is halted"]
pub type DbgCanStopR = crate::BitReader;
#[doc = "Field `DBG_CAN_STOP` writer - bxCAN stopped when core is halted"]
pub type DbgCanStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIMER_STOP` reader - LPTIM1 counter stopped when core is halted"]
pub type DbgLptimerStopR = crate::BitReader;
#[doc = "Field `DBG_LPTIMER_STOP` writer - LPTIM1 counter stopped when core is halted"]
pub type DbgLptimerStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DbgTimer2StopR {
        DbgTimer2StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DbgTim3StopR {
        DbgTim3StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DbgTim4StopR {
        DbgTim4StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DbgTim5StopR {
        DbgTim5StopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer6_stop(&self) -> DbgTimer6StopR {
        DbgTimer6StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DbgTim7StopR {
        DbgTim7StopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DbgRtcStopR {
        DbgRtcStopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DbgWwdgStopR {
        DbgWwdgStopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
        DbgIwdgStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DbgI2c1StopR {
        DbgI2c1StopR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DbgI2c2StopR {
        DbgI2c2StopR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DbgI2c3StopR {
        DbgI2c3StopR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - bxCAN stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_can_stop(&self) -> DbgCanStopR {
        DbgCanStopR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptimer_stop(&self) -> DbgLptimerStopR {
        DbgLptimerStopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&mut self) -> DbgTimer2StopW<Apb1Fzr1Spec> {
        DbgTimer2StopW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DbgTim3StopW<Apb1Fzr1Spec> {
        DbgTim3StopW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DbgTim4StopW<Apb1Fzr1Spec> {
        DbgTim4StopW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DbgTim5StopW<Apb1Fzr1Spec> {
        DbgTim5StopW::new(self, 3)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer6_stop(&mut self) -> DbgTimer6StopW<Apb1Fzr1Spec> {
        DbgTimer6StopW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DbgTim7StopW<Apb1Fzr1Spec> {
        DbgTim7StopW::new(self, 5)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DbgRtcStopW<Apb1Fzr1Spec> {
        DbgRtcStopW::new(self, 10)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DbgWwdgStopW<Apb1Fzr1Spec> {
        DbgWwdgStopW::new(self, 11)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<Apb1Fzr1Spec> {
        DbgIwdgStopW::new(self, 12)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DbgI2c1StopW<Apb1Fzr1Spec> {
        DbgI2c1StopW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DbgI2c2StopW<Apb1Fzr1Spec> {
        DbgI2c2StopW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DbgI2c3StopW<Apb1Fzr1Spec> {
        DbgI2c3StopW::new(self, 23)
    }
    #[doc = "Bit 25 - bxCAN stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_can_stop(&mut self) -> DbgCanStopW<Apb1Fzr1Spec> {
        DbgCanStopW::new(self, 25)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptimer_stop(&mut self) -> DbgLptimerStopW<Apb1Fzr1Spec> {
        DbgLptimerStopW::new(self, 31)
    }
}
#[doc = "APB Low Freeze Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1_fzr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1_fzr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1Fzr1Spec;
impl crate::RegisterSpec for Apb1Fzr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1_fzr1::R`](R) reader structure"]
impl crate::Readable for Apb1Fzr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apb1_fzr1::W`](W) writer structure"]
impl crate::Writable for Apb1Fzr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1_FZR1 to value 0"]
impl crate::Resettable for Apb1Fzr1Spec {}
