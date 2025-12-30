#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `WDRSTT` writer - Watchdog Restart"]
pub type WdrsttW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - Password"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
  #[doc = "Bit 0 - Watchdog Restart"]
  #[inline(always)]
  pub fn wdrstt(&mut self) -> WdrsttW<'_, CrSpec> {
    WdrsttW::new(self, 0)
  }
  #[doc = "Bits 24:31 - Password"]
  #[inline(always)]
  pub fn key(&mut self) -> KeyW<'_, CrSpec> {
    KeyW::new(self, 24)
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
