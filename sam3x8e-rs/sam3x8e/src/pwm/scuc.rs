#[doc = "Register `SCUC` reader"]
pub type R = crate::R<ScucSpec>;
#[doc = "Register `SCUC` writer"]
pub type W = crate::W<ScucSpec>;
#[doc = "Field `UPDULOCK` reader - Synchronous Channels Update Unlock"]
pub type UpdulockR = crate::BitReader;
#[doc = "Field `UPDULOCK` writer - Synchronous Channels Update Unlock"]
pub type UpdulockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
  #[doc = "Bit 0 - Synchronous Channels Update Unlock"]
  #[inline(always)]
  pub fn updulock(&self) -> UpdulockR {
    UpdulockR::new((self.bits & 1) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Synchronous Channels Update Unlock"]
  #[inline(always)]
  pub fn updulock(&mut self) -> UpdulockW<'_, ScucSpec> {
    UpdulockW::new(self, 0)
  }
}
#[doc = "PWM Sync Channels Update Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScucSpec;
impl crate::RegisterSpec for ScucSpec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`scuc::R`](R) reader structure"]
impl crate::Readable for ScucSpec {}
#[doc = "`write(|w| ..)` method takes [`scuc::W`](W) writer structure"]
impl crate::Writable for ScucSpec {
  type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC to value 0"]
impl crate::Resettable for ScucSpec {}
