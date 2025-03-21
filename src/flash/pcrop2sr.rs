#[doc = "Register `PCROP2SR` reader"]
pub type R = crate::R<Pcrop2srSpec>;
#[doc = "Register `PCROP2SR` writer"]
pub type W = crate::W<Pcrop2srSpec>;
#[doc = "Field `PCROP2_STRT` reader - Bank 2 PCROP area start offset"]
pub type Pcrop2StrtR = crate::FieldReader<u16>;
#[doc = "Field `PCROP2_STRT` writer - Bank 2 PCROP area start offset"]
pub type Pcrop2StrtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Bank 2 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop2_strt(&self) -> Pcrop2StrtR {
        Pcrop2StrtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank 2 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop2_strt(&mut self) -> Pcrop2StrtW<Pcrop2srSpec> {
        Pcrop2StrtW::new(self, 0)
    }
}
#[doc = "Flash Bank 2 PCROP Start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop2sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcrop2srSpec;
impl crate::RegisterSpec for Pcrop2srSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop2sr::R`](R) reader structure"]
impl crate::Readable for Pcrop2srSpec {}
#[doc = "`write(|w| ..)` method takes [`pcrop2sr::W`](W) writer structure"]
impl crate::Writable for Pcrop2srSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCROP2SR to value 0xffff_0000"]
impl crate::Resettable for Pcrop2srSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
