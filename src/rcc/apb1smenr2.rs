#[doc = "Register `APB1SMENR2` reader"]
pub type R = crate::R<Apb1smenr2Spec>;
#[doc = "Register `APB1SMENR2` writer"]
pub type W = crate::W<Apb1smenr2Spec>;
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes"]
pub type Lpuart1smenR = crate::BitReader;
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes"]
pub type Lpuart1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes"]
pub type I2c4smenR = crate::BitReader;
#[doc = "Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes"]
pub type I2c4smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPMI1SMEN` reader - Single wire protocol clocks enable during Sleep and Stop modes"]
pub type Swpmi1smenR = crate::BitReader;
#[doc = "Field `SWPMI1SMEN` writer - Single wire protocol clocks enable during Sleep and Stop modes"]
pub type Swpmi1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2SMEN` reader - LPTIM2SMEN"]
pub type Lptim2smenR = crate::BitReader;
#[doc = "Field `LPTIM2SMEN` writer - LPTIM2SMEN"]
pub type Lptim2smenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> Lpuart1smenR {
        Lpuart1smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2c4smenR {
        I2c4smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single wire protocol clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn swpmi1smen(&self) -> Swpmi1smenR {
        Swpmi1smenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2SMEN"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> Lptim2smenR {
        Lptim2smenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> Lpuart1smenW<Apb1smenr2Spec> {
        Lpuart1smenW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c4smen(&mut self) -> I2c4smenW<Apb1smenr2Spec> {
        I2c4smenW::new(self, 1)
    }
    #[doc = "Bit 2 - Single wire protocol clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn swpmi1smen(&mut self) -> Swpmi1smenW<Apb1smenr2Spec> {
        Swpmi1smenW::new(self, 2)
    }
    #[doc = "Bit 5 - LPTIM2SMEN"]
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> Lptim2smenW<Apb1smenr2Spec> {
        Lptim2smenW::new(self, 5)
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1smenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1smenr2Spec;
impl crate::RegisterSpec for Apb1smenr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1smenr2::R`](R) reader structure"]
impl crate::Readable for Apb1smenr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apb1smenr2::W`](W) writer structure"]
impl crate::Writable for Apb1smenr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1SMENR2 to value 0x25"]
impl crate::Resettable for Apb1smenr2Spec {
    const RESET_VALUE: u32 = 0x25;
}
