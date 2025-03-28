#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `ENVR` reader - Voltage reference buffer enable"]
pub type EnvrR = crate::BitReader;
#[doc = "Field `ENVR` writer - Voltage reference buffer enable"]
pub type EnvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIZ` reader - High impedance mode"]
pub type HizR = crate::BitReader;
#[doc = "Field `HIZ` writer - High impedance mode"]
pub type HizW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VRS` reader - Voltage reference scale"]
pub type VrsR = crate::BitReader;
#[doc = "Field `VRS` writer - Voltage reference scale"]
pub type VrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VRR` reader - Voltage reference buffer ready"]
pub type VrrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Voltage reference buffer enable"]
    #[inline(always)]
    pub fn envr(&self) -> EnvrR {
        EnvrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High impedance mode"]
    #[inline(always)]
    pub fn hiz(&self) -> HizR {
        HizR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&self) -> VrsR {
        VrsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VrrR {
        VrrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage reference buffer enable"]
    #[inline(always)]
    pub fn envr(&mut self) -> EnvrW<CsrSpec> {
        EnvrW::new(self, 0)
    }
    #[doc = "Bit 1 - High impedance mode"]
    #[inline(always)]
    pub fn hiz(&mut self) -> HizW<CsrSpec> {
        HizW::new(self, 1)
    }
    #[doc = "Bit 2 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&mut self) -> VrsW<CsrSpec> {
        VrsW::new(self, 2)
    }
}
#[doc = "VREF control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0x02"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0x02;
}
