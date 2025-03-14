#[doc = "Register `WRP2AR` reader"]
pub type R = crate::R<Wrp2arSpec>;
#[doc = "Register `WRP2AR` writer"]
pub type W = crate::W<Wrp2arSpec>;
#[doc = "Field `WRP2A_STRT` reader - Bank 2 WRP first area A start offset"]
pub type Wrp2aStrtR = crate::FieldReader;
#[doc = "Field `WRP2A_STRT` writer - Bank 2 WRP first area A start offset"]
pub type Wrp2aStrtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRP2A_END` reader - Bank 2 WRP first area A end offset"]
pub type Wrp2aEndR = crate::FieldReader;
#[doc = "Field `WRP2A_END` writer - Bank 2 WRP first area A end offset"]
pub type Wrp2aEndW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bank 2 WRP first area A start offset"]
    #[inline(always)]
    pub fn wrp2a_strt(&self) -> Wrp2aStrtR {
        Wrp2aStrtR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Bank 2 WRP first area A end offset"]
    #[inline(always)]
    pub fn wrp2a_end(&self) -> Wrp2aEndR {
        Wrp2aEndR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 2 WRP first area A start offset"]
    #[inline(always)]
    pub fn wrp2a_strt(&mut self) -> Wrp2aStrtW<Wrp2arSpec> {
        Wrp2aStrtW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Bank 2 WRP first area A end offset"]
    #[inline(always)]
    pub fn wrp2a_end(&mut self) -> Wrp2aEndW<Wrp2arSpec> {
        Wrp2aEndW::new(self, 16)
    }
}
#[doc = "Flash Bank 2 WRP area A address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrp2arSpec;
impl crate::RegisterSpec for Wrp2arSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp2ar::R`](R) reader structure"]
impl crate::Readable for Wrp2arSpec {}
#[doc = "`write(|w| ..)` method takes [`wrp2ar::W`](W) writer structure"]
impl crate::Writable for Wrp2arSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRP2AR to value 0xff00_ff00"]
impl crate::Resettable for Wrp2arSpec {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
