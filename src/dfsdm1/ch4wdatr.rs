#[doc = "Register `CH4WDATR` reader"]
pub type R = crate::R<Ch4wdatrSpec>;
#[doc = "Register `CH4WDATR` writer"]
pub type W = crate::W<Ch4wdatrSpec>;
#[doc = "Field `WDATA` reader - WDATA"]
pub type WdataR = crate::FieldReader<u16>;
#[doc = "Field `WDATA` writer - WDATA"]
pub type WdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WdataR {
        WdataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WdataW<Ch4wdatrSpec> {
        WdataW::new(self, 0)
    }
}
#[doc = "CH4WDATR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4wdatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4wdatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4wdatrSpec;
impl crate::RegisterSpec for Ch4wdatrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4wdatr::R`](R) reader structure"]
impl crate::Readable for Ch4wdatrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4wdatr::W`](W) writer structure"]
impl crate::Writable for Ch4wdatrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4WDATR to value 0"]
impl crate::Resettable for Ch4wdatrSpec {}
