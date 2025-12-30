#[doc = "Register `TDR` writer"]
pub type W = crate::W<TdrSpec>;
#[doc = "Field `TD` writer - Transmit Data"]
pub type TdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub type PcsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LastxferW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bits 0:15 - Transmit Data"]
  #[inline(always)]
  pub fn td(&mut self) -> TdW<'_, TdrSpec> {
    TdW::new(self, 0)
  }
  #[doc = "Bits 16:19 - Peripheral Chip Select"]
  #[inline(always)]
  pub fn pcs(&mut self) -> PcsW<'_, TdrSpec> {
    PcsW::new(self, 16)
  }
  #[doc = "Bit 24 - Last Transfer"]
  #[inline(always)]
  pub fn lastxfer(&mut self) -> LastxferW<'_, TdrSpec> {
    LastxferW::new(self, 24)
  }
}
#[doc = "Transmit Data Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TdrSpec {
  type Safety = crate::Unsafe;
}
