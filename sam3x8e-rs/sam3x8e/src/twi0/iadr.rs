#[doc = "Register `IADR` reader"]
pub type R = crate::R<IadrSpec>;
#[doc = "Register `IADR` writer"]
pub type W = crate::W<IadrSpec>;
#[doc = "Field `IADR` reader - Internal Address"]
pub type IadrR = crate::FieldReader<u32>;
#[doc = "Field `IADR` writer - Internal Address"]
pub type IadrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
  #[doc = "Bits 0:23 - Internal Address"]
  #[inline(always)]
  pub fn iadr(&self) -> IadrR {
    IadrR::new(self.bits & 0x00ff_ffff)
  }
}
impl W {
  #[doc = "Bits 0:23 - Internal Address"]
  #[inline(always)]
  pub fn iadr(&mut self) -> IadrW<'_, IadrSpec> {
    IadrW::new(self, 0)
  }
}
#[doc = "Internal Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IadrSpec;
impl crate::RegisterSpec for IadrSpec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`iadr::R`](R) reader structure"]
impl crate::Readable for IadrSpec {}
#[doc = "`write(|w| ..)` method takes [`iadr::W`](W) writer structure"]
impl crate::Writable for IadrSpec {
  type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IADR to value 0"]
impl crate::Resettable for IadrSpec {}
