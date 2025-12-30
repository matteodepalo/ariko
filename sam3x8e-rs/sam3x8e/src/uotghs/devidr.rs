#[doc = "Register `DEVIDR` writer"]
pub type W = crate::W<DevidrSpec>;
#[doc = "Field `SUSPEC` writer - Suspend Interrupt Disable"]
pub type SuspecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSOFEC` writer - Micro Start of Frame Interrupt Disable"]
pub type MsofecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFEC` writer - Start of Frame Interrupt Disable"]
pub type SofecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSTEC` writer - End of Reset Interrupt Disable"]
pub type EorstecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEC` writer - Wake-Up Interrupt Disable"]
pub type WakeupecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSMEC` writer - End of Resume Interrupt Disable"]
pub type EorsmecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSMEC` writer - Upstream Resume Interrupt Disable"]
pub type UprsmecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_0` writer - Endpoint 0 Interrupt Disable"]
pub type Pep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_1` writer - Endpoint 1 Interrupt Disable"]
pub type Pep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_2` writer - Endpoint 2 Interrupt Disable"]
pub type Pep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_3` writer - Endpoint 3 Interrupt Disable"]
pub type Pep3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_4` writer - Endpoint 4 Interrupt Disable"]
pub type Pep4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_5` writer - Endpoint 5 Interrupt Disable"]
pub type Pep5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_6` writer - Endpoint 6 Interrupt Disable"]
pub type Pep6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_7` writer - Endpoint 7 Interrupt Disable"]
pub type Pep7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_8` writer - Endpoint 8 Interrupt Disable"]
pub type Pep8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_9` writer - Endpoint 9 Interrupt Disable"]
pub type Pep9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Disable"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Disable"]
pub type Dma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Disable"]
pub type Dma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Disable"]
pub type Dma4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Disable"]
pub type Dma5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Disable"]
pub type Dma6W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Suspend Interrupt Disable"]
  #[inline(always)]
  pub fn suspec(&mut self) -> SuspecW<'_, DevidrSpec> {
    SuspecW::new(self, 0)
  }
  #[doc = "Bit 1 - Micro Start of Frame Interrupt Disable"]
  #[inline(always)]
  pub fn msofec(&mut self) -> MsofecW<'_, DevidrSpec> {
    MsofecW::new(self, 1)
  }
  #[doc = "Bit 2 - Start of Frame Interrupt Disable"]
  #[inline(always)]
  pub fn sofec(&mut self) -> SofecW<'_, DevidrSpec> {
    SofecW::new(self, 2)
  }
  #[doc = "Bit 3 - End of Reset Interrupt Disable"]
  #[inline(always)]
  pub fn eorstec(&mut self) -> EorstecW<'_, DevidrSpec> {
    EorstecW::new(self, 3)
  }
  #[doc = "Bit 4 - Wake-Up Interrupt Disable"]
  #[inline(always)]
  pub fn wakeupec(&mut self) -> WakeupecW<'_, DevidrSpec> {
    WakeupecW::new(self, 4)
  }
  #[doc = "Bit 5 - End of Resume Interrupt Disable"]
  #[inline(always)]
  pub fn eorsmec(&mut self) -> EorsmecW<'_, DevidrSpec> {
    EorsmecW::new(self, 5)
  }
  #[doc = "Bit 6 - Upstream Resume Interrupt Disable"]
  #[inline(always)]
  pub fn uprsmec(&mut self) -> UprsmecW<'_, DevidrSpec> {
    UprsmecW::new(self, 6)
  }
  #[doc = "Bit 12 - Endpoint 0 Interrupt Disable"]
  #[inline(always)]
  pub fn pep_0(&mut self) -> Pep0W<'_, DevidrSpec> {
    Pep0W::new(self, 12)
  }
  #[doc = "Bit 13 - Endpoint 1 Interrupt Disable"]
  #[inline(always)]
  pub fn pep_1(&mut self) -> Pep1W<'_, DevidrSpec> {
    Pep1W::new(self, 13)
  }
  #[doc = "Bit 14 - Endpoint 2 Interrupt Disable"]
  #[inline(always)]
  pub fn pep_2(&mut self) -> Pep2W<'_, DevidrSpec> {
    Pep2W::new(self, 14)
  }
  #[doc = "Bit 15 - Endpoint 3 Interrupt Disable"]
  #[inline(always)]
  pub fn pep_3(&mut self) -> Pep3W<'_, DevidrSpec> {
    Pep3W::new(self, 15)
  }
  #[doc = "Bit 16 - Endpoint 4 Interrupt Disable"]
  #[inline(always)]
  pub fn pep_4(&mut self) -> Pep4W<'_, DevidrSpec> {
    Pep4W::new(self, 16)
  }
  #[doc = "Bit 17 - Endpoint 5 Interrupt Disable"]
  #[inline(always)]
  pub fn pep_5(&mut self) -> Pep5W<'_, DevidrSpec> {
    Pep5W::new(self, 17)
  }
  #[doc = "Bit 18 - Endpoint 6 Interrupt Disable"]
  #[inline(always)]
  pub fn pep_6(&mut self) -> Pep6W<'_, DevidrSpec> {
    Pep6W::new(self, 18)
  }
  #[doc = "Bit 19 - Endpoint 7 Interrupt Disable"]
  #[inline(always)]
  pub fn pep_7(&mut self) -> Pep7W<'_, DevidrSpec> {
    Pep7W::new(self, 19)
  }
  #[doc = "Bit 20 - Endpoint 8 Interrupt Disable"]
  #[inline(always)]
  pub fn pep_8(&mut self) -> Pep8W<'_, DevidrSpec> {
    Pep8W::new(self, 20)
  }
  #[doc = "Bit 21 - Endpoint 9 Interrupt Disable"]
  #[inline(always)]
  pub fn pep_9(&mut self) -> Pep9W<'_, DevidrSpec> {
    Pep9W::new(self, 21)
  }
  #[doc = "Bit 25 - DMA Channel 1 Interrupt Disable"]
  #[inline(always)]
  pub fn dma_1(&mut self) -> Dma1W<'_, DevidrSpec> {
    Dma1W::new(self, 25)
  }
  #[doc = "Bit 26 - DMA Channel 2 Interrupt Disable"]
  #[inline(always)]
  pub fn dma_2(&mut self) -> Dma2W<'_, DevidrSpec> {
    Dma2W::new(self, 26)
  }
  #[doc = "Bit 27 - DMA Channel 3 Interrupt Disable"]
  #[inline(always)]
  pub fn dma_3(&mut self) -> Dma3W<'_, DevidrSpec> {
    Dma3W::new(self, 27)
  }
  #[doc = "Bit 28 - DMA Channel 4 Interrupt Disable"]
  #[inline(always)]
  pub fn dma_4(&mut self) -> Dma4W<'_, DevidrSpec> {
    Dma4W::new(self, 28)
  }
  #[doc = "Bit 29 - DMA Channel 5 Interrupt Disable"]
  #[inline(always)]
  pub fn dma_5(&mut self) -> Dma5W<'_, DevidrSpec> {
    Dma5W::new(self, 29)
  }
  #[doc = "Bit 30 - DMA Channel 6 Interrupt Disable"]
  #[inline(always)]
  pub fn dma_6(&mut self) -> Dma6W<'_, DevidrSpec> {
    Dma6W::new(self, 30)
  }
}
#[doc = "Device Global Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevidrSpec;
impl crate::RegisterSpec for DevidrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devidr::W`](W) writer structure"]
impl crate::Writable for DevidrSpec {
  type Safety = crate::Unsafe;
}
