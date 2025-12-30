#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
  devctrl: Devctrl,
  devisr: Devisr,
  devicr: Devicr,
  devifr: Devifr,
  devimr: Devimr,
  devidr: Devidr,
  devier: Devier,
  devept: Devept,
  devfnum: Devfnum,
  _reserved9: [u8; 0xdc],
  deveptcfg: [Deveptcfg; 10],
  _reserved10: [u8; 0x08],
  deveptisr: [Deveptisr; 10],
  _reserved11: [u8; 0x08],
  devepticr: [Devepticr; 10],
  _reserved12: [u8; 0x08],
  deveptifr: [Deveptifr; 10],
  _reserved13: [u8; 0x08],
  deveptimr: [Deveptimr; 10],
  _reserved14: [u8; 0x08],
  deveptier: [Deveptier; 10],
  _reserved15: [u8; 0x08],
  deveptidr: [Deveptidr; 10],
  _reserved16: [u8; 0xc8],
  devdmanxtdsc1: Devdmanxtdsc1,
  devdmaaddress1: Devdmaaddress1,
  devdmacontrol1: Devdmacontrol1,
  devdmastatus1: Devdmastatus1,
  devdmanxtdsc2: Devdmanxtdsc2,
  devdmaaddress2: Devdmaaddress2,
  devdmacontrol2: Devdmacontrol2,
  devdmastatus2: Devdmastatus2,
  devdmanxtdsc3: Devdmanxtdsc3,
  devdmaaddress3: Devdmaaddress3,
  devdmacontrol3: Devdmacontrol3,
  devdmastatus3: Devdmastatus3,
  devdmanxtdsc4: Devdmanxtdsc4,
  devdmaaddress4: Devdmaaddress4,
  devdmacontrol4: Devdmacontrol4,
  devdmastatus4: Devdmastatus4,
  devdmanxtdsc5: Devdmanxtdsc5,
  devdmaaddress5: Devdmaaddress5,
  devdmacontrol5: Devdmacontrol5,
  devdmastatus5: Devdmastatus5,
  devdmanxtdsc6: Devdmanxtdsc6,
  devdmaaddress6: Devdmaaddress6,
  devdmacontrol6: Devdmacontrol6,
  devdmastatus6: Devdmastatus6,
  devdmanxtdsc7: Devdmanxtdsc7,
  devdmaaddress7: Devdmaaddress7,
  devdmacontrol7: Devdmacontrol7,
  devdmastatus7: Devdmastatus7,
  _reserved44: [u8; 0x80],
  hstctrl: Hstctrl,
  hstisr: Hstisr,
  hsticr: Hsticr,
  hstifr: Hstifr,
  hstimr: Hstimr,
  hstidr: Hstidr,
  hstier: Hstier,
  hstpip: Hstpip,
  hstfnum: Hstfnum,
  hstaddr1: Hstaddr1,
  hstaddr2: Hstaddr2,
  hstaddr3: Hstaddr3,
  _reserved56: [u8; 0xd0],
  hstpipcfg: [Hstpipcfg; 10],
  _reserved57: [u8; 0x08],
  hstpipisr: [Hstpipisr; 10],
  _reserved58: [u8; 0x08],
  hstpipicr: [Hstpipicr; 10],
  _reserved59: [u8; 0x08],
  hstpipifr: [Hstpipifr; 10],
  _reserved60: [u8; 0x08],
  hstpipimr: [Hstpipimr; 10],
  _reserved61: [u8; 0x08],
  hstpipier: [Hstpipier; 10],
  _reserved62: [u8; 0x08],
  hstpipidr: [Hstpipidr; 10],
  _reserved63: [u8; 0x08],
  hstpipinrq: [Hstpipinrq; 10],
  _reserved64: [u8; 0x08],
  hstpiperr: [Hstpiperr; 10],
  _reserved65: [u8; 0x68],
  hstdmanxtdsc1: Hstdmanxtdsc1,
  hstdmaaddress1: Hstdmaaddress1,
  hstdmacontrol1: Hstdmacontrol1,
  hstdmastatus1: Hstdmastatus1,
  hstdmanxtdsc2: Hstdmanxtdsc2,
  hstdmaaddress2: Hstdmaaddress2,
  hstdmacontrol2: Hstdmacontrol2,
  hstdmastatus2: Hstdmastatus2,
  hstdmanxtdsc3: Hstdmanxtdsc3,
  hstdmaaddress3: Hstdmaaddress3,
  hstdmacontrol3: Hstdmacontrol3,
  hstdmastatus3: Hstdmastatus3,
  hstdmanxtdsc4: Hstdmanxtdsc4,
  hstdmaaddress4: Hstdmaaddress4,
  hstdmacontrol4: Hstdmacontrol4,
  hstdmastatus4: Hstdmastatus4,
  hstdmanxtdsc5: Hstdmanxtdsc5,
  hstdmaaddress5: Hstdmaaddress5,
  hstdmacontrol5: Hstdmacontrol5,
  hstdmastatus5: Hstdmastatus5,
  hstdmanxtdsc6: Hstdmanxtdsc6,
  hstdmaaddress6: Hstdmaaddress6,
  hstdmacontrol6: Hstdmacontrol6,
  hstdmastatus6: Hstdmastatus6,
  hstdmanxtdsc7: Hstdmanxtdsc7,
  hstdmaaddress7: Hstdmaaddress7,
  hstdmacontrol7: Hstdmacontrol7,
  hstdmastatus7: Hstdmastatus7,
  _reserved93: [u8; 0x80],
  ctrl: Ctrl,
  sr: Sr,
  scr: Scr,
  sfr: Sfr,
  _reserved97: [u8; 0x1c],
  fsm: Fsm,
}
impl RegisterBlock {
  #[doc = "0x00 - Device General Control Register"]
  #[inline(always)]
  pub const fn devctrl(&self) -> &Devctrl {
    &self.devctrl
  }
  #[doc = "0x04 - Device Global Interrupt Status Register"]
  #[inline(always)]
  pub const fn devisr(&self) -> &Devisr {
    &self.devisr
  }
  #[doc = "0x08 - Device Global Interrupt Clear Register"]
  #[inline(always)]
  pub const fn devicr(&self) -> &Devicr {
    &self.devicr
  }
  #[doc = "0x0c - Device Global Interrupt Set Register"]
  #[inline(always)]
  pub const fn devifr(&self) -> &Devifr {
    &self.devifr
  }
  #[doc = "0x10 - Device Global Interrupt Mask Register"]
  #[inline(always)]
  pub const fn devimr(&self) -> &Devimr {
    &self.devimr
  }
  #[doc = "0x14 - Device Global Interrupt Disable Register"]
  #[inline(always)]
  pub const fn devidr(&self) -> &Devidr {
    &self.devidr
  }
  #[doc = "0x18 - Device Global Interrupt Enable Register"]
  #[inline(always)]
  pub const fn devier(&self) -> &Devier {
    &self.devier
  }
  #[doc = "0x1c - Device Endpoint Register"]
  #[inline(always)]
  pub const fn devept(&self) -> &Devept {
    &self.devept
  }
  #[doc = "0x20 - Device Frame Number Register"]
  #[inline(always)]
  pub const fn devfnum(&self) -> &Devfnum {
    &self.devfnum
  }
  #[doc = "0x100..0x128 - Device Endpoint Configuration Register (n = 0)"]
  #[inline(always)]
  pub const fn deveptcfg(&self, n: usize) -> &Deveptcfg {
    &self.deveptcfg[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x100..0x128 - Device Endpoint Configuration Register (n = 0)"]
  #[inline(always)]
  pub fn deveptcfg_iter(&self) -> impl Iterator<Item = &Deveptcfg> {
    self.deveptcfg.iter()
  }
  #[doc = "0x130..0x158 - Device Endpoint Status Register (n = 0)"]
  #[inline(always)]
  pub const fn deveptisr(&self, n: usize) -> &Deveptisr {
    &self.deveptisr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x130..0x158 - Device Endpoint Status Register (n = 0)"]
  #[inline(always)]
  pub fn deveptisr_iter(&self) -> impl Iterator<Item = &Deveptisr> {
    self.deveptisr.iter()
  }
  #[doc = "0x160..0x188 - Device Endpoint Clear Register (n = 0)"]
  #[inline(always)]
  pub const fn devepticr(&self, n: usize) -> &Devepticr {
    &self.devepticr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x160..0x188 - Device Endpoint Clear Register (n = 0)"]
  #[inline(always)]
  pub fn devepticr_iter(&self) -> impl Iterator<Item = &Devepticr> {
    self.devepticr.iter()
  }
  #[doc = "0x190..0x1b8 - Device Endpoint Set Register (n = 0)"]
  #[inline(always)]
  pub const fn deveptifr(&self, n: usize) -> &Deveptifr {
    &self.deveptifr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x190..0x1b8 - Device Endpoint Set Register (n = 0)"]
  #[inline(always)]
  pub fn deveptifr_iter(&self) -> impl Iterator<Item = &Deveptifr> {
    self.deveptifr.iter()
  }
  #[doc = "0x1c0..0x1e8 - Device Endpoint Mask Register (n = 0)"]
  #[inline(always)]
  pub const fn deveptimr(&self, n: usize) -> &Deveptimr {
    &self.deveptimr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x1c0..0x1e8 - Device Endpoint Mask Register (n = 0)"]
  #[inline(always)]
  pub fn deveptimr_iter(&self) -> impl Iterator<Item = &Deveptimr> {
    self.deveptimr.iter()
  }
  #[doc = "0x1f0..0x218 - Device Endpoint Enable Register (n = 0)"]
  #[inline(always)]
  pub const fn deveptier(&self, n: usize) -> &Deveptier {
    &self.deveptier[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x1f0..0x218 - Device Endpoint Enable Register (n = 0)"]
  #[inline(always)]
  pub fn deveptier_iter(&self) -> impl Iterator<Item = &Deveptier> {
    self.deveptier.iter()
  }
  #[doc = "0x220..0x248 - Device Endpoint Disable Register (n = 0)"]
  #[inline(always)]
  pub const fn deveptidr(&self, n: usize) -> &Deveptidr {
    &self.deveptidr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x220..0x248 - Device Endpoint Disable Register (n = 0)"]
  #[inline(always)]
  pub fn deveptidr_iter(&self) -> impl Iterator<Item = &Deveptidr> {
    self.deveptidr.iter()
  }
  #[doc = "0x310 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
  #[inline(always)]
  pub const fn devdmanxtdsc1(&self) -> &Devdmanxtdsc1 {
    &self.devdmanxtdsc1
  }
  #[doc = "0x314 - Device DMA Channel Address Register (n = 1)"]
  #[inline(always)]
  pub const fn devdmaaddress1(&self) -> &Devdmaaddress1 {
    &self.devdmaaddress1
  }
  #[doc = "0x318 - Device DMA Channel Control Register (n = 1)"]
  #[inline(always)]
  pub const fn devdmacontrol1(&self) -> &Devdmacontrol1 {
    &self.devdmacontrol1
  }
  #[doc = "0x31c - Device DMA Channel Status Register (n = 1)"]
  #[inline(always)]
  pub const fn devdmastatus1(&self) -> &Devdmastatus1 {
    &self.devdmastatus1
  }
  #[doc = "0x320 - Device DMA Channel Next Descriptor Address Register (n = 2)"]
  #[inline(always)]
  pub const fn devdmanxtdsc2(&self) -> &Devdmanxtdsc2 {
    &self.devdmanxtdsc2
  }
  #[doc = "0x324 - Device DMA Channel Address Register (n = 2)"]
  #[inline(always)]
  pub const fn devdmaaddress2(&self) -> &Devdmaaddress2 {
    &self.devdmaaddress2
  }
  #[doc = "0x328 - Device DMA Channel Control Register (n = 2)"]
  #[inline(always)]
  pub const fn devdmacontrol2(&self) -> &Devdmacontrol2 {
    &self.devdmacontrol2
  }
  #[doc = "0x32c - Device DMA Channel Status Register (n = 2)"]
  #[inline(always)]
  pub const fn devdmastatus2(&self) -> &Devdmastatus2 {
    &self.devdmastatus2
  }
  #[doc = "0x330 - Device DMA Channel Next Descriptor Address Register (n = 3)"]
  #[inline(always)]
  pub const fn devdmanxtdsc3(&self) -> &Devdmanxtdsc3 {
    &self.devdmanxtdsc3
  }
  #[doc = "0x334 - Device DMA Channel Address Register (n = 3)"]
  #[inline(always)]
  pub const fn devdmaaddress3(&self) -> &Devdmaaddress3 {
    &self.devdmaaddress3
  }
  #[doc = "0x338 - Device DMA Channel Control Register (n = 3)"]
  #[inline(always)]
  pub const fn devdmacontrol3(&self) -> &Devdmacontrol3 {
    &self.devdmacontrol3
  }
  #[doc = "0x33c - Device DMA Channel Status Register (n = 3)"]
  #[inline(always)]
  pub const fn devdmastatus3(&self) -> &Devdmastatus3 {
    &self.devdmastatus3
  }
  #[doc = "0x340 - Device DMA Channel Next Descriptor Address Register (n = 4)"]
  #[inline(always)]
  pub const fn devdmanxtdsc4(&self) -> &Devdmanxtdsc4 {
    &self.devdmanxtdsc4
  }
  #[doc = "0x344 - Device DMA Channel Address Register (n = 4)"]
  #[inline(always)]
  pub const fn devdmaaddress4(&self) -> &Devdmaaddress4 {
    &self.devdmaaddress4
  }
  #[doc = "0x348 - Device DMA Channel Control Register (n = 4)"]
  #[inline(always)]
  pub const fn devdmacontrol4(&self) -> &Devdmacontrol4 {
    &self.devdmacontrol4
  }
  #[doc = "0x34c - Device DMA Channel Status Register (n = 4)"]
  #[inline(always)]
  pub const fn devdmastatus4(&self) -> &Devdmastatus4 {
    &self.devdmastatus4
  }
  #[doc = "0x350 - Device DMA Channel Next Descriptor Address Register (n = 5)"]
  #[inline(always)]
  pub const fn devdmanxtdsc5(&self) -> &Devdmanxtdsc5 {
    &self.devdmanxtdsc5
  }
  #[doc = "0x354 - Device DMA Channel Address Register (n = 5)"]
  #[inline(always)]
  pub const fn devdmaaddress5(&self) -> &Devdmaaddress5 {
    &self.devdmaaddress5
  }
  #[doc = "0x358 - Device DMA Channel Control Register (n = 5)"]
  #[inline(always)]
  pub const fn devdmacontrol5(&self) -> &Devdmacontrol5 {
    &self.devdmacontrol5
  }
  #[doc = "0x35c - Device DMA Channel Status Register (n = 5)"]
  #[inline(always)]
  pub const fn devdmastatus5(&self) -> &Devdmastatus5 {
    &self.devdmastatus5
  }
  #[doc = "0x360 - Device DMA Channel Next Descriptor Address Register (n = 6)"]
  #[inline(always)]
  pub const fn devdmanxtdsc6(&self) -> &Devdmanxtdsc6 {
    &self.devdmanxtdsc6
  }
  #[doc = "0x364 - Device DMA Channel Address Register (n = 6)"]
  #[inline(always)]
  pub const fn devdmaaddress6(&self) -> &Devdmaaddress6 {
    &self.devdmaaddress6
  }
  #[doc = "0x368 - Device DMA Channel Control Register (n = 6)"]
  #[inline(always)]
  pub const fn devdmacontrol6(&self) -> &Devdmacontrol6 {
    &self.devdmacontrol6
  }
  #[doc = "0x36c - Device DMA Channel Status Register (n = 6)"]
  #[inline(always)]
  pub const fn devdmastatus6(&self) -> &Devdmastatus6 {
    &self.devdmastatus6
  }
  #[doc = "0x370 - Device DMA Channel Next Descriptor Address Register (n = 7)"]
  #[inline(always)]
  pub const fn devdmanxtdsc7(&self) -> &Devdmanxtdsc7 {
    &self.devdmanxtdsc7
  }
  #[doc = "0x374 - Device DMA Channel Address Register (n = 7)"]
  #[inline(always)]
  pub const fn devdmaaddress7(&self) -> &Devdmaaddress7 {
    &self.devdmaaddress7
  }
  #[doc = "0x378 - Device DMA Channel Control Register (n = 7)"]
  #[inline(always)]
  pub const fn devdmacontrol7(&self) -> &Devdmacontrol7 {
    &self.devdmacontrol7
  }
  #[doc = "0x37c - Device DMA Channel Status Register (n = 7)"]
  #[inline(always)]
  pub const fn devdmastatus7(&self) -> &Devdmastatus7 {
    &self.devdmastatus7
  }
  #[doc = "0x400 - Host General Control Register"]
  #[inline(always)]
  pub const fn hstctrl(&self) -> &Hstctrl {
    &self.hstctrl
  }
  #[doc = "0x404 - Host Global Interrupt Status Register"]
  #[inline(always)]
  pub const fn hstisr(&self) -> &Hstisr {
    &self.hstisr
  }
  #[doc = "0x408 - Host Global Interrupt Clear Register"]
  #[inline(always)]
  pub const fn hsticr(&self) -> &Hsticr {
    &self.hsticr
  }
  #[doc = "0x40c - Host Global Interrupt Set Register"]
  #[inline(always)]
  pub const fn hstifr(&self) -> &Hstifr {
    &self.hstifr
  }
  #[doc = "0x410 - Host Global Interrupt Mask Register"]
  #[inline(always)]
  pub const fn hstimr(&self) -> &Hstimr {
    &self.hstimr
  }
  #[doc = "0x414 - Host Global Interrupt Disable Register"]
  #[inline(always)]
  pub const fn hstidr(&self) -> &Hstidr {
    &self.hstidr
  }
  #[doc = "0x418 - Host Global Interrupt Enable Register"]
  #[inline(always)]
  pub const fn hstier(&self) -> &Hstier {
    &self.hstier
  }
  #[doc = "0x41c - Host Pipe Register"]
  #[inline(always)]
  pub const fn hstpip(&self) -> &Hstpip {
    &self.hstpip
  }
  #[doc = "0x420 - Host Frame Number Register"]
  #[inline(always)]
  pub const fn hstfnum(&self) -> &Hstfnum {
    &self.hstfnum
  }
  #[doc = "0x424 - Host Address 1 Register"]
  #[inline(always)]
  pub const fn hstaddr1(&self) -> &Hstaddr1 {
    &self.hstaddr1
  }
  #[doc = "0x428 - Host Address 2 Register"]
  #[inline(always)]
  pub const fn hstaddr2(&self) -> &Hstaddr2 {
    &self.hstaddr2
  }
  #[doc = "0x42c - Host Address 3 Register"]
  #[inline(always)]
  pub const fn hstaddr3(&self) -> &Hstaddr3 {
    &self.hstaddr3
  }
  #[doc = "0x500..0x528 - Host Pipe Configuration Register (n = 0)"]
  #[inline(always)]
  pub const fn hstpipcfg(&self, n: usize) -> &Hstpipcfg {
    &self.hstpipcfg[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x500..0x528 - Host Pipe Configuration Register (n = 0)"]
  #[inline(always)]
  pub fn hstpipcfg_iter(&self) -> impl Iterator<Item = &Hstpipcfg> {
    self.hstpipcfg.iter()
  }
  #[doc = "0x530..0x558 - Host Pipe Status Register (n = 0)"]
  #[inline(always)]
  pub const fn hstpipisr(&self, n: usize) -> &Hstpipisr {
    &self.hstpipisr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x530..0x558 - Host Pipe Status Register (n = 0)"]
  #[inline(always)]
  pub fn hstpipisr_iter(&self) -> impl Iterator<Item = &Hstpipisr> {
    self.hstpipisr.iter()
  }
  #[doc = "0x560..0x588 - Host Pipe Clear Register (n = 0)"]
  #[inline(always)]
  pub const fn hstpipicr(&self, n: usize) -> &Hstpipicr {
    &self.hstpipicr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x560..0x588 - Host Pipe Clear Register (n = 0)"]
  #[inline(always)]
  pub fn hstpipicr_iter(&self) -> impl Iterator<Item = &Hstpipicr> {
    self.hstpipicr.iter()
  }
  #[doc = "0x590..0x5b8 - Host Pipe Set Register (n = 0)"]
  #[inline(always)]
  pub const fn hstpipifr(&self, n: usize) -> &Hstpipifr {
    &self.hstpipifr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x590..0x5b8 - Host Pipe Set Register (n = 0)"]
  #[inline(always)]
  pub fn hstpipifr_iter(&self) -> impl Iterator<Item = &Hstpipifr> {
    self.hstpipifr.iter()
  }
  #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register (n = 0)"]
  #[inline(always)]
  pub const fn hstpipimr(&self, n: usize) -> &Hstpipimr {
    &self.hstpipimr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register (n = 0)"]
  #[inline(always)]
  pub fn hstpipimr_iter(&self) -> impl Iterator<Item = &Hstpipimr> {
    self.hstpipimr.iter()
  }
  #[doc = "0x5f0..0x618 - Host Pipe Enable Register (n = 0)"]
  #[inline(always)]
  pub const fn hstpipier(&self, n: usize) -> &Hstpipier {
    &self.hstpipier[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x5f0..0x618 - Host Pipe Enable Register (n = 0)"]
  #[inline(always)]
  pub fn hstpipier_iter(&self) -> impl Iterator<Item = &Hstpipier> {
    self.hstpipier.iter()
  }
  #[doc = "0x620..0x648 - Host Pipe Disable Register (n = 0)"]
  #[inline(always)]
  pub const fn hstpipidr(&self, n: usize) -> &Hstpipidr {
    &self.hstpipidr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x620..0x648 - Host Pipe Disable Register (n = 0)"]
  #[inline(always)]
  pub fn hstpipidr_iter(&self) -> impl Iterator<Item = &Hstpipidr> {
    self.hstpipidr.iter()
  }
  #[doc = "0x650..0x678 - Host Pipe IN Request Register (n = 0)"]
  #[inline(always)]
  pub const fn hstpipinrq(&self, n: usize) -> &Hstpipinrq {
    &self.hstpipinrq[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x650..0x678 - Host Pipe IN Request Register (n = 0)"]
  #[inline(always)]
  pub fn hstpipinrq_iter(&self) -> impl Iterator<Item = &Hstpipinrq> {
    self.hstpipinrq.iter()
  }
  #[doc = "0x680..0x6a8 - Host Pipe Error Register (n = 0)"]
  #[inline(always)]
  pub const fn hstpiperr(&self, n: usize) -> &Hstpiperr {
    &self.hstpiperr[n]
  }
  #[doc = "Iterator for array of:"]
  #[doc = "0x680..0x6a8 - Host Pipe Error Register (n = 0)"]
  #[inline(always)]
  pub fn hstpiperr_iter(&self) -> impl Iterator<Item = &Hstpiperr> {
    self.hstpiperr.iter()
  }
  #[doc = "0x710 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
  #[inline(always)]
  pub const fn hstdmanxtdsc1(&self) -> &Hstdmanxtdsc1 {
    &self.hstdmanxtdsc1
  }
  #[doc = "0x714 - Host DMA Channel Address Register (n = 1)"]
  #[inline(always)]
  pub const fn hstdmaaddress1(&self) -> &Hstdmaaddress1 {
    &self.hstdmaaddress1
  }
  #[doc = "0x718 - Host DMA Channel Control Register (n = 1)"]
  #[inline(always)]
  pub const fn hstdmacontrol1(&self) -> &Hstdmacontrol1 {
    &self.hstdmacontrol1
  }
  #[doc = "0x71c - Host DMA Channel Status Register (n = 1)"]
  #[inline(always)]
  pub const fn hstdmastatus1(&self) -> &Hstdmastatus1 {
    &self.hstdmastatus1
  }
  #[doc = "0x720 - Host DMA Channel Next Descriptor Address Register (n = 2)"]
  #[inline(always)]
  pub const fn hstdmanxtdsc2(&self) -> &Hstdmanxtdsc2 {
    &self.hstdmanxtdsc2
  }
  #[doc = "0x724 - Host DMA Channel Address Register (n = 2)"]
  #[inline(always)]
  pub const fn hstdmaaddress2(&self) -> &Hstdmaaddress2 {
    &self.hstdmaaddress2
  }
  #[doc = "0x728 - Host DMA Channel Control Register (n = 2)"]
  #[inline(always)]
  pub const fn hstdmacontrol2(&self) -> &Hstdmacontrol2 {
    &self.hstdmacontrol2
  }
  #[doc = "0x72c - Host DMA Channel Status Register (n = 2)"]
  #[inline(always)]
  pub const fn hstdmastatus2(&self) -> &Hstdmastatus2 {
    &self.hstdmastatus2
  }
  #[doc = "0x730 - Host DMA Channel Next Descriptor Address Register (n = 3)"]
  #[inline(always)]
  pub const fn hstdmanxtdsc3(&self) -> &Hstdmanxtdsc3 {
    &self.hstdmanxtdsc3
  }
  #[doc = "0x734 - Host DMA Channel Address Register (n = 3)"]
  #[inline(always)]
  pub const fn hstdmaaddress3(&self) -> &Hstdmaaddress3 {
    &self.hstdmaaddress3
  }
  #[doc = "0x738 - Host DMA Channel Control Register (n = 3)"]
  #[inline(always)]
  pub const fn hstdmacontrol3(&self) -> &Hstdmacontrol3 {
    &self.hstdmacontrol3
  }
  #[doc = "0x73c - Host DMA Channel Status Register (n = 3)"]
  #[inline(always)]
  pub const fn hstdmastatus3(&self) -> &Hstdmastatus3 {
    &self.hstdmastatus3
  }
  #[doc = "0x740 - Host DMA Channel Next Descriptor Address Register (n = 4)"]
  #[inline(always)]
  pub const fn hstdmanxtdsc4(&self) -> &Hstdmanxtdsc4 {
    &self.hstdmanxtdsc4
  }
  #[doc = "0x744 - Host DMA Channel Address Register (n = 4)"]
  #[inline(always)]
  pub const fn hstdmaaddress4(&self) -> &Hstdmaaddress4 {
    &self.hstdmaaddress4
  }
  #[doc = "0x748 - Host DMA Channel Control Register (n = 4)"]
  #[inline(always)]
  pub const fn hstdmacontrol4(&self) -> &Hstdmacontrol4 {
    &self.hstdmacontrol4
  }
  #[doc = "0x74c - Host DMA Channel Status Register (n = 4)"]
  #[inline(always)]
  pub const fn hstdmastatus4(&self) -> &Hstdmastatus4 {
    &self.hstdmastatus4
  }
  #[doc = "0x750 - Host DMA Channel Next Descriptor Address Register (n = 5)"]
  #[inline(always)]
  pub const fn hstdmanxtdsc5(&self) -> &Hstdmanxtdsc5 {
    &self.hstdmanxtdsc5
  }
  #[doc = "0x754 - Host DMA Channel Address Register (n = 5)"]
  #[inline(always)]
  pub const fn hstdmaaddress5(&self) -> &Hstdmaaddress5 {
    &self.hstdmaaddress5
  }
  #[doc = "0x758 - Host DMA Channel Control Register (n = 5)"]
  #[inline(always)]
  pub const fn hstdmacontrol5(&self) -> &Hstdmacontrol5 {
    &self.hstdmacontrol5
  }
  #[doc = "0x75c - Host DMA Channel Status Register (n = 5)"]
  #[inline(always)]
  pub const fn hstdmastatus5(&self) -> &Hstdmastatus5 {
    &self.hstdmastatus5
  }
  #[doc = "0x760 - Host DMA Channel Next Descriptor Address Register (n = 6)"]
  #[inline(always)]
  pub const fn hstdmanxtdsc6(&self) -> &Hstdmanxtdsc6 {
    &self.hstdmanxtdsc6
  }
  #[doc = "0x764 - Host DMA Channel Address Register (n = 6)"]
  #[inline(always)]
  pub const fn hstdmaaddress6(&self) -> &Hstdmaaddress6 {
    &self.hstdmaaddress6
  }
  #[doc = "0x768 - Host DMA Channel Control Register (n = 6)"]
  #[inline(always)]
  pub const fn hstdmacontrol6(&self) -> &Hstdmacontrol6 {
    &self.hstdmacontrol6
  }
  #[doc = "0x76c - Host DMA Channel Status Register (n = 6)"]
  #[inline(always)]
  pub const fn hstdmastatus6(&self) -> &Hstdmastatus6 {
    &self.hstdmastatus6
  }
  #[doc = "0x770 - Host DMA Channel Next Descriptor Address Register (n = 7)"]
  #[inline(always)]
  pub const fn hstdmanxtdsc7(&self) -> &Hstdmanxtdsc7 {
    &self.hstdmanxtdsc7
  }
  #[doc = "0x774 - Host DMA Channel Address Register (n = 7)"]
  #[inline(always)]
  pub const fn hstdmaaddress7(&self) -> &Hstdmaaddress7 {
    &self.hstdmaaddress7
  }
  #[doc = "0x778 - Host DMA Channel Control Register (n = 7)"]
  #[inline(always)]
  pub const fn hstdmacontrol7(&self) -> &Hstdmacontrol7 {
    &self.hstdmacontrol7
  }
  #[doc = "0x77c - Host DMA Channel Status Register (n = 7)"]
  #[inline(always)]
  pub const fn hstdmastatus7(&self) -> &Hstdmastatus7 {
    &self.hstdmastatus7
  }
  #[doc = "0x800 - General Control Register"]
  #[inline(always)]
  pub const fn ctrl(&self) -> &Ctrl {
    &self.ctrl
  }
  #[doc = "0x804 - General Status Register"]
  #[inline(always)]
  pub const fn sr(&self) -> &Sr {
    &self.sr
  }
  #[doc = "0x808 - General Status Clear Register"]
  #[inline(always)]
  pub const fn scr(&self) -> &Scr {
    &self.scr
  }
  #[doc = "0x80c - General Status Set Register"]
  #[inline(always)]
  pub const fn sfr(&self) -> &Sfr {
    &self.sfr
  }
  #[doc = "0x82c - General Finite State Machine Register"]
  #[inline(always)]
  pub const fn fsm(&self) -> &Fsm {
    &self.fsm
  }
}
#[doc = "DEVCTRL (rw) register accessor: Device General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devctrl`] module"]
#[doc(alias = "DEVCTRL")]
pub type Devctrl = crate::Reg<devctrl::DevctrlSpec>;
#[doc = "Device General Control Register"]
pub mod devctrl;
#[doc = "DEVISR (r) register accessor: Device Global Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devisr`] module"]
#[doc(alias = "DEVISR")]
pub type Devisr = crate::Reg<devisr::DevisrSpec>;
#[doc = "Device Global Interrupt Status Register"]
pub mod devisr;
#[doc = "DEVICR (w) register accessor: Device Global Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devicr`] module"]
#[doc(alias = "DEVICR")]
pub type Devicr = crate::Reg<devicr::DevicrSpec>;
#[doc = "Device Global Interrupt Clear Register"]
pub mod devicr;
#[doc = "DEVIFR (w) register accessor: Device Global Interrupt Set Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devifr`] module"]
#[doc(alias = "DEVIFR")]
pub type Devifr = crate::Reg<devifr::DevifrSpec>;
#[doc = "Device Global Interrupt Set Register"]
pub mod devifr;
#[doc = "DEVIMR (r) register accessor: Device Global Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devimr`] module"]
#[doc(alias = "DEVIMR")]
pub type Devimr = crate::Reg<devimr::DevimrSpec>;
#[doc = "Device Global Interrupt Mask Register"]
pub mod devimr;
#[doc = "DEVIDR (w) register accessor: Device Global Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devidr`] module"]
#[doc(alias = "DEVIDR")]
pub type Devidr = crate::Reg<devidr::DevidrSpec>;
#[doc = "Device Global Interrupt Disable Register"]
pub mod devidr;
#[doc = "DEVIER (w) register accessor: Device Global Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devier`] module"]
#[doc(alias = "DEVIER")]
pub type Devier = crate::Reg<devier::DevierSpec>;
#[doc = "Device Global Interrupt Enable Register"]
pub mod devier;
#[doc = "DEVEPT (rw) register accessor: Device Endpoint Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devept::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devept::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devept`] module"]
#[doc(alias = "DEVEPT")]
pub type Devept = crate::Reg<devept::DeveptSpec>;
#[doc = "Device Endpoint Register"]
pub mod devept;
#[doc = "DEVFNUM (r) register accessor: Device Frame Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devfnum`] module"]
#[doc(alias = "DEVFNUM")]
pub type Devfnum = crate::Reg<devfnum::DevfnumSpec>;
#[doc = "Device Frame Number Register"]
pub mod devfnum;
#[doc = "DEVEPTCFG (rw) register accessor: Device Endpoint Configuration Register (n = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptcfg::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg`] module"]
#[doc(alias = "DEVEPTCFG")]
pub type Deveptcfg = crate::Reg<deveptcfg::DeveptcfgSpec>;
#[doc = "Device Endpoint Configuration Register (n = 0)"]
pub mod deveptcfg;
#[doc = "DEVEPTISR (r) register accessor: Device Endpoint Status Register (n = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr`] module"]
#[doc(alias = "DEVEPTISR")]
pub type Deveptisr = crate::Reg<deveptisr::DeveptisrSpec>;
#[doc = "Device Endpoint Status Register (n = 0)"]
pub mod deveptisr;
#[doc = "DEVEPTICR (w) register accessor: Device Endpoint Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devepticr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr`] module"]
#[doc(alias = "DEVEPTICR")]
pub type Devepticr = crate::Reg<devepticr::DevepticrSpec>;
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub mod devepticr;
#[doc = "DEVEPTIFR (w) register accessor: Device Endpoint Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr`] module"]
#[doc(alias = "DEVEPTIFR")]
pub type Deveptifr = crate::Reg<deveptifr::DeveptifrSpec>;
#[doc = "Device Endpoint Set Register (n = 0)"]
pub mod deveptifr;
#[doc = "DEVEPTIMR (r) register accessor: Device Endpoint Mask Register (n = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr`] module"]
#[doc(alias = "DEVEPTIMR")]
pub type Deveptimr = crate::Reg<deveptimr::DeveptimrSpec>;
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub mod deveptimr;
#[doc = "DEVEPTIER (w) register accessor: Device Endpoint Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier`] module"]
#[doc(alias = "DEVEPTIER")]
pub type Deveptier = crate::Reg<deveptier::DeveptierSpec>;
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub mod deveptier;
#[doc = "DEVEPTIDR (w) register accessor: Device Endpoint Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr`] module"]
#[doc(alias = "DEVEPTIDR")]
pub type Deveptidr = crate::Reg<deveptidr::DeveptidrSpec>;
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub mod deveptidr;
#[doc = "DEVDMANXTDSC1 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmanxtdsc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmanxtdsc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc1`] module"]
#[doc(alias = "DEVDMANXTDSC1")]
pub type Devdmanxtdsc1 = crate::Reg<devdmanxtdsc1::Devdmanxtdsc1Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod devdmanxtdsc1;
#[doc = "DEVDMAADDRESS1 (rw) register accessor: Device DMA Channel Address Register (n = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmaaddress1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmaaddress1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress1`] module"]
#[doc(alias = "DEVDMAADDRESS1")]
pub type Devdmaaddress1 = crate::Reg<devdmaaddress1::Devdmaaddress1Spec>;
#[doc = "Device DMA Channel Address Register (n = 1)"]
pub mod devdmaaddress1;
#[doc = "DEVDMACONTROL1 (rw) register accessor: Device DMA Channel Control Register (n = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmacontrol1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmacontrol1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol1`] module"]
#[doc(alias = "DEVDMACONTROL1")]
pub type Devdmacontrol1 = crate::Reg<devdmacontrol1::Devdmacontrol1Spec>;
#[doc = "Device DMA Channel Control Register (n = 1)"]
pub mod devdmacontrol1;
#[doc = "DEVDMASTATUS1 (rw) register accessor: Device DMA Channel Status Register (n = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmastatus1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmastatus1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus1`] module"]
#[doc(alias = "DEVDMASTATUS1")]
pub type Devdmastatus1 = crate::Reg<devdmastatus1::Devdmastatus1Spec>;
#[doc = "Device DMA Channel Status Register (n = 1)"]
pub mod devdmastatus1;
#[doc = "DEVDMANXTDSC2 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmanxtdsc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmanxtdsc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc2`] module"]
#[doc(alias = "DEVDMANXTDSC2")]
pub type Devdmanxtdsc2 = crate::Reg<devdmanxtdsc2::Devdmanxtdsc2Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod devdmanxtdsc2;
#[doc = "DEVDMAADDRESS2 (rw) register accessor: Device DMA Channel Address Register (n = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmaaddress2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmaaddress2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress2`] module"]
#[doc(alias = "DEVDMAADDRESS2")]
pub type Devdmaaddress2 = crate::Reg<devdmaaddress2::Devdmaaddress2Spec>;
#[doc = "Device DMA Channel Address Register (n = 2)"]
pub mod devdmaaddress2;
#[doc = "DEVDMACONTROL2 (rw) register accessor: Device DMA Channel Control Register (n = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmacontrol2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmacontrol2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol2`] module"]
#[doc(alias = "DEVDMACONTROL2")]
pub type Devdmacontrol2 = crate::Reg<devdmacontrol2::Devdmacontrol2Spec>;
#[doc = "Device DMA Channel Control Register (n = 2)"]
pub mod devdmacontrol2;
#[doc = "DEVDMASTATUS2 (rw) register accessor: Device DMA Channel Status Register (n = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmastatus2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmastatus2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus2`] module"]
#[doc(alias = "DEVDMASTATUS2")]
pub type Devdmastatus2 = crate::Reg<devdmastatus2::Devdmastatus2Spec>;
#[doc = "Device DMA Channel Status Register (n = 2)"]
pub mod devdmastatus2;
#[doc = "DEVDMANXTDSC3 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmanxtdsc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmanxtdsc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc3`] module"]
#[doc(alias = "DEVDMANXTDSC3")]
pub type Devdmanxtdsc3 = crate::Reg<devdmanxtdsc3::Devdmanxtdsc3Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod devdmanxtdsc3;
#[doc = "DEVDMAADDRESS3 (rw) register accessor: Device DMA Channel Address Register (n = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmaaddress3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmaaddress3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress3`] module"]
#[doc(alias = "DEVDMAADDRESS3")]
pub type Devdmaaddress3 = crate::Reg<devdmaaddress3::Devdmaaddress3Spec>;
#[doc = "Device DMA Channel Address Register (n = 3)"]
pub mod devdmaaddress3;
#[doc = "DEVDMACONTROL3 (rw) register accessor: Device DMA Channel Control Register (n = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmacontrol3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmacontrol3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol3`] module"]
#[doc(alias = "DEVDMACONTROL3")]
pub type Devdmacontrol3 = crate::Reg<devdmacontrol3::Devdmacontrol3Spec>;
#[doc = "Device DMA Channel Control Register (n = 3)"]
pub mod devdmacontrol3;
#[doc = "DEVDMASTATUS3 (rw) register accessor: Device DMA Channel Status Register (n = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmastatus3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmastatus3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus3`] module"]
#[doc(alias = "DEVDMASTATUS3")]
pub type Devdmastatus3 = crate::Reg<devdmastatus3::Devdmastatus3Spec>;
#[doc = "Device DMA Channel Status Register (n = 3)"]
pub mod devdmastatus3;
#[doc = "DEVDMANXTDSC4 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmanxtdsc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmanxtdsc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc4`] module"]
#[doc(alias = "DEVDMANXTDSC4")]
pub type Devdmanxtdsc4 = crate::Reg<devdmanxtdsc4::Devdmanxtdsc4Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod devdmanxtdsc4;
#[doc = "DEVDMAADDRESS4 (rw) register accessor: Device DMA Channel Address Register (n = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmaaddress4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmaaddress4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress4`] module"]
#[doc(alias = "DEVDMAADDRESS4")]
pub type Devdmaaddress4 = crate::Reg<devdmaaddress4::Devdmaaddress4Spec>;
#[doc = "Device DMA Channel Address Register (n = 4)"]
pub mod devdmaaddress4;
#[doc = "DEVDMACONTROL4 (rw) register accessor: Device DMA Channel Control Register (n = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmacontrol4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmacontrol4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol4`] module"]
#[doc(alias = "DEVDMACONTROL4")]
pub type Devdmacontrol4 = crate::Reg<devdmacontrol4::Devdmacontrol4Spec>;
#[doc = "Device DMA Channel Control Register (n = 4)"]
pub mod devdmacontrol4;
#[doc = "DEVDMASTATUS4 (rw) register accessor: Device DMA Channel Status Register (n = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmastatus4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmastatus4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus4`] module"]
#[doc(alias = "DEVDMASTATUS4")]
pub type Devdmastatus4 = crate::Reg<devdmastatus4::Devdmastatus4Spec>;
#[doc = "Device DMA Channel Status Register (n = 4)"]
pub mod devdmastatus4;
#[doc = "DEVDMANXTDSC5 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmanxtdsc5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmanxtdsc5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc5`] module"]
#[doc(alias = "DEVDMANXTDSC5")]
pub type Devdmanxtdsc5 = crate::Reg<devdmanxtdsc5::Devdmanxtdsc5Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod devdmanxtdsc5;
#[doc = "DEVDMAADDRESS5 (rw) register accessor: Device DMA Channel Address Register (n = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmaaddress5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmaaddress5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress5`] module"]
#[doc(alias = "DEVDMAADDRESS5")]
pub type Devdmaaddress5 = crate::Reg<devdmaaddress5::Devdmaaddress5Spec>;
#[doc = "Device DMA Channel Address Register (n = 5)"]
pub mod devdmaaddress5;
#[doc = "DEVDMACONTROL5 (rw) register accessor: Device DMA Channel Control Register (n = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmacontrol5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmacontrol5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol5`] module"]
#[doc(alias = "DEVDMACONTROL5")]
pub type Devdmacontrol5 = crate::Reg<devdmacontrol5::Devdmacontrol5Spec>;
#[doc = "Device DMA Channel Control Register (n = 5)"]
pub mod devdmacontrol5;
#[doc = "DEVDMASTATUS5 (rw) register accessor: Device DMA Channel Status Register (n = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmastatus5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmastatus5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus5`] module"]
#[doc(alias = "DEVDMASTATUS5")]
pub type Devdmastatus5 = crate::Reg<devdmastatus5::Devdmastatus5Spec>;
#[doc = "Device DMA Channel Status Register (n = 5)"]
pub mod devdmastatus5;
#[doc = "DEVDMANXTDSC6 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmanxtdsc6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmanxtdsc6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc6`] module"]
#[doc(alias = "DEVDMANXTDSC6")]
pub type Devdmanxtdsc6 = crate::Reg<devdmanxtdsc6::Devdmanxtdsc6Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod devdmanxtdsc6;
#[doc = "DEVDMAADDRESS6 (rw) register accessor: Device DMA Channel Address Register (n = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmaaddress6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmaaddress6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress6`] module"]
#[doc(alias = "DEVDMAADDRESS6")]
pub type Devdmaaddress6 = crate::Reg<devdmaaddress6::Devdmaaddress6Spec>;
#[doc = "Device DMA Channel Address Register (n = 6)"]
pub mod devdmaaddress6;
#[doc = "DEVDMACONTROL6 (rw) register accessor: Device DMA Channel Control Register (n = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmacontrol6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmacontrol6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol6`] module"]
#[doc(alias = "DEVDMACONTROL6")]
pub type Devdmacontrol6 = crate::Reg<devdmacontrol6::Devdmacontrol6Spec>;
#[doc = "Device DMA Channel Control Register (n = 6)"]
pub mod devdmacontrol6;
#[doc = "DEVDMASTATUS6 (rw) register accessor: Device DMA Channel Status Register (n = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmastatus6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmastatus6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus6`] module"]
#[doc(alias = "DEVDMASTATUS6")]
pub type Devdmastatus6 = crate::Reg<devdmastatus6::Devdmastatus6Spec>;
#[doc = "Device DMA Channel Status Register (n = 6)"]
pub mod devdmastatus6;
#[doc = "DEVDMANXTDSC7 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 7)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmanxtdsc7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmanxtdsc7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc7`] module"]
#[doc(alias = "DEVDMANXTDSC7")]
pub type Devdmanxtdsc7 = crate::Reg<devdmanxtdsc7::Devdmanxtdsc7Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod devdmanxtdsc7;
#[doc = "DEVDMAADDRESS7 (rw) register accessor: Device DMA Channel Address Register (n = 7)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmaaddress7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmaaddress7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress7`] module"]
#[doc(alias = "DEVDMAADDRESS7")]
pub type Devdmaaddress7 = crate::Reg<devdmaaddress7::Devdmaaddress7Spec>;
#[doc = "Device DMA Channel Address Register (n = 7)"]
pub mod devdmaaddress7;
#[doc = "DEVDMACONTROL7 (rw) register accessor: Device DMA Channel Control Register (n = 7)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmacontrol7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmacontrol7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol7`] module"]
#[doc(alias = "DEVDMACONTROL7")]
pub type Devdmacontrol7 = crate::Reg<devdmacontrol7::Devdmacontrol7Spec>;
#[doc = "Device DMA Channel Control Register (n = 7)"]
pub mod devdmacontrol7;
#[doc = "DEVDMASTATUS7 (rw) register accessor: Device DMA Channel Status Register (n = 7)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmastatus7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmastatus7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus7`] module"]
#[doc(alias = "DEVDMASTATUS7")]
pub type Devdmastatus7 = crate::Reg<devdmastatus7::Devdmastatus7Spec>;
#[doc = "Device DMA Channel Status Register (n = 7)"]
pub mod devdmastatus7;
#[doc = "HSTCTRL (rw) register accessor: Host General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstctrl`] module"]
#[doc(alias = "HSTCTRL")]
pub type Hstctrl = crate::Reg<hstctrl::HstctrlSpec>;
#[doc = "Host General Control Register"]
pub mod hstctrl;
#[doc = "HSTISR (r) register accessor: Host Global Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstisr`] module"]
#[doc(alias = "HSTISR")]
pub type Hstisr = crate::Reg<hstisr::HstisrSpec>;
#[doc = "Host Global Interrupt Status Register"]
pub mod hstisr;
#[doc = "HSTICR (w) register accessor: Host Global Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsticr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsticr`] module"]
#[doc(alias = "HSTICR")]
pub type Hsticr = crate::Reg<hsticr::HsticrSpec>;
#[doc = "Host Global Interrupt Clear Register"]
pub mod hsticr;
#[doc = "HSTIFR (w) register accessor: Host Global Interrupt Set Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstifr`] module"]
#[doc(alias = "HSTIFR")]
pub type Hstifr = crate::Reg<hstifr::HstifrSpec>;
#[doc = "Host Global Interrupt Set Register"]
pub mod hstifr;
#[doc = "HSTIMR (r) register accessor: Host Global Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstimr`] module"]
#[doc(alias = "HSTIMR")]
pub type Hstimr = crate::Reg<hstimr::HstimrSpec>;
#[doc = "Host Global Interrupt Mask Register"]
pub mod hstimr;
#[doc = "HSTIDR (w) register accessor: Host Global Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstidr`] module"]
#[doc(alias = "HSTIDR")]
pub type Hstidr = crate::Reg<hstidr::HstidrSpec>;
#[doc = "Host Global Interrupt Disable Register"]
pub mod hstidr;
#[doc = "HSTIER (w) register accessor: Host Global Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstier`] module"]
#[doc(alias = "HSTIER")]
pub type Hstier = crate::Reg<hstier::HstierSpec>;
#[doc = "Host Global Interrupt Enable Register"]
pub mod hstier;
#[doc = "HSTPIP (rw) register accessor: Host Pipe Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpip`] module"]
#[doc(alias = "HSTPIP")]
pub type Hstpip = crate::Reg<hstpip::HstpipSpec>;
#[doc = "Host Pipe Register"]
pub mod hstpip;
#[doc = "HSTFNUM (rw) register accessor: Host Frame Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstfnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstfnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstfnum`] module"]
#[doc(alias = "HSTFNUM")]
pub type Hstfnum = crate::Reg<hstfnum::HstfnumSpec>;
#[doc = "Host Frame Number Register"]
pub mod hstfnum;
#[doc = "HSTADDR1 (rw) register accessor: Host Address 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstaddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstaddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstaddr1`] module"]
#[doc(alias = "HSTADDR1")]
pub type Hstaddr1 = crate::Reg<hstaddr1::Hstaddr1Spec>;
#[doc = "Host Address 1 Register"]
pub mod hstaddr1;
#[doc = "HSTADDR2 (rw) register accessor: Host Address 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstaddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstaddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstaddr2`] module"]
#[doc(alias = "HSTADDR2")]
pub type Hstaddr2 = crate::Reg<hstaddr2::Hstaddr2Spec>;
#[doc = "Host Address 2 Register"]
pub mod hstaddr2;
#[doc = "HSTADDR3 (rw) register accessor: Host Address 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstaddr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstaddr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstaddr3`] module"]
#[doc(alias = "HSTADDR3")]
pub type Hstaddr3 = crate::Reg<hstaddr3::Hstaddr3Spec>;
#[doc = "Host Address 3 Register"]
pub mod hstaddr3;
#[doc = "HSTPIPCFG (rw) register accessor: Host Pipe Configuration Register (n = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipcfg::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg`] module"]
#[doc(alias = "HSTPIPCFG")]
pub type Hstpipcfg = crate::Reg<hstpipcfg::HstpipcfgSpec>;
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub mod hstpipcfg;
#[doc = "HSTPIPISR (r) register accessor: Host Pipe Status Register (n = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr`] module"]
#[doc(alias = "HSTPIPISR")]
pub type Hstpipisr = crate::Reg<hstpipisr::HstpipisrSpec>;
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod hstpipisr;
#[doc = "HSTPIPICR (w) register accessor: Host Pipe Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr`] module"]
#[doc(alias = "HSTPIPICR")]
pub type Hstpipicr = crate::Reg<hstpipicr::HstpipicrSpec>;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod hstpipicr;
#[doc = "HSTPIPIFR (w) register accessor: Host Pipe Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr`] module"]
#[doc(alias = "HSTPIPIFR")]
pub type Hstpipifr = crate::Reg<hstpipifr::HstpipifrSpec>;
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod hstpipifr;
#[doc = "HSTPIPIMR (r) register accessor: Host Pipe Mask Register (n = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr`] module"]
#[doc(alias = "HSTPIPIMR")]
pub type Hstpipimr = crate::Reg<hstpipimr::HstpipimrSpec>;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod hstpipimr;
#[doc = "HSTPIPIER (w) register accessor: Host Pipe Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier`] module"]
#[doc(alias = "HSTPIPIER")]
pub type Hstpipier = crate::Reg<hstpipier::HstpipierSpec>;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod hstpipier;
#[doc = "HSTPIPIDR (w) register accessor: Host Pipe Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr`] module"]
#[doc(alias = "HSTPIPIDR")]
pub type Hstpipidr = crate::Reg<hstpipidr::HstpipidrSpec>;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod hstpipidr;
#[doc = "HSTPIPINRQ (rw) register accessor: Host Pipe IN Request Register (n = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipinrq::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipinrq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq`] module"]
#[doc(alias = "HSTPIPINRQ")]
pub type Hstpipinrq = crate::Reg<hstpipinrq::HstpipinrqSpec>;
#[doc = "Host Pipe IN Request Register (n = 0)"]
pub mod hstpipinrq;
#[doc = "HSTPIPERR (rw) register accessor: Host Pipe Error Register (n = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpiperr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpiperr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr`] module"]
#[doc(alias = "HSTPIPERR")]
pub type Hstpiperr = crate::Reg<hstpiperr::HstpiperrSpec>;
#[doc = "Host Pipe Error Register (n = 0)"]
pub mod hstpiperr;
#[doc = "HSTDMANXTDSC1 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmanxtdsc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmanxtdsc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc1`] module"]
#[doc(alias = "HSTDMANXTDSC1")]
pub type Hstdmanxtdsc1 = crate::Reg<hstdmanxtdsc1::Hstdmanxtdsc1Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod hstdmanxtdsc1;
#[doc = "HSTDMAADDRESS1 (rw) register accessor: Host DMA Channel Address Register (n = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress1`] module"]
#[doc(alias = "HSTDMAADDRESS1")]
pub type Hstdmaaddress1 = crate::Reg<hstdmaaddress1::Hstdmaaddress1Spec>;
#[doc = "Host DMA Channel Address Register (n = 1)"]
pub mod hstdmaaddress1;
#[doc = "HSTDMACONTROL1 (rw) register accessor: Host DMA Channel Control Register (n = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmacontrol1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmacontrol1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol1`] module"]
#[doc(alias = "HSTDMACONTROL1")]
pub type Hstdmacontrol1 = crate::Reg<hstdmacontrol1::Hstdmacontrol1Spec>;
#[doc = "Host DMA Channel Control Register (n = 1)"]
pub mod hstdmacontrol1;
#[doc = "HSTDMASTATUS1 (rw) register accessor: Host DMA Channel Status Register (n = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmastatus1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmastatus1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus1`] module"]
#[doc(alias = "HSTDMASTATUS1")]
pub type Hstdmastatus1 = crate::Reg<hstdmastatus1::Hstdmastatus1Spec>;
#[doc = "Host DMA Channel Status Register (n = 1)"]
pub mod hstdmastatus1;
#[doc = "HSTDMANXTDSC2 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmanxtdsc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmanxtdsc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc2`] module"]
#[doc(alias = "HSTDMANXTDSC2")]
pub type Hstdmanxtdsc2 = crate::Reg<hstdmanxtdsc2::Hstdmanxtdsc2Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod hstdmanxtdsc2;
#[doc = "HSTDMAADDRESS2 (rw) register accessor: Host DMA Channel Address Register (n = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress2`] module"]
#[doc(alias = "HSTDMAADDRESS2")]
pub type Hstdmaaddress2 = crate::Reg<hstdmaaddress2::Hstdmaaddress2Spec>;
#[doc = "Host DMA Channel Address Register (n = 2)"]
pub mod hstdmaaddress2;
#[doc = "HSTDMACONTROL2 (rw) register accessor: Host DMA Channel Control Register (n = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmacontrol2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmacontrol2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol2`] module"]
#[doc(alias = "HSTDMACONTROL2")]
pub type Hstdmacontrol2 = crate::Reg<hstdmacontrol2::Hstdmacontrol2Spec>;
#[doc = "Host DMA Channel Control Register (n = 2)"]
pub mod hstdmacontrol2;
#[doc = "HSTDMASTATUS2 (rw) register accessor: Host DMA Channel Status Register (n = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmastatus2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmastatus2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus2`] module"]
#[doc(alias = "HSTDMASTATUS2")]
pub type Hstdmastatus2 = crate::Reg<hstdmastatus2::Hstdmastatus2Spec>;
#[doc = "Host DMA Channel Status Register (n = 2)"]
pub mod hstdmastatus2;
#[doc = "HSTDMANXTDSC3 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmanxtdsc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmanxtdsc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc3`] module"]
#[doc(alias = "HSTDMANXTDSC3")]
pub type Hstdmanxtdsc3 = crate::Reg<hstdmanxtdsc3::Hstdmanxtdsc3Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod hstdmanxtdsc3;
#[doc = "HSTDMAADDRESS3 (rw) register accessor: Host DMA Channel Address Register (n = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress3`] module"]
#[doc(alias = "HSTDMAADDRESS3")]
pub type Hstdmaaddress3 = crate::Reg<hstdmaaddress3::Hstdmaaddress3Spec>;
#[doc = "Host DMA Channel Address Register (n = 3)"]
pub mod hstdmaaddress3;
#[doc = "HSTDMACONTROL3 (rw) register accessor: Host DMA Channel Control Register (n = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmacontrol3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmacontrol3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol3`] module"]
#[doc(alias = "HSTDMACONTROL3")]
pub type Hstdmacontrol3 = crate::Reg<hstdmacontrol3::Hstdmacontrol3Spec>;
#[doc = "Host DMA Channel Control Register (n = 3)"]
pub mod hstdmacontrol3;
#[doc = "HSTDMASTATUS3 (rw) register accessor: Host DMA Channel Status Register (n = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmastatus3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmastatus3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus3`] module"]
#[doc(alias = "HSTDMASTATUS3")]
pub type Hstdmastatus3 = crate::Reg<hstdmastatus3::Hstdmastatus3Spec>;
#[doc = "Host DMA Channel Status Register (n = 3)"]
pub mod hstdmastatus3;
#[doc = "HSTDMANXTDSC4 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmanxtdsc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmanxtdsc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc4`] module"]
#[doc(alias = "HSTDMANXTDSC4")]
pub type Hstdmanxtdsc4 = crate::Reg<hstdmanxtdsc4::Hstdmanxtdsc4Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod hstdmanxtdsc4;
#[doc = "HSTDMAADDRESS4 (rw) register accessor: Host DMA Channel Address Register (n = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress4`] module"]
#[doc(alias = "HSTDMAADDRESS4")]
pub type Hstdmaaddress4 = crate::Reg<hstdmaaddress4::Hstdmaaddress4Spec>;
#[doc = "Host DMA Channel Address Register (n = 4)"]
pub mod hstdmaaddress4;
#[doc = "HSTDMACONTROL4 (rw) register accessor: Host DMA Channel Control Register (n = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmacontrol4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmacontrol4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol4`] module"]
#[doc(alias = "HSTDMACONTROL4")]
pub type Hstdmacontrol4 = crate::Reg<hstdmacontrol4::Hstdmacontrol4Spec>;
#[doc = "Host DMA Channel Control Register (n = 4)"]
pub mod hstdmacontrol4;
#[doc = "HSTDMASTATUS4 (rw) register accessor: Host DMA Channel Status Register (n = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmastatus4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmastatus4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus4`] module"]
#[doc(alias = "HSTDMASTATUS4")]
pub type Hstdmastatus4 = crate::Reg<hstdmastatus4::Hstdmastatus4Spec>;
#[doc = "Host DMA Channel Status Register (n = 4)"]
pub mod hstdmastatus4;
#[doc = "HSTDMANXTDSC5 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmanxtdsc5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmanxtdsc5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc5`] module"]
#[doc(alias = "HSTDMANXTDSC5")]
pub type Hstdmanxtdsc5 = crate::Reg<hstdmanxtdsc5::Hstdmanxtdsc5Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod hstdmanxtdsc5;
#[doc = "HSTDMAADDRESS5 (rw) register accessor: Host DMA Channel Address Register (n = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress5`] module"]
#[doc(alias = "HSTDMAADDRESS5")]
pub type Hstdmaaddress5 = crate::Reg<hstdmaaddress5::Hstdmaaddress5Spec>;
#[doc = "Host DMA Channel Address Register (n = 5)"]
pub mod hstdmaaddress5;
#[doc = "HSTDMACONTROL5 (rw) register accessor: Host DMA Channel Control Register (n = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmacontrol5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmacontrol5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol5`] module"]
#[doc(alias = "HSTDMACONTROL5")]
pub type Hstdmacontrol5 = crate::Reg<hstdmacontrol5::Hstdmacontrol5Spec>;
#[doc = "Host DMA Channel Control Register (n = 5)"]
pub mod hstdmacontrol5;
#[doc = "HSTDMASTATUS5 (rw) register accessor: Host DMA Channel Status Register (n = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmastatus5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmastatus5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus5`] module"]
#[doc(alias = "HSTDMASTATUS5")]
pub type Hstdmastatus5 = crate::Reg<hstdmastatus5::Hstdmastatus5Spec>;
#[doc = "Host DMA Channel Status Register (n = 5)"]
pub mod hstdmastatus5;
#[doc = "HSTDMANXTDSC6 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmanxtdsc6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmanxtdsc6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc6`] module"]
#[doc(alias = "HSTDMANXTDSC6")]
pub type Hstdmanxtdsc6 = crate::Reg<hstdmanxtdsc6::Hstdmanxtdsc6Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod hstdmanxtdsc6;
#[doc = "HSTDMAADDRESS6 (rw) register accessor: Host DMA Channel Address Register (n = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress6`] module"]
#[doc(alias = "HSTDMAADDRESS6")]
pub type Hstdmaaddress6 = crate::Reg<hstdmaaddress6::Hstdmaaddress6Spec>;
#[doc = "Host DMA Channel Address Register (n = 6)"]
pub mod hstdmaaddress6;
#[doc = "HSTDMACONTROL6 (rw) register accessor: Host DMA Channel Control Register (n = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmacontrol6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmacontrol6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol6`] module"]
#[doc(alias = "HSTDMACONTROL6")]
pub type Hstdmacontrol6 = crate::Reg<hstdmacontrol6::Hstdmacontrol6Spec>;
#[doc = "Host DMA Channel Control Register (n = 6)"]
pub mod hstdmacontrol6;
#[doc = "HSTDMASTATUS6 (rw) register accessor: Host DMA Channel Status Register (n = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmastatus6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmastatus6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus6`] module"]
#[doc(alias = "HSTDMASTATUS6")]
pub type Hstdmastatus6 = crate::Reg<hstdmastatus6::Hstdmastatus6Spec>;
#[doc = "Host DMA Channel Status Register (n = 6)"]
pub mod hstdmastatus6;
#[doc = "HSTDMANXTDSC7 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 7)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmanxtdsc7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmanxtdsc7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc7`] module"]
#[doc(alias = "HSTDMANXTDSC7")]
pub type Hstdmanxtdsc7 = crate::Reg<hstdmanxtdsc7::Hstdmanxtdsc7Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod hstdmanxtdsc7;
#[doc = "HSTDMAADDRESS7 (rw) register accessor: Host DMA Channel Address Register (n = 7)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress7`] module"]
#[doc(alias = "HSTDMAADDRESS7")]
pub type Hstdmaaddress7 = crate::Reg<hstdmaaddress7::Hstdmaaddress7Spec>;
#[doc = "Host DMA Channel Address Register (n = 7)"]
pub mod hstdmaaddress7;
#[doc = "HSTDMACONTROL7 (rw) register accessor: Host DMA Channel Control Register (n = 7)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmacontrol7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmacontrol7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol7`] module"]
#[doc(alias = "HSTDMACONTROL7")]
pub type Hstdmacontrol7 = crate::Reg<hstdmacontrol7::Hstdmacontrol7Spec>;
#[doc = "Host DMA Channel Control Register (n = 7)"]
pub mod hstdmacontrol7;
#[doc = "HSTDMASTATUS7 (rw) register accessor: Host DMA Channel Status Register (n = 7)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmastatus7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmastatus7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus7`] module"]
#[doc(alias = "HSTDMASTATUS7")]
pub type Hstdmastatus7 = crate::Reg<hstdmastatus7::Hstdmastatus7Spec>;
#[doc = "Host DMA Channel Status Register (n = 7)"]
pub mod hstdmastatus7;
#[doc = "CTRL (rw) register accessor: General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "General Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: General Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "General Status Register"]
pub mod sr;
#[doc = "SCR (w) register accessor: General Status Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "General Status Clear Register"]
pub mod scr;
#[doc = "SFR (w) register accessor: General Status Set Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr`] module"]
#[doc(alias = "SFR")]
pub type Sfr = crate::Reg<sfr::SfrSpec>;
#[doc = "General Status Set Register"]
pub mod sfr;
#[doc = "FSM (r) register accessor: General Finite State Machine Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm`] module"]
#[doc(alias = "FSM")]
pub type Fsm = crate::Reg<fsm::FsmSpec>;
#[doc = "General Finite State Machine Register"]
pub mod fsm;
