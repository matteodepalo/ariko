#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
  cr: Cr,
  mr: Mr,
  rdr: Rdr,
  tdr: Tdr,
  sr: Sr,
  ier: Ier,
  idr: Idr,
  imr: Imr,
  _reserved8: [u8; 0x10],
  csr: [Csr; 4],
  _reserved9: [u8; 0xa4],
  wpmr: Wpmr,
  wpsr: Wpsr,
}
impl RegisterBlock {
  #[doc = "0x00 - Control Register"]
  #[inline(always)]
  pub const fn cr(&self) -> &Cr {
    &self.cr
  }
  #[doc = "0x04 - Mode Register"]
  #[inline(always)]
  pub const fn mr(&self) -> &Mr {
    &self.mr
  }
  #[doc = "0x08 - Receive Data Register"]
  #[inline(always)]
  pub const fn rdr(&self) -> &Rdr {
    &self.rdr
  }
  #[doc = "0x0c - Transmit Data Register"]
  #[inline(always)]
  pub const fn tdr(&self) -> &Tdr {
    &self.tdr
  }
  #[doc = "0x10 - Status Register"]
  #[inline(always)]
  pub const fn sr(&self) -> &Sr {
    &self.sr
  }
  #[doc = "0x14 - Interrupt Enable Register"]
  #[inline(always)]
  pub const fn ier(&self) -> &Ier {
    &self.ier
  }
  #[doc = "0x18 - Interrupt Disable Register"]
  #[inline(always)]
  pub const fn idr(&self) -> &Idr {
    &self.idr
  }
  #[doc = "0x1c - Interrupt Mask Register"]
  #[inline(always)]
  pub const fn imr(&self) -> &Imr {
    &self.imr
  }
  #[doc = "0x30..0x40 - Chip Select Register"]
  #[inline(always)]
  pub const fn csr(&self, n: usize) -> &Csr {
    &self.csr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x30..0x40 - Chip Select Register"]
  #[inline(always)]
  pub fn csr_iter(&self) -> impl Iterator<Item = &Csr> {
    self.csr.iter()
  }
  #[doc = "0xe4 - Write Protection Control Register"]
  #[inline(always)]
  pub const fn wpmr(&self) -> &Wpmr {
    &self.wpmr
  }
  #[doc = "0xe8 - Write Protection Status Register"]
  #[inline(always)]
  pub const fn wpsr(&self) -> &Wpsr {
    &self.wpsr
  }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`] module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "RDR (r) register accessor: Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`] module"]
#[doc(alias = "RDR")]
pub type Rdr = crate::Reg<rdr::RdrSpec>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "TDR (w) register accessor: Transmit Data Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`] module"]
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TdrSpec>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "CSR (rw) register accessor: Chip Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Chip Select Register"]
pub mod csr;
#[doc = "WPMR (rw) register accessor: Write Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protection Control Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
