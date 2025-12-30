#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `NFCEN` writer - NAND Flash Controller Enable"]
pub type NfcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFCDIS` writer - NAND Flash Controller Disable"]
pub type NfcdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - NAND Flash Controller Enable"]
  #[inline(always)]
  pub fn nfcen(&mut self) -> NfcenW<'_, CtrlSpec> {
    NfcenW::new(self, 0)
  }
  #[doc = "Bit 1 - NAND Flash Controller Disable"]
  #[inline(always)]
  pub fn nfcdis(&mut self) -> NfcdisW<'_, CtrlSpec> {
    NfcdisW::new(self, 1)
  }
}
#[doc = "SMC NFC Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
  type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
