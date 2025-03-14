#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `PG` reader - Programming"]
pub type PgR = crate::BitReader;
#[doc = "Field `PG` writer - Programming"]
pub type PgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Page erase"]
pub type PerR = crate::BitReader;
#[doc = "Field `PER` writer - Page erase"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER1` reader - Bank 1 Mass erase"]
pub type Mer1R = crate::BitReader;
#[doc = "Field `MER1` writer - Bank 1 Mass erase"]
pub type Mer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNB` reader - Page number"]
pub type PnbR = crate::FieldReader;
#[doc = "Field `PNB` writer - Page number"]
pub type PnbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BKER` reader - Bank erase"]
pub type BkerR = crate::BitReader;
#[doc = "Field `BKER` writer - Bank erase"]
pub type BkerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER2` reader - Bank 2 Mass erase"]
pub type Mer2R = crate::BitReader;
#[doc = "Field `MER2` writer - Bank 2 Mass erase"]
pub type Mer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTSTRT` reader - Options modification start"]
pub type OptstrtR = crate::BitReader;
#[doc = "Field `OPTSTRT` writer - Options modification start"]
pub type OptstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTPG` reader - Fast programming"]
pub type FstpgR = crate::BitReader;
#[doc = "Field `FSTPG` writer - Fast programming"]
pub type FstpgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EopieR = crate::BitReader;
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EopieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDERRIE` reader - PCROP read error interrupt enable"]
pub type RderrieR = crate::BitReader;
#[doc = "Field `RDERRIE` writer - PCROP read error interrupt enable"]
pub type RderrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBL_LAUNCH` reader - Force the option byte loading"]
pub type OblLaunchR = crate::BitReader;
#[doc = "Field `OBL_LAUNCH` writer - Force the option byte loading"]
pub type OblLaunchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTLOCK` reader - Options Lock"]
pub type OptlockR = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - Options Lock"]
pub type OptlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - FLASH_CR Lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - FLASH_CR Lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PgR {
        PgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 Mass erase"]
    #[inline(always)]
    pub fn mer1(&self) -> Mer1R {
        Mer1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - Page number"]
    #[inline(always)]
    pub fn pnb(&self) -> PnbR {
        PnbR::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Bank erase"]
    #[inline(always)]
    pub fn bker(&self) -> BkerR {
        BkerR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Bank 2 Mass erase"]
    #[inline(always)]
    pub fn mer2(&self) -> Mer2R {
        Mer2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OptstrtR {
        OptstrtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    pub fn fstpg(&self) -> FstpgR {
        FstpgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EopieR {
        EopieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn rderrie(&self) -> RderrieR {
        RderrieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OblLaunchR {
        OblLaunchR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OptlockR {
        OptlockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLASH_CR Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&mut self) -> PgW<CrSpec> {
        PgW::new(self, 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<CrSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 1 Mass erase"]
    #[inline(always)]
    pub fn mer1(&mut self) -> Mer1W<CrSpec> {
        Mer1W::new(self, 2)
    }
    #[doc = "Bits 3:10 - Page number"]
    #[inline(always)]
    pub fn pnb(&mut self) -> PnbW<CrSpec> {
        PnbW::new(self, 3)
    }
    #[doc = "Bit 11 - Bank erase"]
    #[inline(always)]
    pub fn bker(&mut self) -> BkerW<CrSpec> {
        BkerW::new(self, 11)
    }
    #[doc = "Bit 15 - Bank 2 Mass erase"]
    #[inline(always)]
    pub fn mer2(&mut self) -> Mer2W<CrSpec> {
        Mer2W::new(self, 15)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<CrSpec> {
        StartW::new(self, 16)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&mut self) -> OptstrtW<CrSpec> {
        OptstrtW::new(self, 17)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    pub fn fstpg(&mut self) -> FstpgW<CrSpec> {
        FstpgW::new(self, 18)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&mut self) -> EopieW<CrSpec> {
        EopieW::new(self, 24)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<CrSpec> {
        ErrieW::new(self, 25)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn rderrie(&mut self) -> RderrieW<CrSpec> {
        RderrieW::new(self, 26)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OblLaunchW<CrSpec> {
        OblLaunchW::new(self, 27)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OptlockW<CrSpec> {
        OptlockW::new(self, 30)
    }
    #[doc = "Bit 31 - FLASH_CR Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<CrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "Flash control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0xc000_0000"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
