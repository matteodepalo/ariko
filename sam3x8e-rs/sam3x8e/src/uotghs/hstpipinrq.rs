#[doc = "Register `HSTPIPINRQ[%s]` reader"]
pub type R = crate::R<HstpipinrqSpec>;
#[doc = "Register `HSTPIPINRQ[%s]` writer"]
pub type W = crate::W<HstpipinrqSpec>;
#[doc = "Field `INRQ` reader - IN Request Number before Freeze"]
pub type InrqR = crate::FieldReader;
#[doc = "Field `INRQ` writer - IN Request Number before Freeze"]
pub type InrqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INMODE` reader - IN Request Mode"]
pub type InmodeR = crate::BitReader;
#[doc = "Field `INMODE` writer - IN Request Mode"]
pub type InmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
  #[doc = "Bits 0:7 - IN Request Number before Freeze"]
  #[inline(always)]
  pub fn inrq(&self) -> InrqR {
    InrqR::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bit 8 - IN Request Mode"]
  #[inline(always)]
  pub fn inmode(&self) -> InmodeR {
    InmodeR::new(((self.bits >> 8) & 1) != 0)
  }
}
impl W {
  #[doc = "Bits 0:7 - IN Request Number before Freeze"]
  #[inline(always)]
  pub fn inrq(&mut self) -> InrqW<'_, HstpipinrqSpec> {
    InrqW::new(self, 0)
  }
  #[doc = "Bit 8 - IN Request Mode"]
  #[inline(always)]
  pub fn inmode(&mut self) -> InmodeW<'_, HstpipinrqSpec> {
    InmodeW::new(self, 8)
  }
}
#[doc = "Host Pipe IN Request Register (n = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipinrq::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipinrq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstpipinrqSpec;
impl crate::RegisterSpec for HstpipinrqSpec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipinrq::R`](R) reader structure"]
impl crate::Readable for HstpipinrqSpec {}
#[doc = "`write(|w| ..)` method takes [`hstpipinrq::W`](W) writer structure"]
impl crate::Writable for HstpipinrqSpec {
  type Safety = crate::Unsafe;
}
