#[doc = "Register `GPBR[%s]` reader"]
pub type R = crate::R<GpbrSpec>;
#[doc = "Register `GPBR[%s]` writer"]
pub type W = crate::W<GpbrSpec>;
#[doc = "Field `GPBR_VALUE` reader - Value of GPBR x"]
pub type GpbrValueR = crate::FieldReader<u32>;
#[doc = "Field `GPBR_VALUE` writer - Value of GPBR x"]
pub type GpbrValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
  #[doc = "Bits 0:31 - Value of GPBR x"]
  #[inline(always)]
  pub fn gpbr_value(&self) -> GpbrValueR {
    GpbrValueR::new(self.bits)
  }
}
impl W {
  #[doc = "Bits 0:31 - Value of GPBR x"]
  #[inline(always)]
  pub fn gpbr_value(&mut self) -> GpbrValueW<'_, GpbrSpec> {
    GpbrValueW::new(self, 0)
  }
}
#[doc = "General Purpose Backup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpbrSpec;
impl crate::RegisterSpec for GpbrSpec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`gpbr::R`](R) reader structure"]
impl crate::Readable for GpbrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpbr::W`](W) writer structure"]
impl crate::Writable for GpbrSpec {
  type Safety = crate::Unsafe;
}
