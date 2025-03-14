#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - AES enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - AES enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATYPE` reader - Data type selection (for data in and data out to/from the cryptographic block)"]
pub type DatatypeR = crate::FieldReader;
#[doc = "Field `DATATYPE` writer - Data type selection (for data in and data out to/from the cryptographic block)"]
pub type DatatypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE` reader - AES operating mode"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - AES operating mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHMOD` reader - AES chaining mode"]
pub type ChmodR = crate::FieldReader;
#[doc = "Field `CHMOD` writer - AES chaining mode"]
pub type ChmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CCFC` reader - Computation Complete Flag Clear"]
pub type CcfcR = crate::BitReader;
#[doc = "Field `CCFC` writer - Computation Complete Flag Clear"]
pub type CcfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRC` reader - Error clear"]
pub type ErrcR = crate::BitReader;
#[doc = "Field `ERRC` writer - Error clear"]
pub type ErrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFIE` reader - CCF flag interrupt enable"]
pub type CcfieR = crate::BitReader;
#[doc = "Field `CCFIE` writer - CCF flag interrupt enable"]
pub type CcfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINEN` reader - Enable DMA management of data input phase"]
pub type DmainenR = crate::BitReader;
#[doc = "Field `DMAINEN` writer - Enable DMA management of data input phase"]
pub type DmainenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOUTEN` reader - Enable DMA management of data output phase"]
pub type DmaoutenR = crate::BitReader;
#[doc = "Field `DMAOUTEN` writer - Enable DMA management of data output phase"]
pub type DmaoutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub fn datatype(&self) -> DatatypeR {
        DatatypeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - AES chaining mode"]
    #[inline(always)]
    pub fn chmod(&self) -> ChmodR {
        ChmodR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&self) -> CcfcR {
        CcfcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    pub fn errc(&self) -> ErrcR {
        ErrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    pub fn ccfie(&self) -> CcfieR {
        CcfieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    pub fn dmainen(&self) -> DmainenR {
        DmainenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    pub fn dmaouten(&self) -> DmaoutenR {
        DmaoutenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DatatypeW<CrSpec> {
        DatatypeW::new(self, 1)
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CrSpec> {
        ModeW::new(self, 3)
    }
    #[doc = "Bits 5:6 - AES chaining mode"]
    #[inline(always)]
    pub fn chmod(&mut self) -> ChmodW<CrSpec> {
        ChmodW::new(self, 5)
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&mut self) -> CcfcW<CrSpec> {
        CcfcW::new(self, 7)
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    pub fn errc(&mut self) -> ErrcW<CrSpec> {
        ErrcW::new(self, 8)
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    pub fn ccfie(&mut self) -> CcfieW<CrSpec> {
        CcfieW::new(self, 9)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<CrSpec> {
        ErrieW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    pub fn dmainen(&mut self) -> DmainenW<CrSpec> {
        DmainenW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DmaoutenW<CrSpec> {
        DmaoutenW::new(self, 12)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
