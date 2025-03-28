#[doc = "Register `APB1SMENR1` reader"]
pub type R = crate::R<Apb1smenr1Spec>;
#[doc = "Register `APB1SMENR1` writer"]
pub type W = crate::W<Apb1smenr1Spec>;
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clocks enable during Sleep and Stop modes"]
pub type Tim2smenR = crate::BitReader;
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clocks enable during Sleep and Stop modes"]
pub type Tim2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clocks enable during Sleep and Stop modes"]
pub type Tim3smenR = crate::BitReader;
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clocks enable during Sleep and Stop modes"]
pub type Tim3smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4SMEN` reader - TIM4 timer clocks enable during Sleep and Stop modes"]
pub type Tim4smenR = crate::BitReader;
#[doc = "Field `TIM4SMEN` writer - TIM4 timer clocks enable during Sleep and Stop modes"]
pub type Tim4smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5SMEN` reader - TIM5 timer clocks enable during Sleep and Stop modes"]
pub type Tim5smenR = crate::BitReader;
#[doc = "Field `TIM5SMEN` writer - TIM5 timer clocks enable during Sleep and Stop modes"]
pub type Tim5smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clocks enable during Sleep and Stop modes"]
pub type Tim6smenR = crate::BitReader;
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clocks enable during Sleep and Stop modes"]
pub type Tim6smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clocks enable during Sleep and Stop modes"]
pub type Tim7smenR = crate::BitReader;
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clocks enable during Sleep and Stop modes"]
pub type Tim7smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDSMEN` reader - LCD clocks enable during Sleep and Stop modes"]
pub type LcdsmenR = crate::BitReader;
#[doc = "Field `LCDSMEN` writer - LCD clocks enable during Sleep and Stop modes"]
pub type LcdsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep and Stop modes"]
pub type RtcapbsmenR = crate::BitReader;
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep and Stop modes"]
pub type RtcapbsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGSMEN` reader - Window watchdog clocks enable during Sleep and Stop modes"]
pub type WwdgsmenR = crate::BitReader;
#[doc = "Field `WWDGSMEN` writer - Window watchdog clocks enable during Sleep and Stop modes"]
pub type WwdgsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2SMEN` reader - SPI2 clocks enable during Sleep and Stop modes"]
pub type Spi2smenR = crate::BitReader;
#[doc = "Field `SPI2SMEN` writer - SPI2 clocks enable during Sleep and Stop modes"]
pub type Spi2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes"]
pub type Sp3smenR = crate::BitReader;
#[doc = "Field `SP3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes"]
pub type Sp3smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2SMEN` reader - USART2 clocks enable during Sleep and Stop modes"]
pub type Usart2smenR = crate::BitReader;
#[doc = "Field `USART2SMEN` writer - USART2 clocks enable during Sleep and Stop modes"]
pub type Usart2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3SMEN` reader - USART3 clocks enable during Sleep and Stop modes"]
pub type Usart3smenR = crate::BitReader;
#[doc = "Field `USART3SMEN` writer - USART3 clocks enable during Sleep and Stop modes"]
pub type Usart3smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4SMEN` reader - UART4 clocks enable during Sleep and Stop modes"]
pub type Uart4smenR = crate::BitReader;
#[doc = "Field `UART4SMEN` writer - UART4 clocks enable during Sleep and Stop modes"]
pub type Uart4smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5SMEN` reader - UART5 clocks enable during Sleep and Stop modes"]
pub type Uart5smenR = crate::BitReader;
#[doc = "Field `UART5SMEN` writer - UART5 clocks enable during Sleep and Stop modes"]
pub type Uart5smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes"]
pub type I2c1smenR = crate::BitReader;
#[doc = "Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes"]
pub type I2c1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes"]
pub type I2c2smenR = crate::BitReader;
#[doc = "Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes"]
pub type I2c2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes"]
pub type I2c3smenR = crate::BitReader;
#[doc = "Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes"]
pub type I2c3smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1SMEN` reader - CAN1 clocks enable during Sleep and Stop modes"]
pub type Can1smenR = crate::BitReader;
#[doc = "Field `CAN1SMEN` writer - CAN1 clocks enable during Sleep and Stop modes"]
pub type Can1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2SMEN` reader - CAN2 clocks enable during Sleep and Stop modes"]
pub type Can2smenR = crate::BitReader;
#[doc = "Field `CAN2SMEN` writer - CAN2 clocks enable during Sleep and Stop modes"]
pub type Can2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSMEN` reader - Power interface clocks enable during Sleep and Stop modes"]
pub type PwrsmenR = crate::BitReader;
#[doc = "Field `PWRSMEN` writer - Power interface clocks enable during Sleep and Stop modes"]
pub type PwrsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1SMEN` reader - DAC1 interface clocks enable during Sleep and Stop modes"]
pub type Dac1smenR = crate::BitReader;
#[doc = "Field `DAC1SMEN` writer - DAC1 interface clocks enable during Sleep and Stop modes"]
pub type Dac1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPSMEN` reader - OPAMP interface clocks enable during Sleep and Stop modes"]
pub type OpampsmenR = crate::BitReader;
#[doc = "Field `OPAMPSMEN` writer - OPAMP interface clocks enable during Sleep and Stop modes"]
pub type OpampsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during Sleep and Stop modes"]
pub type Lptim1smenR = crate::BitReader;
#[doc = "Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during Sleep and Stop modes"]
pub type Lptim1smenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim2smen(&self) -> Tim2smenR {
        Tim2smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim3smen(&self) -> Tim3smenR {
        Tim3smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim4smen(&self) -> Tim4smenR {
        Tim4smenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim5smen(&self) -> Tim5smenR {
        Tim5smenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim6smen(&self) -> Tim6smenR {
        Tim6smenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim7smen(&self) -> Tim7smenR {
        Tim7smenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lcdsmen(&self) -> LcdsmenR {
        LcdsmenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RtcapbsmenR {
        RtcapbsmenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WwdgsmenR {
        WwdgsmenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi2smen(&self) -> Spi2smenR {
        Spi2smenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sp3smen(&self) -> Sp3smenR {
        Sp3smenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart2smen(&self) -> Usart2smenR {
        Usart2smenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart3smen(&self) -> Usart3smenR {
        Usart3smenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn uart4smen(&self) -> Uart4smenR {
        Uart4smenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn uart5smen(&self) -> Uart5smenR {
        Uart5smenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2c1smenR {
        I2c1smenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2c2smenR {
        I2c2smenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2c3smenR {
        I2c3smenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn can1smen(&self) -> Can1smenR {
        Can1smenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn can2smen(&self) -> Can2smenR {
        Can2smenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PwrsmenR {
        PwrsmenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dac1smen(&self) -> Dac1smenR {
        Dac1smenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn opampsmen(&self) -> OpampsmenR {
        OpampsmenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> Lptim1smenR {
        Lptim1smenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim2smen(&mut self) -> Tim2smenW<Apb1smenr1Spec> {
        Tim2smenW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim3smen(&mut self) -> Tim3smenW<Apb1smenr1Spec> {
        Tim3smenW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim4smen(&mut self) -> Tim4smenW<Apb1smenr1Spec> {
        Tim4smenW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim5smen(&mut self) -> Tim5smenW<Apb1smenr1Spec> {
        Tim5smenW::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim6smen(&mut self) -> Tim6smenW<Apb1smenr1Spec> {
        Tim6smenW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim7smen(&mut self) -> Tim7smenW<Apb1smenr1Spec> {
        Tim7smenW::new(self, 5)
    }
    #[doc = "Bit 9 - LCD clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lcdsmen(&mut self) -> LcdsmenW<Apb1smenr1Spec> {
        LcdsmenW::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RtcapbsmenW<Apb1smenr1Spec> {
        RtcapbsmenW::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WwdgsmenW<Apb1smenr1Spec> {
        WwdgsmenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi2smen(&mut self) -> Spi2smenW<Apb1smenr1Spec> {
        Spi2smenW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sp3smen(&mut self) -> Sp3smenW<Apb1smenr1Spec> {
        Sp3smenW::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart2smen(&mut self) -> Usart2smenW<Apb1smenr1Spec> {
        Usart2smenW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart3smen(&mut self) -> Usart3smenW<Apb1smenr1Spec> {
        Usart3smenW::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn uart4smen(&mut self) -> Uart4smenW<Apb1smenr1Spec> {
        Uart4smenW::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn uart5smen(&mut self) -> Uart5smenW<Apb1smenr1Spec> {
        Uart5smenW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2c1smenW<Apb1smenr1Spec> {
        I2c1smenW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2c2smenW<Apb1smenr1Spec> {
        I2c2smenW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2c3smenW<Apb1smenr1Spec> {
        I2c3smenW::new(self, 23)
    }
    #[doc = "Bit 25 - CAN1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn can1smen(&mut self) -> Can1smenW<Apb1smenr1Spec> {
        Can1smenW::new(self, 25)
    }
    #[doc = "Bit 26 - CAN2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn can2smen(&mut self) -> Can2smenW<Apb1smenr1Spec> {
        Can2smenW::new(self, 26)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PwrsmenW<Apb1smenr1Spec> {
        PwrsmenW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dac1smen(&mut self) -> Dac1smenW<Apb1smenr1Spec> {
        Dac1smenW::new(self, 29)
    }
    #[doc = "Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn opampsmen(&mut self) -> OpampsmenW<Apb1smenr1Spec> {
        OpampsmenW::new(self, 30)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> Lptim1smenW<Apb1smenr1Spec> {
        Lptim1smenW::new(self, 31)
    }
}
#[doc = "APB1SMENR1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1smenr1Spec;
impl crate::RegisterSpec for Apb1smenr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1smenr1::R`](R) reader structure"]
impl crate::Readable for Apb1smenr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apb1smenr1::W`](W) writer structure"]
impl crate::Writable for Apb1smenr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1SMENR1 to value 0xf2fe_ca3f"]
impl crate::Resettable for Apb1smenr1Spec {
    const RESET_VALUE: u32 = 0xf2fe_ca3f;
}
