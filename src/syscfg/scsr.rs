#[doc = "Register `SCSR` reader"]
pub type R = crate::R<ScsrSpec>;
#[doc = "Register `SCSR` writer"]
pub type W = crate::W<ScsrSpec>;
#[doc = "Field `SRAM2ER` reader - SRAM2 Erase"]
pub type Sram2erR = crate::BitReader;
#[doc = "Field `SRAM2ER` writer - SRAM2 Erase"]
pub type Sram2erW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2BSY` reader - SRAM2 busy by erase operation"]
pub type Sram2bsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SRAM2 Erase"]
    #[inline(always)]
    pub fn sram2er(&self) -> Sram2erR {
        Sram2erR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2 busy by erase operation"]
    #[inline(always)]
    pub fn sram2bsy(&self) -> Sram2bsyR {
        Sram2bsyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM2 Erase"]
    #[inline(always)]
    pub fn sram2er(&mut self) -> Sram2erW<ScsrSpec> {
        Sram2erW::new(self, 0)
    }
}
#[doc = "SCSR\n\nYou can [`read`](crate::Reg::read) this register and get [`scsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScsrSpec;
impl crate::RegisterSpec for ScsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scsr::R`](R) reader structure"]
impl crate::Readable for ScsrSpec {}
#[doc = "`write(|w| ..)` method takes [`scsr::W`](W) writer structure"]
impl crate::Writable for ScsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for ScsrSpec {}
