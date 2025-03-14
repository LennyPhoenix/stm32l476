#[doc = "Register `APB1RSTR1` reader"]
pub type R = crate::R<Apb1rstr1Spec>;
#[doc = "Register `APB1RSTR1` writer"]
pub type W = crate::W<Apb1rstr1Spec>;
#[doc = "Field `TIM2RST` reader - TIM2 timer reset"]
pub type Tim2rstR = crate::BitReader;
#[doc = "Field `TIM2RST` writer - TIM2 timer reset"]
pub type Tim2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3RST` reader - TIM3 timer reset"]
pub type Tim3rstR = crate::BitReader;
#[doc = "Field `TIM3RST` writer - TIM3 timer reset"]
pub type Tim3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4RST` reader - TIM3 timer reset"]
pub type Tim4rstR = crate::BitReader;
#[doc = "Field `TIM4RST` writer - TIM3 timer reset"]
pub type Tim4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5RST` reader - TIM5 timer reset"]
pub type Tim5rstR = crate::BitReader;
#[doc = "Field `TIM5RST` writer - TIM5 timer reset"]
pub type Tim5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6RST` reader - TIM6 timer reset"]
pub type Tim6rstR = crate::BitReader;
#[doc = "Field `TIM6RST` writer - TIM6 timer reset"]
pub type Tim6rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7RST` reader - TIM7 timer reset"]
pub type Tim7rstR = crate::BitReader;
#[doc = "Field `TIM7RST` writer - TIM7 timer reset"]
pub type Tim7rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDRST` reader - LCD interface reset"]
pub type LcdrstR = crate::BitReader;
#[doc = "Field `LCDRST` writer - LCD interface reset"]
pub type LcdrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type Spi2rstR = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type Spi2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3RST` reader - SPI3 reset"]
pub type Spi3rstR = crate::BitReader;
#[doc = "Field `SPI3RST` writer - SPI3 reset"]
pub type Spi3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub type Usart2rstR = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub type Usart2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3RST` reader - USART3 reset"]
pub type Usart3rstR = crate::BitReader;
#[doc = "Field `USART3RST` writer - USART3 reset"]
pub type Usart3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4RST` reader - UART4 reset"]
pub type Uart4rstR = crate::BitReader;
#[doc = "Field `UART4RST` writer - UART4 reset"]
pub type Uart4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5RST` reader - UART5 reset"]
pub type Uart5rstR = crate::BitReader;
#[doc = "Field `UART5RST` writer - UART5 reset"]
pub type Uart5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2c1rstR = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2c2rstR = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2c2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3RST` reader - I2C3 reset"]
pub type I2c3rstR = crate::BitReader;
#[doc = "Field `I2C3RST` writer - I2C3 reset"]
pub type I2c3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSRST` reader - CRS reset"]
pub type CrsrstR = crate::BitReader;
#[doc = "Field `CRSRST` writer - CRS reset"]
pub type CrsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1RST` reader - CAN1 reset"]
pub type Can1rstR = crate::BitReader;
#[doc = "Field `CAN1RST` writer - CAN1 reset"]
pub type Can1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2RST` reader - CAN2 reset"]
pub type Can2rstR = crate::BitReader;
#[doc = "Field `CAN2RST` writer - CAN2 reset"]
pub type Can2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PwrrstR = crate::BitReader;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PwrrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1RST` reader - DAC1 interface reset"]
pub type Dac1rstR = crate::BitReader;
#[doc = "Field `DAC1RST` writer - DAC1 interface reset"]
pub type Dac1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPRST` reader - OPAMP interface reset"]
pub type OpamprstR = crate::BitReader;
#[doc = "Field `OPAMPRST` writer - OPAMP interface reset"]
pub type OpamprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1RST` reader - Low Power Timer 1 reset"]
pub type Lptim1rstR = crate::BitReader;
#[doc = "Field `LPTIM1RST` writer - Low Power Timer 1 reset"]
pub type Lptim1rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> Tim2rstR {
        Tim2rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> Tim3rstR {
        Tim3rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> Tim4rstR {
        Tim4rstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> Tim5rstR {
        Tim5rstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> Tim6rstR {
        Tim6rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> Tim7rstR {
        Tim7rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD interface reset"]
    #[inline(always)]
    pub fn lcdrst(&self) -> LcdrstR {
        LcdrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> Spi2rstR {
        Spi2rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> Spi3rstR {
        Spi3rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> Usart2rstR {
        Usart2rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> Usart3rstR {
        Usart3rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> Uart4rstR {
        Uart4rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 reset"]
    #[inline(always)]
    pub fn uart5rst(&self) -> Uart5rstR {
        Uart5rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2c2rstR {
        I2c2rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2c3rstR {
        I2c3rstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS reset"]
    #[inline(always)]
    pub fn crsrst(&self) -> CrsrstR {
        CrsrstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&self) -> Can1rstR {
        Can1rstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline(always)]
    pub fn can2rst(&self) -> Can2rstR {
        Can2rstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PwrrstR {
        PwrrstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface reset"]
    #[inline(always)]
    pub fn dac1rst(&self) -> Dac1rstR {
        Dac1rstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP interface reset"]
    #[inline(always)]
    pub fn opamprst(&self) -> OpamprstR {
        OpamprstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> Lptim1rstR {
        Lptim1rstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> Tim2rstW<Apb1rstr1Spec> {
        Tim2rstW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim3rst(&mut self) -> Tim3rstW<Apb1rstr1Spec> {
        Tim3rstW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim4rst(&mut self) -> Tim4rstW<Apb1rstr1Spec> {
        Tim4rstW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 timer reset"]
    #[inline(always)]
    pub fn tim5rst(&mut self) -> Tim5rstW<Apb1rstr1Spec> {
        Tim5rstW::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 timer reset"]
    #[inline(always)]
    pub fn tim6rst(&mut self) -> Tim6rstW<Apb1rstr1Spec> {
        Tim6rstW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline(always)]
    pub fn tim7rst(&mut self) -> Tim7rstW<Apb1rstr1Spec> {
        Tim7rstW::new(self, 5)
    }
    #[doc = "Bit 9 - LCD interface reset"]
    #[inline(always)]
    pub fn lcdrst(&mut self) -> LcdrstW<Apb1rstr1Spec> {
        LcdrstW::new(self, 9)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> Spi2rstW<Apb1rstr1Spec> {
        Spi2rstW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> Spi3rstW<Apb1rstr1Spec> {
        Spi3rstW::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> Usart2rstW<Apb1rstr1Spec> {
        Usart2rstW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&mut self) -> Usart3rstW<Apb1rstr1Spec> {
        Usart3rstW::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 reset"]
    #[inline(always)]
    pub fn uart4rst(&mut self) -> Uart4rstW<Apb1rstr1Spec> {
        Uart4rstW::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 reset"]
    #[inline(always)]
    pub fn uart5rst(&mut self) -> Uart5rstW<Apb1rstr1Spec> {
        Uart5rstW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2c1rstW<Apb1rstr1Spec> {
        I2c1rstW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2c2rstW<Apb1rstr1Spec> {
        I2c2rstW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2c3rstW<Apb1rstr1Spec> {
        I2c3rstW::new(self, 23)
    }
    #[doc = "Bit 24 - CRS reset"]
    #[inline(always)]
    pub fn crsrst(&mut self) -> CrsrstW<Apb1rstr1Spec> {
        CrsrstW::new(self, 24)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&mut self) -> Can1rstW<Apb1rstr1Spec> {
        Can1rstW::new(self, 25)
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline(always)]
    pub fn can2rst(&mut self) -> Can2rstW<Apb1rstr1Spec> {
        Can2rstW::new(self, 26)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PwrrstW<Apb1rstr1Spec> {
        PwrrstW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface reset"]
    #[inline(always)]
    pub fn dac1rst(&mut self) -> Dac1rstW<Apb1rstr1Spec> {
        Dac1rstW::new(self, 29)
    }
    #[doc = "Bit 30 - OPAMP interface reset"]
    #[inline(always)]
    pub fn opamprst(&mut self) -> OpamprstW<Apb1rstr1Spec> {
        OpamprstW::new(self, 30)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> Lptim1rstW<Apb1rstr1Spec> {
        Lptim1rstW::new(self, 31)
    }
}
#[doc = "APB1 peripheral reset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1rstr1Spec;
impl crate::RegisterSpec for Apb1rstr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr1::R`](R) reader structure"]
impl crate::Readable for Apb1rstr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr1::W`](W) writer structure"]
impl crate::Writable for Apb1rstr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1RSTR1 to value 0"]
impl crate::Resettable for Apb1rstr1Spec {}
