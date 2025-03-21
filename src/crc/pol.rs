#[doc = "Register `POL` reader"]
pub type R = crate::R<PolSpec>;
#[doc = "Register `POL` writer"]
pub type W = crate::W<PolSpec>;
#[doc = "Field `Polynomialcoefficients` reader - Programmable polynomial"]
pub type PolynomialcoefficientsR = crate::FieldReader<u32>;
#[doc = "Field `Polynomialcoefficients` writer - Programmable polynomial"]
pub type PolynomialcoefficientsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Programmable polynomial"]
    #[inline(always)]
    pub fn polynomialcoefficients(&self) -> PolynomialcoefficientsR {
        PolynomialcoefficientsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable polynomial"]
    #[inline(always)]
    pub fn polynomialcoefficients(&mut self) -> PolynomialcoefficientsW<PolSpec> {
        PolynomialcoefficientsW::new(self, 0)
    }
}
#[doc = "polynomial\n\nYou can [`read`](crate::Reg::read) this register and get [`pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PolSpec;
impl crate::RegisterSpec for PolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pol::R`](R) reader structure"]
impl crate::Readable for PolSpec {}
#[doc = "`write(|w| ..)` method takes [`pol::W`](W) writer structure"]
impl crate::Writable for PolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POL to value 0x04c1_1db7"]
impl crate::Resettable for PolSpec {
    const RESET_VALUE: u32 = 0x04c1_1db7;
}
