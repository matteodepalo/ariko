#[doc = "Register `HSTPIPIFR[%s]` writer"]
pub type W = crate::W<HstpipifrSpec>;
#[doc = "Field `RXINIS` writer - Received IN Data Interrupt Set"]
pub type RxinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIS` writer - Transmitted OUT Data Interrupt Set"]
pub type TxoutisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTPIS` writer - Transmitted SETUP Interrupt Set"]
pub type TxstpisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIS` writer - Pipe Error Interrupt Set"]
pub type PerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIS` writer - NAKed Interrupt Set"]
pub type NakedisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OverfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDIS` writer - Received STALLed Interrupt Set"]
pub type RxstalldisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIS` writer - Short Packet Interrupt Set"]
pub type ShortpacketisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Set"]
pub type NbusybksW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Received IN Data Interrupt Set"]
  #[inline(always)]
  pub fn rxinis(&mut self) -> RxinisW<'_, HstpipifrSpec> {
    RxinisW::new(self, 0)
  }
  #[doc = "Bit 1 - Transmitted OUT Data Interrupt Set"]
  #[inline(always)]
  pub fn txoutis(&mut self) -> TxoutisW<'_, HstpipifrSpec> {
    TxoutisW::new(self, 1)
  }
  #[doc = "Bit 2 - Transmitted SETUP Interrupt Set"]
  #[inline(always)]
  pub fn txstpis(&mut self) -> TxstpisW<'_, HstpipifrSpec> {
    TxstpisW::new(self, 2)
  }
  #[doc = "Bit 3 - Pipe Error Interrupt Set"]
  #[inline(always)]
  pub fn perris(&mut self) -> PerrisW<'_, HstpipifrSpec> {
    PerrisW::new(self, 3)
  }
  #[doc = "Bit 4 - NAKed Interrupt Set"]
  #[inline(always)]
  pub fn nakedis(&mut self) -> NakedisW<'_, HstpipifrSpec> {
    NakedisW::new(self, 4)
  }
  #[doc = "Bit 5 - Overflow Interrupt Set"]
  #[inline(always)]
  pub fn overfis(&mut self) -> OverfisW<'_, HstpipifrSpec> {
    OverfisW::new(self, 5)
  }
  #[doc = "Bit 6 - Received STALLed Interrupt Set"]
  #[inline(always)]
  pub fn rxstalldis(&mut self) -> RxstalldisW<'_, HstpipifrSpec> {
    RxstalldisW::new(self, 6)
  }
  #[doc = "Bit 7 - Short Packet Interrupt Set"]
  #[inline(always)]
  pub fn shortpacketis(&mut self) -> ShortpacketisW<'_, HstpipifrSpec> {
    ShortpacketisW::new(self, 7)
  }
  #[doc = "Bit 12 - Number of Busy Banks Set"]
  #[inline(always)]
  pub fn nbusybks(&mut self) -> NbusybksW<'_, HstpipifrSpec> {
    NbusybksW::new(self, 12)
  }
}
#[doc = "Host Pipe Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipifr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstpipifrSpec;
impl crate::RegisterSpec for HstpipifrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipifr::W`](W) writer structure"]
impl crate::Writable for HstpipifrSpec {
  type Safety = crate::Unsafe;
}
