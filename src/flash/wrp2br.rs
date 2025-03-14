#[doc = "Register `WRP2BR` reader"]
pub type R = crate::R<Wrp2brSpec>;
#[doc = "Register `WRP2BR` writer"]
pub type W = crate::W<Wrp2brSpec>;
#[doc = "Field `WRP2B_STRT` reader - Bank 2 WRP second area B start offset"]
pub type Wrp2bStrtR = crate::FieldReader;
#[doc = "Field `WRP2B_STRT` writer - Bank 2 WRP second area B start offset"]
pub type Wrp2bStrtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRP2B_END` reader - Bank 2 WRP second area B end offset"]
pub type Wrp2bEndR = crate::FieldReader;
#[doc = "Field `WRP2B_END` writer - Bank 2 WRP second area B end offset"]
pub type Wrp2bEndW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bank 2 WRP second area B start offset"]
    #[inline(always)]
    pub fn wrp2b_strt(&self) -> Wrp2bStrtR {
        Wrp2bStrtR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Bank 2 WRP second area B end offset"]
    #[inline(always)]
    pub fn wrp2b_end(&self) -> Wrp2bEndR {
        Wrp2bEndR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 2 WRP second area B start offset"]
    #[inline(always)]
    pub fn wrp2b_strt(&mut self) -> Wrp2bStrtW<Wrp2brSpec> {
        Wrp2bStrtW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Bank 2 WRP second area B end offset"]
    #[inline(always)]
    pub fn wrp2b_end(&mut self) -> Wrp2bEndW<Wrp2brSpec> {
        Wrp2bEndW::new(self, 16)
    }
}
#[doc = "Flash Bank 2 WRP area B address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp2br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrp2brSpec;
impl crate::RegisterSpec for Wrp2brSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp2br::R`](R) reader structure"]
impl crate::Readable for Wrp2brSpec {}
#[doc = "`write(|w| ..)` method takes [`wrp2br::W`](W) writer structure"]
impl crate::Writable for Wrp2brSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRP2BR to value 0xff00_ff00"]
impl crate::Resettable for Wrp2brSpec {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
