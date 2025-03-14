#[doc = "Register `LCD_CLR` writer"]
pub type W = crate::W<LcdClrSpec>;
#[doc = "Start of frame flag clear This bit is written by software to clear the SOF flag in the LCD_SR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sofc {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clear SOF flag."]
    B0x1 = 1,
}
impl From<Sofc> for bool {
    #[inline(always)]
    fn from(variant: Sofc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFC` writer - Start of frame flag clear This bit is written by software to clear the SOF flag in the LCD_SR register."]
pub type SofcW<'a, REG> = crate::BitWriter<'a, REG, Sofc>;
impl<'a, REG> SofcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sofc::B0x0)
    }
    #[doc = "Clear SOF flag."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sofc::B0x1)
    }
}
#[doc = "Update display done clear This bit is written by software to clear the UDD flag in the LCD_SR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uddc {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clear UDD flag."]
    B0x1 = 1,
}
impl From<Uddc> for bool {
    #[inline(always)]
    fn from(variant: Uddc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDDC` writer - Update display done clear This bit is written by software to clear the UDD flag in the LCD_SR register."]
pub type UddcW<'a, REG> = crate::BitWriter<'a, REG, Uddc>;
impl<'a, REG> UddcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uddc::B0x0)
    }
    #[doc = "Clear UDD flag."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uddc::B0x1)
    }
}
impl W {
    #[doc = "Bit 1 - Start of frame flag clear This bit is written by software to clear the SOF flag in the LCD_SR register."]
    #[inline(always)]
    pub fn sofc(&mut self) -> SofcW<LcdClrSpec> {
        SofcW::new(self, 1)
    }
    #[doc = "Bit 3 - Update display done clear This bit is written by software to clear the UDD flag in the LCD_SR register."]
    #[inline(always)]
    pub fn uddc(&mut self) -> UddcW<LcdClrSpec> {
        UddcW::new(self, 3)
    }
}
#[doc = "LCD clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdClrSpec;
impl crate::RegisterSpec for LcdClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lcd_clr::W`](W) writer structure"]
impl crate::Writable for LcdClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_CLR to value 0"]
impl crate::Resettable for LcdClrSpec {}
