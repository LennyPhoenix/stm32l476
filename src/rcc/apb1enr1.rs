#[doc = "Register `APB1ENR1` reader"]
pub type R = crate::R<Apb1enr1Spec>;
#[doc = "Register `APB1ENR1` writer"]
pub type W = crate::W<Apb1enr1Spec>;
#[doc = "Field `TIM2EN` reader - TIM2 timer clock enable"]
pub type Tim2enR = crate::BitReader;
#[doc = "Field `TIM2EN` writer - TIM2 timer clock enable"]
pub type Tim2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3EN` reader - TIM3 timer clock enable"]
pub type Tim3enR = crate::BitReader;
#[doc = "Field `TIM3EN` writer - TIM3 timer clock enable"]
pub type Tim3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4EN` reader - TIM4 timer clock enable"]
pub type Tim4enR = crate::BitReader;
#[doc = "Field `TIM4EN` writer - TIM4 timer clock enable"]
pub type Tim4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5EN` reader - TIM5 timer clock enable"]
pub type Tim5enR = crate::BitReader;
#[doc = "Field `TIM5EN` writer - TIM5 timer clock enable"]
pub type Tim5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6EN` reader - TIM6 timer clock enable"]
pub type Tim6enR = crate::BitReader;
#[doc = "Field `TIM6EN` writer - TIM6 timer clock enable"]
pub type Tim6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7EN` reader - TIM7 timer clock enable"]
pub type Tim7enR = crate::BitReader;
#[doc = "Field `TIM7EN` writer - TIM7 timer clock enable"]
pub type Tim7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDEN` reader - LCD clock enable"]
pub type LcdenR = crate::BitReader;
#[doc = "Field `LCDEN` writer - LCD clock enable"]
pub type LcdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable"]
pub type RtcapbenR = crate::BitReader;
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable"]
pub type RtcapbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub type WwdgenR = crate::BitReader;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub type WwdgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub type Spi2enR = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub type Spi2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP3EN` reader - SPI3 clock enable"]
pub type Sp3enR = crate::BitReader;
#[doc = "Field `SP3EN` writer - SPI3 clock enable"]
pub type Sp3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub type Usart2enR = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub type Usart2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3EN` reader - USART3 clock enable"]
pub type Usart3enR = crate::BitReader;
#[doc = "Field `USART3EN` writer - USART3 clock enable"]
pub type Usart3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4EN` reader - UART4 clock enable"]
pub type Uart4enR = crate::BitReader;
#[doc = "Field `UART4EN` writer - UART4 clock enable"]
pub type Uart4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5EN` reader - UART5 clock enable"]
pub type Uart5enR = crate::BitReader;
#[doc = "Field `UART5EN` writer - UART5 clock enable"]
pub type Uart5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub type I2c1enR = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
pub type I2c2enR = crate::BitReader;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub type I2c2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3EN` reader - I2C3 clock enable"]
pub type I2c3enR = crate::BitReader;
#[doc = "Field `I2C3EN` writer - I2C3 clock enable"]
pub type I2c3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSEN` reader - Clock Recovery System clock enable"]
pub type CrsenR = crate::BitReader;
#[doc = "Field `CRSEN` writer - Clock Recovery System clock enable"]
pub type CrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1EN` reader - CAN1 clock enable"]
pub type Can1enR = crate::BitReader;
#[doc = "Field `CAN1EN` writer - CAN1 clock enable"]
pub type Can1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2EN` reader - CAN2 clock enable"]
pub type Can2enR = crate::BitReader;
#[doc = "Field `CAN2EN` writer - CAN2 clock enable"]
pub type Can2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub type PwrenR = crate::BitReader;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub type PwrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1EN` reader - DAC1 interface clock enable"]
pub type Dac1enR = crate::BitReader;
#[doc = "Field `DAC1EN` writer - DAC1 interface clock enable"]
pub type Dac1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPEN` reader - OPAMP interface clock enable"]
pub type OpampenR = crate::BitReader;
#[doc = "Field `OPAMPEN` writer - OPAMP interface clock enable"]
pub type OpampenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1EN` reader - Low power timer 1 clock enable"]
pub type Lptim1enR = crate::BitReader;
#[doc = "Field `LPTIM1EN` writer - Low power timer 1 clock enable"]
pub type Lptim1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> Tim2enR {
        Tim2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> Tim3enR {
        Tim3enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> Tim4enR {
        Tim4enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer clock enable"]
    #[inline(always)]
    pub fn tim5en(&self) -> Tim5enR {
        Tim5enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> Tim6enR {
        Tim6enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> Tim7enR {
        Tim7enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clock enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LcdenR {
        LcdenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RtcapbenR {
        RtcapbenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WwdgenR {
        WwdgenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> Spi2enR {
        Spi2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn sp3en(&self) -> Sp3enR {
        Sp3enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> Usart3enR {
        Usart3enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> Uart4enR {
        Uart4enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    pub fn uart5en(&self) -> Uart5enR {
        Uart5enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2c3enR {
        I2c3enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Clock Recovery System clock enable"]
    #[inline(always)]
    pub fn crsen(&self) -> CrsenR {
        CrsenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&self) -> Can1enR {
        Can1enR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2 clock enable"]
    #[inline(always)]
    pub fn can2en(&self) -> Can2enR {
        Can2enR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PwrenR {
        PwrenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable"]
    #[inline(always)]
    pub fn dac1en(&self) -> Dac1enR {
        Dac1enR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP interface clock enable"]
    #[inline(always)]
    pub fn opampen(&self) -> OpampenR {
        OpampenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> Lptim1enR {
        Lptim1enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&mut self) -> Tim2enW<Apb1enr1Spec> {
        Tim2enW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable"]
    #[inline(always)]
    pub fn tim3en(&mut self) -> Tim3enW<Apb1enr1Spec> {
        Tim3enW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable"]
    #[inline(always)]
    pub fn tim4en(&mut self) -> Tim4enW<Apb1enr1Spec> {
        Tim4enW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 timer clock enable"]
    #[inline(always)]
    pub fn tim5en(&mut self) -> Tim5enW<Apb1enr1Spec> {
        Tim5enW::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable"]
    #[inline(always)]
    pub fn tim6en(&mut self) -> Tim6enW<Apb1enr1Spec> {
        Tim6enW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline(always)]
    pub fn tim7en(&mut self) -> Tim7enW<Apb1enr1Spec> {
        Tim7enW::new(self, 5)
    }
    #[doc = "Bit 9 - LCD clock enable"]
    #[inline(always)]
    pub fn lcden(&mut self) -> LcdenW<Apb1enr1Spec> {
        LcdenW::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RtcapbenW<Apb1enr1Spec> {
        RtcapbenW::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WwdgenW<Apb1enr1Spec> {
        WwdgenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> Spi2enW<Apb1enr1Spec> {
        Spi2enW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn sp3en(&mut self) -> Sp3enW<Apb1enr1Spec> {
        Sp3enW::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> Usart2enW<Apb1enr1Spec> {
        Usart2enW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&mut self) -> Usart3enW<Apb1enr1Spec> {
        Usart3enW::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&mut self) -> Uart4enW<Apb1enr1Spec> {
        Uart4enW::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    pub fn uart5en(&mut self) -> Uart5enW<Apb1enr1Spec> {
        Uart5enW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2c1enW<Apb1enr1Spec> {
        I2c1enW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2c2enW<Apb1enr1Spec> {
        I2c2enW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2c3enW<Apb1enr1Spec> {
        I2c3enW::new(self, 23)
    }
    #[doc = "Bit 24 - Clock Recovery System clock enable"]
    #[inline(always)]
    pub fn crsen(&mut self) -> CrsenW<Apb1enr1Spec> {
        CrsenW::new(self, 24)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&mut self) -> Can1enW<Apb1enr1Spec> {
        Can1enW::new(self, 25)
    }
    #[doc = "Bit 26 - CAN2 clock enable"]
    #[inline(always)]
    pub fn can2en(&mut self) -> Can2enW<Apb1enr1Spec> {
        Can2enW::new(self, 26)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PwrenW<Apb1enr1Spec> {
        PwrenW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable"]
    #[inline(always)]
    pub fn dac1en(&mut self) -> Dac1enW<Apb1enr1Spec> {
        Dac1enW::new(self, 29)
    }
    #[doc = "Bit 30 - OPAMP interface clock enable"]
    #[inline(always)]
    pub fn opampen(&mut self) -> OpampenW<Apb1enr1Spec> {
        OpampenW::new(self, 30)
    }
    #[doc = "Bit 31 - Low power timer 1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&mut self) -> Lptim1enW<Apb1enr1Spec> {
        Lptim1enW::new(self, 31)
    }
}
#[doc = "APB1ENR1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enr1Spec;
impl crate::RegisterSpec for Apb1enr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr1::R`](R) reader structure"]
impl crate::Readable for Apb1enr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apb1enr1::W`](W) writer structure"]
impl crate::Writable for Apb1enr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1ENR1 to value 0"]
impl crate::Resettable for Apb1enr1Spec {}
