#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `RXEN` writer - Receive Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIS` writer - Receive Disable"]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` writer - Transmit Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` writer - Transmit Disable"]
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Receive Enable"]
  #[inline(always)]
  pub fn rxen(&mut self) -> RxenW<'_, CrSpec> {
    RxenW::new(self, 0)
  }
  #[doc = "Bit 1 - Receive Disable"]
  #[inline(always)]
  pub fn rxdis(&mut self) -> RxdisW<'_, CrSpec> {
    RxdisW::new(self, 1)
  }
  #[doc = "Bit 8 - Transmit Enable"]
  #[inline(always)]
  pub fn txen(&mut self) -> TxenW<'_, CrSpec> {
    TxenW::new(self, 8)
  }
  #[doc = "Bit 9 - Transmit Disable"]
  #[inline(always)]
  pub fn txdis(&mut self) -> TxdisW<'_, CrSpec> {
    TxdisW::new(self, 9)
  }
  #[doc = "Bit 15 - Software Reset"]
  #[inline(always)]
  pub fn swrst(&mut self) -> SwrstW<'_, CrSpec> {
    SwrstW::new(self, 15)
  }
}
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
  type Safety = crate::Unsafe;
}
