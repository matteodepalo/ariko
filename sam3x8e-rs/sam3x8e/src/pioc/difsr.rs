#[doc = "Register `DIFSR` writer"]
pub type W = crate::W<DifsrSpec>;
#[doc = "Field `P0` writer - Debouncing Filtering Select."]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Debouncing Filtering Select."]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Debouncing Filtering Select."]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Debouncing Filtering Select."]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Debouncing Filtering Select."]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Debouncing Filtering Select."]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Debouncing Filtering Select."]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Debouncing Filtering Select."]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Debouncing Filtering Select."]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Debouncing Filtering Select."]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Debouncing Filtering Select."]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Debouncing Filtering Select."]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Debouncing Filtering Select."]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Debouncing Filtering Select."]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Debouncing Filtering Select."]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Debouncing Filtering Select."]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Debouncing Filtering Select."]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Debouncing Filtering Select."]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Debouncing Filtering Select."]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Debouncing Filtering Select."]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Debouncing Filtering Select."]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Debouncing Filtering Select."]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Debouncing Filtering Select."]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Debouncing Filtering Select."]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Debouncing Filtering Select."]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Debouncing Filtering Select."]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Debouncing Filtering Select."]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Debouncing Filtering Select."]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Debouncing Filtering Select."]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Debouncing Filtering Select."]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Debouncing Filtering Select."]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Debouncing Filtering Select."]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p0(&mut self) -> P0W<'_, DifsrSpec> {
    P0W::new(self, 0)
  }
  #[doc = "Bit 1 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p1(&mut self) -> P1W<'_, DifsrSpec> {
    P1W::new(self, 1)
  }
  #[doc = "Bit 2 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p2(&mut self) -> P2W<'_, DifsrSpec> {
    P2W::new(self, 2)
  }
  #[doc = "Bit 3 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p3(&mut self) -> P3W<'_, DifsrSpec> {
    P3W::new(self, 3)
  }
  #[doc = "Bit 4 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p4(&mut self) -> P4W<'_, DifsrSpec> {
    P4W::new(self, 4)
  }
  #[doc = "Bit 5 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p5(&mut self) -> P5W<'_, DifsrSpec> {
    P5W::new(self, 5)
  }
  #[doc = "Bit 6 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p6(&mut self) -> P6W<'_, DifsrSpec> {
    P6W::new(self, 6)
  }
  #[doc = "Bit 7 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p7(&mut self) -> P7W<'_, DifsrSpec> {
    P7W::new(self, 7)
  }
  #[doc = "Bit 8 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p8(&mut self) -> P8W<'_, DifsrSpec> {
    P8W::new(self, 8)
  }
  #[doc = "Bit 9 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p9(&mut self) -> P9W<'_, DifsrSpec> {
    P9W::new(self, 9)
  }
  #[doc = "Bit 10 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p10(&mut self) -> P10W<'_, DifsrSpec> {
    P10W::new(self, 10)
  }
  #[doc = "Bit 11 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p11(&mut self) -> P11W<'_, DifsrSpec> {
    P11W::new(self, 11)
  }
  #[doc = "Bit 12 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p12(&mut self) -> P12W<'_, DifsrSpec> {
    P12W::new(self, 12)
  }
  #[doc = "Bit 13 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p13(&mut self) -> P13W<'_, DifsrSpec> {
    P13W::new(self, 13)
  }
  #[doc = "Bit 14 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p14(&mut self) -> P14W<'_, DifsrSpec> {
    P14W::new(self, 14)
  }
  #[doc = "Bit 15 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p15(&mut self) -> P15W<'_, DifsrSpec> {
    P15W::new(self, 15)
  }
  #[doc = "Bit 16 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p16(&mut self) -> P16W<'_, DifsrSpec> {
    P16W::new(self, 16)
  }
  #[doc = "Bit 17 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p17(&mut self) -> P17W<'_, DifsrSpec> {
    P17W::new(self, 17)
  }
  #[doc = "Bit 18 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p18(&mut self) -> P18W<'_, DifsrSpec> {
    P18W::new(self, 18)
  }
  #[doc = "Bit 19 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p19(&mut self) -> P19W<'_, DifsrSpec> {
    P19W::new(self, 19)
  }
  #[doc = "Bit 20 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p20(&mut self) -> P20W<'_, DifsrSpec> {
    P20W::new(self, 20)
  }
  #[doc = "Bit 21 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p21(&mut self) -> P21W<'_, DifsrSpec> {
    P21W::new(self, 21)
  }
  #[doc = "Bit 22 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p22(&mut self) -> P22W<'_, DifsrSpec> {
    P22W::new(self, 22)
  }
  #[doc = "Bit 23 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p23(&mut self) -> P23W<'_, DifsrSpec> {
    P23W::new(self, 23)
  }
  #[doc = "Bit 24 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p24(&mut self) -> P24W<'_, DifsrSpec> {
    P24W::new(self, 24)
  }
  #[doc = "Bit 25 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p25(&mut self) -> P25W<'_, DifsrSpec> {
    P25W::new(self, 25)
  }
  #[doc = "Bit 26 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p26(&mut self) -> P26W<'_, DifsrSpec> {
    P26W::new(self, 26)
  }
  #[doc = "Bit 27 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p27(&mut self) -> P27W<'_, DifsrSpec> {
    P27W::new(self, 27)
  }
  #[doc = "Bit 28 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p28(&mut self) -> P28W<'_, DifsrSpec> {
    P28W::new(self, 28)
  }
  #[doc = "Bit 29 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p29(&mut self) -> P29W<'_, DifsrSpec> {
    P29W::new(self, 29)
  }
  #[doc = "Bit 30 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p30(&mut self) -> P30W<'_, DifsrSpec> {
    P30W::new(self, 30)
  }
  #[doc = "Bit 31 - Debouncing Filtering Select."]
  #[inline(always)]
  pub fn p31(&mut self) -> P31W<'_, DifsrSpec> {
    P31W::new(self, 31)
  }
}
#[doc = "Debouncing Input Filter Select Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DifsrSpec;
impl crate::RegisterSpec for DifsrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`difsr::W`](W) writer structure"]
impl crate::Writable for DifsrSpec {
  type Safety = crate::Unsafe;
}
