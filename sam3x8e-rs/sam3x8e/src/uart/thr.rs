#[doc = "Register `THR` writer"]
pub type W = crate::W<ThrSpec>;
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub type TxchrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
  #[doc = "Bits 0:7 - Character to be Transmitted"]
  #[inline(always)]
  pub fn txchr(&mut self) -> TxchrW<'_, ThrSpec> {
    TxchrW::new(self, 0)
  }
}
#[doc = "Transmit Holding Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrSpec;
impl crate::RegisterSpec for ThrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for ThrSpec {
  type Safety = crate::Unsafe;
}
