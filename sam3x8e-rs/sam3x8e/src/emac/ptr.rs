#[doc = "Register `PTR` reader"]
pub type R = crate::R<PtrSpec>;
#[doc = "Register `PTR` writer"]
pub type W = crate::W<PtrSpec>;
#[doc = "Field `PTIME` reader - Pause Time"]
pub type PtimeR = crate::FieldReader<u16>;
#[doc = "Field `PTIME` writer - Pause Time"]
pub type PtimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
  #[doc = "Bits 0:15 - Pause Time"]
  #[inline(always)]
  pub fn ptime(&self) -> PtimeR {
    PtimeR::new((self.bits & 0xffff) as u16)
  }
}
impl W {
  #[doc = "Bits 0:15 - Pause Time"]
  #[inline(always)]
  pub fn ptime(&mut self) -> PtimeW<'_, PtrSpec> {
    PtimeW::new(self, 0)
  }
}
#[doc = "Pause Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtrSpec;
impl crate::RegisterSpec for PtrSpec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`ptr::R`](R) reader structure"]
impl crate::Readable for PtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ptr::W`](W) writer structure"]
impl crate::Writable for PtrSpec {
  type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTR to value 0"]
impl crate::Resettable for PtrSpec {}
