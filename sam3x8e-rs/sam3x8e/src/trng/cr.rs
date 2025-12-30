#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ENABLE` writer - Enables the TRNG to provide random values"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - Security Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
  #[doc = "Bit 0 - Enables the TRNG to provide random values"]
  #[inline(always)]
  pub fn enable(&mut self) -> EnableW<'_, CrSpec> {
    EnableW::new(self, 0)
  }
  #[doc = "Bits 8:31 - Security Key"]
  #[inline(always)]
  pub fn key(&mut self) -> KeyW<'_, CrSpec> {
    KeyW::new(self, 8)
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
