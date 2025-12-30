#[doc = "Register `PULSE5` reader"]
pub type R = crate::R<Pulse5Spec>;
#[doc = "Register `PULSE5` writer"]
pub type W = crate::W<Pulse5Spec>;
#[doc = "Field `NWE_PULSE` reader - NWE Pulse Length"]
pub type NwePulseR = crate::FieldReader;
#[doc = "Field `NWE_PULSE` writer - NWE Pulse Length"]
pub type NwePulseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NCS_WR_PULSE` reader - NCS Pulse Length in WRITE Access"]
pub type NcsWrPulseR = crate::FieldReader;
#[doc = "Field `NCS_WR_PULSE` writer - NCS Pulse Length in WRITE Access"]
pub type NcsWrPulseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NRD_PULSE` reader - NRD Pulse Length"]
pub type NrdPulseR = crate::FieldReader;
#[doc = "Field `NRD_PULSE` writer - NRD Pulse Length"]
pub type NrdPulseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NCS_RD_PULSE` reader - NCS Pulse Length in READ Access"]
pub type NcsRdPulseR = crate::FieldReader;
#[doc = "Field `NCS_RD_PULSE` writer - NCS Pulse Length in READ Access"]
pub type NcsRdPulseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
  #[doc = "Bits 0:5 - NWE Pulse Length"]
  #[inline(always)]
  pub fn nwe_pulse(&self) -> NwePulseR {
    NwePulseR::new((self.bits & 0x3f) as u8)
  }
  #[doc = "Bits 8:13 - NCS Pulse Length in WRITE Access"]
  #[inline(always)]
  pub fn ncs_wr_pulse(&self) -> NcsWrPulseR {
    NcsWrPulseR::new(((self.bits >> 8) & 0x3f) as u8)
  }
  #[doc = "Bits 16:21 - NRD Pulse Length"]
  #[inline(always)]
  pub fn nrd_pulse(&self) -> NrdPulseR {
    NrdPulseR::new(((self.bits >> 16) & 0x3f) as u8)
  }
  #[doc = "Bits 24:29 - NCS Pulse Length in READ Access"]
  #[inline(always)]
  pub fn ncs_rd_pulse(&self) -> NcsRdPulseR {
    NcsRdPulseR::new(((self.bits >> 24) & 0x3f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:5 - NWE Pulse Length"]
  #[inline(always)]
  pub fn nwe_pulse(&mut self) -> NwePulseW<'_, Pulse5Spec> {
    NwePulseW::new(self, 0)
  }
  #[doc = "Bits 8:13 - NCS Pulse Length in WRITE Access"]
  #[inline(always)]
  pub fn ncs_wr_pulse(&mut self) -> NcsWrPulseW<'_, Pulse5Spec> {
    NcsWrPulseW::new(self, 8)
  }
  #[doc = "Bits 16:21 - NRD Pulse Length"]
  #[inline(always)]
  pub fn nrd_pulse(&mut self) -> NrdPulseW<'_, Pulse5Spec> {
    NrdPulseW::new(self, 16)
  }
  #[doc = "Bits 24:29 - NCS Pulse Length in READ Access"]
  #[inline(always)]
  pub fn ncs_rd_pulse(&mut self) -> NcsRdPulseW<'_, Pulse5Spec> {
    NcsRdPulseW::new(self, 24)
  }
}
#[doc = "SMC Pulse Register (CS_number = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`pulse5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulse5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pulse5Spec;
impl crate::RegisterSpec for Pulse5Spec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`pulse5::R`](R) reader structure"]
impl crate::Readable for Pulse5Spec {}
#[doc = "`write(|w| ..)` method takes [`pulse5::W`](W) writer structure"]
impl crate::Writable for Pulse5Spec {
  type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PULSE5 to value 0x0101_0101"]
impl crate::Resettable for Pulse5Spec {
  const RESET_VALUE: u32 = 0x0101_0101;
}
