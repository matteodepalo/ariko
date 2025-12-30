#[doc = "Register `PMC_SCDR` writer"]
pub type W = crate::W<PmcScdrSpec>;
#[doc = "Field `UOTGCLK` writer - Disable USB OTG Clock (48 MHz, USB_48M) for UTMI"]
pub type UotgclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Disable"]
pub type Pck0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Disable"]
pub type Pck1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK2` writer - Programmable Clock 2 Output Disable"]
pub type Pck2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 5 - Disable USB OTG Clock (48 MHz, USB_48M) for UTMI"]
  #[inline(always)]
  pub fn uotgclk(&mut self) -> UotgclkW<'_, PmcScdrSpec> {
    UotgclkW::new(self, 5)
  }
  #[doc = "Bit 8 - Programmable Clock 0 Output Disable"]
  #[inline(always)]
  pub fn pck0(&mut self) -> Pck0W<'_, PmcScdrSpec> {
    Pck0W::new(self, 8)
  }
  #[doc = "Bit 9 - Programmable Clock 1 Output Disable"]
  #[inline(always)]
  pub fn pck1(&mut self) -> Pck1W<'_, PmcScdrSpec> {
    Pck1W::new(self, 9)
  }
  #[doc = "Bit 10 - Programmable Clock 2 Output Disable"]
  #[inline(always)]
  pub fn pck2(&mut self) -> Pck2W<'_, PmcScdrSpec> {
    Pck2W::new(self, 10)
  }
}
#[doc = "System Clock Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_scdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcScdrSpec;
impl crate::RegisterSpec for PmcScdrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_scdr::W`](W) writer structure"]
impl crate::Writable for PmcScdrSpec {
  type Safety = crate::Unsafe;
}
