#[doc = "Register `CHER` writer"]
pub type W = crate::W<CherSpec>;
#[doc = "Field `CH0` writer - Channel 0 Enable"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Channel 1 Enable"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Channel 0 Enable"]
  #[inline(always)]
  pub fn ch0(&mut self) -> Ch0W<'_, CherSpec> {
    Ch0W::new(self, 0)
  }
  #[doc = "Bit 1 - Channel 1 Enable"]
  #[inline(always)]
  pub fn ch1(&mut self) -> Ch1W<'_, CherSpec> {
    Ch1W::new(self, 1)
  }
}
#[doc = "Channel Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cher::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CherSpec;
impl crate::RegisterSpec for CherSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cher::W`](W) writer structure"]
impl crate::Writable for CherSpec {
  type Safety = crate::Unsafe;
}
