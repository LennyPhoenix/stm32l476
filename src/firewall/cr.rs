#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `FPA` reader - Firewall pre alarm"]
pub type FpaR = crate::BitReader;
#[doc = "Field `FPA` writer - Firewall pre alarm"]
pub type FpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDS` reader - Volatile data shared"]
pub type VdsR = crate::BitReader;
#[doc = "Field `VDS` writer - Volatile data shared"]
pub type VdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDE` reader - Volatile data execution"]
pub type VdeR = crate::BitReader;
#[doc = "Field `VDE` writer - Volatile data execution"]
pub type VdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Firewall pre alarm"]
    #[inline(always)]
    pub fn fpa(&self) -> FpaR {
        FpaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Volatile data shared"]
    #[inline(always)]
    pub fn vds(&self) -> VdsR {
        VdsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Volatile data execution"]
    #[inline(always)]
    pub fn vde(&self) -> VdeR {
        VdeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Firewall pre alarm"]
    #[inline(always)]
    pub fn fpa(&mut self) -> FpaW<CrSpec> {
        FpaW::new(self, 0)
    }
    #[doc = "Bit 1 - Volatile data shared"]
    #[inline(always)]
    pub fn vds(&mut self) -> VdsW<CrSpec> {
        VdsW::new(self, 1)
    }
    #[doc = "Bit 2 - Volatile data execution"]
    #[inline(always)]
    pub fn vde(&mut self) -> VdeW<CrSpec> {
        VdeW::new(self, 2)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
