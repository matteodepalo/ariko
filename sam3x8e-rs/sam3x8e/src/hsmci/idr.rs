#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `CMDRDY` writer - Command Ready Interrupt Disable"]
pub type CmdrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Disable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKE` writer - Data Block Ended Interrupt Disable"]
pub type BlkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIP` writer - Data Transfer in Progress Interrupt Disable"]
pub type DtipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTBUSY` writer - Data Not Busy Interrupt Disable"]
pub type NotbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOIRQforSlotA` writer - "]
pub type SdioirqforSlotAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOIRQforSlotB` writer - "]
pub type SdioirqforSlotBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOWAIT` writer - SDIO Read Wait Operation Status Interrupt Disable"]
pub type SdiowaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRCV` writer - Completion Signal received interrupt Disable"]
pub type CsrcvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RINDE` writer - Response Index Error Interrupt Disable"]
pub type RindeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIRE` writer - Response Direction Error Interrupt Disable"]
pub type RdireW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRCE` writer - Response CRC Error Interrupt Disable"]
pub type RcrceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RENDE` writer - Response End Bit Error Interrupt Disable"]
pub type RendeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOE` writer - Response Time-out Error Interrupt Disable"]
pub type RtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCE` writer - Data CRC Error Interrupt Disable"]
pub type DcrceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOE` writer - Data Time-out Error Interrupt Disable"]
pub type DtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTOE` writer - Completion Signal Time out Error Interrupt Disable"]
pub type CstoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKOVRE` writer - DMA Block Overrun Error Interrupt Disable"]
pub type BlkovreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMADONE` writer - DMA Transfer completed Interrupt Disable"]
pub type DmadoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEMPTY` writer - FIFO empty Interrupt Disable"]
pub type FifoemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt Disable"]
pub type XfrdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKRCV` writer - Boot Acknowledge Interrupt Disable"]
pub type AckrcvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKRCVE` writer - Boot Acknowledge Error Interrupt Disable"]
pub type AckrcveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Interrupt Disable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - Underrun Interrupt Disable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Command Ready Interrupt Disable"]
  #[inline(always)]
  pub fn cmdrdy(&mut self) -> CmdrdyW<'_, IdrSpec> {
    CmdrdyW::new(self, 0)
  }
  #[doc = "Bit 1 - Receiver Ready Interrupt Disable"]
  #[inline(always)]
  pub fn rxrdy(&mut self) -> RxrdyW<'_, IdrSpec> {
    RxrdyW::new(self, 1)
  }
  #[doc = "Bit 2 - Transmit Ready Interrupt Disable"]
  #[inline(always)]
  pub fn txrdy(&mut self) -> TxrdyW<'_, IdrSpec> {
    TxrdyW::new(self, 2)
  }
  #[doc = "Bit 3 - Data Block Ended Interrupt Disable"]
  #[inline(always)]
  pub fn blke(&mut self) -> BlkeW<'_, IdrSpec> {
    BlkeW::new(self, 3)
  }
  #[doc = "Bit 4 - Data Transfer in Progress Interrupt Disable"]
  #[inline(always)]
  pub fn dtip(&mut self) -> DtipW<'_, IdrSpec> {
    DtipW::new(self, 4)
  }
  #[doc = "Bit 5 - Data Not Busy Interrupt Disable"]
  #[inline(always)]
  pub fn notbusy(&mut self) -> NotbusyW<'_, IdrSpec> {
    NotbusyW::new(self, 5)
  }
  #[doc = "Bit 8"]
  #[inline(always)]
  pub fn sdioirqfor_slot_a(&mut self) -> SdioirqforSlotAW<'_, IdrSpec> {
    SdioirqforSlotAW::new(self, 8)
  }
  #[doc = "Bit 9"]
  #[inline(always)]
  pub fn sdioirqfor_slot_b(&mut self) -> SdioirqforSlotBW<'_, IdrSpec> {
    SdioirqforSlotBW::new(self, 9)
  }
  #[doc = "Bit 12 - SDIO Read Wait Operation Status Interrupt Disable"]
  #[inline(always)]
  pub fn sdiowait(&mut self) -> SdiowaitW<'_, IdrSpec> {
    SdiowaitW::new(self, 12)
  }
  #[doc = "Bit 13 - Completion Signal received interrupt Disable"]
  #[inline(always)]
  pub fn csrcv(&mut self) -> CsrcvW<'_, IdrSpec> {
    CsrcvW::new(self, 13)
  }
  #[doc = "Bit 16 - Response Index Error Interrupt Disable"]
  #[inline(always)]
  pub fn rinde(&mut self) -> RindeW<'_, IdrSpec> {
    RindeW::new(self, 16)
  }
  #[doc = "Bit 17 - Response Direction Error Interrupt Disable"]
  #[inline(always)]
  pub fn rdire(&mut self) -> RdireW<'_, IdrSpec> {
    RdireW::new(self, 17)
  }
  #[doc = "Bit 18 - Response CRC Error Interrupt Disable"]
  #[inline(always)]
  pub fn rcrce(&mut self) -> RcrceW<'_, IdrSpec> {
    RcrceW::new(self, 18)
  }
  #[doc = "Bit 19 - Response End Bit Error Interrupt Disable"]
  #[inline(always)]
  pub fn rende(&mut self) -> RendeW<'_, IdrSpec> {
    RendeW::new(self, 19)
  }
  #[doc = "Bit 20 - Response Time-out Error Interrupt Disable"]
  #[inline(always)]
  pub fn rtoe(&mut self) -> RtoeW<'_, IdrSpec> {
    RtoeW::new(self, 20)
  }
  #[doc = "Bit 21 - Data CRC Error Interrupt Disable"]
  #[inline(always)]
  pub fn dcrce(&mut self) -> DcrceW<'_, IdrSpec> {
    DcrceW::new(self, 21)
  }
  #[doc = "Bit 22 - Data Time-out Error Interrupt Disable"]
  #[inline(always)]
  pub fn dtoe(&mut self) -> DtoeW<'_, IdrSpec> {
    DtoeW::new(self, 22)
  }
  #[doc = "Bit 23 - Completion Signal Time out Error Interrupt Disable"]
  #[inline(always)]
  pub fn cstoe(&mut self) -> CstoeW<'_, IdrSpec> {
    CstoeW::new(self, 23)
  }
  #[doc = "Bit 24 - DMA Block Overrun Error Interrupt Disable"]
  #[inline(always)]
  pub fn blkovre(&mut self) -> BlkovreW<'_, IdrSpec> {
    BlkovreW::new(self, 24)
  }
  #[doc = "Bit 25 - DMA Transfer completed Interrupt Disable"]
  #[inline(always)]
  pub fn dmadone(&mut self) -> DmadoneW<'_, IdrSpec> {
    DmadoneW::new(self, 25)
  }
  #[doc = "Bit 26 - FIFO empty Interrupt Disable"]
  #[inline(always)]
  pub fn fifoempty(&mut self) -> FifoemptyW<'_, IdrSpec> {
    FifoemptyW::new(self, 26)
  }
  #[doc = "Bit 27 - Transfer Done Interrupt Disable"]
  #[inline(always)]
  pub fn xfrdone(&mut self) -> XfrdoneW<'_, IdrSpec> {
    XfrdoneW::new(self, 27)
  }
  #[doc = "Bit 28 - Boot Acknowledge Interrupt Disable"]
  #[inline(always)]
  pub fn ackrcv(&mut self) -> AckrcvW<'_, IdrSpec> {
    AckrcvW::new(self, 28)
  }
  #[doc = "Bit 29 - Boot Acknowledge Error Interrupt Disable"]
  #[inline(always)]
  pub fn ackrcve(&mut self) -> AckrcveW<'_, IdrSpec> {
    AckrcveW::new(self, 29)
  }
  #[doc = "Bit 30 - Overrun Interrupt Disable"]
  #[inline(always)]
  pub fn ovre(&mut self) -> OvreW<'_, IdrSpec> {
    OvreW::new(self, 30)
  }
  #[doc = "Bit 31 - Underrun Interrupt Disable"]
  #[inline(always)]
  pub fn unre(&mut self) -> UnreW<'_, IdrSpec> {
    UnreW::new(self, 31)
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
