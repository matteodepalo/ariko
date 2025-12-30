#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `ACKDIS` writer - Acknowledge Update Interrupt Disable"]
pub type AckdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRDIS` writer - Alarm Interrupt Disable"]
pub type AlrdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECDIS` writer - Second Event Interrupt Disable"]
pub type SecdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMDIS` writer - Time Event Interrupt Disable"]
pub type TimdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALDIS` writer - Calendar Event Interrupt Disable"]
pub type CaldisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Acknowledge Update Interrupt Disable"]
  #[inline(always)]
  pub fn ackdis(&mut self) -> AckdisW<'_, IdrSpec> {
    AckdisW::new(self, 0)
  }
  #[doc = "Bit 1 - Alarm Interrupt Disable"]
  #[inline(always)]
  pub fn alrdis(&mut self) -> AlrdisW<'_, IdrSpec> {
    AlrdisW::new(self, 1)
  }
  #[doc = "Bit 2 - Second Event Interrupt Disable"]
  #[inline(always)]
  pub fn secdis(&mut self) -> SecdisW<'_, IdrSpec> {
    SecdisW::new(self, 2)
  }
  #[doc = "Bit 3 - Time Event Interrupt Disable"]
  #[inline(always)]
  pub fn timdis(&mut self) -> TimdisW<'_, IdrSpec> {
    TimdisW::new(self, 3)
  }
  #[doc = "Bit 4 - Calendar Event Interrupt Disable"]
  #[inline(always)]
  pub fn caldis(&mut self) -> CaldisW<'_, IdrSpec> {
    CaldisW::new(self, 4)
  }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
  type Safety = crate::Unsafe;
}
