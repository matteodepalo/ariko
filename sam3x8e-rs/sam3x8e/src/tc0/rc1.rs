#[doc = "Register `RC1` reader"]
pub type R = crate::R<Rc1Spec>;
#[doc = "Register `RC1` writer"]
pub type W = crate::W<Rc1Spec>;
#[doc = "Field `RC` reader - Register C"]
pub type RcR = crate::FieldReader<u32>;
#[doc = "Field `RC` writer - Register C"]
pub type RcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
  #[doc = "Bits 0:31 - Register C"]
  #[inline(always)]
  pub fn rc(&self) -> RcR {
    RcR::new(self.bits)
  }
}
impl W {
  #[doc = "Bits 0:31 - Register C"]
  #[inline(always)]
  pub fn rc(&mut self) -> RcW<'_, Rc1Spec> {
    RcW::new(self, 0)
  }
}
#[doc = "Register C (channel = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`rc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rc1Spec;
impl crate::RegisterSpec for Rc1Spec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`rc1::R`](R) reader structure"]
impl crate::Readable for Rc1Spec {}
#[doc = "`write(|w| ..)` method takes [`rc1::W`](W) writer structure"]
impl crate::Writable for Rc1Spec {
  type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RC1 to value 0"]
impl crate::Resettable for Rc1Spec {}
