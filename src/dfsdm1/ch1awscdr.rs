# [doc = "Register `CH1AWSCDR` reader"] pub type R = crate :: R < Ch1awscdrSpec > ; # [doc = "Register `CH1AWSCDR` writer"] pub type W = crate :: W < Ch1awscdrSpec > ; # [doc = "Field `SCDT` reader - SCDT"] pub type ScdtR = crate :: FieldReader ; # [doc = "Field `SCDT` writer - SCDT"] pub type ScdtW < 'a , REG > = crate :: FieldWriter < 'a , REG , 8 > ; # [doc = "Field `BKSCD` reader - BKSCD"] pub type BkscdR = crate :: FieldReader ; # [doc = "Field `BKSCD` writer - BKSCD"] pub type BkscdW < 'a , REG > = crate :: FieldWriter < 'a , REG , 4 > ; # [doc = "Field `AWFOSR` reader - AWFOSR"] pub type AwfosrR = crate :: FieldReader ; # [doc = "Field `AWFOSR` writer - AWFOSR"] pub type AwfosrW < 'a , REG > = crate :: FieldWriter < 'a , REG , 5 > ; # [doc = "Field `AWFORD` reader - AWFORD"] pub type AwfordR = crate :: FieldReader ; # [doc = "Field `AWFORD` writer - AWFORD"] pub type AwfordW < 'a , REG > = crate :: FieldWriter < 'a , REG , 2 > ; impl R { # [doc = "Bits 0:7 - SCDT"] # [inline (always)] pub fn scdt (& self) -> ScdtR { ScdtR :: new ((self . bits & 0xff) as u8) } # [doc = "Bits 12:15 - BKSCD"] # [inline (always)] pub fn bkscd (& self) -> BkscdR { BkscdR :: new (((self . bits >> 12) & 0x0f) as u8) } # [doc = "Bits 16:20 - AWFOSR"] # [inline (always)] pub fn awfosr (& self) -> AwfosrR { AwfosrR :: new (((self . bits >> 16) & 0x1f) as u8) } # [doc = "Bits 22:23 - AWFORD"] # [inline (always)] pub fn awford (& self) -> AwfordR { AwfordR :: new (((self . bits >> 22) & 3) as u8) } } impl W { # [doc = "Bits 0:7 - SCDT"] # [inline (always)] pub fn scdt (& mut self) -> ScdtW < Ch1awscdrSpec > { ScdtW :: new (self , 0) } # [doc = "Bits 12:15 - BKSCD"] # [inline (always)] pub fn bkscd (& mut self) -> BkscdW < Ch1awscdrSpec > { BkscdW :: new (self , 12) } # [doc = "Bits 16:20 - AWFOSR"] # [inline (always)] pub fn awfosr (& mut self) -> AwfosrW < Ch1awscdrSpec > { AwfosrW :: new (self , 16) } # [doc = "Bits 22:23 - AWFORD"] # [inline (always)] pub fn awford (& mut self) -> AwfordW < Ch1awscdrSpec > { AwfordW :: new (self , 22) } } # [doc = "CH1AWSCDR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1awscdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1awscdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct Ch1awscdrSpec ; impl crate :: RegisterSpec for Ch1awscdrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`ch1awscdr::R`](R) reader structure"] impl crate :: Readable for Ch1awscdrSpec { } # [doc = "`write(|w| ..)` method takes [`ch1awscdr::W`](W) writer structure"] impl crate :: Writable for Ch1awscdrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets CH1AWSCDR to value 0"] impl crate :: Resettable for Ch1awscdrSpec { }