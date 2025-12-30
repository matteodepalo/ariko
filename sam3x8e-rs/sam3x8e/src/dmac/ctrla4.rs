#[doc = "Register `CTRLA4` reader"]
pub type R = crate::R<Ctrla4Spec>;
#[doc = "Register `CTRLA4` writer"]
pub type W = crate::W<Ctrla4Spec>;
#[doc = "Field `BTSIZE` reader - Buffer Transfer Size"]
pub type BtsizeR = crate::FieldReader<u16>;
#[doc = "Field `BTSIZE` writer - Buffer Transfer Size"]
pub type BtsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Source Chunk Transfer Size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scsize {
  #[doc = "0: 1 data transferred"]
  Chk1 = 0,
  #[doc = "1: 4 data transferred"]
  Chk4 = 1,
  #[doc = "2: 8 data transferred"]
  Chk8 = 2,
  #[doc = "3: 16 data transferred"]
  Chk16 = 3,
  #[doc = "4: 32 data transferred"]
  Chk32 = 4,
  #[doc = "5: 64 data transferred"]
  Chk64 = 5,
  #[doc = "6: 128 data transferred"]
  Chk128 = 6,
  #[doc = "7: 256 data transferred"]
  Chk256 = 7,
}
impl From<Scsize> for u8 {
  #[inline(always)]
  fn from(variant: Scsize) -> Self {
    variant as _
  }
}
impl crate::FieldSpec for Scsize {
  type Ux = u8;
}
impl crate::IsEnum for Scsize {}
#[doc = "Field `SCSIZE` reader - Source Chunk Transfer Size."]
pub type ScsizeR = crate::FieldReader<Scsize>;
impl ScsizeR {
  #[doc = "Get enumerated values variant"]
  #[inline(always)]
  pub const fn variant(&self) -> Scsize {
    match self.bits {
      0 => Scsize::Chk1,
      1 => Scsize::Chk4,
      2 => Scsize::Chk8,
      3 => Scsize::Chk16,
      4 => Scsize::Chk32,
      5 => Scsize::Chk64,
      6 => Scsize::Chk128,
      7 => Scsize::Chk256,
      _ => unreachable!(),
    }
  }
  #[doc = "1 data transferred"]
  #[inline(always)]
  pub fn is_chk_1(&self) -> bool {
    *self == Scsize::Chk1
  }
  #[doc = "4 data transferred"]
  #[inline(always)]
  pub fn is_chk_4(&self) -> bool {
    *self == Scsize::Chk4
  }
  #[doc = "8 data transferred"]
  #[inline(always)]
  pub fn is_chk_8(&self) -> bool {
    *self == Scsize::Chk8
  }
  #[doc = "16 data transferred"]
  #[inline(always)]
  pub fn is_chk_16(&self) -> bool {
    *self == Scsize::Chk16
  }
  #[doc = "32 data transferred"]
  #[inline(always)]
  pub fn is_chk_32(&self) -> bool {
    *self == Scsize::Chk32
  }
  #[doc = "64 data transferred"]
  #[inline(always)]
  pub fn is_chk_64(&self) -> bool {
    *self == Scsize::Chk64
  }
  #[doc = "128 data transferred"]
  #[inline(always)]
  pub fn is_chk_128(&self) -> bool {
    *self == Scsize::Chk128
  }
  #[doc = "256 data transferred"]
  #[inline(always)]
  pub fn is_chk_256(&self) -> bool {
    *self == Scsize::Chk256
  }
}
#[doc = "Field `SCSIZE` writer - Source Chunk Transfer Size."]
pub type ScsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Scsize, crate::Safe>;
impl<'a, REG> ScsizeW<'a, REG>
where
  REG: crate::Writable + crate::RegisterSpec,
  REG::Ux: From<u8>,
{
  #[doc = "1 data transferred"]
  #[inline(always)]
  pub fn chk_1(self) -> &'a mut crate::W<REG> {
    self.variant(Scsize::Chk1)
  }
  #[doc = "4 data transferred"]
  #[inline(always)]
  pub fn chk_4(self) -> &'a mut crate::W<REG> {
    self.variant(Scsize::Chk4)
  }
  #[doc = "8 data transferred"]
  #[inline(always)]
  pub fn chk_8(self) -> &'a mut crate::W<REG> {
    self.variant(Scsize::Chk8)
  }
  #[doc = "16 data transferred"]
  #[inline(always)]
  pub fn chk_16(self) -> &'a mut crate::W<REG> {
    self.variant(Scsize::Chk16)
  }
  #[doc = "32 data transferred"]
  #[inline(always)]
  pub fn chk_32(self) -> &'a mut crate::W<REG> {
    self.variant(Scsize::Chk32)
  }
  #[doc = "64 data transferred"]
  #[inline(always)]
  pub fn chk_64(self) -> &'a mut crate::W<REG> {
    self.variant(Scsize::Chk64)
  }
  #[doc = "128 data transferred"]
  #[inline(always)]
  pub fn chk_128(self) -> &'a mut crate::W<REG> {
    self.variant(Scsize::Chk128)
  }
  #[doc = "256 data transferred"]
  #[inline(always)]
  pub fn chk_256(self) -> &'a mut crate::W<REG> {
    self.variant(Scsize::Chk256)
  }
}
#[doc = "Destination Chunk Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcsize {
  #[doc = "0: 1 data transferred"]
  Chk1 = 0,
  #[doc = "1: 4 data transferred"]
  Chk4 = 1,
  #[doc = "2: 8 data transferred"]
  Chk8 = 2,
  #[doc = "3: 16 data transferred"]
  Chk16 = 3,
  #[doc = "4: 32 data transferred"]
  Chk32 = 4,
  #[doc = "5: 64 data transferred"]
  Chk64 = 5,
  #[doc = "6: 128 data transferred"]
  Chk128 = 6,
  #[doc = "7: 256 data transferred"]
  Chk256 = 7,
}
impl From<Dcsize> for u8 {
  #[inline(always)]
  fn from(variant: Dcsize) -> Self {
    variant as _
  }
}
impl crate::FieldSpec for Dcsize {
  type Ux = u8;
}
impl crate::IsEnum for Dcsize {}
#[doc = "Field `DCSIZE` reader - Destination Chunk Transfer Size"]
pub type DcsizeR = crate::FieldReader<Dcsize>;
impl DcsizeR {
  #[doc = "Get enumerated values variant"]
  #[inline(always)]
  pub const fn variant(&self) -> Dcsize {
    match self.bits {
      0 => Dcsize::Chk1,
      1 => Dcsize::Chk4,
      2 => Dcsize::Chk8,
      3 => Dcsize::Chk16,
      4 => Dcsize::Chk32,
      5 => Dcsize::Chk64,
      6 => Dcsize::Chk128,
      7 => Dcsize::Chk256,
      _ => unreachable!(),
    }
  }
  #[doc = "1 data transferred"]
  #[inline(always)]
  pub fn is_chk_1(&self) -> bool {
    *self == Dcsize::Chk1
  }
  #[doc = "4 data transferred"]
  #[inline(always)]
  pub fn is_chk_4(&self) -> bool {
    *self == Dcsize::Chk4
  }
  #[doc = "8 data transferred"]
  #[inline(always)]
  pub fn is_chk_8(&self) -> bool {
    *self == Dcsize::Chk8
  }
  #[doc = "16 data transferred"]
  #[inline(always)]
  pub fn is_chk_16(&self) -> bool {
    *self == Dcsize::Chk16
  }
  #[doc = "32 data transferred"]
  #[inline(always)]
  pub fn is_chk_32(&self) -> bool {
    *self == Dcsize::Chk32
  }
  #[doc = "64 data transferred"]
  #[inline(always)]
  pub fn is_chk_64(&self) -> bool {
    *self == Dcsize::Chk64
  }
  #[doc = "128 data transferred"]
  #[inline(always)]
  pub fn is_chk_128(&self) -> bool {
    *self == Dcsize::Chk128
  }
  #[doc = "256 data transferred"]
  #[inline(always)]
  pub fn is_chk_256(&self) -> bool {
    *self == Dcsize::Chk256
  }
}
#[doc = "Field `DCSIZE` writer - Destination Chunk Transfer Size"]
pub type DcsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dcsize, crate::Safe>;
impl<'a, REG> DcsizeW<'a, REG>
where
  REG: crate::Writable + crate::RegisterSpec,
  REG::Ux: From<u8>,
{
  #[doc = "1 data transferred"]
  #[inline(always)]
  pub fn chk_1(self) -> &'a mut crate::W<REG> {
    self.variant(Dcsize::Chk1)
  }
  #[doc = "4 data transferred"]
  #[inline(always)]
  pub fn chk_4(self) -> &'a mut crate::W<REG> {
    self.variant(Dcsize::Chk4)
  }
  #[doc = "8 data transferred"]
  #[inline(always)]
  pub fn chk_8(self) -> &'a mut crate::W<REG> {
    self.variant(Dcsize::Chk8)
  }
  #[doc = "16 data transferred"]
  #[inline(always)]
  pub fn chk_16(self) -> &'a mut crate::W<REG> {
    self.variant(Dcsize::Chk16)
  }
  #[doc = "32 data transferred"]
  #[inline(always)]
  pub fn chk_32(self) -> &'a mut crate::W<REG> {
    self.variant(Dcsize::Chk32)
  }
  #[doc = "64 data transferred"]
  #[inline(always)]
  pub fn chk_64(self) -> &'a mut crate::W<REG> {
    self.variant(Dcsize::Chk64)
  }
  #[doc = "128 data transferred"]
  #[inline(always)]
  pub fn chk_128(self) -> &'a mut crate::W<REG> {
    self.variant(Dcsize::Chk128)
  }
  #[doc = "256 data transferred"]
  #[inline(always)]
  pub fn chk_256(self) -> &'a mut crate::W<REG> {
    self.variant(Dcsize::Chk256)
  }
}
#[doc = "Transfer Width for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SrcWidth {
  #[doc = "0: the transfer size is set to 8-bit width"]
  Byte = 0,
  #[doc = "1: the transfer size is set to 16-bit width"]
  HalfWord = 1,
  #[doc = "2: the transfer size is set to 32-bit width"]
  Word = 2,
}
impl From<SrcWidth> for u8 {
  #[inline(always)]
  fn from(variant: SrcWidth) -> Self {
    variant as _
  }
}
impl crate::FieldSpec for SrcWidth {
  type Ux = u8;
}
impl crate::IsEnum for SrcWidth {}
#[doc = "Field `SRC_WIDTH` reader - Transfer Width for the Source"]
pub type SrcWidthR = crate::FieldReader<SrcWidth>;
impl SrcWidthR {
  #[doc = "Get enumerated values variant"]
  #[inline(always)]
  pub const fn variant(&self) -> Option<SrcWidth> {
    match self.bits {
      0 => Some(SrcWidth::Byte),
      1 => Some(SrcWidth::HalfWord),
      2 => Some(SrcWidth::Word),
      _ => None,
    }
  }
  #[doc = "the transfer size is set to 8-bit width"]
  #[inline(always)]
  pub fn is_byte(&self) -> bool {
    *self == SrcWidth::Byte
  }
  #[doc = "the transfer size is set to 16-bit width"]
  #[inline(always)]
  pub fn is_half_word(&self) -> bool {
    *self == SrcWidth::HalfWord
  }
  #[doc = "the transfer size is set to 32-bit width"]
  #[inline(always)]
  pub fn is_word(&self) -> bool {
    *self == SrcWidth::Word
  }
}
#[doc = "Field `SRC_WIDTH` writer - Transfer Width for the Source"]
pub type SrcWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, SrcWidth>;
impl<'a, REG> SrcWidthW<'a, REG>
where
  REG: crate::Writable + crate::RegisterSpec,
  REG::Ux: From<u8>,
{
  #[doc = "the transfer size is set to 8-bit width"]
  #[inline(always)]
  pub fn byte(self) -> &'a mut crate::W<REG> {
    self.variant(SrcWidth::Byte)
  }
  #[doc = "the transfer size is set to 16-bit width"]
  #[inline(always)]
  pub fn half_word(self) -> &'a mut crate::W<REG> {
    self.variant(SrcWidth::HalfWord)
  }
  #[doc = "the transfer size is set to 32-bit width"]
  #[inline(always)]
  pub fn word(self) -> &'a mut crate::W<REG> {
    self.variant(SrcWidth::Word)
  }
}
#[doc = "Transfer Width for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DstWidth {
  #[doc = "0: the transfer size is set to 8-bit width"]
  Byte = 0,
  #[doc = "1: the transfer size is set to 16-bit width"]
  HalfWord = 1,
  #[doc = "2: the transfer size is set to 32-bit width"]
  Word = 2,
}
impl From<DstWidth> for u8 {
  #[inline(always)]
  fn from(variant: DstWidth) -> Self {
    variant as _
  }
}
impl crate::FieldSpec for DstWidth {
  type Ux = u8;
}
impl crate::IsEnum for DstWidth {}
#[doc = "Field `DST_WIDTH` reader - Transfer Width for the Destination"]
pub type DstWidthR = crate::FieldReader<DstWidth>;
impl DstWidthR {
  #[doc = "Get enumerated values variant"]
  #[inline(always)]
  pub const fn variant(&self) -> Option<DstWidth> {
    match self.bits {
      0 => Some(DstWidth::Byte),
      1 => Some(DstWidth::HalfWord),
      2 => Some(DstWidth::Word),
      _ => None,
    }
  }
  #[doc = "the transfer size is set to 8-bit width"]
  #[inline(always)]
  pub fn is_byte(&self) -> bool {
    *self == DstWidth::Byte
  }
  #[doc = "the transfer size is set to 16-bit width"]
  #[inline(always)]
  pub fn is_half_word(&self) -> bool {
    *self == DstWidth::HalfWord
  }
  #[doc = "the transfer size is set to 32-bit width"]
  #[inline(always)]
  pub fn is_word(&self) -> bool {
    *self == DstWidth::Word
  }
}
#[doc = "Field `DST_WIDTH` writer - Transfer Width for the Destination"]
pub type DstWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, DstWidth>;
impl<'a, REG> DstWidthW<'a, REG>
where
  REG: crate::Writable + crate::RegisterSpec,
  REG::Ux: From<u8>,
{
  #[doc = "the transfer size is set to 8-bit width"]
  #[inline(always)]
  pub fn byte(self) -> &'a mut crate::W<REG> {
    self.variant(DstWidth::Byte)
  }
  #[doc = "the transfer size is set to 16-bit width"]
  #[inline(always)]
  pub fn half_word(self) -> &'a mut crate::W<REG> {
    self.variant(DstWidth::HalfWord)
  }
  #[doc = "the transfer size is set to 32-bit width"]
  #[inline(always)]
  pub fn word(self) -> &'a mut crate::W<REG> {
    self.variant(DstWidth::Word)
  }
}
#[doc = "Field `DONE` reader - "]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - "]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
  #[doc = "Bits 0:15 - Buffer Transfer Size"]
  #[inline(always)]
  pub fn btsize(&self) -> BtsizeR {
    BtsizeR::new((self.bits & 0xffff) as u16)
  }
  #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
  #[inline(always)]
  pub fn scsize(&self) -> ScsizeR {
    ScsizeR::new(((self.bits >> 16) & 7) as u8)
  }
  #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
  #[inline(always)]
  pub fn dcsize(&self) -> DcsizeR {
    DcsizeR::new(((self.bits >> 20) & 7) as u8)
  }
  #[doc = "Bits 24:25 - Transfer Width for the Source"]
  #[inline(always)]
  pub fn src_width(&self) -> SrcWidthR {
    SrcWidthR::new(((self.bits >> 24) & 3) as u8)
  }
  #[doc = "Bits 28:29 - Transfer Width for the Destination"]
  #[inline(always)]
  pub fn dst_width(&self) -> DstWidthR {
    DstWidthR::new(((self.bits >> 28) & 3) as u8)
  }
  #[doc = "Bit 31"]
  #[inline(always)]
  pub fn done(&self) -> DoneR {
    DoneR::new(((self.bits >> 31) & 1) != 0)
  }
}
impl W {
  #[doc = "Bits 0:15 - Buffer Transfer Size"]
  #[inline(always)]
  pub fn btsize(&mut self) -> BtsizeW<'_, Ctrla4Spec> {
    BtsizeW::new(self, 0)
  }
  #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
  #[inline(always)]
  pub fn scsize(&mut self) -> ScsizeW<'_, Ctrla4Spec> {
    ScsizeW::new(self, 16)
  }
  #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
  #[inline(always)]
  pub fn dcsize(&mut self) -> DcsizeW<'_, Ctrla4Spec> {
    DcsizeW::new(self, 20)
  }
  #[doc = "Bits 24:25 - Transfer Width for the Source"]
  #[inline(always)]
  pub fn src_width(&mut self) -> SrcWidthW<'_, Ctrla4Spec> {
    SrcWidthW::new(self, 24)
  }
  #[doc = "Bits 28:29 - Transfer Width for the Destination"]
  #[inline(always)]
  pub fn dst_width(&mut self) -> DstWidthW<'_, Ctrla4Spec> {
    DstWidthW::new(self, 28)
  }
  #[doc = "Bit 31"]
  #[inline(always)]
  pub fn done(&mut self) -> DoneW<'_, Ctrla4Spec> {
    DoneW::new(self, 31)
  }
}
#[doc = "DMAC Channel Control A Register (ch_num = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrla4Spec;
impl crate::RegisterSpec for Ctrla4Spec {
  type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla4::R`](R) reader structure"]
impl crate::Readable for Ctrla4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrla4::W`](W) writer structure"]
impl crate::Writable for Ctrla4Spec {
  type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA4 to value 0"]
impl crate::Resettable for Ctrla4Spec {}
