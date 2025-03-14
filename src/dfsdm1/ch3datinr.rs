#[doc = "Register `CH3DATINR` reader"]
pub type R = crate::R<Ch3datinrSpec>;
#[doc = "Register `CH3DATINR` writer"]
pub type W = crate::W<Ch3datinrSpec>;
#[doc = "Field `INDAT0` reader - INDAT0"]
pub type Indat0R = crate::FieldReader<u16>;
#[doc = "Field `INDAT0` writer - INDAT0"]
pub type Indat0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INDAT1` reader - INDAT1"]
pub type Indat1R = crate::FieldReader<u16>;
#[doc = "Field `INDAT1` writer - INDAT1"]
pub type Indat1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - INDAT0"]
    #[inline(always)]
    pub fn indat0(&self) -> Indat0R {
        Indat0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - INDAT1"]
    #[inline(always)]
    pub fn indat1(&self) -> Indat1R {
        Indat1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - INDAT0"]
    #[inline(always)]
    pub fn indat0(&mut self) -> Indat0W<Ch3datinrSpec> {
        Indat0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - INDAT1"]
    #[inline(always)]
    pub fn indat1(&mut self) -> Indat1W<Ch3datinrSpec> {
        Indat1W::new(self, 16)
    }
}
#[doc = "CH3DATINR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3datinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3datinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3datinrSpec;
impl crate::RegisterSpec for Ch3datinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3datinr::R`](R) reader structure"]
impl crate::Readable for Ch3datinrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3datinr::W`](W) writer structure"]
impl crate::Writable for Ch3datinrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3DATINR to value 0"]
impl crate::Resettable for Ch3datinrSpec {}
