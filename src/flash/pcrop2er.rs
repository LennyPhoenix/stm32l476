#[doc = "Register `PCROP2ER` reader"]
pub type R = crate::R<Pcrop2erSpec>;
#[doc = "Register `PCROP2ER` writer"]
pub type W = crate::W<Pcrop2erSpec>;
#[doc = "Field `PCROP2_END` reader - Bank 2 PCROP area end offset"]
pub type Pcrop2EndR = crate::FieldReader<u16>;
#[doc = "Field `PCROP2_END` writer - Bank 2 PCROP area end offset"]
pub type Pcrop2EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Bank 2 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop2_end(&self) -> Pcrop2EndR {
        Pcrop2EndR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank 2 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop2_end(&mut self) -> Pcrop2EndW<Pcrop2erSpec> {
        Pcrop2EndW::new(self, 0)
    }
}
#[doc = "Flash Bank 2 PCROP End address register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop2er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcrop2erSpec;
impl crate::RegisterSpec for Pcrop2erSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop2er::R`](R) reader structure"]
impl crate::Readable for Pcrop2erSpec {}
#[doc = "`write(|w| ..)` method takes [`pcrop2er::W`](W) writer structure"]
impl crate::Writable for Pcrop2erSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCROP2ER to value 0xffff_0000"]
impl crate::Resettable for Pcrop2erSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
