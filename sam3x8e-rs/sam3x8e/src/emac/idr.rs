#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `MFD` writer - Management Frame sent"]
pub type MfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUBR` writer - Receive Used Bit Read"]
pub type RxubrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUBR` writer - Transmit Used Bit Read"]
pub type TxubrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUND` writer - Ethernet Transmit Buffer Underrun"]
pub type TundW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLE` writer - Retry Limit Exceeded"]
pub type RleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERR` writer - "]
pub type TxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type RovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESP` writer - Hresp not OK"]
pub type HrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFR` writer - Pause Frame Received"]
pub type PfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTZ` writer - Pause Time Zero"]
pub type PtzW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Management Frame sent"]
  #[inline(always)]
  pub fn mfd(&mut self) -> MfdW<'_, IdrSpec> {
    MfdW::new(self, 0)
  }
  #[doc = "Bit 1 - Receive Complete"]
  #[inline(always)]
  pub fn rcomp(&mut self) -> RcompW<'_, IdrSpec> {
    RcompW::new(self, 1)
  }
  #[doc = "Bit 2 - Receive Used Bit Read"]
  #[inline(always)]
  pub fn rxubr(&mut self) -> RxubrW<'_, IdrSpec> {
    RxubrW::new(self, 2)
  }
  #[doc = "Bit 3 - Transmit Used Bit Read"]
  #[inline(always)]
  pub fn txubr(&mut self) -> TxubrW<'_, IdrSpec> {
    TxubrW::new(self, 3)
  }
  #[doc = "Bit 4 - Ethernet Transmit Buffer Underrun"]
  #[inline(always)]
  pub fn tund(&mut self) -> TundW<'_, IdrSpec> {
    TundW::new(self, 4)
  }
  #[doc = "Bit 5 - Retry Limit Exceeded"]
  #[inline(always)]
  pub fn rle(&mut self) -> RleW<'_, IdrSpec> {
    RleW::new(self, 5)
  }
  #[doc = "Bit 6"]
  #[inline(always)]
  pub fn txerr(&mut self) -> TxerrW<'_, IdrSpec> {
    TxerrW::new(self, 6)
  }
  #[doc = "Bit 7 - Transmit Complete"]
  #[inline(always)]
  pub fn tcomp(&mut self) -> TcompW<'_, IdrSpec> {
    TcompW::new(self, 7)
  }
  #[doc = "Bit 10 - Receive Overrun"]
  #[inline(always)]
  pub fn rovr(&mut self) -> RovrW<'_, IdrSpec> {
    RovrW::new(self, 10)
  }
  #[doc = "Bit 11 - Hresp not OK"]
  #[inline(always)]
  pub fn hresp(&mut self) -> HrespW<'_, IdrSpec> {
    HrespW::new(self, 11)
  }
  #[doc = "Bit 12 - Pause Frame Received"]
  #[inline(always)]
  pub fn pfr(&mut self) -> PfrW<'_, IdrSpec> {
    PfrW::new(self, 12)
  }
  #[doc = "Bit 13 - Pause Time Zero"]
  #[inline(always)]
  pub fn ptz(&mut self) -> PtzW<'_, IdrSpec> {
    PtzW::new(self, 13)
  }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
  type Safety = crate::Unsafe;
}
