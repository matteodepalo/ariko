#[doc = "Register `HSTPIPIDR[%s]` writer"]
pub type W = crate::W<HstpipidrSpec>;
#[doc = "Field `RXINEC` writer - Received IN Data Interrupt Disable"]
pub type RxinecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTEC` writer - Transmitted OUT Data Interrupt Disable"]
pub type TxoutecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTPEC` writer - Transmitted SETUP Interrupt Disable"]
pub type TxstpecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERREC` writer - Pipe Error Interrupt Disable"]
pub type PerrecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDEC` writer - NAKed Interrupt Disable"]
pub type NakedecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIEC` writer - Overflow Interrupt Disable"]
pub type OverfiecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDEC` writer - Received STALLed Interrupt Disable"]
pub type RxstalldecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIEC` writer - Short Packet Interrupt Disable"]
pub type ShortpacketiecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Disable"]
pub type NbusybkecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Disable"]
pub type FifoconcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDISHDMAC` writer - Pipe Interrupts Disable HDMA Request Disable"]
pub type PdishdmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFREEZEC` writer - Pipe Freeze Disable"]
pub type PfreezecW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Received IN Data Interrupt Disable"]
  #[inline(always)]
  pub fn rxinec(&mut self) -> RxinecW<'_, HstpipidrSpec> {
    RxinecW::new(self, 0)
  }
  #[doc = "Bit 1 - Transmitted OUT Data Interrupt Disable"]
  #[inline(always)]
  pub fn txoutec(&mut self) -> TxoutecW<'_, HstpipidrSpec> {
    TxoutecW::new(self, 1)
  }
  #[doc = "Bit 2 - Transmitted SETUP Interrupt Disable"]
  #[inline(always)]
  pub fn txstpec(&mut self) -> TxstpecW<'_, HstpipidrSpec> {
    TxstpecW::new(self, 2)
  }
  #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
  #[inline(always)]
  pub fn perrec(&mut self) -> PerrecW<'_, HstpipidrSpec> {
    PerrecW::new(self, 3)
  }
  #[doc = "Bit 4 - NAKed Interrupt Disable"]
  #[inline(always)]
  pub fn nakedec(&mut self) -> NakedecW<'_, HstpipidrSpec> {
    NakedecW::new(self, 4)
  }
  #[doc = "Bit 5 - Overflow Interrupt Disable"]
  #[inline(always)]
  pub fn overfiec(&mut self) -> OverfiecW<'_, HstpipidrSpec> {
    OverfiecW::new(self, 5)
  }
  #[doc = "Bit 6 - Received STALLed Interrupt Disable"]
  #[inline(always)]
  pub fn rxstalldec(&mut self) -> RxstalldecW<'_, HstpipidrSpec> {
    RxstalldecW::new(self, 6)
  }
  #[doc = "Bit 7 - Short Packet Interrupt Disable"]
  #[inline(always)]
  pub fn shortpacketiec(&mut self) -> ShortpacketiecW<'_, HstpipidrSpec> {
    ShortpacketiecW::new(self, 7)
  }
  #[doc = "Bit 12 - Number of Busy Banks Disable"]
  #[inline(always)]
  pub fn nbusybkec(&mut self) -> NbusybkecW<'_, HstpipidrSpec> {
    NbusybkecW::new(self, 12)
  }
  #[doc = "Bit 14 - FIFO Control Disable"]
  #[inline(always)]
  pub fn fifoconc(&mut self) -> FifoconcW<'_, HstpipidrSpec> {
    FifoconcW::new(self, 14)
  }
  #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Disable"]
  #[inline(always)]
  pub fn pdishdmac(&mut self) -> PdishdmacW<'_, HstpipidrSpec> {
    PdishdmacW::new(self, 16)
  }
  #[doc = "Bit 17 - Pipe Freeze Disable"]
  #[inline(always)]
  pub fn pfreezec(&mut self) -> PfreezecW<'_, HstpipidrSpec> {
    PfreezecW::new(self, 17)
  }
}
#[doc = "Host Pipe Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstpipidrSpec;
impl crate::RegisterSpec for HstpipidrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipidr::W`](W) writer structure"]
impl crate::Writable for HstpipidrSpec {
  type Safety = crate::Unsafe;
}
