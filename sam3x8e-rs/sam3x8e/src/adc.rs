#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
  cr: Cr,
  mr: Mr,
  seqr1: Seqr1,
  seqr2: Seqr2,
  cher: Cher,
  chdr: Chdr,
  chsr: Chsr,
  _reserved7: [u8; 0x04],
  lcdr: Lcdr,
  ier: Ier,
  idr: Idr,
  imr: Imr,
  isr: Isr,
  _reserved12: [u8; 0x08],
  over: Over,
  emr: Emr,
  cwr: Cwr,
  cgr: Cgr,
  cor: Cor,
  cdr: [Cdr; 16],
  _reserved18: [u8; 0x04],
  acr: Acr,
  _reserved19: [u8; 0x4c],
  wpmr: Wpmr,
  wpsr: Wpsr,
  _reserved21: [u8; 0x14],
  rpr: Rpr,
  rcr: Rcr,
  _reserved23: [u8; 0x08],
  rnpr: Rnpr,
  rncr: Rncr,
  _reserved25: [u8; 0x08],
  ptcr: Ptcr,
  ptsr: Ptsr,
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
  #[doc = "0x08 - Channel Sequence Register 1"]
  #[inline(always)]
  pub const fn seqr1(&self) -> &Seqr1 {
    &self.seqr1
  }
  #[doc = "0x0c - Channel Sequence Register 2"]
  #[inline(always)]
  pub const fn seqr2(&self) -> &Seqr2 {
    &self.seqr2
  }
  #[doc = "0x10 - Channel Enable Register"]
  #[inline(always)]
  pub const fn cher(&self) -> &Cher {
    &self.cher
  }
  #[doc = "0x14 - Channel Disable Register"]
  #[inline(always)]
  pub const fn chdr(&self) -> &Chdr {
    &self.chdr
  }
  #[doc = "0x18 - Channel Status Register"]
  #[inline(always)]
  pub const fn chsr(&self) -> &Chsr {
    &self.chsr
  }
  #[doc = "0x20 - Last Converted Data Register"]
  #[inline(always)]
  pub const fn lcdr(&self) -> &Lcdr {
    &self.lcdr
  }
  #[doc = "0x24 - Interrupt Enable Register"]
  #[inline(always)]
  pub const fn ier(&self) -> &Ier {
    &self.ier
  }
  #[doc = "0x28 - Interrupt Disable Register"]
  #[inline(always)]
  pub const fn idr(&self) -> &Idr {
    &self.idr
  }
  #[doc = "0x2c - Interrupt Mask Register"]
  #[inline(always)]
  pub const fn imr(&self) -> &Imr {
    &self.imr
  }
  #[doc = "0x30 - Interrupt Status Register"]
  #[inline(always)]
  pub const fn isr(&self) -> &Isr {
    &self.isr
  }
  #[doc = "0x3c - Overrun Status Register"]
  #[inline(always)]
  pub const fn over(&self) -> &Over {
    &self.over
  }
  #[doc = "0x40 - Extended Mode Register"]
  #[inline(always)]
  pub const fn emr(&self) -> &Emr {
    &self.emr
  }
  #[doc = "0x44 - Compare Window Register"]
  #[inline(always)]
  pub const fn cwr(&self) -> &Cwr {
    &self.cwr
  }
  #[doc = "0x48 - Channel Gain Register"]
  #[inline(always)]
  pub const fn cgr(&self) -> &Cgr {
    &self.cgr
  }
  #[doc = "0x4c - Channel Offset Register"]
  #[inline(always)]
  pub const fn cor(&self) -> &Cor {
    &self.cor
  }
  #[doc = "0x50..0x90 - Channel Data Register"]
  #[inline(always)]
  pub const fn cdr(&self, n: usize) -> &Cdr {
    &self.cdr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x50..0x90 - Channel Data Register"]
  #[inline(always)]
  pub fn cdr_iter(&self) -> impl Iterator<Item = &Cdr> {
    self.cdr.iter()
  }
  #[doc = "0x94 - Analog Control Register"]
  #[inline(always)]
  pub const fn acr(&self) -> &Acr {
    &self.acr
  }
  #[doc = "0xe4 - Write Protect Mode Register"]
  #[inline(always)]
  pub const fn wpmr(&self) -> &Wpmr {
    &self.wpmr
  }
  #[doc = "0xe8 - Write Protect Status Register"]
  #[inline(always)]
  pub const fn wpsr(&self) -> &Wpsr {
    &self.wpsr
  }
  #[doc = "0x100 - Receive Pointer Register"]
  #[inline(always)]
  pub const fn rpr(&self) -> &Rpr {
    &self.rpr
  }
  #[doc = "0x104 - Receive Counter Register"]
  #[inline(always)]
  pub const fn rcr(&self) -> &Rcr {
    &self.rcr
  }
  #[doc = "0x110 - Receive Next Pointer Register"]
  #[inline(always)]
  pub const fn rnpr(&self) -> &Rnpr {
    &self.rnpr
  }
  #[doc = "0x114 - Receive Next Counter Register"]
  #[inline(always)]
  pub const fn rncr(&self) -> &Rncr {
    &self.rncr
  }
  #[doc = "0x120 - Transfer Control Register"]
  #[inline(always)]
  pub const fn ptcr(&self) -> &Ptcr {
    &self.ptcr
  }
  #[doc = "0x124 - Transfer Status Register"]
  #[inline(always)]
  pub const fn ptsr(&self) -> &Ptsr {
    &self.ptsr
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
#[doc = "SEQR1 (rw) register accessor: Channel Sequence Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`seqr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqr1`] module"]
#[doc(alias = "SEQR1")]
pub type Seqr1 = crate::Reg<seqr1::Seqr1Spec>;
#[doc = "Channel Sequence Register 1"]
pub mod seqr1;
#[doc = "SEQR2 (rw) register accessor: Channel Sequence Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`seqr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqr2`] module"]
#[doc(alias = "SEQR2")]
pub type Seqr2 = crate::Reg<seqr2::Seqr2Spec>;
#[doc = "Channel Sequence Register 2"]
pub mod seqr2;
#[doc = "CHER (w) register accessor: Channel Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cher::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cher`] module"]
#[doc(alias = "CHER")]
pub type Cher = crate::Reg<cher::CherSpec>;
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "CHDR (w) register accessor: Channel Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdr`] module"]
#[doc(alias = "CHDR")]
pub type Chdr = crate::Reg<chdr::ChdrSpec>;
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "CHSR (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsr`] module"]
#[doc(alias = "CHSR")]
pub type Chsr = crate::Reg<chsr::ChsrSpec>;
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "LCDR (r) register accessor: Last Converted Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdr`] module"]
#[doc(alias = "LCDR")]
pub type Lcdr = crate::Reg<lcdr::LcdrSpec>;
#[doc = "Last Converted Data Register"]
pub mod lcdr;
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
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "OVER (r) register accessor: Overrun Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`over::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@over`] module"]
#[doc(alias = "OVER")]
pub type Over = crate::Reg<over::OverSpec>;
#[doc = "Overrun Status Register"]
pub mod over;
#[doc = "EMR (rw) register accessor: Extended Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`] module"]
#[doc(alias = "EMR")]
pub type Emr = crate::Reg<emr::EmrSpec>;
#[doc = "Extended Mode Register"]
pub mod emr;
#[doc = "CWR (rw) register accessor: Compare Window Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwr`] module"]
#[doc(alias = "CWR")]
pub type Cwr = crate::Reg<cwr::CwrSpec>;
#[doc = "Compare Window Register"]
pub mod cwr;
#[doc = "CGR (rw) register accessor: Channel Gain Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgr`] module"]
#[doc(alias = "CGR")]
pub type Cgr = crate::Reg<cgr::CgrSpec>;
#[doc = "Channel Gain Register"]
pub mod cgr;
#[doc = "COR (rw) register accessor: Channel Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cor`] module"]
#[doc(alias = "COR")]
pub type Cor = crate::Reg<cor::CorSpec>;
#[doc = "Channel Offset Register"]
pub mod cor;
#[doc = "CDR (r) register accessor: Channel Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr`] module"]
#[doc(alias = "CDR")]
pub type Cdr = crate::Reg<cdr::CdrSpec>;
#[doc = "Channel Data Register"]
pub mod cdr;
#[doc = "ACR (rw) register accessor: Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Analog Control Register"]
pub mod acr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "RPR (rw) register accessor: Receive Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr`] module"]
#[doc(alias = "RPR")]
pub type Rpr = crate::Reg<rpr::RprSpec>;
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "RCR (rw) register accessor: Receive Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`] module"]
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RcrSpec>;
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "RNPR (rw) register accessor: Receive Next Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnpr`] module"]
#[doc(alias = "RNPR")]
pub type Rnpr = crate::Reg<rnpr::RnprSpec>;
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "RNCR (rw) register accessor: Receive Next Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rncr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rncr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rncr`] module"]
#[doc(alias = "RNCR")]
pub type Rncr = crate::Reg<rncr::RncrSpec>;
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "PTCR (w) register accessor: Transfer Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptcr`] module"]
#[doc(alias = "PTCR")]
pub type Ptcr = crate::Reg<ptcr::PtcrSpec>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR (r) register accessor: Transfer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptsr`] module"]
#[doc(alias = "PTSR")]
pub type Ptsr = crate::Reg<ptsr::PtsrSpec>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
