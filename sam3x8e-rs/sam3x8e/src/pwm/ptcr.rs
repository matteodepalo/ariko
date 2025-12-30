#[doc = "Register `PTCR` writer"]
pub type W = crate::W<PtcrSpec>;
#[doc = "Field `RXTEN` writer - Receiver Transfer Enable"]
pub type RxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTDIS` writer - Receiver Transfer Disable"]
pub type RxtdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTEN` writer - Transmitter Transfer Enable"]
pub type TxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTDIS` writer - Transmitter Transfer Disable"]
pub type TxtdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Receiver Transfer Enable"]
  #[inline(always)]
  pub fn rxten(&mut self) -> RxtenW<'_, PtcrSpec> {
    RxtenW::new(self, 0)
  }
  #[doc = "Bit 1 - Receiver Transfer Disable"]
  #[inline(always)]
  pub fn rxtdis(&mut self) -> RxtdisW<'_, PtcrSpec> {
    RxtdisW::new(self, 1)
  }
  #[doc = "Bit 8 - Transmitter Transfer Enable"]
  #[inline(always)]
  pub fn txten(&mut self) -> TxtenW<'_, PtcrSpec> {
    TxtenW::new(self, 8)
  }
  #[doc = "Bit 9 - Transmitter Transfer Disable"]
  #[inline(always)]
  pub fn txtdis(&mut self) -> TxtdisW<'_, PtcrSpec> {
    TxtdisW::new(self, 9)
  }
}
#[doc = "Transfer Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtcrSpec;
impl crate::RegisterSpec for PtcrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ptcr::W`](W) writer structure"]
impl crate::Writable for PtcrSpec {
  type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTCR to value 0"]
impl crate::Resettable for PtcrSpec {}
