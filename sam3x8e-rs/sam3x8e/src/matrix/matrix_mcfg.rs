#[doc = "Register `MATRIX_MCFG[%s]` reader"]
pub type R = crate::R<MatrixMcfgSpec>;
#[doc = "Register `MATRIX_MCFG[%s]` writer"]
pub type W = crate::W<MatrixMcfgSpec>;
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type UlbtR = crate::FieldReader;
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type UlbtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
  #[doc = "Bits 0:2 - Undefined Length Burst Type"]
  #[inline(always)]
  pub fn ulbt(&self) -> UlbtR {
    UlbtR::new((self.bits & 7) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - Undefined Length Burst Type"]
  #[inline(always)]
  pub fn ulbt(&mut self) -> UlbtW<'_, MatrixMcfgSpec> {
    UlbtW::new(self, 0)
  }
}
#[doc = "Master Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_mcfg::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_mcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatrixMcfgSpec;
impl crate::RegisterSpec for MatrixMcfgSpec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_mcfg::R`](R) reader structure"]
impl crate::Readable for MatrixMcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`matrix_mcfg::W`](W) writer structure"]
impl crate::Writable for MatrixMcfgSpec {
  type Safety = crate::Unsafe;
}
