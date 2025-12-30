#[doc = "Register `FELLSR` writer"]
pub type W = crate::W<FellsrSpec>;
#[doc = "Field `P0` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Falling Edge/Low Level Interrupt Selection."]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p0(&mut self) -> P0W<'_, FellsrSpec> {
    P0W::new(self, 0)
  }
  #[doc = "Bit 1 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p1(&mut self) -> P1W<'_, FellsrSpec> {
    P1W::new(self, 1)
  }
  #[doc = "Bit 2 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p2(&mut self) -> P2W<'_, FellsrSpec> {
    P2W::new(self, 2)
  }
  #[doc = "Bit 3 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p3(&mut self) -> P3W<'_, FellsrSpec> {
    P3W::new(self, 3)
  }
  #[doc = "Bit 4 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p4(&mut self) -> P4W<'_, FellsrSpec> {
    P4W::new(self, 4)
  }
  #[doc = "Bit 5 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p5(&mut self) -> P5W<'_, FellsrSpec> {
    P5W::new(self, 5)
  }
  #[doc = "Bit 6 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p6(&mut self) -> P6W<'_, FellsrSpec> {
    P6W::new(self, 6)
  }
  #[doc = "Bit 7 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p7(&mut self) -> P7W<'_, FellsrSpec> {
    P7W::new(self, 7)
  }
  #[doc = "Bit 8 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p8(&mut self) -> P8W<'_, FellsrSpec> {
    P8W::new(self, 8)
  }
  #[doc = "Bit 9 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p9(&mut self) -> P9W<'_, FellsrSpec> {
    P9W::new(self, 9)
  }
  #[doc = "Bit 10 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p10(&mut self) -> P10W<'_, FellsrSpec> {
    P10W::new(self, 10)
  }
  #[doc = "Bit 11 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p11(&mut self) -> P11W<'_, FellsrSpec> {
    P11W::new(self, 11)
  }
  #[doc = "Bit 12 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p12(&mut self) -> P12W<'_, FellsrSpec> {
    P12W::new(self, 12)
  }
  #[doc = "Bit 13 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p13(&mut self) -> P13W<'_, FellsrSpec> {
    P13W::new(self, 13)
  }
  #[doc = "Bit 14 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p14(&mut self) -> P14W<'_, FellsrSpec> {
    P14W::new(self, 14)
  }
  #[doc = "Bit 15 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p15(&mut self) -> P15W<'_, FellsrSpec> {
    P15W::new(self, 15)
  }
  #[doc = "Bit 16 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p16(&mut self) -> P16W<'_, FellsrSpec> {
    P16W::new(self, 16)
  }
  #[doc = "Bit 17 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p17(&mut self) -> P17W<'_, FellsrSpec> {
    P17W::new(self, 17)
  }
  #[doc = "Bit 18 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p18(&mut self) -> P18W<'_, FellsrSpec> {
    P18W::new(self, 18)
  }
  #[doc = "Bit 19 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p19(&mut self) -> P19W<'_, FellsrSpec> {
    P19W::new(self, 19)
  }
  #[doc = "Bit 20 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p20(&mut self) -> P20W<'_, FellsrSpec> {
    P20W::new(self, 20)
  }
  #[doc = "Bit 21 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p21(&mut self) -> P21W<'_, FellsrSpec> {
    P21W::new(self, 21)
  }
  #[doc = "Bit 22 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p22(&mut self) -> P22W<'_, FellsrSpec> {
    P22W::new(self, 22)
  }
  #[doc = "Bit 23 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p23(&mut self) -> P23W<'_, FellsrSpec> {
    P23W::new(self, 23)
  }
  #[doc = "Bit 24 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p24(&mut self) -> P24W<'_, FellsrSpec> {
    P24W::new(self, 24)
  }
  #[doc = "Bit 25 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p25(&mut self) -> P25W<'_, FellsrSpec> {
    P25W::new(self, 25)
  }
  #[doc = "Bit 26 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p26(&mut self) -> P26W<'_, FellsrSpec> {
    P26W::new(self, 26)
  }
  #[doc = "Bit 27 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p27(&mut self) -> P27W<'_, FellsrSpec> {
    P27W::new(self, 27)
  }
  #[doc = "Bit 28 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p28(&mut self) -> P28W<'_, FellsrSpec> {
    P28W::new(self, 28)
  }
  #[doc = "Bit 29 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p29(&mut self) -> P29W<'_, FellsrSpec> {
    P29W::new(self, 29)
  }
  #[doc = "Bit 30 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p30(&mut self) -> P30W<'_, FellsrSpec> {
    P30W::new(self, 30)
  }
  #[doc = "Bit 31 - Falling Edge/Low Level Interrupt Selection."]
  #[inline(always)]
  pub fn p31(&mut self) -> P31W<'_, FellsrSpec> {
    P31W::new(self, 31)
  }
}
#[doc = "Falling Edge/Low Level Select Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fellsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FellsrSpec;
impl crate::RegisterSpec for FellsrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fellsr::W`](W) writer structure"]
impl crate::Writable for FellsrSpec {
  type Safety = crate::Unsafe;
}
