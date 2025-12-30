#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `DATRDY` writer - Data Ready Interrupt Disable"]
pub type DatrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Data Ready Interrupt Disable"]
  #[inline(always)]
  pub fn datrdy(&mut self) -> DatrdyW<'_, IdrSpec> {
    DatrdyW::new(self, 0)
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
