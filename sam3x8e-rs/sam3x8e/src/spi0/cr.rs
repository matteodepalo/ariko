#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `SPIEN` writer - SPI Enable"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIDIS` writer - SPI Disable"]
pub type SpidisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - SPI Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LastxferW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - SPI Enable"]
  #[inline(always)]
  pub fn spien(&mut self) -> SpienW<'_, CrSpec> {
    SpienW::new(self, 0)
  }
  #[doc = "Bit 1 - SPI Disable"]
  #[inline(always)]
  pub fn spidis(&mut self) -> SpidisW<'_, CrSpec> {
    SpidisW::new(self, 1)
  }
  #[doc = "Bit 7 - SPI Software Reset"]
  #[inline(always)]
  pub fn swrst(&mut self) -> SwrstW<'_, CrSpec> {
    SwrstW::new(self, 7)
  }
  #[doc = "Bit 24 - Last Transfer"]
  #[inline(always)]
  pub fn lastxfer(&mut self) -> LastxferW<'_, CrSpec> {
    LastxferW::new(self, 24)
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
