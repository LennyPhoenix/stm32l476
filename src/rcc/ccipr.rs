#[doc = "Register `CCIPR` reader"]
pub type R = crate::R<CciprSpec>;
#[doc = "Register `CCIPR` writer"]
pub type W = crate::W<CciprSpec>;
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub type Usart1selR = crate::FieldReader;
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub type Usart1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART2SEL` reader - USART2 clock source selection"]
pub type Usart2selR = crate::FieldReader;
#[doc = "Field `USART2SEL` writer - USART2 clock source selection"]
pub type Usart2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART3SEL` reader - USART3 clock source selection"]
pub type Usart3selR = crate::FieldReader;
#[doc = "Field `USART3SEL` writer - USART3 clock source selection"]
pub type Usart3selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART4SEL` reader - UART4 clock source selection"]
pub type Uart4selR = crate::FieldReader;
#[doc = "Field `UART4SEL` writer - UART4 clock source selection"]
pub type Uart4selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART5SEL` reader - UART5 clock source selection"]
pub type Uart5selR = crate::FieldReader;
#[doc = "Field `UART5SEL` writer - UART5 clock source selection"]
pub type Uart5selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection"]
pub type Lpuart1selR = crate::FieldReader;
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection"]
pub type Lpuart1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub type I2c1selR = crate::FieldReader;
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2c1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection"]
pub type I2c2selR = crate::FieldReader;
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection"]
pub type I2c2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection"]
pub type I2c3selR = crate::FieldReader;
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection"]
pub type I2c3selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection"]
pub type Lptim1selR = crate::FieldReader;
#[doc = "Field `LPTIM1SEL` writer - Low power timer 1 clock source selection"]
pub type Lptim1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTIM2SEL` reader - Low power timer 2 clock source selection"]
pub type Lptim2selR = crate::FieldReader;
#[doc = "Field `LPTIM2SEL` writer - Low power timer 2 clock source selection"]
pub type Lptim2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAI1SEL` reader - SAI1 clock source selection"]
pub type Sai1selR = crate::FieldReader;
#[doc = "Field `SAI1SEL` writer - SAI1 clock source selection"]
pub type Sai1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAI2SEL` reader - SAI2 clock source selection"]
pub type Sai2selR = crate::FieldReader;
#[doc = "Field `SAI2SEL` writer - SAI2 clock source selection"]
pub type Sai2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK48SEL` reader - 48 MHz clock source selection"]
pub type Clk48selR = crate::FieldReader;
#[doc = "Field `CLK48SEL` writer - 48 MHz clock source selection"]
pub type Clk48selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCSEL` reader - ADCs clock source selection"]
pub type AdcselR = crate::FieldReader;
#[doc = "Field `ADCSEL` writer - ADCs clock source selection"]
pub type AdcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWPMI1SEL` reader - SWPMI1 clock source selection"]
pub type Swpmi1selR = crate::BitReader;
#[doc = "Field `SWPMI1SEL` writer - SWPMI1 clock source selection"]
pub type Swpmi1selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDMSEL` reader - DFSDM clock source selection"]
pub type DfsdmselR = crate::BitReader;
#[doc = "Field `DFSDMSEL` writer - DFSDM clock source selection"]
pub type DfsdmselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> Usart1selR {
        Usart1selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> Usart2selR {
        Usart2selR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&self) -> Usart3selR {
        Usart3selR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&self) -> Uart4selR {
        Uart4selR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&self) -> Uart5selR {
        Uart5selR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> Lpuart1selR {
        Lpuart1selR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2c1selR {
        I2c1selR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2c2selR {
        I2c2selR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2c3selR {
        I2c3selR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> Lptim1selR {
        Lptim1selR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> Lptim2selR {
        Lptim2selR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> Sai1selR {
        Sai1selR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&self) -> Sai2selR {
        Sai2selR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    pub fn clk48sel(&self) -> Clk48selR {
        Clk48selR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> AdcselR {
        AdcselR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SWPMI1 clock source selection"]
    #[inline(always)]
    pub fn swpmi1sel(&self) -> Swpmi1selR {
        Swpmi1selR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DFSDM clock source selection"]
    #[inline(always)]
    pub fn dfsdmsel(&self) -> DfsdmselR {
        DfsdmselR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&mut self) -> Usart1selW<CciprSpec> {
        Usart1selW::new(self, 0)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&mut self) -> Usart2selW<CciprSpec> {
        Usart2selW::new(self, 2)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&mut self) -> Usart3selW<CciprSpec> {
        Usart3selW::new(self, 4)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&mut self) -> Uart4selW<CciprSpec> {
        Uart4selW::new(self, 6)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&mut self) -> Uart5selW<CciprSpec> {
        Uart5selW::new(self, 8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> Lpuart1selW<CciprSpec> {
        Lpuart1selW::new(self, 10)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2c1selW<CciprSpec> {
        I2c1selW::new(self, 12)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2c2selW<CciprSpec> {
        I2c2selW::new(self, 14)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2c3selW<CciprSpec> {
        I2c3selW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> Lptim1selW<CciprSpec> {
        Lptim1selW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> Lptim2selW<CciprSpec> {
        Lptim2selW::new(self, 20)
    }
    #[doc = "Bits 22:23 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&mut self) -> Sai1selW<CciprSpec> {
        Sai1selW::new(self, 22)
    }
    #[doc = "Bits 24:25 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&mut self) -> Sai2selW<CciprSpec> {
        Sai2selW::new(self, 24)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    pub fn clk48sel(&mut self) -> Clk48selW<CciprSpec> {
        Clk48selW::new(self, 26)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&mut self) -> AdcselW<CciprSpec> {
        AdcselW::new(self, 28)
    }
    #[doc = "Bit 30 - SWPMI1 clock source selection"]
    #[inline(always)]
    pub fn swpmi1sel(&mut self) -> Swpmi1selW<CciprSpec> {
        Swpmi1selW::new(self, 30)
    }
    #[doc = "Bit 31 - DFSDM clock source selection"]
    #[inline(always)]
    pub fn dfsdmsel(&mut self) -> DfsdmselW<CciprSpec> {
        DfsdmselW::new(self, 31)
    }
}
#[doc = "CCIPR\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciprSpec;
impl crate::RegisterSpec for CciprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr::R`](R) reader structure"]
impl crate::Readable for CciprSpec {}
#[doc = "`write(|w| ..)` method takes [`ccipr::W`](W) writer structure"]
impl crate::Writable for CciprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CciprSpec {}
