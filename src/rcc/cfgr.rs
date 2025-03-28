#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `SW` reader - System clock switch"]
pub type SwR = crate::FieldReader;
#[doc = "Field `SW` writer - System clock switch"]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SwsR = crate::FieldReader;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HpreR = crate::FieldReader;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE1` reader - PB low-speed prescaler (APB1)"]
pub type Ppre1R = crate::FieldReader;
#[doc = "Field `PPRE1` writer - PB low-speed prescaler (APB1)"]
pub type Ppre1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PPRE2` reader - APB high-speed prescaler (APB2)"]
pub type Ppre2R = crate::FieldReader;
#[doc = "Field `PPRE2` writer - APB high-speed prescaler (APB2)"]
pub type Ppre2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STOPWUCK` reader - Wakeup from Stop and CSS backup clock selection"]
pub type StopwuckR = crate::BitReader;
#[doc = "Field `STOPWUCK` writer - Wakeup from Stop and CSS backup clock selection"]
pub type StopwuckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCOSEL` reader - Microcontroller clock output"]
pub type McoselR = crate::FieldReader;
#[doc = "Field `MCOSEL` writer - Microcontroller clock output"]
pub type McoselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler"]
pub type McopreR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - PB low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> Ppre1R {
        Ppre1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> Ppre2R {
        Ppre2R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    pub fn stopwuck(&self) -> StopwuckR {
        StopwuckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&self) -> McoselR {
        McoselR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> McopreR {
        McopreR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SwW<CfgrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HpreW<CfgrSpec> {
        HpreW::new(self, 4)
    }
    #[doc = "Bits 8:10 - PB low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> Ppre1W<CfgrSpec> {
        Ppre1W::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> Ppre2W<CfgrSpec> {
        Ppre2W::new(self, 11)
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    pub fn stopwuck(&mut self) -> StopwuckW<CfgrSpec> {
        StopwuckW::new(self, 15)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&mut self) -> McoselW<CfgrSpec> {
        McoselW::new(self, 24)
    }
}
#[doc = "Clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
