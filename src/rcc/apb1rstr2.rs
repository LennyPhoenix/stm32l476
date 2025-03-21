#[doc = "Register `APB1RSTR2` reader"]
pub type R = crate::R<Apb1rstr2Spec>;
#[doc = "Register `APB1RSTR2` writer"]
pub type W = crate::W<Apb1rstr2Spec>;
#[doc = "Field `LPUART1RST` reader - Low-power UART 1 reset"]
pub type Lpuart1rstR = crate::BitReader;
#[doc = "Field `LPUART1RST` writer - Low-power UART 1 reset"]
pub type Lpuart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4RST` reader - I2C4 reset"]
pub type I2c4rstR = crate::BitReader;
#[doc = "Field `I2C4RST` writer - I2C4 reset"]
pub type I2c4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPMI1RST` reader - Single wire protocol reset"]
pub type Swpmi1rstR = crate::BitReader;
#[doc = "Field `SWPMI1RST` writer - Single wire protocol reset"]
pub type Swpmi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2RST` reader - Low-power timer 2 reset"]
pub type Lptim2rstR = crate::BitReader;
#[doc = "Field `LPTIM2RST` writer - Low-power timer 2 reset"]
pub type Lptim2rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> Lpuart1rstR {
        Lpuart1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 reset"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2c4rstR {
        I2c4rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single wire protocol reset"]
    #[inline(always)]
    pub fn swpmi1rst(&self) -> Swpmi1rstR {
        Swpmi1rstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> Lptim2rstR {
        Lptim2rstR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> Lpuart1rstW<Apb1rstr2Spec> {
        Lpuart1rstW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C4 reset"]
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2c4rstW<Apb1rstr2Spec> {
        I2c4rstW::new(self, 1)
    }
    #[doc = "Bit 2 - Single wire protocol reset"]
    #[inline(always)]
    pub fn swpmi1rst(&mut self) -> Swpmi1rstW<Apb1rstr2Spec> {
        Swpmi1rstW::new(self, 2)
    }
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> Lptim2rstW<Apb1rstr2Spec> {
        Lptim2rstW::new(self, 5)
    }
}
#[doc = "APB1 peripheral reset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1rstr2Spec;
impl crate::RegisterSpec for Apb1rstr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr2::R`](R) reader structure"]
impl crate::Readable for Apb1rstr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr2::W`](W) writer structure"]
impl crate::Writable for Apb1rstr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1RSTR2 to value 0"]
impl crate::Resettable for Apb1rstr2Spec {}
