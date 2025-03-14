#[doc = "Register `FIFO` reader"]
pub type R = crate::R<FifoSpec>;
#[doc = "Register `FIFO` writer"]
pub type W = crate::W<FifoSpec>;
#[doc = "Field `FIFOData` reader - Receive and transmit FIFO data"]
pub type FifodataR = crate::FieldReader<u32>;
#[doc = "Field `FIFOData` writer - Receive and transmit FIFO data"]
pub type FifodataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data"]
    #[inline(always)]
    pub fn fifodata(&self) -> FifodataR {
        FifodataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data"]
    #[inline(always)]
    pub fn fifodata(&mut self) -> FifodataW<FifoSpec> {
        FifodataW::new(self, 0)
    }
}
#[doc = "data FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoSpec;
impl crate::RegisterSpec for FifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo::W`](W) writer structure"]
impl crate::Writable for FifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FifoSpec {}
