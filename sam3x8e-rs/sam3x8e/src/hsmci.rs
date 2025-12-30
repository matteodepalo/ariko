#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
  cr: Cr,
  mr: Mr,
  dtor: Dtor,
  sdcr: Sdcr,
  argr: Argr,
  cmdr: Cmdr,
  blkr: Blkr,
  cstor: Cstor,
  rspr: [Rspr; 4],
  rdr: Rdr,
  tdr: Tdr,
  _reserved11: [u8; 0x08],
  sr: Sr,
  ier: Ier,
  idr: Idr,
  imr: Imr,
  dma: Dma,
  cfg: Cfg,
  _reserved17: [u8; 0x8c],
  wpmr: Wpmr,
  wpsr: Wpsr,
  _reserved19: [u8; 0x0114],
  fifo: [Fifo; 256],
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
  #[doc = "0x08 - Data Timeout Register"]
  #[inline(always)]
  pub const fn dtor(&self) -> &Dtor {
    &self.dtor
  }
  #[doc = "0x0c - SD/SDIO Card Register"]
  #[inline(always)]
  pub const fn sdcr(&self) -> &Sdcr {
    &self.sdcr
  }
  #[doc = "0x10 - Argument Register"]
  #[inline(always)]
  pub const fn argr(&self) -> &Argr {
    &self.argr
  }
  #[doc = "0x14 - Command Register"]
  #[inline(always)]
  pub const fn cmdr(&self) -> &Cmdr {
    &self.cmdr
  }
  #[doc = "0x18 - Block Register"]
  #[inline(always)]
  pub const fn blkr(&self) -> &Blkr {
    &self.blkr
  }
  #[doc = "0x1c - Completion Signal Timeout Register"]
  #[inline(always)]
  pub const fn cstor(&self) -> &Cstor {
    &self.cstor
  }
  #[doc = "0x20..0x30 - Response Register"]
  #[inline(always)]
  pub const fn rspr(&self, n: usize) -> &Rspr {
    &self.rspr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x20..0x30 - Response Register"]
  #[inline(always)]
  pub fn rspr_iter(&self) -> impl Iterator<Item = &Rspr> {
    self.rspr.iter()
  }
  #[doc = "0x30 - Receive Data Register"]
  #[inline(always)]
  pub const fn rdr(&self) -> &Rdr {
    &self.rdr
  }
  #[doc = "0x34 - Transmit Data Register"]
  #[inline(always)]
  pub const fn tdr(&self) -> &Tdr {
    &self.tdr
  }
  #[doc = "0x40 - Status Register"]
  #[inline(always)]
  pub const fn sr(&self) -> &Sr {
    &self.sr
  }
  #[doc = "0x44 - Interrupt Enable Register"]
  #[inline(always)]
  pub const fn ier(&self) -> &Ier {
    &self.ier
  }
  #[doc = "0x48 - Interrupt Disable Register"]
  #[inline(always)]
  pub const fn idr(&self) -> &Idr {
    &self.idr
  }
  #[doc = "0x4c - Interrupt Mask Register"]
  #[inline(always)]
  pub const fn imr(&self) -> &Imr {
    &self.imr
  }
  #[doc = "0x50 - DMA Configuration Register"]
  #[inline(always)]
  pub const fn dma(&self) -> &Dma {
    &self.dma
  }
  #[doc = "0x54 - Configuration Register"]
  #[inline(always)]
  pub const fn cfg(&self) -> &Cfg {
    &self.cfg
  }
  #[doc = "0xe4 - Write Protection Mode Register"]
  #[inline(always)]
  pub const fn wpmr(&self) -> &Wpmr {
    &self.wpmr
  }
  #[doc = "0xe8 - Write Protection Status Register"]
  #[inline(always)]
  pub const fn wpsr(&self) -> &Wpsr {
    &self.wpsr
  }
  #[doc = "0x200..0x600 - FIFO Memory Aperture0"]
  #[inline(always)]
  pub const fn fifo(&self, n: usize) -> &Fifo {
    &self.fifo[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x200..0x600 - FIFO Memory Aperture0"]
  #[inline(always)]
  pub fn fifo_iter(&self) -> impl Iterator<Item = &Fifo> {
    self.fifo.iter()
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
#[doc = "DTOR (rw) register accessor: Data Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtor`] module"]
#[doc(alias = "DTOR")]
pub type Dtor = crate::Reg<dtor::DtorSpec>;
#[doc = "Data Timeout Register"]
pub mod dtor;
#[doc = "SDCR (rw) register accessor: SD/SDIO Card Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdcr`] module"]
#[doc(alias = "SDCR")]
pub type Sdcr = crate::Reg<sdcr::SdcrSpec>;
#[doc = "SD/SDIO Card Register"]
pub mod sdcr;
#[doc = "ARGR (rw) register accessor: Argument Register\n\nYou can [`read`](crate::Reg::read) this register and get [`argr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argr`] module"]
#[doc(alias = "ARGR")]
pub type Argr = crate::Reg<argr::ArgrSpec>;
#[doc = "Argument Register"]
pub mod argr;
#[doc = "CMDR (w) register accessor: Command Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdr`] module"]
#[doc(alias = "CMDR")]
pub type Cmdr = crate::Reg<cmdr::CmdrSpec>;
#[doc = "Command Register"]
pub mod cmdr;
#[doc = "BLKR (rw) register accessor: Block Register\n\nYou can [`read`](crate::Reg::read) this register and get [`blkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blkr`] module"]
#[doc(alias = "BLKR")]
pub type Blkr = crate::Reg<blkr::BlkrSpec>;
#[doc = "Block Register"]
pub mod blkr;
#[doc = "CSTOR (rw) register accessor: Completion Signal Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cstor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cstor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cstor`] module"]
#[doc(alias = "CSTOR")]
pub type Cstor = crate::Reg<cstor::CstorSpec>;
#[doc = "Completion Signal Timeout Register"]
pub mod cstor;
#[doc = "RSPR (r) register accessor: Response Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rspr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspr`] module"]
#[doc(alias = "RSPR")]
pub type Rspr = crate::Reg<rspr::RsprSpec>;
#[doc = "Response Register"]
pub mod rspr;
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
#[doc = "DMA (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`] module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "DMA Configuration Register"]
pub mod dma;
#[doc = "CFG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
#[doc = "FIFO (rw) register accessor: FIFO Memory Aperture0\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`] module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "FIFO Memory Aperture0"]
pub mod fifo;
