#[doc = "Register `RSPR[%s]` reader"]
pub type R = crate::R<RsprSpec>;
#[doc = "Field `RSP` reader - Response"]
pub type RspR = crate::FieldReader<u32>;
impl R {
  #[doc = "Bits 0:31 - Response"]
  #[inline(always)]
  pub fn rsp(&self) -> RspR {
    RspR::new(self.bits)
  }
}
#[doc = "Response Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rspr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsprSpec;
impl crate::RegisterSpec for RsprSpec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`rspr::R`](R) reader structure"]
impl crate::Readable for RsprSpec {}
