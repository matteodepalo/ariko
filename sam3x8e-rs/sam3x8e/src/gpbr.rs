#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
  gpbr: [Gpbr; 8],
}
impl RegisterBlock {
  #[doc = "0x00..0x20 - General Purpose Backup Register"]
  #[inline(always)]
  pub const fn gpbr(&self, n: usize) -> &Gpbr {
    &self.gpbr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x00..0x20 - General Purpose Backup Register"]
  #[inline(always)]
  pub fn gpbr_iter(&self) -> impl Iterator<Item = &Gpbr> {
    self.gpbr.iter()
  }
}
#[doc = "GPBR (rw) register accessor: General Purpose Backup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr`] module"]
#[doc(alias = "GPBR")]
pub type Gpbr = crate::Reg<gpbr::GpbrSpec>;
#[doc = "General Purpose Backup Register"]
pub mod gpbr;
