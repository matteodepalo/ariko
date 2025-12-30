#[doc = "Register `QIER` writer"]
pub type W = crate::W<QierSpec>;
#[doc = "Field `IDX` writer - InDeX"]
pub type IdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` writer - DIRection CHanGe"]
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QERR` writer - Quadrature ERRor"]
pub type QerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
  #[doc = "Bit 0 - InDeX"]
  #[inline(always)]
  pub fn idx(&mut self) -> IdxW<'_, QierSpec> {
    IdxW::new(self, 0)
  }
  #[doc = "Bit 1 - DIRection CHanGe"]
  #[inline(always)]
  pub fn dirchg(&mut self) -> DirchgW<'_, QierSpec> {
    DirchgW::new(self, 1)
  }
  #[doc = "Bit 2 - Quadrature ERRor"]
  #[inline(always)]
  pub fn qerr(&mut self) -> QerrW<'_, QierSpec> {
    QerrW::new(self, 2)
  }
}
#[doc = "QDEC Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QierSpec;
impl crate::RegisterSpec for QierSpec {
  type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qier::W`](W) writer structure"]
impl crate::Writable for QierSpec {
  type Safety = crate::Unsafe;
}
