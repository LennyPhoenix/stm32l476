#[doc = "Register `SWPR` writer"]
pub type W = crate::W<SwprSpec>;
#[doc = "Field `P0WP` writer - P0WP"]
pub type P0wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1WP` writer - P1WP"]
pub type P1wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2WP` writer - P2WP"]
pub type P2wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3WP` writer - P3WP"]
pub type P3wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4WP` writer - P4WP"]
pub type P4wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5WP` writer - P5WP"]
pub type P5wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6WP` writer - P6WP"]
pub type P6wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7WP` writer - P7WP"]
pub type P7wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8WP` writer - P8WP"]
pub type P8wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9WP` writer - P9WP"]
pub type P9wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10WP` writer - P10WP"]
pub type P10wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11WP` writer - P11WP"]
pub type P11wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12WP` writer - P12WP"]
pub type P12wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13WP` writer - P13WP"]
pub type P13wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14WP` writer - P14WP"]
pub type P14wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15WP` writer - P15WP"]
pub type P15wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16WP` writer - P16WP"]
pub type P16wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17WP` writer - P17WP"]
pub type P17wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18WP` writer - P18WP"]
pub type P18wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19WP` writer - P19WP"]
pub type P19wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20WP` writer - P20WP"]
pub type P20wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21WP` writer - P21WP"]
pub type P21wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22WP` writer - P22WP"]
pub type P22wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23WP` writer - P23WP"]
pub type P23wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24WP` writer - P24WP"]
pub type P24wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25WP` writer - P25WP"]
pub type P25wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26WP` writer - P26WP"]
pub type P26wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27WP` writer - P27WP"]
pub type P27wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28WP` writer - P28WP"]
pub type P28wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29WP` writer - P29WP"]
pub type P29wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30WP` writer - P30WP"]
pub type P30wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31WP` writer - SRAM2 page 31 write protection"]
pub type P31wpW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - P0WP"]
    #[inline(always)]
    pub fn p0wp(&mut self) -> P0wpW<SwprSpec> {
        P0wpW::new(self, 0)
    }
    #[doc = "Bit 1 - P1WP"]
    #[inline(always)]
    pub fn p1wp(&mut self) -> P1wpW<SwprSpec> {
        P1wpW::new(self, 1)
    }
    #[doc = "Bit 2 - P2WP"]
    #[inline(always)]
    pub fn p2wp(&mut self) -> P2wpW<SwprSpec> {
        P2wpW::new(self, 2)
    }
    #[doc = "Bit 3 - P3WP"]
    #[inline(always)]
    pub fn p3wp(&mut self) -> P3wpW<SwprSpec> {
        P3wpW::new(self, 3)
    }
    #[doc = "Bit 4 - P4WP"]
    #[inline(always)]
    pub fn p4wp(&mut self) -> P4wpW<SwprSpec> {
        P4wpW::new(self, 4)
    }
    #[doc = "Bit 5 - P5WP"]
    #[inline(always)]
    pub fn p5wp(&mut self) -> P5wpW<SwprSpec> {
        P5wpW::new(self, 5)
    }
    #[doc = "Bit 6 - P6WP"]
    #[inline(always)]
    pub fn p6wp(&mut self) -> P6wpW<SwprSpec> {
        P6wpW::new(self, 6)
    }
    #[doc = "Bit 7 - P7WP"]
    #[inline(always)]
    pub fn p7wp(&mut self) -> P7wpW<SwprSpec> {
        P7wpW::new(self, 7)
    }
    #[doc = "Bit 8 - P8WP"]
    #[inline(always)]
    pub fn p8wp(&mut self) -> P8wpW<SwprSpec> {
        P8wpW::new(self, 8)
    }
    #[doc = "Bit 9 - P9WP"]
    #[inline(always)]
    pub fn p9wp(&mut self) -> P9wpW<SwprSpec> {
        P9wpW::new(self, 9)
    }
    #[doc = "Bit 10 - P10WP"]
    #[inline(always)]
    pub fn p10wp(&mut self) -> P10wpW<SwprSpec> {
        P10wpW::new(self, 10)
    }
    #[doc = "Bit 11 - P11WP"]
    #[inline(always)]
    pub fn p11wp(&mut self) -> P11wpW<SwprSpec> {
        P11wpW::new(self, 11)
    }
    #[doc = "Bit 12 - P12WP"]
    #[inline(always)]
    pub fn p12wp(&mut self) -> P12wpW<SwprSpec> {
        P12wpW::new(self, 12)
    }
    #[doc = "Bit 13 - P13WP"]
    #[inline(always)]
    pub fn p13wp(&mut self) -> P13wpW<SwprSpec> {
        P13wpW::new(self, 13)
    }
    #[doc = "Bit 14 - P14WP"]
    #[inline(always)]
    pub fn p14wp(&mut self) -> P14wpW<SwprSpec> {
        P14wpW::new(self, 14)
    }
    #[doc = "Bit 15 - P15WP"]
    #[inline(always)]
    pub fn p15wp(&mut self) -> P15wpW<SwprSpec> {
        P15wpW::new(self, 15)
    }
    #[doc = "Bit 16 - P16WP"]
    #[inline(always)]
    pub fn p16wp(&mut self) -> P16wpW<SwprSpec> {
        P16wpW::new(self, 16)
    }
    #[doc = "Bit 17 - P17WP"]
    #[inline(always)]
    pub fn p17wp(&mut self) -> P17wpW<SwprSpec> {
        P17wpW::new(self, 17)
    }
    #[doc = "Bit 18 - P18WP"]
    #[inline(always)]
    pub fn p18wp(&mut self) -> P18wpW<SwprSpec> {
        P18wpW::new(self, 18)
    }
    #[doc = "Bit 19 - P19WP"]
    #[inline(always)]
    pub fn p19wp(&mut self) -> P19wpW<SwprSpec> {
        P19wpW::new(self, 19)
    }
    #[doc = "Bit 20 - P20WP"]
    #[inline(always)]
    pub fn p20wp(&mut self) -> P20wpW<SwprSpec> {
        P20wpW::new(self, 20)
    }
    #[doc = "Bit 21 - P21WP"]
    #[inline(always)]
    pub fn p21wp(&mut self) -> P21wpW<SwprSpec> {
        P21wpW::new(self, 21)
    }
    #[doc = "Bit 22 - P22WP"]
    #[inline(always)]
    pub fn p22wp(&mut self) -> P22wpW<SwprSpec> {
        P22wpW::new(self, 22)
    }
    #[doc = "Bit 23 - P23WP"]
    #[inline(always)]
    pub fn p23wp(&mut self) -> P23wpW<SwprSpec> {
        P23wpW::new(self, 23)
    }
    #[doc = "Bit 24 - P24WP"]
    #[inline(always)]
    pub fn p24wp(&mut self) -> P24wpW<SwprSpec> {
        P24wpW::new(self, 24)
    }
    #[doc = "Bit 25 - P25WP"]
    #[inline(always)]
    pub fn p25wp(&mut self) -> P25wpW<SwprSpec> {
        P25wpW::new(self, 25)
    }
    #[doc = "Bit 26 - P26WP"]
    #[inline(always)]
    pub fn p26wp(&mut self) -> P26wpW<SwprSpec> {
        P26wpW::new(self, 26)
    }
    #[doc = "Bit 27 - P27WP"]
    #[inline(always)]
    pub fn p27wp(&mut self) -> P27wpW<SwprSpec> {
        P27wpW::new(self, 27)
    }
    #[doc = "Bit 28 - P28WP"]
    #[inline(always)]
    pub fn p28wp(&mut self) -> P28wpW<SwprSpec> {
        P28wpW::new(self, 28)
    }
    #[doc = "Bit 29 - P29WP"]
    #[inline(always)]
    pub fn p29wp(&mut self) -> P29wpW<SwprSpec> {
        P29wpW::new(self, 29)
    }
    #[doc = "Bit 30 - P30WP"]
    #[inline(always)]
    pub fn p30wp(&mut self) -> P30wpW<SwprSpec> {
        P30wpW::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM2 page 31 write protection"]
    #[inline(always)]
    pub fn p31wp(&mut self) -> P31wpW<SwprSpec> {
        P31wpW::new(self, 31)
    }
}
#[doc = "SWPR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwprSpec;
impl crate::RegisterSpec for SwprSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swpr::W`](W) writer structure"]
impl crate::Writable for SwprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWPR to value 0"]
impl crate::Resettable for SwprSpec {}
