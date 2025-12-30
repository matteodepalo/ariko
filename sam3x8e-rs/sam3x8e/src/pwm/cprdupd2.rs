#[doc = "Register `CPRDUPD2` writer"]
pub type W = crate::W<Cprdupd2Spec>;
#[doc = "Field `CPRDUPD` writer - Channel Period Update"]
pub type CprdupdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
  #[doc = "Bits 0:23 - Channel Period Update"]
  #[inline(always)]
  pub fn cprdupd(&mut self) -> CprdupdW<'_, Cprdupd2Spec> {
    CprdupdW::new(self, 0)
  }
}
#[doc = "PWM Channel Period Update Register (ch_num = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cprdupd2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cprdupd2Spec;
impl crate::RegisterSpec for Cprdupd2Spec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cprdupd2::W`](W) writer structure"]
impl crate::Writable for Cprdupd2Spec {
  type Safety = crate::Unsafe;
}
