#[doc = "Register `SODR` writer"]
pub type W = crate::W<SodrSpec>;
#[doc = "Field `P0` writer - Set Output Data"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Set Output Data"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Set Output Data"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Set Output Data"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Set Output Data"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Set Output Data"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Set Output Data"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Set Output Data"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Set Output Data"]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Set Output Data"]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Set Output Data"]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Set Output Data"]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Set Output Data"]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Set Output Data"]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Set Output Data"]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Set Output Data"]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Set Output Data"]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Set Output Data"]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Set Output Data"]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Set Output Data"]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Set Output Data"]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Set Output Data"]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Set Output Data"]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Set Output Data"]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Set Output Data"]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Set Output Data"]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Set Output Data"]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Set Output Data"]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Set Output Data"]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Set Output Data"]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Set Output Data"]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Set Output Data"]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - Set Output Data"]
  #[inline(always)]
  pub fn p0(&mut self) -> P0W<'_, SodrSpec> {
    P0W::new(self, 0)
  }
  #[doc = "Bit 1 - Set Output Data"]
  #[inline(always)]
  pub fn p1(&mut self) -> P1W<'_, SodrSpec> {
    P1W::new(self, 1)
  }
  #[doc = "Bit 2 - Set Output Data"]
  #[inline(always)]
  pub fn p2(&mut self) -> P2W<'_, SodrSpec> {
    P2W::new(self, 2)
  }
  #[doc = "Bit 3 - Set Output Data"]
  #[inline(always)]
  pub fn p3(&mut self) -> P3W<'_, SodrSpec> {
    P3W::new(self, 3)
  }
  #[doc = "Bit 4 - Set Output Data"]
  #[inline(always)]
  pub fn p4(&mut self) -> P4W<'_, SodrSpec> {
    P4W::new(self, 4)
  }
  #[doc = "Bit 5 - Set Output Data"]
  #[inline(always)]
  pub fn p5(&mut self) -> P5W<'_, SodrSpec> {
    P5W::new(self, 5)
  }
  #[doc = "Bit 6 - Set Output Data"]
  #[inline(always)]
  pub fn p6(&mut self) -> P6W<'_, SodrSpec> {
    P6W::new(self, 6)
  }
  #[doc = "Bit 7 - Set Output Data"]
  #[inline(always)]
  pub fn p7(&mut self) -> P7W<'_, SodrSpec> {
    P7W::new(self, 7)
  }
  #[doc = "Bit 8 - Set Output Data"]
  #[inline(always)]
  pub fn p8(&mut self) -> P8W<'_, SodrSpec> {
    P8W::new(self, 8)
  }
  #[doc = "Bit 9 - Set Output Data"]
  #[inline(always)]
  pub fn p9(&mut self) -> P9W<'_, SodrSpec> {
    P9W::new(self, 9)
  }
  #[doc = "Bit 10 - Set Output Data"]
  #[inline(always)]
  pub fn p10(&mut self) -> P10W<'_, SodrSpec> {
    P10W::new(self, 10)
  }
  #[doc = "Bit 11 - Set Output Data"]
  #[inline(always)]
  pub fn p11(&mut self) -> P11W<'_, SodrSpec> {
    P11W::new(self, 11)
  }
  #[doc = "Bit 12 - Set Output Data"]
  #[inline(always)]
  pub fn p12(&mut self) -> P12W<'_, SodrSpec> {
    P12W::new(self, 12)
  }
  #[doc = "Bit 13 - Set Output Data"]
  #[inline(always)]
  pub fn p13(&mut self) -> P13W<'_, SodrSpec> {
    P13W::new(self, 13)
  }
  #[doc = "Bit 14 - Set Output Data"]
  #[inline(always)]
  pub fn p14(&mut self) -> P14W<'_, SodrSpec> {
    P14W::new(self, 14)
  }
  #[doc = "Bit 15 - Set Output Data"]
  #[inline(always)]
  pub fn p15(&mut self) -> P15W<'_, SodrSpec> {
    P15W::new(self, 15)
  }
  #[doc = "Bit 16 - Set Output Data"]
  #[inline(always)]
  pub fn p16(&mut self) -> P16W<'_, SodrSpec> {
    P16W::new(self, 16)
  }
  #[doc = "Bit 17 - Set Output Data"]
  #[inline(always)]
  pub fn p17(&mut self) -> P17W<'_, SodrSpec> {
    P17W::new(self, 17)
  }
  #[doc = "Bit 18 - Set Output Data"]
  #[inline(always)]
  pub fn p18(&mut self) -> P18W<'_, SodrSpec> {
    P18W::new(self, 18)
  }
  #[doc = "Bit 19 - Set Output Data"]
  #[inline(always)]
  pub fn p19(&mut self) -> P19W<'_, SodrSpec> {
    P19W::new(self, 19)
  }
  #[doc = "Bit 20 - Set Output Data"]
  #[inline(always)]
  pub fn p20(&mut self) -> P20W<'_, SodrSpec> {
    P20W::new(self, 20)
  }
  #[doc = "Bit 21 - Set Output Data"]
  #[inline(always)]
  pub fn p21(&mut self) -> P21W<'_, SodrSpec> {
    P21W::new(self, 21)
  }
  #[doc = "Bit 22 - Set Output Data"]
  #[inline(always)]
  pub fn p22(&mut self) -> P22W<'_, SodrSpec> {
    P22W::new(self, 22)
  }
  #[doc = "Bit 23 - Set Output Data"]
  #[inline(always)]
  pub fn p23(&mut self) -> P23W<'_, SodrSpec> {
    P23W::new(self, 23)
  }
  #[doc = "Bit 24 - Set Output Data"]
  #[inline(always)]
  pub fn p24(&mut self) -> P24W<'_, SodrSpec> {
    P24W::new(self, 24)
  }
  #[doc = "Bit 25 - Set Output Data"]
  #[inline(always)]
  pub fn p25(&mut self) -> P25W<'_, SodrSpec> {
    P25W::new(self, 25)
  }
  #[doc = "Bit 26 - Set Output Data"]
  #[inline(always)]
  pub fn p26(&mut self) -> P26W<'_, SodrSpec> {
    P26W::new(self, 26)
  }
  #[doc = "Bit 27 - Set Output Data"]
  #[inline(always)]
  pub fn p27(&mut self) -> P27W<'_, SodrSpec> {
    P27W::new(self, 27)
  }
  #[doc = "Bit 28 - Set Output Data"]
  #[inline(always)]
  pub fn p28(&mut self) -> P28W<'_, SodrSpec> {
    P28W::new(self, 28)
  }
  #[doc = "Bit 29 - Set Output Data"]
  #[inline(always)]
  pub fn p29(&mut self) -> P29W<'_, SodrSpec> {
    P29W::new(self, 29)
  }
  #[doc = "Bit 30 - Set Output Data"]
  #[inline(always)]
  pub fn p30(&mut self) -> P30W<'_, SodrSpec> {
    P30W::new(self, 30)
  }
  #[doc = "Bit 31 - Set Output Data"]
  #[inline(always)]
  pub fn p31(&mut self) -> P31W<'_, SodrSpec> {
    P31W::new(self, 31)
  }
}
#[doc = "Set Output Data Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sodr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SodrSpec;
impl crate::RegisterSpec for SodrSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sodr::W`](W) writer structure"]
impl crate::Writable for SodrSpec {
  type Safety = crate::Unsafe;
}
