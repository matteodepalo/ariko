#[doc = "Register `TPR` reader"]
pub type R = crate::R<TprSpec>;
#[doc = "Register `TPR` writer"]
pub type W = crate::W<TprSpec>;
#[doc = "Field `TXPTR` reader - Transmit Counter Register"]
pub type TxptrR = crate::FieldReader<u32>;
#[doc = "Field `TXPTR` writer - Transmit Counter Register"]
pub type TxptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
  #[doc = "Bits 0:31 - Transmit Counter Register"]
  #[inline(always)]
  pub fn txptr(&self) -> TxptrR {
    TxptrR::new(self.bits)
  }
}
impl W {
  #[doc = "Bits 0:31 - Transmit Counter Register"]
  #[inline(always)]
  pub fn txptr(&mut self) -> TxptrW<'_, TprSpec> {
    TxptrW::new(self, 0)
  }
}
#[doc = "Transmit Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TprSpec;
impl crate::RegisterSpec for TprSpec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`tpr::R`](R) reader structure"]
impl crate::Readable for TprSpec {}
#[doc = "`write(|w| ..)` method takes [`tpr::W`](W) writer structure"]
impl crate::Writable for TprSpec {
  type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPR to value 0"]
impl crate::Resettable for TprSpec {}
