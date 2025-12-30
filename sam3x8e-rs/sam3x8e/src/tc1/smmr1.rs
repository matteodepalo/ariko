#[doc = "Register `SMMR1` reader"]
pub type R = crate::R<Smmr1Spec>;
#[doc = "Register `SMMR1` writer"]
pub type W = crate::W<Smmr1Spec>;
#[doc = "Field `GCEN` reader - Gray Count Enable"]
pub type GcenR = crate::BitReader;
#[doc = "Field `GCEN` writer - Gray Count Enable"]
pub type GcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWN` reader - DOWN Count"]
pub type DownR = crate::BitReader;
#[doc = "Field `DOWN` writer - DOWN Count"]
pub type DownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
  #[doc = "Bit 0 - Gray Count Enable"]
  #[inline(always)]
  pub fn gcen(&self) -> GcenR {
    GcenR::new((self.bits & 1) != 0)
  }
  #[doc = "Bit 1 - DOWN Count"]
  #[inline(always)]
  pub fn down(&self) -> DownR {
    DownR::new(((self.bits >> 1) & 1) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Gray Count Enable"]
  #[inline(always)]
  pub fn gcen(&mut self) -> GcenW<'_, Smmr1Spec> {
    GcenW::new(self, 0)
  }
  #[doc = "Bit 1 - DOWN Count"]
  #[inline(always)]
  pub fn down(&mut self) -> DownW<'_, Smmr1Spec> {
    DownW::new(self, 1)
  }
}
#[doc = "Stepper Motor Mode Register (channel = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`smmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smmr1Spec;
impl crate::RegisterSpec for Smmr1Spec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`smmr1::R`](R) reader structure"]
impl crate::Readable for Smmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`smmr1::W`](W) writer structure"]
impl crate::Writable for Smmr1Spec {
  type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMMR1 to value 0"]
impl crate::Resettable for Smmr1Spec {}
