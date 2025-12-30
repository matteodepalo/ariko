#[doc = "Register `FIFO[%s]` reader"]
pub type R = crate::R<FifoSpec>;
#[doc = "Register `FIFO[%s]` writer"]
pub type W = crate::W<FifoSpec>;
#[doc = "Field `DATA` reader - Data to Read or Data to Write"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data to Read or Data to Write"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
  #[doc = "Bits 0:31 - Data to Read or Data to Write"]
  #[inline(always)]
  pub fn data(&self) -> DataR {
    DataR::new(self.bits)
  }
}
impl W {
  #[doc = "Bits 0:31 - Data to Read or Data to Write"]
  #[inline(always)]
  pub fn data(&mut self) -> DataW<'_, FifoSpec> {
    DataW::new(self, 0)
  }
}
#[doc = "FIFO Memory Aperture0\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoSpec;
impl crate::RegisterSpec for FifoSpec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo::W`](W) writer structure"]
impl crate::Writable for FifoSpec {
  type Safety = crate::Unsafe;
}
