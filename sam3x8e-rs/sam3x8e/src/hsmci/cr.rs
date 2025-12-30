#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `MCIEN` writer - Multi-Media Interface Enable"]
pub type McienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCIDIS` writer - Multi-Media Interface Disable"]
pub type McidisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWSEN` writer - Power Save Mode Enable"]
pub type PwsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWSDIS` writer - Power Save Mode Disable"]
pub type PwsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Multi-Media Interface Enable"]
  #[inline(always)]
  pub fn mcien(&mut self) -> McienW<'_, CrSpec> {
    McienW::new(self, 0)
  }
  #[doc = "Bit 1 - Multi-Media Interface Disable"]
  #[inline(always)]
  pub fn mcidis(&mut self) -> McidisW<'_, CrSpec> {
    McidisW::new(self, 1)
  }
  #[doc = "Bit 2 - Power Save Mode Enable"]
  #[inline(always)]
  pub fn pwsen(&mut self) -> PwsenW<'_, CrSpec> {
    PwsenW::new(self, 2)
  }
  #[doc = "Bit 3 - Power Save Mode Disable"]
  #[inline(always)]
  pub fn pwsdis(&mut self) -> PwsdisW<'_, CrSpec> {
    PwsdisW::new(self, 3)
  }
  #[doc = "Bit 7 - Software Reset"]
  #[inline(always)]
  pub fn swrst(&mut self) -> SwrstW<'_, CrSpec> {
    SwrstW::new(self, 7)
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
