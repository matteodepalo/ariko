#[doc = "Register `DEVEPTIDR[%s]` writer"]
pub type W = crate::W<DeveptidrSpec>;
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TxinecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RxoutecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPEC` writer - Received SETUP Interrupt Clear"]
pub type RxstpecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTEC` writer - NAKed OUT Interrupt Clear"]
pub type NakoutecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINEC` writer - NAKed IN Interrupt Clear"]
pub type NakinecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OverfecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDEC` writer - STALLed Interrupt Clear"]
pub type StalledecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type ShortpacketecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NbusybkecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FifoconcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EpdishdmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETDISC` writer - NYET Token Disable Clear"]
pub type NyetdiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQC` writer - STALL Request Clear"]
pub type StallrqcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
  #[inline(always)]
  pub fn txinec(&mut self) -> TxinecW<'_, DeveptidrSpec> {
    TxinecW::new(self, 0)
  }
  #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
  #[inline(always)]
  pub fn rxoutec(&mut self) -> RxoutecW<'_, DeveptidrSpec> {
    RxoutecW::new(self, 1)
  }
  #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
  #[inline(always)]
  pub fn rxstpec(&mut self) -> RxstpecW<'_, DeveptidrSpec> {
    RxstpecW::new(self, 2)
  }
  #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
  #[inline(always)]
  pub fn nakoutec(&mut self) -> NakoutecW<'_, DeveptidrSpec> {
    NakoutecW::new(self, 3)
  }
  #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
  #[inline(always)]
  pub fn nakinec(&mut self) -> NakinecW<'_, DeveptidrSpec> {
    NakinecW::new(self, 4)
  }
  #[doc = "Bit 5 - Overflow Interrupt Clear"]
  #[inline(always)]
  pub fn overfec(&mut self) -> OverfecW<'_, DeveptidrSpec> {
    OverfecW::new(self, 5)
  }
  #[doc = "Bit 6 - STALLed Interrupt Clear"]
  #[inline(always)]
  pub fn stalledec(&mut self) -> StalledecW<'_, DeveptidrSpec> {
    StalledecW::new(self, 6)
  }
  #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
  #[inline(always)]
  pub fn shortpacketec(&mut self) -> ShortpacketecW<'_, DeveptidrSpec> {
    ShortpacketecW::new(self, 7)
  }
  #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
  #[inline(always)]
  pub fn nbusybkec(&mut self) -> NbusybkecW<'_, DeveptidrSpec> {
    NbusybkecW::new(self, 12)
  }
  #[doc = "Bit 14 - FIFO Control Clear"]
  #[inline(always)]
  pub fn fifoconc(&mut self) -> FifoconcW<'_, DeveptidrSpec> {
    FifoconcW::new(self, 14)
  }
  #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
  #[inline(always)]
  pub fn epdishdmac(&mut self) -> EpdishdmacW<'_, DeveptidrSpec> {
    EpdishdmacW::new(self, 16)
  }
  #[doc = "Bit 17 - NYET Token Disable Clear"]
  #[inline(always)]
  pub fn nyetdisc(&mut self) -> NyetdiscW<'_, DeveptidrSpec> {
    NyetdiscW::new(self, 17)
  }
  #[doc = "Bit 19 - STALL Request Clear"]
  #[inline(always)]
  pub fn stallrqc(&mut self) -> StallrqcW<'_, DeveptidrSpec> {
    StallrqcW::new(self, 19)
  }
}
#[doc = "Device Endpoint Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeveptidrSpec;
impl crate::RegisterSpec for DeveptidrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptidr::W`](W) writer structure"]
impl crate::Writable for DeveptidrSpec {
  type Safety = crate::Unsafe;
}
