#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `PROCRST` writer - Processor Reset"]
pub type ProcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRST` writer - Peripheral Reset"]
pub type PerrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRST` writer - External Reset"]
pub type ExtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - System Reset Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
  #[doc = "Bit 0 - Processor Reset"]
  #[inline(always)]
  pub fn procrst(&mut self) -> ProcrstW<'_, CrSpec> {
    ProcrstW::new(self, 0)
  }
  #[doc = "Bit 2 - Peripheral Reset"]
  #[inline(always)]
  pub fn perrst(&mut self) -> PerrstW<'_, CrSpec> {
    PerrstW::new(self, 2)
  }
  #[doc = "Bit 3 - External Reset"]
  #[inline(always)]
  pub fn extrst(&mut self) -> ExtrstW<'_, CrSpec> {
    ExtrstW::new(self, 3)
  }
  #[doc = "Bits 24:31 - System Reset Key"]
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
