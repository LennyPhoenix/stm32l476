#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `MODE1` reader - DAC Channel 1 mode"]
pub type Mode1R = crate::FieldReader;
#[doc = "Field `MODE1` writer - DAC Channel 1 mode"]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODE2` reader - DAC Channel 2 mode"]
pub type Mode2R = crate::FieldReader;
#[doc = "Field `MODE2` writer - DAC Channel 2 mode"]
pub type Mode2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - DAC Channel 1 mode"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode"]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC Channel 1 mode"]
    #[inline(always)]
    pub fn mode1(&mut self) -> Mode1W<McrSpec> {
        Mode1W::new(self, 0)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode"]
    #[inline(always)]
    pub fn mode2(&mut self) -> Mode2W<McrSpec> {
        Mode2W::new(self, 16)
    }
}
#[doc = "mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {}
