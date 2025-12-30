#[doc = "Register `SCCR` writer"]
pub type W = crate::W<SccrSpec>;
#[doc = "Field `ACKCLR` writer - Acknowledge Clear"]
pub type AckclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRCLR` writer - Alarm Clear"]
pub type AlrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECCLR` writer - Second Clear"]
pub type SecclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMCLR` writer - Time Clear"]
pub type TimclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALCLR` writer - Calendar Clear"]
pub type CalclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Acknowledge Clear"]
  #[inline(always)]
  pub fn ackclr(&mut self) -> AckclrW<'_, SccrSpec> {
    AckclrW::new(self, 0)
  }
  #[doc = "Bit 1 - Alarm Clear"]
  #[inline(always)]
  pub fn alrclr(&mut self) -> AlrclrW<'_, SccrSpec> {
    AlrclrW::new(self, 1)
  }
  #[doc = "Bit 2 - Second Clear"]
  #[inline(always)]
  pub fn secclr(&mut self) -> SecclrW<'_, SccrSpec> {
    SecclrW::new(self, 2)
  }
  #[doc = "Bit 3 - Time Clear"]
  #[inline(always)]
  pub fn timclr(&mut self) -> TimclrW<'_, SccrSpec> {
    TimclrW::new(self, 3)
  }
  #[doc = "Bit 4 - Calendar Clear"]
  #[inline(always)]
  pub fn calclr(&mut self) -> CalclrW<'_, SccrSpec> {
    CalclrW::new(self, 4)
  }
}
#[doc = "Status Clear Command Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccrSpec;
impl crate::RegisterSpec for SccrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sccr::W`](W) writer structure"]
impl crate::Writable for SccrSpec {
  type Safety = crate::Unsafe;
}
