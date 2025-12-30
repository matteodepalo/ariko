#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable."]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` writer - End of Conversion Interrupt Disable"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Disable"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Disable"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Transmit Ready Interrupt Disable."]
  #[inline(always)]
  pub fn txrdy(&mut self) -> TxrdyW<'_, IdrSpec> {
    TxrdyW::new(self, 0)
  }
  #[doc = "Bit 1 - End of Conversion Interrupt Disable"]
  #[inline(always)]
  pub fn eoc(&mut self) -> EocW<'_, IdrSpec> {
    EocW::new(self, 1)
  }
  #[doc = "Bit 2 - End of Transmit Buffer Interrupt Disable"]
  #[inline(always)]
  pub fn endtx(&mut self) -> EndtxW<'_, IdrSpec> {
    EndtxW::new(self, 2)
  }
  #[doc = "Bit 3 - Transmit Buffer Empty Interrupt Disable"]
  #[inline(always)]
  pub fn txbufe(&mut self) -> TxbufeW<'_, IdrSpec> {
    TxbufeW::new(self, 3)
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
