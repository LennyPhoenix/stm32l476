#[doc = "Register `RXCRCR` reader"]
pub type R = crate::R<RxcrcrSpec>;
#[doc = "Field `RxCRC` reader - Rx CRC register"]
pub type RxCrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Rx CRC register"]
    #[inline(always)]
    pub fn rx_crc(&self) -> RxCrcR {
        RxCrcR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RX CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcrcrSpec;
impl crate::RegisterSpec for RxcrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrcr::R`](R) reader structure"]
impl crate::Readable for RxcrcrSpec {}
#[doc = "`reset()` method sets RXCRCR to value 0"]
impl crate::Resettable for RxcrcrSpec {}
