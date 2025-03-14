#[doc = "Register `ICSCR` reader"]
pub type R = crate::R<IcscrSpec>;
#[doc = "Register `ICSCR` writer"]
pub type W = crate::W<IcscrSpec>;
#[doc = "Field `MSICAL` reader - MSI clock calibration"]
pub type MsicalR = crate::FieldReader;
#[doc = "Field `MSITRIM` reader - MSI clock trimming"]
pub type MsitrimR = crate::FieldReader;
#[doc = "Field `MSITRIM` writer - MSI clock trimming"]
pub type MsitrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSICAL` reader - HSI clock calibration"]
pub type HsicalR = crate::FieldReader;
#[doc = "Field `HSITRIM` reader - HSI clock trimming"]
pub type HsitrimR = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - HSI clock trimming"]
pub type HsitrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:7 - MSI clock calibration"]
    #[inline(always)]
    pub fn msical(&self) -> MsicalR {
        MsicalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&self) -> MsitrimR {
        MsitrimR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HsicalR {
        HsicalR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HsitrimR {
        HsitrimR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&mut self) -> MsitrimW<IcscrSpec> {
        MsitrimW::new(self, 8)
    }
    #[doc = "Bits 24:30 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HsitrimW<IcscrSpec> {
        HsitrimW::new(self, 24)
    }
}
#[doc = "Internal clock sources calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`icscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcscrSpec;
impl crate::RegisterSpec for IcscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icscr::R`](R) reader structure"]
impl crate::Readable for IcscrSpec {}
#[doc = "`write(|w| ..)` method takes [`icscr::W`](W) writer structure"]
impl crate::Writable for IcscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICSCR to value 0x1000_0000"]
impl crate::Resettable for IcscrSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
