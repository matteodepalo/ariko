#[doc = "Register `CMPVUPD4` writer"]
pub type W = crate::W<Cmpvupd4Spec>;
#[doc = "Field `CVUPD` writer - Comparison x Value Update"]
pub type CvupdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `CVMUPD` writer - Comparison x Value Mode Update"]
pub type CvmupdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bits 0:23 - Comparison x Value Update"]
  #[inline(always)]
  pub fn cvupd(&mut self) -> CvupdW<'_, Cmpvupd4Spec> {
    CvupdW::new(self, 0)
  }
  #[doc = "Bit 24 - Comparison x Value Mode Update"]
  #[inline(always)]
  pub fn cvmupd(&mut self) -> CvmupdW<'_, Cmpvupd4Spec> {
    CvmupdW::new(self, 24)
  }
}
#[doc = "PWM Comparison 4 Value Update Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpvupd4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpvupd4Spec;
impl crate::RegisterSpec for Cmpvupd4Spec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmpvupd4::W`](W) writer structure"]
impl crate::Writable for Cmpvupd4Spec {
  type Safety = crate::Unsafe;
}
