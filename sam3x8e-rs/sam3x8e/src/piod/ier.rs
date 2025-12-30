#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `P0` writer - Input Change Interrupt Enable"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Input Change Interrupt Enable"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Input Change Interrupt Enable"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Input Change Interrupt Enable"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Input Change Interrupt Enable"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Input Change Interrupt Enable"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Input Change Interrupt Enable"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Input Change Interrupt Enable"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Input Change Interrupt Enable"]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Input Change Interrupt Enable"]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Input Change Interrupt Enable"]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Input Change Interrupt Enable"]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Input Change Interrupt Enable"]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Input Change Interrupt Enable"]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Input Change Interrupt Enable"]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Input Change Interrupt Enable"]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Input Change Interrupt Enable"]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Input Change Interrupt Enable"]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Input Change Interrupt Enable"]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Input Change Interrupt Enable"]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Input Change Interrupt Enable"]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Input Change Interrupt Enable"]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Input Change Interrupt Enable"]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Input Change Interrupt Enable"]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Input Change Interrupt Enable"]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Input Change Interrupt Enable"]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Input Change Interrupt Enable"]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Input Change Interrupt Enable"]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Input Change Interrupt Enable"]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Input Change Interrupt Enable"]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Input Change Interrupt Enable"]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Input Change Interrupt Enable"]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p0(&mut self) -> P0W<'_, IerSpec> {
    P0W::new(self, 0)
  }
  #[doc = "Bit 1 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p1(&mut self) -> P1W<'_, IerSpec> {
    P1W::new(self, 1)
  }
  #[doc = "Bit 2 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p2(&mut self) -> P2W<'_, IerSpec> {
    P2W::new(self, 2)
  }
  #[doc = "Bit 3 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p3(&mut self) -> P3W<'_, IerSpec> {
    P3W::new(self, 3)
  }
  #[doc = "Bit 4 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p4(&mut self) -> P4W<'_, IerSpec> {
    P4W::new(self, 4)
  }
  #[doc = "Bit 5 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p5(&mut self) -> P5W<'_, IerSpec> {
    P5W::new(self, 5)
  }
  #[doc = "Bit 6 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p6(&mut self) -> P6W<'_, IerSpec> {
    P6W::new(self, 6)
  }
  #[doc = "Bit 7 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p7(&mut self) -> P7W<'_, IerSpec> {
    P7W::new(self, 7)
  }
  #[doc = "Bit 8 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p8(&mut self) -> P8W<'_, IerSpec> {
    P8W::new(self, 8)
  }
  #[doc = "Bit 9 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p9(&mut self) -> P9W<'_, IerSpec> {
    P9W::new(self, 9)
  }
  #[doc = "Bit 10 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p10(&mut self) -> P10W<'_, IerSpec> {
    P10W::new(self, 10)
  }
  #[doc = "Bit 11 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p11(&mut self) -> P11W<'_, IerSpec> {
    P11W::new(self, 11)
  }
  #[doc = "Bit 12 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p12(&mut self) -> P12W<'_, IerSpec> {
    P12W::new(self, 12)
  }
  #[doc = "Bit 13 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p13(&mut self) -> P13W<'_, IerSpec> {
    P13W::new(self, 13)
  }
  #[doc = "Bit 14 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p14(&mut self) -> P14W<'_, IerSpec> {
    P14W::new(self, 14)
  }
  #[doc = "Bit 15 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p15(&mut self) -> P15W<'_, IerSpec> {
    P15W::new(self, 15)
  }
  #[doc = "Bit 16 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p16(&mut self) -> P16W<'_, IerSpec> {
    P16W::new(self, 16)
  }
  #[doc = "Bit 17 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p17(&mut self) -> P17W<'_, IerSpec> {
    P17W::new(self, 17)
  }
  #[doc = "Bit 18 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p18(&mut self) -> P18W<'_, IerSpec> {
    P18W::new(self, 18)
  }
  #[doc = "Bit 19 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p19(&mut self) -> P19W<'_, IerSpec> {
    P19W::new(self, 19)
  }
  #[doc = "Bit 20 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p20(&mut self) -> P20W<'_, IerSpec> {
    P20W::new(self, 20)
  }
  #[doc = "Bit 21 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p21(&mut self) -> P21W<'_, IerSpec> {
    P21W::new(self, 21)
  }
  #[doc = "Bit 22 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p22(&mut self) -> P22W<'_, IerSpec> {
    P22W::new(self, 22)
  }
  #[doc = "Bit 23 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p23(&mut self) -> P23W<'_, IerSpec> {
    P23W::new(self, 23)
  }
  #[doc = "Bit 24 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p24(&mut self) -> P24W<'_, IerSpec> {
    P24W::new(self, 24)
  }
  #[doc = "Bit 25 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p25(&mut self) -> P25W<'_, IerSpec> {
    P25W::new(self, 25)
  }
  #[doc = "Bit 26 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p26(&mut self) -> P26W<'_, IerSpec> {
    P26W::new(self, 26)
  }
  #[doc = "Bit 27 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p27(&mut self) -> P27W<'_, IerSpec> {
    P27W::new(self, 27)
  }
  #[doc = "Bit 28 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p28(&mut self) -> P28W<'_, IerSpec> {
    P28W::new(self, 28)
  }
  #[doc = "Bit 29 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p29(&mut self) -> P29W<'_, IerSpec> {
    P29W::new(self, 29)
  }
  #[doc = "Bit 30 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p30(&mut self) -> P30W<'_, IerSpec> {
    P30W::new(self, 30)
  }
  #[doc = "Bit 31 - Input Change Interrupt Enable"]
  #[inline(always)]
  pub fn p31(&mut self) -> P31W<'_, IerSpec> {
    P31W::new(self, 31)
  }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
  type Safety = crate::Unsafe;
}
