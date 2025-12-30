#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
  matrix_mcfg: [MatrixMcfg; 6],
  _reserved1: [u8; 0x28],
  matrix_scfg: [MatrixScfg; 9],
  _reserved2: [u8; 0x1c],
  matrix_pras0: MatrixPras0,
  _reserved3: [u8; 0x04],
  matrix_pras1: MatrixPras1,
  _reserved4: [u8; 0x04],
  matrix_pras2: MatrixPras2,
  _reserved5: [u8; 0x04],
  matrix_pras3: MatrixPras3,
  _reserved6: [u8; 0x04],
  matrix_pras4: MatrixPras4,
  _reserved7: [u8; 0x04],
  matrix_pras5: MatrixPras5,
  _reserved8: [u8; 0x04],
  matrix_pras6: MatrixPras6,
  _reserved9: [u8; 0x04],
  matrix_pras7: MatrixPras7,
  _reserved10: [u8; 0x04],
  matrix_pras8: MatrixPras8,
  _reserved11: [u8; 0x3c],
  matrix_mrcr: MatrixMrcr,
  _reserved12: [u8; 0x10],
  ccfg_sysio: CcfgSysio,
  _reserved13: [u8; 0xcc],
  matrix_wpmr: MatrixWpmr,
  matrix_wpsr: MatrixWpsr,
}
impl RegisterBlock {
  #[doc = "0x00..0x18 - Master Configuration Register"]
  #[inline(always)]
  pub const fn matrix_mcfg(&self, n: usize) -> &MatrixMcfg {
    &self.matrix_mcfg[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x00..0x18 - Master Configuration Register"]
  #[inline(always)]
  pub fn matrix_mcfg_iter(&self) -> impl Iterator<Item = &MatrixMcfg> {
    self.matrix_mcfg.iter()
  }
  #[doc = "0x40..0x64 - Slave Configuration Register"]
  #[inline(always)]
  pub const fn matrix_scfg(&self, n: usize) -> &MatrixScfg {
    &self.matrix_scfg[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x40..0x64 - Slave Configuration Register"]
  #[inline(always)]
  pub fn matrix_scfg_iter(&self) -> impl Iterator<Item = &MatrixScfg> {
    self.matrix_scfg.iter()
  }
  #[doc = "0x80 - Priority Register A for Slave 0"]
  #[inline(always)]
  pub const fn matrix_pras0(&self) -> &MatrixPras0 {
    &self.matrix_pras0
  }
  #[doc = "0x88 - Priority Register A for Slave 1"]
  #[inline(always)]
  pub const fn matrix_pras1(&self) -> &MatrixPras1 {
    &self.matrix_pras1
  }
  #[doc = "0x90 - Priority Register A for Slave 2"]
  #[inline(always)]
  pub const fn matrix_pras2(&self) -> &MatrixPras2 {
    &self.matrix_pras2
  }
  #[doc = "0x98 - Priority Register A for Slave 3"]
  #[inline(always)]
  pub const fn matrix_pras3(&self) -> &MatrixPras3 {
    &self.matrix_pras3
  }
  #[doc = "0xa0 - Priority Register A for Slave 4"]
  #[inline(always)]
  pub const fn matrix_pras4(&self) -> &MatrixPras4 {
    &self.matrix_pras4
  }
  #[doc = "0xa8 - Priority Register A for Slave 5"]
  #[inline(always)]
  pub const fn matrix_pras5(&self) -> &MatrixPras5 {
    &self.matrix_pras5
  }
  #[doc = "0xb0 - Priority Register A for Slave 6"]
  #[inline(always)]
  pub const fn matrix_pras6(&self) -> &MatrixPras6 {
    &self.matrix_pras6
  }
  #[doc = "0xb8 - Priority Register A for Slave 7"]
  #[inline(always)]
  pub const fn matrix_pras7(&self) -> &MatrixPras7 {
    &self.matrix_pras7
  }
  #[doc = "0xc0 - Priority Register A for Slave 8"]
  #[inline(always)]
  pub const fn matrix_pras8(&self) -> &MatrixPras8 {
    &self.matrix_pras8
  }
  #[doc = "0x100 - Master Remap Control Register"]
  #[inline(always)]
  pub const fn matrix_mrcr(&self) -> &MatrixMrcr {
    &self.matrix_mrcr
  }
  #[doc = "0x114 - System I/O Configuration register"]
  #[inline(always)]
  pub const fn ccfg_sysio(&self) -> &CcfgSysio {
    &self.ccfg_sysio
  }
  #[doc = "0x1e4 - Write Protect Mode Register"]
  #[inline(always)]
  pub const fn matrix_wpmr(&self) -> &MatrixWpmr {
    &self.matrix_wpmr
  }
  #[doc = "0x1e8 - Write Protect Status Register"]
  #[inline(always)]
  pub const fn matrix_wpsr(&self) -> &MatrixWpsr {
    &self.matrix_wpsr
  }
}
#[doc = "MATRIX_MCFG (rw) register accessor: Master Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_mcfg::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_mcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg`] module"]
#[doc(alias = "MATRIX_MCFG")]
pub type MatrixMcfg = crate::Reg<matrix_mcfg::MatrixMcfgSpec>;
#[doc = "Master Configuration Register"]
pub mod matrix_mcfg;
#[doc = "MATRIX_SCFG (rw) register accessor: Slave Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_scfg::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_scfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg`] module"]
#[doc(alias = "MATRIX_SCFG")]
pub type MatrixScfg = crate::Reg<matrix_scfg::MatrixScfgSpec>;
#[doc = "Slave Configuration Register"]
pub mod matrix_scfg;
#[doc = "MATRIX_PRAS0 (rw) register accessor: Priority Register A for Slave 0\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras0`] module"]
#[doc(alias = "MATRIX_PRAS0")]
pub type MatrixPras0 = crate::Reg<matrix_pras0::MatrixPras0Spec>;
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "MATRIX_PRAS1 (rw) register accessor: Priority Register A for Slave 1\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras1`] module"]
#[doc(alias = "MATRIX_PRAS1")]
pub type MatrixPras1 = crate::Reg<matrix_pras1::MatrixPras1Spec>;
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "MATRIX_PRAS2 (rw) register accessor: Priority Register A for Slave 2\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras2`] module"]
#[doc(alias = "MATRIX_PRAS2")]
pub type MatrixPras2 = crate::Reg<matrix_pras2::MatrixPras2Spec>;
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "MATRIX_PRAS3 (rw) register accessor: Priority Register A for Slave 3\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras3`] module"]
#[doc(alias = "MATRIX_PRAS3")]
pub type MatrixPras3 = crate::Reg<matrix_pras3::MatrixPras3Spec>;
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "MATRIX_PRAS4 (rw) register accessor: Priority Register A for Slave 4\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras4`] module"]
#[doc(alias = "MATRIX_PRAS4")]
pub type MatrixPras4 = crate::Reg<matrix_pras4::MatrixPras4Spec>;
#[doc = "Priority Register A for Slave 4"]
pub mod matrix_pras4;
#[doc = "MATRIX_PRAS5 (rw) register accessor: Priority Register A for Slave 5\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras5`] module"]
#[doc(alias = "MATRIX_PRAS5")]
pub type MatrixPras5 = crate::Reg<matrix_pras5::MatrixPras5Spec>;
#[doc = "Priority Register A for Slave 5"]
pub mod matrix_pras5;
#[doc = "MATRIX_PRAS6 (rw) register accessor: Priority Register A for Slave 6\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras6`] module"]
#[doc(alias = "MATRIX_PRAS6")]
pub type MatrixPras6 = crate::Reg<matrix_pras6::MatrixPras6Spec>;
#[doc = "Priority Register A for Slave 6"]
pub mod matrix_pras6;
#[doc = "MATRIX_PRAS7 (rw) register accessor: Priority Register A for Slave 7\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras7`] module"]
#[doc(alias = "MATRIX_PRAS7")]
pub type MatrixPras7 = crate::Reg<matrix_pras7::MatrixPras7Spec>;
#[doc = "Priority Register A for Slave 7"]
pub mod matrix_pras7;
#[doc = "MATRIX_PRAS8 (rw) register accessor: Priority Register A for Slave 8\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras8`] module"]
#[doc(alias = "MATRIX_PRAS8")]
pub type MatrixPras8 = crate::Reg<matrix_pras8::MatrixPras8Spec>;
#[doc = "Priority Register A for Slave 8"]
pub mod matrix_pras8;
#[doc = "MATRIX_MRCR (rw) register accessor: Master Remap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_mrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_mrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mrcr`] module"]
#[doc(alias = "MATRIX_MRCR")]
pub type MatrixMrcr = crate::Reg<matrix_mrcr::MatrixMrcrSpec>;
#[doc = "Master Remap Control Register"]
pub mod matrix_mrcr;
#[doc = "CCFG_SYSIO (rw) register accessor: System I/O Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_sysio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_sysio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_sysio`] module"]
#[doc(alias = "CCFG_SYSIO")]
pub type CcfgSysio = crate::Reg<ccfg_sysio::CcfgSysioSpec>;
#[doc = "System I/O Configuration register"]
pub mod ccfg_sysio;
#[doc = "MATRIX_WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_wpmr`] module"]
#[doc(alias = "MATRIX_WPMR")]
pub type MatrixWpmr = crate::Reg<matrix_wpmr::MatrixWpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod matrix_wpmr;
#[doc = "MATRIX_WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_wpsr`] module"]
#[doc(alias = "MATRIX_WPSR")]
pub type MatrixWpsr = crate::Reg<matrix_wpsr::MatrixWpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod matrix_wpsr;
