#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::vk::*;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1ChromaSamplePosition(pub i32);

impl StdVideoAV1ChromaSamplePosition {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNKNOWN: Self = Self(0);
    pub const VERTICAL: Self = Self(1);
    pub const COLOCATED: Self = Self(2);
    pub const RESERVED: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1ColorPrimaries(pub i32);

impl StdVideoAV1ColorPrimaries {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BT_709: Self = Self(1);
    pub const UNSPECIFIED: Self = Self(2);
    pub const BT_470_M: Self = Self(4);
    pub const BT_470_B_G: Self = Self(5);
    pub const BT_601: Self = Self(6);
    pub const SMPTE_240: Self = Self(7);
    pub const GENERIC_FILM: Self = Self(8);
    pub const BT_2020: Self = Self(9);
    pub const XYZ: Self = Self(10);
    pub const SMPTE_431: Self = Self(11);
    pub const SMPTE_432: Self = Self(12);
    pub const EBU_3213: Self = Self(22);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1FrameRestorationType(pub i32);

impl StdVideoAV1FrameRestorationType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NONE: Self = Self(0);
    pub const WIENER: Self = Self(1);
    pub const SGRPROJ: Self = Self(2);
    pub const SWITCHABLE: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1FrameType(pub i32);

impl StdVideoAV1FrameType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const KEY: Self = Self(0);
    pub const INTER: Self = Self(1);
    pub const INTRA_ONLY: Self = Self(2);
    pub const SWITCH: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1InterpolationFilter(pub i32);

impl StdVideoAV1InterpolationFilter {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const EIGHTTAP: Self = Self(0);
    pub const EIGHTTAP_SMOOTH: Self = Self(1);
    pub const EIGHTTAP_SHARP: Self = Self(2);
    pub const BILINEAR: Self = Self(3);
    pub const SWITCHABLE: Self = Self(4);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1Level(pub i32);

impl StdVideoAV1Level {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _2_0: Self = Self(0);
    pub const _2_1: Self = Self(1);
    pub const _2_2: Self = Self(2);
    pub const _2_3: Self = Self(3);
    pub const _3_0: Self = Self(4);
    pub const _3_1: Self = Self(5);
    pub const _3_2: Self = Self(6);
    pub const _3_3: Self = Self(7);
    pub const _4_0: Self = Self(8);
    pub const _4_1: Self = Self(9);
    pub const _4_2: Self = Self(10);
    pub const _4_3: Self = Self(11);
    pub const _5_0: Self = Self(12);
    pub const _5_1: Self = Self(13);
    pub const _5_2: Self = Self(14);
    pub const _5_3: Self = Self(15);
    pub const _6_0: Self = Self(16);
    pub const _6_1: Self = Self(17);
    pub const _6_2: Self = Self(18);
    pub const _6_3: Self = Self(19);
    pub const _7_0: Self = Self(20);
    pub const _7_1: Self = Self(21);
    pub const _7_2: Self = Self(22);
    pub const _7_3: Self = Self(23);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1MatrixCoefficients(pub i32);

impl StdVideoAV1MatrixCoefficients {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const IDENTITY: Self = Self(0);
    pub const BT_709: Self = Self(1);
    pub const UNSPECIFIED: Self = Self(2);
    pub const RESERVED_3: Self = Self(3);
    pub const FCC: Self = Self(4);
    pub const BT_470_B_G: Self = Self(5);
    pub const BT_601: Self = Self(6);
    pub const SMPTE_240: Self = Self(7);
    pub const SMPTE_YCGCO: Self = Self(8);
    pub const BT_2020_NCL: Self = Self(9);
    pub const BT_2020_CL: Self = Self(10);
    pub const SMPTE_2085: Self = Self(11);
    pub const CHROMAT_NCL: Self = Self(12);
    pub const CHROMAT_CL: Self = Self(13);
    pub const ICTCP: Self = Self(14);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1Profile(pub i32);

impl StdVideoAV1Profile {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MAIN: Self = Self(0);
    pub const HIGH: Self = Self(1);
    pub const PROFESSIONAL: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1ReferenceName(pub i32);

impl StdVideoAV1ReferenceName {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INTRA_FRAME: Self = Self(0);
    pub const LAST_FRAME: Self = Self(1);
    pub const LAST2_FRAME: Self = Self(2);
    pub const LAST3_FRAME: Self = Self(3);
    pub const GOLDEN_FRAME: Self = Self(4);
    pub const BWDREF_FRAME: Self = Self(5);
    pub const ALTREF2_FRAME: Self = Self(6);
    pub const ALTREF_FRAME: Self = Self(7);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1TransferCharacteristics(pub i32);

impl StdVideoAV1TransferCharacteristics {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const RESERVED_0: Self = Self(0);
    pub const BT_709: Self = Self(1);
    pub const UNSPECIFIED: Self = Self(2);
    pub const RESERVED_3: Self = Self(3);
    pub const BT_470_M: Self = Self(4);
    pub const BT_470_B_G: Self = Self(5);
    pub const BT_601: Self = Self(6);
    pub const SMPTE_240: Self = Self(7);
    pub const LINEAR: Self = Self(8);
    pub const LOG_100: Self = Self(9);
    pub const LOG_100_SQRT10: Self = Self(10);
    pub const IEC_61966: Self = Self(11);
    pub const BT_1361: Self = Self(12);
    pub const SRGB: Self = Self(13);
    pub const BT_2020_10_BIT: Self = Self(14);
    pub const BT_2020_12_BIT: Self = Self(15);
    pub const SMPTE_2084: Self = Self(16);
    pub const SMPTE_428: Self = Self(17);
    pub const HLG: Self = Self(18);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoAV1TxMode(pub i32);

impl StdVideoAV1TxMode {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ONLY_4X4: Self = Self(0);
    pub const LARGEST: Self = Self(1);
    pub const SELECT: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoDecodeH264FieldOrderCount(pub i32);

impl StdVideoDecodeH264FieldOrderCount {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TOP: Self = Self(0);
    pub const BOTTOM: Self = Self(1);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264AspectRatioIdc(pub i32);

impl StdVideoH264AspectRatioIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNSPECIFIED: Self = Self(0);
    pub const SQUARE: Self = Self(1);
    pub const _12_11: Self = Self(2);
    pub const _10_11: Self = Self(3);
    pub const _16_11: Self = Self(4);
    pub const _40_33: Self = Self(5);
    pub const _24_11: Self = Self(6);
    pub const _20_11: Self = Self(7);
    pub const _32_11: Self = Self(8);
    pub const _80_33: Self = Self(9);
    pub const _18_11: Self = Self(10);
    pub const _15_11: Self = Self(11);
    pub const _64_33: Self = Self(12);
    pub const _160_99: Self = Self(13);
    pub const _4_3: Self = Self(14);
    pub const _3_2: Self = Self(15);
    pub const _2_1: Self = Self(16);
    pub const EXTENDED_SAR: Self = Self(255);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264CabacInitIdc(pub i32);

impl StdVideoH264CabacInitIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _0: Self = Self(0);
    pub const _1: Self = Self(1);
    pub const _2: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264ChromaFormatIdc(pub i32);

impl StdVideoH264ChromaFormatIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MONOCHROME: Self = Self(0);
    pub const _420: Self = Self(1);
    pub const _422: Self = Self(2);
    pub const _444: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264DisableDeblockingFilterIdc(pub i32);

impl StdVideoH264DisableDeblockingFilterIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DISABLED: Self = Self(0);
    pub const ENABLED: Self = Self(1);
    pub const PARTIAL: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264LevelIdc(pub i32);

impl StdVideoH264LevelIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _1_0: Self = Self(0);
    pub const _1_1: Self = Self(1);
    pub const _1_2: Self = Self(2);
    pub const _1_3: Self = Self(3);
    pub const _2_0: Self = Self(4);
    pub const _2_1: Self = Self(5);
    pub const _2_2: Self = Self(6);
    pub const _3_0: Self = Self(7);
    pub const _3_1: Self = Self(8);
    pub const _3_2: Self = Self(9);
    pub const _4_0: Self = Self(10);
    pub const _4_1: Self = Self(11);
    pub const _4_2: Self = Self(12);
    pub const _5_0: Self = Self(13);
    pub const _5_1: Self = Self(14);
    pub const _5_2: Self = Self(15);
    pub const _6_0: Self = Self(16);
    pub const _6_1: Self = Self(17);
    pub const _6_2: Self = Self(18);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264MemMgmtControlOp(pub i32);

impl StdVideoH264MemMgmtControlOp {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const END: Self = Self(0);
    pub const UNMARK_SHORT_TERM: Self = Self(1);
    pub const UNMARK_LONG_TERM: Self = Self(2);
    pub const MARK_LONG_TERM: Self = Self(3);
    pub const SET_MAX_LONG_TERM_INDEX: Self = Self(4);
    pub const UNMARK_ALL: Self = Self(5);
    pub const MARK_CURRENT_AS_LONG_TERM: Self = Self(6);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264ModificationOfPicNumsIdc(pub i32);

impl StdVideoH264ModificationOfPicNumsIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SHORT_TERM_SUBTRACT: Self = Self(0);
    pub const SHORT_TERM_ADD: Self = Self(1);
    pub const LONG_TERM: Self = Self(2);
    pub const END: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264NonVclNaluType(pub i32);

impl StdVideoH264NonVclNaluType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SPS: Self = Self(0);
    pub const PPS: Self = Self(1);
    pub const AUD: Self = Self(2);
    pub const PREFIX: Self = Self(3);
    pub const END_OF_SEQUENCE: Self = Self(4);
    pub const END_OF_STREAM: Self = Self(5);
    pub const PRECODED: Self = Self(6);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264PictureType(pub i32);

impl StdVideoH264PictureType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const IDR: Self = Self(5);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264PocType(pub i32);

impl StdVideoH264PocType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _0: Self = Self(0);
    pub const _1: Self = Self(1);
    pub const _2: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264ProfileIdc(pub i32);

impl StdVideoH264ProfileIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BASELINE: Self = Self(66);
    pub const MAIN: Self = Self(77);
    pub const HIGH: Self = Self(100);
    pub const HIGH_444_PREDICTIVE: Self = Self(244);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264SliceType(pub i32);

impl StdVideoH264SliceType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH264WeightedBipredIdc(pub i32);

impl StdVideoH264WeightedBipredIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const EXPLICIT: Self = Self(1);
    pub const IMPLICIT: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH265AspectRatioIdc(pub i32);

impl StdVideoH265AspectRatioIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNSPECIFIED: Self = Self(0);
    pub const SQUARE: Self = Self(1);
    pub const _12_11: Self = Self(2);
    pub const _10_11: Self = Self(3);
    pub const _16_11: Self = Self(4);
    pub const _40_33: Self = Self(5);
    pub const _24_11: Self = Self(6);
    pub const _20_11: Self = Self(7);
    pub const _32_11: Self = Self(8);
    pub const _80_33: Self = Self(9);
    pub const _18_11: Self = Self(10);
    pub const _15_11: Self = Self(11);
    pub const _64_33: Self = Self(12);
    pub const _160_99: Self = Self(13);
    pub const _4_3: Self = Self(14);
    pub const _3_2: Self = Self(15);
    pub const _2_1: Self = Self(16);
    pub const EXTENDED_SAR: Self = Self(255);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH265ChromaFormatIdc(pub i32);

impl StdVideoH265ChromaFormatIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MONOCHROME: Self = Self(0);
    pub const _420: Self = Self(1);
    pub const _422: Self = Self(2);
    pub const _444: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH265LevelIdc(pub i32);

impl StdVideoH265LevelIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _1_0: Self = Self(0);
    pub const _2_0: Self = Self(1);
    pub const _2_1: Self = Self(2);
    pub const _3_0: Self = Self(3);
    pub const _3_1: Self = Self(4);
    pub const _4_0: Self = Self(5);
    pub const _4_1: Self = Self(6);
    pub const _5_0: Self = Self(7);
    pub const _5_1: Self = Self(8);
    pub const _5_2: Self = Self(9);
    pub const _6_0: Self = Self(10);
    pub const _6_1: Self = Self(11);
    pub const _6_2: Self = Self(12);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH265PictureType(pub i32);

impl StdVideoH265PictureType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const IDR: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH265ProfileIdc(pub i32);

impl StdVideoH265ProfileIdc {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MAIN: Self = Self(1);
    pub const MAIN_10: Self = Self(2);
    pub const MAIN_STILL_PICTURE: Self = Self(3);
    pub const FORMAT_RANGE_EXTENSIONS: Self = Self(4);
    pub const SCC_EXTENSIONS: Self = Self(9);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoH265SliceType(pub i32);

impl StdVideoH265SliceType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const B: Self = Self(0);
    pub const P: Self = Self(1);
    pub const I: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoVP9ColorSpace(pub i32);

impl StdVideoVP9ColorSpace {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNKNOWN: Self = Self(0);
    pub const BT_601: Self = Self(1);
    pub const BT_709: Self = Self(2);
    pub const SMPTE_170: Self = Self(3);
    pub const SMPTE_240: Self = Self(4);
    pub const BT_2020: Self = Self(5);
    pub const RESERVED: Self = Self(6);
    pub const RGB: Self = Self(7);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoVP9FrameType(pub i32);

impl StdVideoVP9FrameType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const KEY: Self = Self(0);
    pub const NON_KEY: Self = Self(1);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoVP9InterpolationFilter(pub i32);

impl StdVideoVP9InterpolationFilter {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const EIGHTTAP: Self = Self(0);
    pub const EIGHTTAP_SMOOTH: Self = Self(1);
    pub const EIGHTTAP_SHARP: Self = Self(2);
    pub const BILINEAR: Self = Self(3);
    pub const SWITCHABLE: Self = Self(4);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoVP9Level(pub i32);

impl StdVideoVP9Level {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _1_0: Self = Self(0);
    pub const _1_1: Self = Self(1);
    pub const _2_0: Self = Self(2);
    pub const _2_1: Self = Self(3);
    pub const _3_0: Self = Self(4);
    pub const _3_1: Self = Self(5);
    pub const _4_0: Self = Self(6);
    pub const _4_1: Self = Self(7);
    pub const _5_0: Self = Self(8);
    pub const _5_1: Self = Self(9);
    pub const _5_2: Self = Self(10);
    pub const _6_0: Self = Self(11);
    pub const _6_1: Self = Self(12);
    pub const _6_2: Self = Self(13);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoVP9Profile(pub i32);

impl StdVideoVP9Profile {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _0: Self = Self(0);
    pub const _1: Self = Self(1);
    pub const _2: Self = Self(2);
    pub const _3: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StdVideoVP9ReferenceName(pub i32);

impl StdVideoVP9ReferenceName {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INTRA_FRAME: Self = Self(0);
    pub const LAST_FRAME: Self = Self(1);
    pub const GOLDEN_FRAME: Self = Self(2);
    pub const ALTREF_FRAME: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccelerationStructureBuildTypeKHR(pub i32);

impl AccelerationStructureBuildTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR: Self = Self(0);
    pub const ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR: Self = Self(1);
    pub const ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccelerationStructureCompatibilityKHR(pub i32);

impl AccelerationStructureCompatibilityKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR: Self = Self(0);
    pub const ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccelerationStructureMemoryRequirementsTypeNV(pub i32);

impl AccelerationStructureMemoryRequirementsTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV: Self = Self(0);
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV: Self = Self(1);
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccelerationStructureMotionInstanceTypeNV(pub i32);

impl AccelerationStructureMotionInstanceTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV: Self = Self(0);
    pub const ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV: Self = Self(1);
    pub const ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccelerationStructureTypeKHR(pub i32);

impl AccelerationStructureTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR: Self = Self(0);
    pub const ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR: Self = Self(1);
    pub const ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR: Self = Self(2);
}

pub type AccelerationStructureTypeNV = AccelerationStructureTypeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AntiLagModeAMD(pub i32);

impl AntiLagModeAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ANTI_LAG_MODE_DRIVER_CONTROL_AMD: Self = Self(0);
    pub const ANTI_LAG_MODE_ON_AMD: Self = Self(1);
    pub const ANTI_LAG_MODE_OFF_AMD: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AntiLagStageAMD(pub i32);

impl AntiLagStageAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ANTI_LAG_STAGE_INPUT_AMD: Self = Self(0);
    pub const ANTI_LAG_STAGE_PRESENT_AMD: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AttachmentLoadOp(pub i32);

impl AttachmentLoadOp {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const LOAD: Self = Self(0);
    pub const CLEAR: Self = Self(1);
    pub const DONT_CARE: Self = Self(2);
    pub const NONE: Self = Self(1000400000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AttachmentStoreOp(pub i32);

impl AttachmentStoreOp {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const STORE: Self = Self(0);
    pub const DONT_CARE: Self = Self(1);
    pub const NONE: Self = Self(1000301000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BlendFactor(pub i32);

impl BlendFactor {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SRC_COLOR: Self = Self(2);
    pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
    pub const DST_COLOR: Self = Self(4);
    pub const ONE_MINUS_DST_COLOR: Self = Self(5);
    pub const SRC_ALPHA: Self = Self(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
    pub const DST_ALPHA: Self = Self(8);
    pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
    pub const CONSTANT_COLOR: Self = Self(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    pub const CONSTANT_ALPHA: Self = Self(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
    pub const SRC_ALPHA_SATURATE: Self = Self(14);
    pub const SRC1_COLOR: Self = Self(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
    pub const SRC1_ALPHA: Self = Self(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BlendOp(pub i32);

impl BlendOp {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const REVERSE_SUBTRACT: Self = Self(2);
    pub const MIN: Self = Self(3);
    pub const MAX: Self = Self(4);
    pub const ZERO_EXT: Self = Self(1000148000);
    pub const SRC_EXT: Self = Self(1000148001);
    pub const DST_EXT: Self = Self(1000148002);
    pub const SRC_OVER_EXT: Self = Self(1000148003);
    pub const DST_OVER_EXT: Self = Self(1000148004);
    pub const SRC_IN_EXT: Self = Self(1000148005);
    pub const DST_IN_EXT: Self = Self(1000148006);
    pub const SRC_OUT_EXT: Self = Self(1000148007);
    pub const DST_OUT_EXT: Self = Self(1000148008);
    pub const SRC_ATOP_EXT: Self = Self(1000148009);
    pub const DST_ATOP_EXT: Self = Self(1000148010);
    pub const XOR_EXT: Self = Self(1000148011);
    pub const MULTIPLY_EXT: Self = Self(1000148012);
    pub const SCREEN_EXT: Self = Self(1000148013);
    pub const OVERLAY_EXT: Self = Self(1000148014);
    pub const DARKEN_EXT: Self = Self(1000148015);
    pub const LIGHTEN_EXT: Self = Self(1000148016);
    pub const COLORDODGE_EXT: Self = Self(1000148017);
    pub const COLORBURN_EXT: Self = Self(1000148018);
    pub const HARDLIGHT_EXT: Self = Self(1000148019);
    pub const SOFTLIGHT_EXT: Self = Self(1000148020);
    pub const DIFFERENCE_EXT: Self = Self(1000148021);
    pub const EXCLUSION_EXT: Self = Self(1000148022);
    pub const INVERT_EXT: Self = Self(1000148023);
    pub const INVERT_RGB_EXT: Self = Self(1000148024);
    pub const LINEARDODGE_EXT: Self = Self(1000148025);
    pub const LINEARBURN_EXT: Self = Self(1000148026);
    pub const VIVIDLIGHT_EXT: Self = Self(1000148027);
    pub const LINEARLIGHT_EXT: Self = Self(1000148028);
    pub const PINLIGHT_EXT: Self = Self(1000148029);
    pub const HARDMIX_EXT: Self = Self(1000148030);
    pub const HSL_HUE_EXT: Self = Self(1000148031);
    pub const HSL_SATURATION_EXT: Self = Self(1000148032);
    pub const HSL_COLOR_EXT: Self = Self(1000148033);
    pub const HSL_LUMINOSITY_EXT: Self = Self(1000148034);
    pub const PLUS_EXT: Self = Self(1000148035);
    pub const PLUS_CLAMPED_EXT: Self = Self(1000148036);
    pub const PLUS_CLAMPED_ALPHA_EXT: Self = Self(1000148037);
    pub const PLUS_DARKER_EXT: Self = Self(1000148038);
    pub const MINUS_EXT: Self = Self(1000148039);
    pub const MINUS_CLAMPED_EXT: Self = Self(1000148040);
    pub const CONTRAST_EXT: Self = Self(1000148041);
    pub const INVERT_OVG_EXT: Self = Self(1000148042);
    pub const RED_EXT: Self = Self(1000148043);
    pub const GREEN_EXT: Self = Self(1000148044);
    pub const BLUE_EXT: Self = Self(1000148045);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BlendOverlapEXT(pub i32);

impl BlendOverlapEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BLEND_OVERLAP_UNCORRELATED_EXT: Self = Self(0);
    pub const BLEND_OVERLAP_DISJOINT_EXT: Self = Self(1);
    pub const BLEND_OVERLAP_CONJOINT_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BlockMatchWindowCompareModeQCOM(pub i32);

impl BlockMatchWindowCompareModeQCOM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BLOCK_MATCH_WINDOW_COMPARE_MODE_MIN_QCOM: Self = Self(0);
    pub const BLOCK_MATCH_WINDOW_COMPARE_MODE_MAX_QCOM: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BorderColor(pub i32);

impl BorderColor {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
    pub const INT_TRANSPARENT_BLACK: Self = Self(1);
    pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
    pub const INT_OPAQUE_BLACK: Self = Self(3);
    pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
    pub const INT_OPAQUE_WHITE: Self = Self(5);
    pub const FLOAT_CUSTOM_EXT: Self = Self(1000287003);
    pub const INT_CUSTOM_EXT: Self = Self(1000287004);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BuildAccelerationStructureModeKHR(pub i32);

impl BuildAccelerationStructureModeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR: Self = Self(0);
    pub const BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BuildMicromapModeEXT(pub i32);

impl BuildMicromapModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BUILD_MICROMAP_MODE_BUILD_EXT: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ChromaLocation(pub i32);

impl ChromaLocation {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COSITED_EVEN: Self = Self(0);
    pub const MIDPOINT: Self = Self(1);
}

pub type ChromaLocationKHR = ChromaLocation;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureOpModeNV(pub i32);

impl ClusterAccelerationStructureOpModeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_IMPLICIT_DESTINATIONS_NV: Self = Self(0);
    pub const CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_EXPLICIT_DESTINATIONS_NV: Self = Self(1);
    pub const CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_COMPUTE_SIZES_NV: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureOpTypeNV(pub i32);

impl ClusterAccelerationStructureOpTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_MOVE_OBJECTS_NV: Self = Self(0);
    pub const CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(1);
    pub const CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_NV: Self = Self(2);
    pub const CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV: Self =
        Self(3);
    pub const CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_INSTANTIATE_TRIANGLE_CLUSTER_NV: Self =
        Self(4);
    pub const CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_GET_CLUSTER_TEMPLATE_INDICES_NV: Self =
        Self(5);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureTypeNV(pub i32);

impl ClusterAccelerationStructureTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CLUSTER_ACCELERATION_STRUCTURE_TYPE_CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(0);
    pub const CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_NV: Self = Self(1);
    pub const CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CoarseSampleOrderTypeNV(pub i32);

impl CoarseSampleOrderTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV: Self = Self(0);
    pub const COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV: Self = Self(1);
    pub const COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV: Self = Self(2);
    pub const COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ColorSpaceKHR(pub i32);

impl ColorSpaceKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COLOR_SPACE_SRGB_NONLINEAR_KHR: Self = Self(0);
    pub const COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    pub const COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    pub const COLOR_SPACE_DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    pub const COLOR_SPACE_DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    pub const COLOR_SPACE_BT709_LINEAR_EXT: Self = Self(1000104005);
    pub const COLOR_SPACE_BT709_NONLINEAR_EXT: Self = Self(1000104006);
    pub const COLOR_SPACE_BT2020_LINEAR_EXT: Self = Self(1000104007);
    pub const COLOR_SPACE_HDR10_ST2084_EXT: Self = Self(1000104008);
    pub const COLOR_SPACE_DOLBYVISION_EXT: Self = Self(1000104009);
    pub const COLOR_SPACE_HDR10_HLG_EXT: Self = Self(1000104010);
    pub const COLOR_SPACE_ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    pub const COLOR_SPACE_ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    pub const COLOR_SPACE_PASS_THROUGH_EXT: Self = Self(1000104013);
    pub const COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    pub const COLOR_SPACE_DISPLAY_NATIVE_AMD: Self = Self(1000213000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandBufferLevel(pub i32);

impl CommandBufferLevel {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CompareOp(pub i32);

impl CompareOp {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LESS_OR_EQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GREATER_OR_EQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ComponentSwizzle(pub i32);

impl ComponentSwizzle {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const IDENTITY: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const ONE: Self = Self(2);
    pub const R: Self = Self(3);
    pub const G: Self = Self(4);
    pub const B: Self = Self(5);
    pub const A: Self = Self(6);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ComponentTypeKHR(pub i32);

impl ComponentTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COMPONENT_TYPE_FLOAT16_KHR: Self = Self(0);
    pub const COMPONENT_TYPE_FLOAT32_KHR: Self = Self(1);
    pub const COMPONENT_TYPE_FLOAT64_KHR: Self = Self(2);
    pub const COMPONENT_TYPE_SINT8_KHR: Self = Self(3);
    pub const COMPONENT_TYPE_SINT16_KHR: Self = Self(4);
    pub const COMPONENT_TYPE_SINT32_KHR: Self = Self(5);
    pub const COMPONENT_TYPE_SINT64_KHR: Self = Self(6);
    pub const COMPONENT_TYPE_UINT8_KHR: Self = Self(7);
    pub const COMPONENT_TYPE_UINT16_KHR: Self = Self(8);
    pub const COMPONENT_TYPE_UINT32_KHR: Self = Self(9);
    pub const COMPONENT_TYPE_UINT64_KHR: Self = Self(10);
    pub const COMPONENT_TYPE_BFLOAT16_KHR: Self = Self(1000141000);
    pub const COMPONENT_TYPE_SINT8_PACKED_NV: Self = Self(1000491000);
    pub const COMPONENT_TYPE_UINT8_PACKED_NV: Self = Self(1000491001);
    pub const COMPONENT_TYPE_FLOAT8_E4M3_EXT: Self = Self(1000491002);
    pub const COMPONENT_TYPE_FLOAT8_E5M2_EXT: Self = Self(1000491003);
}

pub type ComponentTypeNV = ComponentTypeKHR;

#[cfg(feature = "beta")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CompressedTriangleFormatAMDX(pub i32);

#[cfg(feature = "beta")]
impl CompressedTriangleFormatAMDX {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_AMDX: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ConservativeRasterizationModeEXT(pub i32);

impl ConservativeRasterizationModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT: Self = Self(0);
    pub const CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT: Self = Self(1);
    pub const CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CooperativeVectorMatrixLayoutNV(pub i32);

impl CooperativeVectorMatrixLayoutNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COOPERATIVE_VECTOR_MATRIX_LAYOUT_ROW_MAJOR_NV: Self = Self(0);
    pub const COOPERATIVE_VECTOR_MATRIX_LAYOUT_COLUMN_MAJOR_NV: Self = Self(1);
    pub const COOPERATIVE_VECTOR_MATRIX_LAYOUT_INFERENCING_OPTIMAL_NV: Self = Self(2);
    pub const COOPERATIVE_VECTOR_MATRIX_LAYOUT_TRAINING_OPTIMAL_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CopyAccelerationStructureModeKHR(pub i32);

impl CopyAccelerationStructureModeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR: Self = Self(0);
    pub const COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR: Self = Self(1);
    pub const COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR: Self = Self(2);
    pub const COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR: Self = Self(3);
}

pub type CopyAccelerationStructureModeNV = CopyAccelerationStructureModeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CopyMicromapModeEXT(pub i32);

impl CopyMicromapModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COPY_MICROMAP_MODE_CLONE_EXT: Self = Self(0);
    pub const COPY_MICROMAP_MODE_SERIALIZE_EXT: Self = Self(1);
    pub const COPY_MICROMAP_MODE_DESERIALIZE_EXT: Self = Self(2);
    pub const COPY_MICROMAP_MODE_COMPACT_EXT: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CoverageModulationModeNV(pub i32);

impl CoverageModulationModeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COVERAGE_MODULATION_MODE_NONE_NV: Self = Self(0);
    pub const COVERAGE_MODULATION_MODE_RGB_NV: Self = Self(1);
    pub const COVERAGE_MODULATION_MODE_ALPHA_NV: Self = Self(2);
    pub const COVERAGE_MODULATION_MODE_RGBA_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CoverageReductionModeNV(pub i32);

impl CoverageReductionModeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COVERAGE_REDUCTION_MODE_MERGE_NV: Self = Self(0);
    pub const COVERAGE_REDUCTION_MODE_TRUNCATE_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CubicFilterWeightsQCOM(pub i32);

impl CubicFilterWeightsQCOM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CUBIC_FILTER_WEIGHTS_CATMULL_ROM_QCOM: Self = Self(0);
    pub const CUBIC_FILTER_WEIGHTS_ZERO_TANGENT_CARDINAL_QCOM: Self = Self(1);
    pub const CUBIC_FILTER_WEIGHTS_B_SPLINE_QCOM: Self = Self(2);
    pub const CUBIC_FILTER_WEIGHTS_MITCHELL_NETRAVALI_QCOM: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphModelCacheTypeQCOM(pub i32);

impl DataGraphModelCacheTypeQCOM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DATA_GRAPH_MODEL_CACHE_TYPE_GENERIC_BINARY_QCOM: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphOpticalFlowPerformanceLevelARM(pub i32);

impl DataGraphOpticalFlowPerformanceLevelARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_ARM: Self = Self(0);
    pub const DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_ARM: Self = Self(1);
    pub const DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_ARM: Self = Self(2);
    pub const DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_ARM: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphPipelineNodeConnectionTypeARM(pub i32);

impl DataGraphPipelineNodeConnectionTypeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DATA_GRAPH_PIPELINE_NODE_CONNECTION_TYPE_OPTICAL_FLOW_INPUT_ARM: Self =
        Self(1000631000);
    pub const DATA_GRAPH_PIPELINE_NODE_CONNECTION_TYPE_OPTICAL_FLOW_REFERENCE_ARM: Self =
        Self(1000631001);
    pub const DATA_GRAPH_PIPELINE_NODE_CONNECTION_TYPE_OPTICAL_FLOW_HINT_ARM: Self =
        Self(1000631002);
    pub const DATA_GRAPH_PIPELINE_NODE_CONNECTION_TYPE_OPTICAL_FLOW_FLOW_VECTOR_ARM: Self =
        Self(1000631003);
    pub const DATA_GRAPH_PIPELINE_NODE_CONNECTION_TYPE_OPTICAL_FLOW_COST_ARM: Self =
        Self(1000631004);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphPipelineNodeTypeARM(pub i32);

impl DataGraphPipelineNodeTypeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DATA_GRAPH_PIPELINE_NODE_TYPE_OPTICAL_FLOW_ARM: Self = Self(1000631000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphPipelinePropertyARM(pub i32);

impl DataGraphPipelinePropertyARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DATA_GRAPH_PIPELINE_PROPERTY_CREATION_LOG_ARM: Self = Self(0);
    pub const DATA_GRAPH_PIPELINE_PROPERTY_IDENTIFIER_ARM: Self = Self(1);
    pub const DATA_GRAPH_PIPELINE_PROPERTY_NEURAL_ACCELERATOR_DEBUG_DATABASE_ARM: Self =
        Self(1000676000);
    pub const DATA_GRAPH_PIPELINE_PROPERTY_NEURAL_ACCELERATOR_STATISTICS_INFO_ARM: Self =
        Self(1000676001);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphPipelineSessionBindPointARM(pub i32);

impl DataGraphPipelineSessionBindPointARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TRANSIENT_ARM: Self = Self(0);
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_OPTICAL_FLOW_CACHE_ARM: Self =
        Self(1000631001);
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_NEURAL_ACCELERATOR_STATISTICS_ARM: Self =
        Self(1000676000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphPipelineSessionBindPointTypeARM(pub i32);

impl DataGraphPipelineSessionBindPointTypeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TYPE_MEMORY_ARM: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphTOSALevelARM(pub i32);

impl DataGraphTOSALevelARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DATA_GRAPH_TOSA_LEVEL_NONE_ARM: Self = Self(0);
    pub const DATA_GRAPH_TOSA_LEVEL_8K_ARM: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DebugReportObjectTypeEXT(pub i32);

impl DebugReportObjectTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: Self = Self(0);
    pub const DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: Self = Self(1);
    pub const DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: Self = Self(2);
    pub const DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: Self = Self(3);
    pub const DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: Self = Self(4);
    pub const DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: Self = Self(5);
    pub const DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: Self = Self(6);
    pub const DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: Self = Self(7);
    pub const DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: Self = Self(8);
    pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: Self = Self(9);
    pub const DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: Self = Self(10);
    pub const DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: Self = Self(11);
    pub const DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: Self = Self(12);
    pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: Self = Self(13);
    pub const DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: Self = Self(14);
    pub const DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: Self = Self(15);
    pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: Self = Self(16);
    pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: Self = Self(17);
    pub const DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: Self = Self(18);
    pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: Self = Self(19);
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT: Self = Self(20);
    pub const DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: Self = Self(21);
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: Self = Self(22);
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: Self = Self(23);
    pub const DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: Self = Self(24);
    pub const DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: Self = Self(25);
    pub const DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: Self = Self(26);
    pub const DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: Self = Self(27);
    pub const DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: Self = Self(28);
    pub const DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: Self = Self(29);
    pub const DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: Self = Self(30);
    pub const DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT: Self = Self(33);
    pub const DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT: Self = Self(1000029000);
    pub const DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT: Self = Self(1000029001);
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT: Self = Self(1000085000);
    pub const DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT: Self = Self(1000150000);
    pub const DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT: Self = Self(1000156000);
    pub const DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT: Self = Self(1000165000);
    pub const DEBUG_REPORT_OBJECT_TYPE_CUDA_MODULE_NV_EXT: Self = Self(1000307000);
    pub const DEBUG_REPORT_OBJECT_TYPE_CUDA_FUNCTION_NV_EXT: Self = Self(1000307001);
    pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT: Self = Self(1000366000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DefaultVertexAttributeValueKHR(pub i32);

impl DefaultVertexAttributeValueKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ZERO_KHR: Self = Self(0);
    pub const DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ONE_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DepthBiasRepresentationEXT(pub i32);

impl DepthBiasRepresentationEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORMAT_EXT: Self = Self(0);
    pub const DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT: Self = Self(1);
    pub const DEPTH_BIAS_REPRESENTATION_FLOAT_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DepthClampModeEXT(pub i32);

impl DepthClampModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEPTH_CLAMP_MODE_VIEWPORT_RANGE_EXT: Self = Self(0);
    pub const DEPTH_CLAMP_MODE_USER_DEFINED_RANGE_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorMappingSourceEXT(pub i32);

impl DescriptorMappingSourceEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_CONSTANT_OFFSET_EXT: Self = Self(0);
    pub const DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_PUSH_INDEX_EXT: Self = Self(1);
    pub const DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_EXT: Self = Self(2);
    pub const DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT: Self = Self(3);
    pub const DESCRIPTOR_MAPPING_SOURCE_RESOURCE_HEAP_DATA_EXT: Self = Self(4);
    pub const DESCRIPTOR_MAPPING_SOURCE_PUSH_DATA_EXT: Self = Self(5);
    pub const DESCRIPTOR_MAPPING_SOURCE_PUSH_ADDRESS_EXT: Self = Self(6);
    pub const DESCRIPTOR_MAPPING_SOURCE_INDIRECT_ADDRESS_EXT: Self = Self(7);
    pub const DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_SHADER_RECORD_INDEX_EXT: Self = Self(8);
    pub const DESCRIPTOR_MAPPING_SOURCE_SHADER_RECORD_DATA_EXT: Self = Self(9);
    pub const DESCRIPTOR_MAPPING_SOURCE_SHADER_RECORD_ADDRESS_EXT: Self = Self(10);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorType(pub i32);

impl DescriptorType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SAMPLER: Self = Self(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const SAMPLED_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE: Self = Self(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const UNIFORM_BUFFER: Self = Self(6);
    pub const STORAGE_BUFFER: Self = Self(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const INPUT_ATTACHMENT: Self = Self(10);
    pub const INLINE_UNIFORM_BLOCK: Self = Self(1000138000);
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    pub const MUTABLE_EXT: Self = Self(1000351000);
    pub const SAMPLE_WEIGHT_IMAGE_QCOM: Self = Self(1000440000);
    pub const BLOCK_MATCH_IMAGE_QCOM: Self = Self(1000440001);
    pub const TENSOR_ARM: Self = Self(1000460000);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorUpdateTemplateType(pub i32);

impl DescriptorUpdateTemplateType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DESCRIPTOR_SET: Self = Self(0);
    pub const PUSH_DESCRIPTORS: Self = Self(1);
}

pub type DescriptorUpdateTemplateTypeKHR = DescriptorUpdateTemplateType;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceAddressBindingTypeEXT(pub i32);

impl DeviceAddressBindingTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE_ADDRESS_BINDING_TYPE_BIND_EXT: Self = Self(0);
    pub const DEVICE_ADDRESS_BINDING_TYPE_UNBIND_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceEventTypeEXT(pub i32);

impl DeviceEventTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceFaultAddressTypeKHR(pub i32);

impl DeviceFaultAddressTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE_FAULT_ADDRESS_TYPE_NONE_KHR: Self = Self(0);
    pub const DEVICE_FAULT_ADDRESS_TYPE_READ_INVALID_KHR: Self = Self(1);
    pub const DEVICE_FAULT_ADDRESS_TYPE_WRITE_INVALID_KHR: Self = Self(2);
    pub const DEVICE_FAULT_ADDRESS_TYPE_EXECUTE_INVALID_KHR: Self = Self(3);
    pub const DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_UNKNOWN_KHR: Self = Self(4);
    pub const DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_INVALID_KHR: Self = Self(5);
    pub const DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_FAULT_KHR: Self = Self(6);
}

pub type DeviceFaultAddressTypeEXT = DeviceFaultAddressTypeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceFaultVendorBinaryHeaderVersionKHR(pub i32);

impl DeviceFaultVendorBinaryHeaderVersionKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_KHR: Self = Self(1);
}

pub type DeviceFaultVendorBinaryHeaderVersionEXT = DeviceFaultVendorBinaryHeaderVersionKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceMemoryReportEventTypeEXT(pub i32);

impl DeviceMemoryReportEventTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT: Self = Self(0);
    pub const DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT: Self = Self(1);
    pub const DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT: Self = Self(2);
    pub const DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT: Self = Self(3);
    pub const DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DirectDriverLoadingModeLUNARG(pub i32);

impl DirectDriverLoadingModeLUNARG {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG: Self = Self(0);
    pub const DIRECT_DRIVER_LOADING_MODE_INCLUSIVE_LUNARG: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DiscardRectangleModeEXT(pub i32);

impl DiscardRectangleModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: Self = Self(0);
    pub const DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: Self = Self(1);
}

#[cfg(feature = "beta")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DisplacementMicromapFormatNV(pub i32);

#[cfg(feature = "beta")]
impl DisplacementMicromapFormatNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DISPLACEMENT_MICROMAP_FORMAT_64_TRIANGLES_64_BYTES_NV: Self = Self(1);
    pub const DISPLACEMENT_MICROMAP_FORMAT_256_TRIANGLES_128_BYTES_NV: Self = Self(2);
    pub const DISPLACEMENT_MICROMAP_FORMAT_1024_TRIANGLES_128_BYTES_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DisplayEventTypeEXT(pub i32);

impl DisplayEventTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DisplayPowerStateEXT(pub i32);

impl DisplayPowerStateEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DISPLAY_POWER_STATE_OFF_EXT: Self = Self(0);
    pub const DISPLAY_POWER_STATE_SUSPEND_EXT: Self = Self(1);
    pub const DISPLAY_POWER_STATE_ON_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DisplaySurfaceStereoTypeNV(pub i32);

impl DisplaySurfaceStereoTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DISPLAY_SURFACE_STEREO_TYPE_NONE_NV: Self = Self(0);
    pub const DISPLAY_SURFACE_STEREO_TYPE_ONBOARD_DIN_NV: Self = Self(1);
    pub const DISPLAY_SURFACE_STEREO_TYPE_HDMI_3D_NV: Self = Self(2);
    pub const DISPLAY_SURFACE_STEREO_TYPE_INBAND_DISPLAYPORT_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DriverId(pub i32);

impl DriverId {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const AMD_PROPRIETARY: Self = Self(1);
    pub const AMD_OPEN_SOURCE: Self = Self(2);
    pub const MESA_RADV: Self = Self(3);
    pub const NVIDIA_PROPRIETARY: Self = Self(4);
    pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    pub const INTEL_OPEN_SOURCE_MESA: Self = Self(6);
    pub const IMAGINATION_PROPRIETARY: Self = Self(7);
    pub const QUALCOMM_PROPRIETARY: Self = Self(8);
    pub const ARM_PROPRIETARY: Self = Self(9);
    pub const GOOGLE_SWIFTSHADER: Self = Self(10);
    pub const GGP_PROPRIETARY: Self = Self(11);
    pub const BROADCOM_PROPRIETARY: Self = Self(12);
    pub const MESA_LLVMPIPE: Self = Self(13);
    pub const MOLTENVK: Self = Self(14);
    pub const COREAVI_PROPRIETARY: Self = Self(15);
    pub const JUICE_PROPRIETARY: Self = Self(16);
    pub const VERISILICON_PROPRIETARY: Self = Self(17);
    pub const MESA_TURNIP: Self = Self(18);
    pub const MESA_V3DV: Self = Self(19);
    pub const MESA_PANVK: Self = Self(20);
    pub const SAMSUNG_PROPRIETARY: Self = Self(21);
    pub const MESA_VENUS: Self = Self(22);
    pub const MESA_DOZEN: Self = Self(23);
    pub const MESA_NVK: Self = Self(24);
    pub const IMAGINATION_OPEN_SOURCE_MESA: Self = Self(25);
    pub const MESA_HONEYKRISP: Self = Self(26);
    pub const VULKAN_SC_EMULATION_ON_VULKAN: Self = Self(27);
    pub const MESA_KOSMICKRISP: Self = Self(28);
}

pub type DriverIdKHR = DriverId;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DynamicState(pub i32);

impl DynamicState {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VIEWPORT: Self = Self(0);
    pub const SCISSOR: Self = Self(1);
    pub const LINE_WIDTH: Self = Self(2);
    pub const DEPTH_BIAS: Self = Self(3);
    pub const BLEND_CONSTANTS: Self = Self(4);
    pub const DEPTH_BOUNDS: Self = Self(5);
    pub const STENCIL_COMPARE_MASK: Self = Self(6);
    pub const STENCIL_WRITE_MASK: Self = Self(7);
    pub const STENCIL_REFERENCE: Self = Self(8);
    pub const VIEWPORT_W_SCALING_NV: Self = Self(1000087000);
    pub const DISCARD_RECTANGLE_EXT: Self = Self(1000099000);
    pub const DISCARD_RECTANGLE_ENABLE_EXT: Self = Self(1000099001);
    pub const DISCARD_RECTANGLE_MODE_EXT: Self = Self(1000099002);
    pub const SAMPLE_LOCATIONS_EXT: Self = Self(1000143000);
    pub const VIEWPORT_SHADING_RATE_PALETTE_NV: Self = Self(1000164004);
    pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV: Self = Self(1000164006);
    pub const EXCLUSIVE_SCISSOR_ENABLE_NV: Self = Self(1000205000);
    pub const EXCLUSIVE_SCISSOR_NV: Self = Self(1000205001);
    pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226000);
    pub const LINE_STIPPLE: Self = Self(1000259000);
    pub const CULL_MODE: Self = Self(1000267000);
    pub const FRONT_FACE: Self = Self(1000267001);
    pub const PRIMITIVE_TOPOLOGY: Self = Self(1000267002);
    pub const VIEWPORT_WITH_COUNT: Self = Self(1000267003);
    pub const SCISSOR_WITH_COUNT: Self = Self(1000267004);
    pub const VERTEX_INPUT_BINDING_STRIDE: Self = Self(1000267005);
    pub const DEPTH_TEST_ENABLE: Self = Self(1000267006);
    pub const DEPTH_WRITE_ENABLE: Self = Self(1000267007);
    pub const DEPTH_COMPARE_OP: Self = Self(1000267008);
    pub const DEPTH_BOUNDS_TEST_ENABLE: Self = Self(1000267009);
    pub const STENCIL_TEST_ENABLE: Self = Self(1000267010);
    pub const STENCIL_OP: Self = Self(1000267011);
    pub const RAY_TRACING_PIPELINE_STACK_SIZE_KHR: Self = Self(1000347000);
    pub const VERTEX_INPUT_EXT: Self = Self(1000352000);
    pub const PATCH_CONTROL_POINTS_EXT: Self = Self(1000377000);
    pub const RASTERIZER_DISCARD_ENABLE: Self = Self(1000377001);
    pub const DEPTH_BIAS_ENABLE: Self = Self(1000377002);
    pub const LOGIC_OP_EXT: Self = Self(1000377003);
    pub const PRIMITIVE_RESTART_ENABLE: Self = Self(1000377004);
    pub const COLOR_WRITE_ENABLE_EXT: Self = Self(1000381000);
    pub const TESSELLATION_DOMAIN_ORIGIN_EXT: Self = Self(1000455002);
    pub const DEPTH_CLAMP_ENABLE_EXT: Self = Self(1000455003);
    pub const POLYGON_MODE_EXT: Self = Self(1000455004);
    pub const RASTERIZATION_SAMPLES_EXT: Self = Self(1000455005);
    pub const SAMPLE_MASK_EXT: Self = Self(1000455006);
    pub const ALPHA_TO_COVERAGE_ENABLE_EXT: Self = Self(1000455007);
    pub const ALPHA_TO_ONE_ENABLE_EXT: Self = Self(1000455008);
    pub const LOGIC_OP_ENABLE_EXT: Self = Self(1000455009);
    pub const COLOR_BLEND_ENABLE_EXT: Self = Self(1000455010);
    pub const COLOR_BLEND_EQUATION_EXT: Self = Self(1000455011);
    pub const COLOR_WRITE_MASK_EXT: Self = Self(1000455012);
    pub const RASTERIZATION_STREAM_EXT: Self = Self(1000455013);
    pub const CONSERVATIVE_RASTERIZATION_MODE_EXT: Self = Self(1000455014);
    pub const EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT: Self = Self(1000455015);
    pub const DEPTH_CLIP_ENABLE_EXT: Self = Self(1000455016);
    pub const SAMPLE_LOCATIONS_ENABLE_EXT: Self = Self(1000455017);
    pub const COLOR_BLEND_ADVANCED_EXT: Self = Self(1000455018);
    pub const PROVOKING_VERTEX_MODE_EXT: Self = Self(1000455019);
    pub const LINE_RASTERIZATION_MODE_EXT: Self = Self(1000455020);
    pub const LINE_STIPPLE_ENABLE_EXT: Self = Self(1000455021);
    pub const DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT: Self = Self(1000455022);
    pub const VIEWPORT_W_SCALING_ENABLE_NV: Self = Self(1000455023);
    pub const VIEWPORT_SWIZZLE_NV: Self = Self(1000455024);
    pub const COVERAGE_TO_COLOR_ENABLE_NV: Self = Self(1000455025);
    pub const COVERAGE_TO_COLOR_LOCATION_NV: Self = Self(1000455026);
    pub const COVERAGE_MODULATION_MODE_NV: Self = Self(1000455027);
    pub const COVERAGE_MODULATION_TABLE_ENABLE_NV: Self = Self(1000455028);
    pub const COVERAGE_MODULATION_TABLE_NV: Self = Self(1000455029);
    pub const SHADING_RATE_IMAGE_ENABLE_NV: Self = Self(1000455030);
    pub const REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV: Self = Self(1000455031);
    pub const COVERAGE_REDUCTION_MODE_NV: Self = Self(1000455032);
    pub const ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT: Self = Self(1000524000);
    pub const DEPTH_CLAMP_RANGE_EXT: Self = Self(1000582000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Filter(pub i32);

impl Filter {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
    pub const CUBIC_EXT: Self = Self(1000015000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Format(pub i32);

impl Format {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNDEFINED: Self = Self(0);
    pub const R4G4_UNORM_PACK8: Self = Self(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
    pub const R5G6B5_UNORM_PACK16: Self = Self(4);
    pub const B5G6R5_UNORM_PACK16: Self = Self(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
    pub const R8_UNORM: Self = Self(9);
    pub const R8_SNORM: Self = Self(10);
    pub const R8_USCALED: Self = Self(11);
    pub const R8_SSCALED: Self = Self(12);
    pub const R8_UINT: Self = Self(13);
    pub const R8_SINT: Self = Self(14);
    pub const R8_SRGB: Self = Self(15);
    pub const R8G8_UNORM: Self = Self(16);
    pub const R8G8_SNORM: Self = Self(17);
    pub const R8G8_USCALED: Self = Self(18);
    pub const R8G8_SSCALED: Self = Self(19);
    pub const R8G8_UINT: Self = Self(20);
    pub const R8G8_SINT: Self = Self(21);
    pub const R8G8_SRGB: Self = Self(22);
    pub const R8G8B8_UNORM: Self = Self(23);
    pub const R8G8B8_SNORM: Self = Self(24);
    pub const R8G8B8_USCALED: Self = Self(25);
    pub const R8G8B8_SSCALED: Self = Self(26);
    pub const R8G8B8_UINT: Self = Self(27);
    pub const R8G8B8_SINT: Self = Self(28);
    pub const R8G8B8_SRGB: Self = Self(29);
    pub const B8G8R8_UNORM: Self = Self(30);
    pub const B8G8R8_SNORM: Self = Self(31);
    pub const B8G8R8_USCALED: Self = Self(32);
    pub const B8G8R8_SSCALED: Self = Self(33);
    pub const B8G8R8_UINT: Self = Self(34);
    pub const B8G8R8_SINT: Self = Self(35);
    pub const B8G8R8_SRGB: Self = Self(36);
    pub const R8G8B8A8_UNORM: Self = Self(37);
    pub const R8G8B8A8_SNORM: Self = Self(38);
    pub const R8G8B8A8_USCALED: Self = Self(39);
    pub const R8G8B8A8_SSCALED: Self = Self(40);
    pub const R8G8B8A8_UINT: Self = Self(41);
    pub const R8G8B8A8_SINT: Self = Self(42);
    pub const R8G8B8A8_SRGB: Self = Self(43);
    pub const B8G8R8A8_UNORM: Self = Self(44);
    pub const B8G8R8A8_SNORM: Self = Self(45);
    pub const B8G8R8A8_USCALED: Self = Self(46);
    pub const B8G8R8A8_SSCALED: Self = Self(47);
    pub const B8G8R8A8_UINT: Self = Self(48);
    pub const B8G8R8A8_SINT: Self = Self(49);
    pub const B8G8R8A8_SRGB: Self = Self(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
    pub const R16_UNORM: Self = Self(70);
    pub const R16_SNORM: Self = Self(71);
    pub const R16_USCALED: Self = Self(72);
    pub const R16_SSCALED: Self = Self(73);
    pub const R16_UINT: Self = Self(74);
    pub const R16_SINT: Self = Self(75);
    pub const R16_SFLOAT: Self = Self(76);
    pub const R16G16_UNORM: Self = Self(77);
    pub const R16G16_SNORM: Self = Self(78);
    pub const R16G16_USCALED: Self = Self(79);
    pub const R16G16_SSCALED: Self = Self(80);
    pub const R16G16_UINT: Self = Self(81);
    pub const R16G16_SINT: Self = Self(82);
    pub const R16G16_SFLOAT: Self = Self(83);
    pub const R16G16B16_UNORM: Self = Self(84);
    pub const R16G16B16_SNORM: Self = Self(85);
    pub const R16G16B16_USCALED: Self = Self(86);
    pub const R16G16B16_SSCALED: Self = Self(87);
    pub const R16G16B16_UINT: Self = Self(88);
    pub const R16G16B16_SINT: Self = Self(89);
    pub const R16G16B16_SFLOAT: Self = Self(90);
    pub const R16G16B16A16_UNORM: Self = Self(91);
    pub const R16G16B16A16_SNORM: Self = Self(92);
    pub const R16G16B16A16_USCALED: Self = Self(93);
    pub const R16G16B16A16_SSCALED: Self = Self(94);
    pub const R16G16B16A16_UINT: Self = Self(95);
    pub const R16G16B16A16_SINT: Self = Self(96);
    pub const R16G16B16A16_SFLOAT: Self = Self(97);
    pub const R32_UINT: Self = Self(98);
    pub const R32_SINT: Self = Self(99);
    pub const R32_SFLOAT: Self = Self(100);
    pub const R32G32_UINT: Self = Self(101);
    pub const R32G32_SINT: Self = Self(102);
    pub const R32G32_SFLOAT: Self = Self(103);
    pub const R32G32B32_UINT: Self = Self(104);
    pub const R32G32B32_SINT: Self = Self(105);
    pub const R32G32B32_SFLOAT: Self = Self(106);
    pub const R32G32B32A32_UINT: Self = Self(107);
    pub const R32G32B32A32_SINT: Self = Self(108);
    pub const R32G32B32A32_SFLOAT: Self = Self(109);
    pub const R64_UINT: Self = Self(110);
    pub const R64_SINT: Self = Self(111);
    pub const R64_SFLOAT: Self = Self(112);
    pub const R64G64_UINT: Self = Self(113);
    pub const R64G64_SINT: Self = Self(114);
    pub const R64G64_SFLOAT: Self = Self(115);
    pub const R64G64B64_UINT: Self = Self(116);
    pub const R64G64B64_SINT: Self = Self(117);
    pub const R64G64B64_SFLOAT: Self = Self(118);
    pub const R64G64B64A64_UINT: Self = Self(119);
    pub const R64G64B64A64_SINT: Self = Self(120);
    pub const R64G64B64A64_SFLOAT: Self = Self(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
    pub const D16_UNORM: Self = Self(124);
    pub const X8_D24_UNORM_PACK32: Self = Self(125);
    pub const D32_SFLOAT: Self = Self(126);
    pub const S8_UINT: Self = Self(127);
    pub const D16_UNORM_S8_UINT: Self = Self(128);
    pub const D24_UNORM_S8_UINT: Self = Self(129);
    pub const D32_SFLOAT_S8_UINT: Self = Self(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
    pub const BC2_UNORM_BLOCK: Self = Self(135);
    pub const BC2_SRGB_BLOCK: Self = Self(136);
    pub const BC3_UNORM_BLOCK: Self = Self(137);
    pub const BC3_SRGB_BLOCK: Self = Self(138);
    pub const BC4_UNORM_BLOCK: Self = Self(139);
    pub const BC4_SNORM_BLOCK: Self = Self(140);
    pub const BC5_UNORM_BLOCK: Self = Self(141);
    pub const BC5_SNORM_BLOCK: Self = Self(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
    pub const BC7_UNORM_BLOCK: Self = Self(145);
    pub const BC7_SRGB_BLOCK: Self = Self(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
    pub const ASTC_4x4_UNORM_BLOCK: Self = Self(157);
    pub const ASTC_4x4_SRGB_BLOCK: Self = Self(158);
    pub const ASTC_5x4_UNORM_BLOCK: Self = Self(159);
    pub const ASTC_5x4_SRGB_BLOCK: Self = Self(160);
    pub const ASTC_5x5_UNORM_BLOCK: Self = Self(161);
    pub const ASTC_5x5_SRGB_BLOCK: Self = Self(162);
    pub const ASTC_6x5_UNORM_BLOCK: Self = Self(163);
    pub const ASTC_6x5_SRGB_BLOCK: Self = Self(164);
    pub const ASTC_6x6_UNORM_BLOCK: Self = Self(165);
    pub const ASTC_6x6_SRGB_BLOCK: Self = Self(166);
    pub const ASTC_8x5_UNORM_BLOCK: Self = Self(167);
    pub const ASTC_8x5_SRGB_BLOCK: Self = Self(168);
    pub const ASTC_8x6_UNORM_BLOCK: Self = Self(169);
    pub const ASTC_8x6_SRGB_BLOCK: Self = Self(170);
    pub const ASTC_8x8_UNORM_BLOCK: Self = Self(171);
    pub const ASTC_8x8_SRGB_BLOCK: Self = Self(172);
    pub const ASTC_10x5_UNORM_BLOCK: Self = Self(173);
    pub const ASTC_10x5_SRGB_BLOCK: Self = Self(174);
    pub const ASTC_10x6_UNORM_BLOCK: Self = Self(175);
    pub const ASTC_10x6_SRGB_BLOCK: Self = Self(176);
    pub const ASTC_10x8_UNORM_BLOCK: Self = Self(177);
    pub const ASTC_10x8_SRGB_BLOCK: Self = Self(178);
    pub const ASTC_10x10_UNORM_BLOCK: Self = Self(179);
    pub const ASTC_10x10_SRGB_BLOCK: Self = Self(180);
    pub const ASTC_12x10_UNORM_BLOCK: Self = Self(181);
    pub const ASTC_12x10_SRGB_BLOCK: Self = Self(182);
    pub const ASTC_12x12_UNORM_BLOCK: Self = Self(183);
    pub const ASTC_12x12_SRGB_BLOCK: Self = Self(184);
    pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054000);
    pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054001);
    pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054002);
    pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054003);
    pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054004);
    pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054005);
    pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054006);
    pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054007);
    pub const ASTC_4x4_SFLOAT_BLOCK: Self = Self(1000066000);
    pub const ASTC_5x4_SFLOAT_BLOCK: Self = Self(1000066001);
    pub const ASTC_5x5_SFLOAT_BLOCK: Self = Self(1000066002);
    pub const ASTC_6x5_SFLOAT_BLOCK: Self = Self(1000066003);
    pub const ASTC_6x6_SFLOAT_BLOCK: Self = Self(1000066004);
    pub const ASTC_8x5_SFLOAT_BLOCK: Self = Self(1000066005);
    pub const ASTC_8x6_SFLOAT_BLOCK: Self = Self(1000066006);
    pub const ASTC_8x8_SFLOAT_BLOCK: Self = Self(1000066007);
    pub const ASTC_10x5_SFLOAT_BLOCK: Self = Self(1000066008);
    pub const ASTC_10x6_SFLOAT_BLOCK: Self = Self(1000066009);
    pub const ASTC_10x8_SFLOAT_BLOCK: Self = Self(1000066010);
    pub const ASTC_10x10_SFLOAT_BLOCK: Self = Self(1000066011);
    pub const ASTC_12x10_SFLOAT_BLOCK: Self = Self(1000066012);
    pub const ASTC_12x12_SFLOAT_BLOCK: Self = Self(1000066013);
    pub const G8B8G8R8_422_UNORM: Self = Self(1000156000);
    pub const B8G8R8G8_422_UNORM: Self = Self(1000156001);
    pub const G8_B8_R8_3PLANE_420_UNORM: Self = Self(1000156002);
    pub const G8_B8R8_2PLANE_420_UNORM: Self = Self(1000156003);
    pub const G8_B8_R8_3PLANE_422_UNORM: Self = Self(1000156004);
    pub const G8_B8R8_2PLANE_422_UNORM: Self = Self(1000156005);
    pub const G8_B8_R8_3PLANE_444_UNORM: Self = Self(1000156006);
    pub const R10X6_UNORM_PACK16: Self = Self(1000156007);
    pub const R10X6G10X6_UNORM_2PACK16: Self = Self(1000156008);
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self(1000156009);
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self(1000156010);
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self(1000156011);
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self(1000156012);
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self(1000156013);
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self(1000156014);
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self(1000156015);
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self(1000156016);
    pub const R12X4_UNORM_PACK16: Self = Self(1000156017);
    pub const R12X4G12X4_UNORM_2PACK16: Self = Self(1000156018);
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self(1000156019);
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self(1000156020);
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self(1000156021);
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self(1000156022);
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self(1000156023);
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self(1000156024);
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self(1000156025);
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self(1000156026);
    pub const G16B16G16R16_422_UNORM: Self = Self(1000156027);
    pub const B16G16R16G16_422_UNORM: Self = Self(1000156028);
    pub const G16_B16_R16_3PLANE_420_UNORM: Self = Self(1000156029);
    pub const G16_B16R16_2PLANE_420_UNORM: Self = Self(1000156030);
    pub const G16_B16_R16_3PLANE_422_UNORM: Self = Self(1000156031);
    pub const G16_B16R16_2PLANE_422_UNORM: Self = Self(1000156032);
    pub const G16_B16_R16_3PLANE_444_UNORM: Self = Self(1000156033);
    pub const ASTC_3x3x3_UNORM_BLOCK_EXT: Self = Self(1000288000);
    pub const ASTC_3x3x3_SRGB_BLOCK_EXT: Self = Self(1000288001);
    pub const ASTC_3x3x3_SFLOAT_BLOCK_EXT: Self = Self(1000288002);
    pub const ASTC_4x3x3_UNORM_BLOCK_EXT: Self = Self(1000288003);
    pub const ASTC_4x3x3_SRGB_BLOCK_EXT: Self = Self(1000288004);
    pub const ASTC_4x3x3_SFLOAT_BLOCK_EXT: Self = Self(1000288005);
    pub const ASTC_4x4x3_UNORM_BLOCK_EXT: Self = Self(1000288006);
    pub const ASTC_4x4x3_SRGB_BLOCK_EXT: Self = Self(1000288007);
    pub const ASTC_4x4x3_SFLOAT_BLOCK_EXT: Self = Self(1000288008);
    pub const ASTC_4x4x4_UNORM_BLOCK_EXT: Self = Self(1000288009);
    pub const ASTC_4x4x4_SRGB_BLOCK_EXT: Self = Self(1000288010);
    pub const ASTC_4x4x4_SFLOAT_BLOCK_EXT: Self = Self(1000288011);
    pub const ASTC_5x4x4_UNORM_BLOCK_EXT: Self = Self(1000288012);
    pub const ASTC_5x4x4_SRGB_BLOCK_EXT: Self = Self(1000288013);
    pub const ASTC_5x4x4_SFLOAT_BLOCK_EXT: Self = Self(1000288014);
    pub const ASTC_5x5x4_UNORM_BLOCK_EXT: Self = Self(1000288015);
    pub const ASTC_5x5x4_SRGB_BLOCK_EXT: Self = Self(1000288016);
    pub const ASTC_5x5x4_SFLOAT_BLOCK_EXT: Self = Self(1000288017);
    pub const ASTC_5x5x5_UNORM_BLOCK_EXT: Self = Self(1000288018);
    pub const ASTC_5x5x5_SRGB_BLOCK_EXT: Self = Self(1000288019);
    pub const ASTC_5x5x5_SFLOAT_BLOCK_EXT: Self = Self(1000288020);
    pub const ASTC_6x5x5_UNORM_BLOCK_EXT: Self = Self(1000288021);
    pub const ASTC_6x5x5_SRGB_BLOCK_EXT: Self = Self(1000288022);
    pub const ASTC_6x5x5_SFLOAT_BLOCK_EXT: Self = Self(1000288023);
    pub const ASTC_6x6x5_UNORM_BLOCK_EXT: Self = Self(1000288024);
    pub const ASTC_6x6x5_SRGB_BLOCK_EXT: Self = Self(1000288025);
    pub const ASTC_6x6x5_SFLOAT_BLOCK_EXT: Self = Self(1000288026);
    pub const ASTC_6x6x6_UNORM_BLOCK_EXT: Self = Self(1000288027);
    pub const ASTC_6x6x6_SRGB_BLOCK_EXT: Self = Self(1000288028);
    pub const ASTC_6x6x6_SFLOAT_BLOCK_EXT: Self = Self(1000288029);
    pub const G8_B8R8_2PLANE_444_UNORM: Self = Self(1000330000);
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: Self = Self(1000330001);
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: Self = Self(1000330002);
    pub const G16_B16R16_2PLANE_444_UNORM: Self = Self(1000330003);
    pub const A4R4G4B4_UNORM_PACK16: Self = Self(1000340000);
    pub const A4B4G4R4_UNORM_PACK16: Self = Self(1000340001);
    pub const R8_BOOL_ARM: Self = Self(1000460000);
    pub const R16_SFLOAT_FPENCODING_BFLOAT16_ARM: Self = Self(1000460001);
    pub const R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM: Self = Self(1000460002);
    pub const R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM: Self = Self(1000460003);
    pub const R16G16_SFIXED5_NV: Self = Self(1000464000);
    pub const A1B5G5R5_UNORM_PACK16: Self = Self(1000470000);
    pub const A8_UNORM: Self = Self(1000470001);
    pub const R10X6_UINT_PACK16_ARM: Self = Self(1000609000);
    pub const R10X6G10X6_UINT_2PACK16_ARM: Self = Self(1000609001);
    pub const R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM: Self = Self(1000609002);
    pub const R12X4_UINT_PACK16_ARM: Self = Self(1000609003);
    pub const R12X4G12X4_UINT_2PACK16_ARM: Self = Self(1000609004);
    pub const R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM: Self = Self(1000609005);
    pub const R14X2_UINT_PACK16_ARM: Self = Self(1000609006);
    pub const R14X2G14X2_UINT_2PACK16_ARM: Self = Self(1000609007);
    pub const R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM: Self = Self(1000609008);
    pub const R14X2_UNORM_PACK16_ARM: Self = Self(1000609009);
    pub const R14X2G14X2_UNORM_2PACK16_ARM: Self = Self(1000609010);
    pub const R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM: Self = Self(1000609011);
    pub const G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM: Self = Self(1000609012);
    pub const G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM: Self = Self(1000609013);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FragmentShadingRateCombinerOpKHR(pub i32);

impl FragmentShadingRateCombinerOpKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR: Self = Self(0);
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR: Self = Self(1);
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR: Self = Self(2);
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR: Self = Self(3);
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FragmentShadingRateNV(pub i32);

impl FragmentShadingRateNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: Self = Self(0);
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(1);
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(4);
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(5);
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(6);
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(10);
    pub const FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: Self = Self(11);
    pub const FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: Self = Self(12);
    pub const FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: Self = Self(13);
    pub const FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: Self = Self(14);
    pub const FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV: Self = Self(15);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FragmentShadingRateTypeNV(pub i32);

impl FragmentShadingRateTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV: Self = Self(0);
    pub const FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FrontFace(pub i32);

impl FrontFace {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COUNTER_CLOCKWISE: Self = Self(0);
    pub const CLOCKWISE: Self = Self(1);
}

#[cfg(feature = "win32")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FullScreenExclusiveEXT(pub i32);

#[cfg(feature = "win32")]
impl FullScreenExclusiveEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: Self = Self(0);
    pub const FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: Self = Self(1);
    pub const FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: Self = Self(2);
    pub const FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct GeometryTypeKHR(pub i32);

impl GeometryTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const GEOMETRY_TYPE_TRIANGLES_KHR: Self = Self(0);
    pub const GEOMETRY_TYPE_AABBS_KHR: Self = Self(1);
    pub const GEOMETRY_TYPE_INSTANCES_KHR: Self = Self(2);
    pub const GEOMETRY_TYPE_SPHERES_NV: Self = Self(1000429004);
    pub const GEOMETRY_TYPE_LINEAR_SWEPT_SPHERES_NV: Self = Self(1000429005);
    #[cfg(feature = "beta")]
    pub const GEOMETRY_TYPE_DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX: Self = Self(1000478000);
}

pub type GeometryTypeNV = GeometryTypeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageLayout(pub i32);

impl ImageLayout {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNDEFINED: Self = Self(0);
    pub const GENERAL: Self = Self(1);
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
    pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
    pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
    pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
    pub const PREINITIALIZED: Self = Self(8);
    pub const PRESENT_SRC_KHR: Self = Self(1000001002);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1000024000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(1000024001);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(1000024002);
    pub const SHARED_PRESENT_KHR: Self = Self(1000111000);
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000117000);
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000117001);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000164003);
    pub const FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: Self = Self(1000218000);
    pub const RENDERING_LOCAL_READ: Self = Self(1000232000);
    pub const DEPTH_ATTACHMENT_OPTIMAL: Self = Self(1000241000);
    pub const DEPTH_READ_ONLY_OPTIMAL: Self = Self(1000241001);
    pub const STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000241002);
    pub const STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000241003);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1000299001);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1000299002);
    pub const READ_ONLY_OPTIMAL: Self = Self(1000314000);
    pub const ATTACHMENT_OPTIMAL: Self = Self(1000314001);
    pub const ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT: Self = Self(1000339000);
    pub const TENSOR_ALIASING_ARM: Self = Self(1000460000);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_KHR: Self = Self(1000553000);
    pub const ZERO_INITIALIZED_EXT: Self = Self(1000620000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageTiling(pub i32);

impl ImageTiling {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPTIMAL: Self = Self(0);
    pub const LINEAR: Self = Self(1);
    pub const DRM_FORMAT_MODIFIER_EXT: Self = Self(1000158000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageType(pub i32);

impl ImageType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _1D: Self = Self(0);
    pub const _2D: Self = Self(1);
    pub const _3D: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageViewType(pub i32);

impl ImageViewType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _1D: Self = Self(0);
    pub const _2D: Self = Self(1);
    pub const _3D: Self = Self(2);
    pub const CUBE: Self = Self(3);
    pub const _1D_ARRAY: Self = Self(4);
    pub const _2D_ARRAY: Self = Self(5);
    pub const CUBE_ARRAY: Self = Self(6);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndexType(pub i32);

impl IndexType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UINT16: Self = Self(0);
    pub const UINT32: Self = Self(1);
    pub const NONE_KHR: Self = Self(1000165000);
    pub const UINT8: Self = Self(1000265000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectCommandsTokenTypeEXT(pub i32);

impl IndirectCommandsTokenTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_EXECUTION_SET_EXT: Self = Self(0);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_EXT: Self = Self(1);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_SEQUENCE_INDEX_EXT: Self = Self(2);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_EXT: Self = Self(3);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_EXT: Self = Self(4);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_EXT: Self = Self(5);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_EXT: Self = Self(6);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_COUNT_EXT: Self = Self(7);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_COUNT_EXT: Self = Self(8);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_EXT: Self = Self(9);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_DATA_EXT: Self = Self(1000135000);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_DATA_SEQUENCE_INDEX_EXT: Self = Self(1000135001);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_MESH_TASKS_NV_EXT: Self = Self(1000202002);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_MESH_TASKS_COUNT_NV_EXT: Self = Self(1000202003);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_MESH_TASKS_EXT: Self = Self(1000328000);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_MESH_TASKS_COUNT_EXT: Self = Self(1000328001);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_TRACE_RAYS2_EXT: Self = Self(1000386004);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectCommandsTokenTypeNV(pub i32);

impl IndirectCommandsTokenTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV: Self = Self(0);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV: Self = Self(1);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV: Self = Self(2);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV: Self = Self(3);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV: Self = Self(4);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV: Self = Self(5);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV: Self = Self(6);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV: Self = Self(7);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_DATA_NV: Self = Self(1000135000);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_MESH_TASKS_NV: Self = Self(1000328000);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_PIPELINE_NV: Self = Self(1000428003);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_NV: Self = Self(1000428004);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectExecutionSetInfoTypeEXT(pub i32);

impl IndirectExecutionSetInfoTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INDIRECT_EXECUTION_SET_INFO_TYPE_PIPELINES_EXT: Self = Self(0);
    pub const INDIRECT_EXECUTION_SET_INFO_TYPE_SHADER_OBJECTS_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct InternalAllocationType(pub i32);

impl InternalAllocationType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const EXECUTABLE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct LatencyMarkerNV(pub i32);

impl LatencyMarkerNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const LATENCY_MARKER_SIMULATION_START_NV: Self = Self(0);
    pub const LATENCY_MARKER_SIMULATION_END_NV: Self = Self(1);
    pub const LATENCY_MARKER_RENDERSUBMIT_START_NV: Self = Self(2);
    pub const LATENCY_MARKER_RENDERSUBMIT_END_NV: Self = Self(3);
    pub const LATENCY_MARKER_PRESENT_START_NV: Self = Self(4);
    pub const LATENCY_MARKER_PRESENT_END_NV: Self = Self(5);
    pub const LATENCY_MARKER_INPUT_SAMPLE_NV: Self = Self(6);
    pub const LATENCY_MARKER_TRIGGER_FLASH_NV: Self = Self(7);
    pub const LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_START_NV: Self = Self(8);
    pub const LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_END_NV: Self = Self(9);
    pub const LATENCY_MARKER_OUT_OF_BAND_PRESENT_START_NV: Self = Self(10);
    pub const LATENCY_MARKER_OUT_OF_BAND_PRESENT_END_NV: Self = Self(11);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct LayerSettingTypeEXT(pub i32);

impl LayerSettingTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const LAYER_SETTING_TYPE_BOOL32_EXT: Self = Self(0);
    pub const LAYER_SETTING_TYPE_INT32_EXT: Self = Self(1);
    pub const LAYER_SETTING_TYPE_INT64_EXT: Self = Self(2);
    pub const LAYER_SETTING_TYPE_UINT32_EXT: Self = Self(3);
    pub const LAYER_SETTING_TYPE_UINT64_EXT: Self = Self(4);
    pub const LAYER_SETTING_TYPE_FLOAT32_EXT: Self = Self(5);
    pub const LAYER_SETTING_TYPE_FLOAT64_EXT: Self = Self(6);
    pub const LAYER_SETTING_TYPE_STRING_EXT: Self = Self(7);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct LayeredDriverUnderlyingApiMSFT(pub i32);

impl LayeredDriverUnderlyingApiMSFT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const LAYERED_DRIVER_UNDERLYING_API_NONE_MSFT: Self = Self(0);
    pub const LAYERED_DRIVER_UNDERLYING_API_D3D12_MSFT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct LineRasterizationMode(pub i32);

impl LineRasterizationMode {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const RECTANGULAR: Self = Self(1);
    pub const BRESENHAM: Self = Self(2);
    pub const RECTANGULAR_SMOOTH: Self = Self(3);
}

pub type LineRasterizationModeEXT = LineRasterizationMode;

pub type LineRasterizationModeKHR = LineRasterizationMode;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct LogicOp(pub i32);

impl LogicOp {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CLEAR: Self = Self(0);
    pub const AND: Self = Self(1);
    pub const AND_REVERSE: Self = Self(2);
    pub const COPY: Self = Self(3);
    pub const AND_INVERTED: Self = Self(4);
    pub const NO_OP: Self = Self(5);
    pub const XOR: Self = Self(6);
    pub const OR: Self = Self(7);
    pub const NOR: Self = Self(8);
    pub const EQUIVALENT: Self = Self(9);
    pub const INVERT: Self = Self(10);
    pub const OR_REVERSE: Self = Self(11);
    pub const COPY_INVERTED: Self = Self(12);
    pub const OR_INVERTED: Self = Self(13);
    pub const NAND: Self = Self(14);
    pub const SET: Self = Self(15);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryOverallocationBehaviorAMD(pub i32);

impl MemoryOverallocationBehaviorAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD: Self = Self(0);
    pub const MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD: Self = Self(1);
    pub const MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MicromapTypeEXT(pub i32);

impl MicromapTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MICROMAP_TYPE_OPACITY_MICROMAP_EXT: Self = Self(0);
    #[cfg(feature = "beta")]
    pub const MICROMAP_TYPE_DISPLACEMENT_MICROMAP_NV: Self = Self(1000397000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct NeuralAcceleratorStatisticsModeARM(pub i32);

impl NeuralAcceleratorStatisticsModeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NEURAL_ACCELERATOR_STATISTICS_MODE_DISABLED_ARM: Self = Self(0);
    pub const NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS0_ARM: Self = Self(1);
    pub const NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS1_ARM: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ObjectType(pub i32);

impl ObjectType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNKNOWN: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
    pub const PHYSICAL_DEVICE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const QUEUE: Self = Self(4);
    pub const SEMAPHORE: Self = Self(5);
    pub const COMMAND_BUFFER: Self = Self(6);
    pub const FENCE: Self = Self(7);
    pub const DEVICE_MEMORY: Self = Self(8);
    pub const BUFFER: Self = Self(9);
    pub const IMAGE: Self = Self(10);
    pub const EVENT: Self = Self(11);
    pub const QUERY_POOL: Self = Self(12);
    pub const BUFFER_VIEW: Self = Self(13);
    pub const IMAGE_VIEW: Self = Self(14);
    pub const SHADER_MODULE: Self = Self(15);
    pub const PIPELINE_CACHE: Self = Self(16);
    pub const PIPELINE_LAYOUT: Self = Self(17);
    pub const RENDER_PASS: Self = Self(18);
    pub const PIPELINE: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const SAMPLER: Self = Self(21);
    pub const DESCRIPTOR_POOL: Self = Self(22);
    pub const DESCRIPTOR_SET: Self = Self(23);
    pub const FRAMEBUFFER: Self = Self(24);
    pub const COMMAND_POOL: Self = Self(25);
    pub const SURFACE_KHR: Self = Self(1000000000);
    pub const SWAPCHAIN_KHR: Self = Self(1000001000);
    pub const DISPLAY_KHR: Self = Self(1000002000);
    pub const DISPLAY_MODE_KHR: Self = Self(1000002001);
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(1000011000);
    pub const VIDEO_SESSION_KHR: Self = Self(1000023000);
    pub const VIDEO_SESSION_PARAMETERS_KHR: Self = Self(1000023001);
    pub const CU_MODULE_NVX: Self = Self(1000029000);
    pub const CU_FUNCTION_NVX: Self = Self(1000029001);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
    pub const DEBUG_UTILS_MESSENGER_EXT: Self = Self(1000128000);
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    pub const VALIDATION_CACHE_EXT: Self = Self(1000160000);
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    pub const PERFORMANCE_CONFIGURATION_INTEL: Self = Self(1000210000);
    pub const DEFERRED_OPERATION_KHR: Self = Self(1000268000);
    pub const INDIRECT_COMMANDS_LAYOUT_NV: Self = Self(1000277000);
    pub const PRIVATE_DATA_SLOT: Self = Self(1000295000);
    #[cfg(feature = "beta")]
    pub const CUDA_MODULE_NV: Self = Self(1000307000);
    #[cfg(feature = "beta")]
    pub const CUDA_FUNCTION_NV: Self = Self(1000307001);
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
    pub const MICROMAP_EXT: Self = Self(1000396000);
    pub const TENSOR_ARM: Self = Self(1000460000);
    pub const TENSOR_VIEW_ARM: Self = Self(1000460001);
    pub const OPTICAL_FLOW_SESSION_NV: Self = Self(1000464000);
    pub const SHADER_EXT: Self = Self(1000482000);
    pub const PIPELINE_BINARY_KHR: Self = Self(1000483000);
    pub const DATA_GRAPH_PIPELINE_SESSION_ARM: Self = Self(1000507000);
    pub const EXTERNAL_COMPUTE_QUEUE_NV: Self = Self(1000556000);
    pub const INDIRECT_COMMANDS_LAYOUT_EXT: Self = Self(1000572000);
    pub const INDIRECT_EXECUTION_SET_EXT: Self = Self(1000572001);
    pub const SHADER_INSTRUMENTATION_ARM: Self = Self(1000607000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpacityMicromapFormatEXT(pub i32);

impl OpacityMicromapFormatEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPACITY_MICROMAP_FORMAT_2_STATE_EXT: Self = Self(1);
    pub const OPACITY_MICROMAP_FORMAT_4_STATE_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpacityMicromapSpecialIndexEXT(pub i32);

impl OpacityMicromapSpecialIndexEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPACITY_MICROMAP_SPECIAL_INDEX_CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV: Self =
        Self(-5i32);
    pub const OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_OPAQUE_EXT: Self = Self(-4i32);
    pub const OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_TRANSPARENT_EXT: Self = Self(-3i32);
    pub const OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_OPAQUE_EXT: Self = Self(-2i32);
    pub const OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_TRANSPARENT_EXT: Self = Self(-1i32);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpticalFlowPerformanceLevelNV(pub i32);

impl OpticalFlowPerformanceLevelNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_NV: Self = Self(0);
    pub const OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_NV: Self = Self(1);
    pub const OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_NV: Self = Self(2);
    pub const OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpticalFlowSessionBindingPointNV(pub i32);

impl OpticalFlowSessionBindingPointNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPTICAL_FLOW_SESSION_BINDING_POINT_UNKNOWN_NV: Self = Self(0);
    pub const OPTICAL_FLOW_SESSION_BINDING_POINT_INPUT_NV: Self = Self(1);
    pub const OPTICAL_FLOW_SESSION_BINDING_POINT_REFERENCE_NV: Self = Self(2);
    pub const OPTICAL_FLOW_SESSION_BINDING_POINT_HINT_NV: Self = Self(3);
    pub const OPTICAL_FLOW_SESSION_BINDING_POINT_FLOW_VECTOR_NV: Self = Self(4);
    pub const OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_FLOW_VECTOR_NV: Self = Self(5);
    pub const OPTICAL_FLOW_SESSION_BINDING_POINT_COST_NV: Self = Self(6);
    pub const OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_COST_NV: Self = Self(7);
    pub const OPTICAL_FLOW_SESSION_BINDING_POINT_GLOBAL_FLOW_NV: Self = Self(8);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OutOfBandQueueTypeNV(pub i32);

impl OutOfBandQueueTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OUT_OF_BAND_QUEUE_TYPE_RENDER_NV: Self = Self(0);
    pub const OUT_OF_BAND_QUEUE_TYPE_PRESENT_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PartitionedAccelerationStructureOpTypeNV(pub i32);

impl PartitionedAccelerationStructureOpTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_INSTANCE_NV: Self = Self(0);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_UPDATE_INSTANCE_NV: Self = Self(1);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_PARTITION_TRANSLATION_NV: Self =
        Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerfHintTypeQCOM(pub i32);

impl PerfHintTypeQCOM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PERF_HINT_TYPE_DEFAULT_QCOM: Self = Self(0);
    pub const PERF_HINT_TYPE_FREQUENCY_MIN_QCOM: Self = Self(1);
    pub const PERF_HINT_TYPE_FREQUENCY_MAX_QCOM: Self = Self(2);
    pub const PERF_HINT_TYPE_FREQUENCY_SCALED_QCOM: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerformanceConfigurationTypeINTEL(pub i32);

impl PerformanceConfigurationTypeINTEL {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self =
        Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerformanceCounterScopeKHR(pub i32);

impl PerformanceCounterScopeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR: Self = Self(0);
    pub const PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR: Self = Self(1);
    pub const PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerformanceCounterStorageKHR(pub i32);

impl PerformanceCounterStorageKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PERFORMANCE_COUNTER_STORAGE_INT32_KHR: Self = Self(0);
    pub const PERFORMANCE_COUNTER_STORAGE_INT64_KHR: Self = Self(1);
    pub const PERFORMANCE_COUNTER_STORAGE_UINT32_KHR: Self = Self(2);
    pub const PERFORMANCE_COUNTER_STORAGE_UINT64_KHR: Self = Self(3);
    pub const PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR: Self = Self(4);
    pub const PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR: Self = Self(5);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerformanceCounterUnitKHR(pub i32);

impl PerformanceCounterUnitKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PERFORMANCE_COUNTER_UNIT_GENERIC_KHR: Self = Self(0);
    pub const PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR: Self = Self(1);
    pub const PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR: Self = Self(2);
    pub const PERFORMANCE_COUNTER_UNIT_BYTES_KHR: Self = Self(3);
    pub const PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR: Self = Self(4);
    pub const PERFORMANCE_COUNTER_UNIT_KELVIN_KHR: Self = Self(5);
    pub const PERFORMANCE_COUNTER_UNIT_WATTS_KHR: Self = Self(6);
    pub const PERFORMANCE_COUNTER_UNIT_VOLTS_KHR: Self = Self(7);
    pub const PERFORMANCE_COUNTER_UNIT_AMPS_KHR: Self = Self(8);
    pub const PERFORMANCE_COUNTER_UNIT_HERTZ_KHR: Self = Self(9);
    pub const PERFORMANCE_COUNTER_UNIT_CYCLES_KHR: Self = Self(10);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerformanceOverrideTypeINTEL(pub i32);

impl PerformanceOverrideTypeINTEL {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL: Self = Self(0);
    pub const PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerformanceParameterTypeINTEL(pub i32);

impl PerformanceParameterTypeINTEL {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL: Self = Self(0);
    pub const PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerformanceValueTypeINTEL(pub i32);

impl PerformanceValueTypeINTEL {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PERFORMANCE_VALUE_TYPE_UINT32_INTEL: Self = Self(0);
    pub const PERFORMANCE_VALUE_TYPE_UINT64_INTEL: Self = Self(1);
    pub const PERFORMANCE_VALUE_TYPE_FLOAT_INTEL: Self = Self(2);
    pub const PERFORMANCE_VALUE_TYPE_BOOL_INTEL: Self = Self(3);
    pub const PERFORMANCE_VALUE_TYPE_STRING_INTEL: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PhysicalDeviceDataGraphOperationTypeARM(pub i32);

impl PhysicalDeviceDataGraphOperationTypeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_TYPE_SPIRV_EXTENDED_INSTRUCTION_SET_ARM: Self =
        Self(0);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_TYPE_NEURAL_MODEL_QCOM: Self = Self(1000629000);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_TYPE_BUILTIN_MODEL_QCOM: Self = Self(1000629001);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_TYPE_OPTICAL_FLOW_ARM: Self = Self(1000631000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PhysicalDeviceDataGraphProcessingEngineTypeARM(pub i32);

impl PhysicalDeviceDataGraphProcessingEngineTypeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PHYSICAL_DEVICE_DATA_GRAPH_PROCESSING_ENGINE_TYPE_DEFAULT_ARM: Self = Self(0);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_PROCESSING_ENGINE_TYPE_NEURAL_QCOM: Self =
        Self(1000629000);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_PROCESSING_ENGINE_TYPE_COMPUTE_QCOM: Self =
        Self(1000629001);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PhysicalDeviceLayeredApiKHR(pub i32);

impl PhysicalDeviceLayeredApiKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PHYSICAL_DEVICE_LAYERED_API_VULKAN_KHR: Self = Self(0);
    pub const PHYSICAL_DEVICE_LAYERED_API_D3D12_KHR: Self = Self(1);
    pub const PHYSICAL_DEVICE_LAYERED_API_METAL_KHR: Self = Self(2);
    pub const PHYSICAL_DEVICE_LAYERED_API_OPENGL_KHR: Self = Self(3);
    pub const PHYSICAL_DEVICE_LAYERED_API_OPENGLES_KHR: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PhysicalDeviceType(pub i32);

impl PhysicalDeviceType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OTHER: Self = Self(0);
    pub const INTEGRATED_GPU: Self = Self(1);
    pub const DISCRETE_GPU: Self = Self(2);
    pub const VIRTUAL_GPU: Self = Self(3);
    pub const CPU: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineBindPoint(pub i32);

impl PipelineBindPoint {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const GRAPHICS: Self = Self(0);
    pub const COMPUTE: Self = Self(1);
    #[cfg(feature = "beta")]
    pub const EXECUTION_GRAPH_AMDX: Self = Self(1000134000);
    pub const RAY_TRACING_KHR: Self = Self(1000165000);
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(1000369003);
    pub const DATA_GRAPH_ARM: Self = Self(1000507000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCacheHeaderVersion(pub i32);

impl PipelineCacheHeaderVersion {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ONE: Self = Self(1);
    pub const DATA_GRAPH_QCOM: Self = Self(1000629000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineExecutableStatisticFormatKHR(pub i32);

impl PipelineExecutableStatisticFormatKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR: Self = Self(0);
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR: Self = Self(1);
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR: Self = Self(2);
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineRobustnessBufferBehavior(pub i32);

impl PipelineRobustnessBufferBehavior {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE_DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(1);
    pub const ROBUST_BUFFER_ACCESS: Self = Self(2);
    pub const ROBUST_BUFFER_ACCESS_2: Self = Self(3);
}

pub type PipelineRobustnessBufferBehaviorEXT = PipelineRobustnessBufferBehavior;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineRobustnessImageBehavior(pub i32);

impl PipelineRobustnessImageBehavior {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE_DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(1);
    pub const ROBUST_IMAGE_ACCESS: Self = Self(2);
    pub const ROBUST_IMAGE_ACCESS_2: Self = Self(3);
}

pub type PipelineRobustnessImageBehaviorEXT = PipelineRobustnessImageBehavior;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PointClippingBehavior(pub i32);

impl PointClippingBehavior {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
}

pub type PointClippingBehaviorKHR = PointClippingBehavior;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PolygonMode(pub i32);

impl PolygonMode {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FILL: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const POINT: Self = Self(2);
    pub const FILL_RECTANGLE_NV: Self = Self(1000153000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PresentModeKHR(pub i32);

impl PresentModeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PRESENT_MODE_IMMEDIATE_KHR: Self = Self(0);
    pub const PRESENT_MODE_MAILBOX_KHR: Self = Self(1);
    pub const PRESENT_MODE_FIFO_KHR: Self = Self(2);
    pub const PRESENT_MODE_FIFO_RELAXED_KHR: Self = Self(3);
    pub const PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR: Self = Self(1000111000);
    pub const PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR: Self = Self(1000111001);
    pub const PRESENT_MODE_FIFO_LATEST_READY_KHR: Self = Self(1000361000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PrimitiveTopology(pub i32);

impl PrimitiveTopology {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const POINT_LIST: Self = Self(0);
    pub const LINE_LIST: Self = Self(1);
    pub const LINE_STRIP: Self = Self(2);
    pub const TRIANGLE_LIST: Self = Self(3);
    pub const TRIANGLE_STRIP: Self = Self(4);
    pub const TRIANGLE_FAN: Self = Self(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
    pub const PATCH_LIST: Self = Self(10);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ProvokingVertexModeEXT(pub i32);

impl ProvokingVertexModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT: Self = Self(0);
    pub const PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryPoolSamplingModeINTEL(pub i32);

impl QueryPoolSamplingModeINTEL {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryResultStatusKHR(pub i32);

impl QueryResultStatusKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const QUERY_RESULT_STATUS_INSUFFICIENT_BITSTREAM_BUFFER_RANGE_KHR: Self =
        Self(-1000299000i32);
    pub const QUERY_RESULT_STATUS_ERROR_KHR: Self = Self(-1i32);
    pub const QUERY_RESULT_STATUS_NOT_READY_KHR: Self = Self(0);
    pub const QUERY_RESULT_STATUS_COMPLETE_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryType(pub i32);

impl QueryType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OCCLUSION: Self = Self(0);
    pub const PIPELINE_STATISTICS: Self = Self(1);
    pub const TIMESTAMP: Self = Self(2);
    pub const RESULT_STATUS_ONLY_KHR: Self = Self(1000023000);
    pub const TRANSFORM_FEEDBACK_STREAM_EXT: Self = Self(1000028004);
    pub const PERFORMANCE_QUERY_KHR: Self = Self(1000116000);
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: Self = Self(1000150001);
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV: Self = Self(1000165000);
    pub const PERFORMANCE_QUERY_INTEL: Self = Self(1000210000);
    pub const VIDEO_ENCODE_FEEDBACK_KHR: Self = Self(1000299000);
    pub const MESH_PRIMITIVES_GENERATED_EXT: Self = Self(1000328000);
    pub const PRIMITIVES_GENERATED_EXT: Self = Self(1000382000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR: Self =
        Self(1000386000);
    pub const ACCELERATION_STRUCTURE_SIZE_KHR: Self = Self(1000386001);
    pub const MICROMAP_SERIALIZATION_SIZE_EXT: Self = Self(1000396000);
    pub const MICROMAP_COMPACTED_SIZE_EXT: Self = Self(1000396001);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueueGlobalPriority(pub i32);

impl QueueGlobalPriority {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const LOW: Self = Self(128);
    pub const MEDIUM: Self = Self(256);
    pub const HIGH: Self = Self(512);
    pub const REALTIME: Self = Self(1024);
}

pub type QueueGlobalPriorityEXT = QueueGlobalPriority;

pub type QueueGlobalPriorityKHR = QueueGlobalPriority;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RasterizationOrderAMD(pub i32);

impl RasterizationOrderAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const RASTERIZATION_ORDER_STRICT_AMD: Self = Self(0);
    pub const RASTERIZATION_ORDER_RELAXED_AMD: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RayTracingInvocationReorderModeEXT(pub i32);

impl RayTracingInvocationReorderModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const RAY_TRACING_INVOCATION_REORDER_MODE_NONE_EXT: Self = Self(0);
    pub const RAY_TRACING_INVOCATION_REORDER_MODE_REORDER_EXT: Self = Self(1);
}

pub type RayTracingInvocationReorderModeNV = RayTracingInvocationReorderModeEXT;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RayTracingLssIndexingModeNV(pub i32);

impl RayTracingLssIndexingModeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const RAY_TRACING_LSS_INDEXING_MODE_LIST_NV: Self = Self(0);
    pub const RAY_TRACING_LSS_INDEXING_MODE_SUCCESSIVE_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RayTracingLssPrimitiveEndCapsModeNV(pub i32);

impl RayTracingLssPrimitiveEndCapsModeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_NONE_NV: Self = Self(0);
    pub const RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_CHAINED_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RayTracingShaderGroupTypeKHR(pub i32);

impl RayTracingShaderGroupTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR: Self = Self(0);
    pub const RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR: Self = Self(1);
    pub const RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR: Self = Self(2);
}

pub type RayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VkResult(pub i32);

impl VkResult {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ERROR_NOT_ENOUGH_SPACE_KHR: Self = Self(-1000483000i32);
    pub const ERROR_COMPRESSION_EXHAUSTED_EXT: Self = Self(-1000338000i32);
    pub const ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: Self = Self(-1000299000i32);
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: Self = Self(-1000257000i32);
    pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: Self = Self(-1000255000i32);
    pub const ERROR_PRESENT_TIMING_QUEUE_FULL_EXT: Self = Self(-1000208000i32);
    pub const ERROR_NOT_PERMITTED: Self = Self(-1000174001i32);
    pub const ERROR_FRAGMENTATION: Self = Self(-1000161000i32);
    pub const ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: Self = Self(-1000158000i32);
    pub const ERROR_INVALID_EXTERNAL_HANDLE: Self = Self(-1000072003i32);
    pub const ERROR_OUT_OF_POOL_MEMORY: Self = Self(-1000069000i32);
    pub const ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: Self = Self(-1000023005i32);
    pub const ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: Self = Self(-1000023004i32);
    pub const ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: Self = Self(-1000023003i32);
    pub const ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: Self = Self(-1000023002i32);
    pub const ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: Self = Self(-1000023001i32);
    pub const ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: Self = Self(-1000023000i32);
    pub const ERROR_INVALID_SHADER_NV: Self = Self(-1000012000i32);
    pub const ERROR_VALIDATION_FAILED: Self = Self(-1000011001i32);
    pub const ERROR_INCOMPATIBLE_DISPLAY_KHR: Self = Self(-1000003001i32);
    pub const ERROR_OUT_OF_DATE_KHR: Self = Self(-1000001004i32);
    pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1000000001i32);
    pub const ERROR_SURFACE_LOST_KHR: Self = Self(-1000000000i32);
    pub const ERROR_UNKNOWN: Self = Self(-13i32);
    pub const ERROR_FRAGMENTED_POOL: Self = Self(-12i32);
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11i32);
    pub const ERROR_TOO_MANY_OBJECTS: Self = Self(-10i32);
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9i32);
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(-8i32);
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7i32);
    pub const ERROR_LAYER_NOT_PRESENT: Self = Self(-6i32);
    pub const ERROR_MEMORY_MAP_FAILED: Self = Self(-5i32);
    pub const ERROR_DEVICE_LOST: Self = Self(-4i32);
    pub const ERROR_INITIALIZATION_FAILED: Self = Self(-3i32);
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2i32);
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1i32);
    pub const SUCCESS: Self = Self(0);
    pub const NOT_READY: Self = Self(1);
    pub const TIMEOUT: Self = Self(2);
    pub const EVENT_SET: Self = Self(3);
    pub const EVENT_RESET: Self = Self(4);
    pub const INCOMPLETE: Self = Self(5);
    pub const SUBOPTIMAL_KHR: Self = Self(1000001003);
    pub const THREAD_IDLE_KHR: Self = Self(1000268000);
    pub const THREAD_DONE_KHR: Self = Self(1000268001);
    pub const OPERATION_DEFERRED_KHR: Self = Self(1000268002);
    pub const OPERATION_NOT_DEFERRED_KHR: Self = Self(1000268003);
    pub const PIPELINE_COMPILE_REQUIRED: Self = Self(1000297000);
    pub const INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1000482000);
    pub const PIPELINE_BINARY_MISSING_KHR: Self = Self(1000483000);
}

impl VkResult {
    pub const fn into_result(self) -> core::result::Result<(), Self> {
        if self.0 >= 0 { Ok(()) } else { Err(self) }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SamplerAddressMode(pub i32);

impl SamplerAddressMode {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const REPEAT: Self = Self(0);
    pub const MIRRORED_REPEAT: Self = Self(1);
    pub const CLAMP_TO_EDGE: Self = Self(2);
    pub const CLAMP_TO_BORDER: Self = Self(3);
    pub const MIRROR_CLAMP_TO_EDGE: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SamplerMipmapMode(pub i32);

impl SamplerMipmapMode {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SamplerReductionMode(pub i32);

impl SamplerReductionMode {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(2);
    pub const WEIGHTED_AVERAGE_RANGECLAMP_QCOM: Self = Self(1000521000);
}

pub type SamplerReductionModeEXT = SamplerReductionMode;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SamplerYcbcrModelConversion(pub i32);

impl SamplerYcbcrModelConversion {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const RGB_IDENTITY: Self = Self(0);
    pub const YCBCR_IDENTITY: Self = Self(1);
    pub const YCBCR_709: Self = Self(2);
    pub const YCBCR_601: Self = Self(3);
    pub const YCBCR_2020: Self = Self(4);
}

pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SamplerYcbcrRange(pub i32);

impl SamplerYcbcrRange {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ITU_FULL: Self = Self(0);
    pub const ITU_NARROW: Self = Self(1);
}

pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ScopeKHR(pub i32);

impl ScopeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SCOPE_DEVICE_KHR: Self = Self(1);
    pub const SCOPE_WORKGROUP_KHR: Self = Self(2);
    pub const SCOPE_SUBGROUP_KHR: Self = Self(3);
    pub const SCOPE_QUEUE_FAMILY_KHR: Self = Self(5);
}

pub type ScopeNV = ScopeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SemaphoreType(pub i32);

impl SemaphoreType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BINARY: Self = Self(0);
    pub const TIMELINE: Self = Self(1);
}

pub type SemaphoreTypeKHR = SemaphoreType;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderCodeTypeEXT(pub i32);

impl ShaderCodeTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SHADER_CODE_TYPE_BINARY_EXT: Self = Self(0);
    pub const SHADER_CODE_TYPE_SPIRV_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderFloatControlsIndependence(pub i32);

impl ShaderFloatControlsIndependence {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _32_BIT_ONLY: Self = Self(0);
    pub const ALL: Self = Self(1);
    pub const NONE: Self = Self(2);
}

pub type ShaderFloatControlsIndependenceKHR = ShaderFloatControlsIndependence;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderGroupShaderKHR(pub i32);

impl ShaderGroupShaderKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SHADER_GROUP_SHADER_GENERAL_KHR: Self = Self(0);
    pub const SHADER_GROUP_SHADER_CLOSEST_HIT_KHR: Self = Self(1);
    pub const SHADER_GROUP_SHADER_ANY_HIT_KHR: Self = Self(2);
    pub const SHADER_GROUP_SHADER_INTERSECTION_KHR: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderInfoTypeAMD(pub i32);

impl ShaderInfoTypeAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SHADER_INFO_TYPE_STATISTICS_AMD: Self = Self(0);
    pub const SHADER_INFO_TYPE_BINARY_AMD: Self = Self(1);
    pub const SHADER_INFO_TYPE_DISASSEMBLY_AMD: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShadingRatePaletteEntryNV(pub i32);

impl ShadingRatePaletteEntryNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV: Self = Self(0);
    pub const SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV: Self = Self(1);
    pub const SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV: Self = Self(2);
    pub const SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV: Self = Self(3);
    pub const SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV: Self = Self(4);
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV: Self = Self(5);
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(6);
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(7);
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(8);
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(10);
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(11);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SharingMode(pub i32);

impl SharingMode {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const EXCLUSIVE: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StencilOp(pub i32);

impl StencilOp {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCREMENT_AND_CLAMP: Self = Self(3);
    pub const DECREMENT_AND_CLAMP: Self = Self(4);
    pub const INVERT: Self = Self(5);
    pub const INCREMENT_AND_WRAP: Self = Self(6);
    pub const DECREMENT_AND_WRAP: Self = Self(7);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StructureType(pub i32);

impl StructureType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const APPLICATION_INFO: Self = Self(0);
    pub const INSTANCE_CREATE_INFO: Self = Self(1);
    pub const DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
    pub const DEVICE_CREATE_INFO: Self = Self(3);
    pub const SUBMIT_INFO: Self = Self(4);
    pub const MEMORY_ALLOCATE_INFO: Self = Self(5);
    pub const MAPPED_MEMORY_RANGE: Self = Self(6);
    pub const BIND_SPARSE_INFO: Self = Self(7);
    pub const FENCE_CREATE_INFO: Self = Self(8);
    pub const SEMAPHORE_CREATE_INFO: Self = Self(9);
    pub const EVENT_CREATE_INFO: Self = Self(10);
    pub const QUERY_POOL_CREATE_INFO: Self = Self(11);
    pub const BUFFER_CREATE_INFO: Self = Self(12);
    pub const BUFFER_VIEW_CREATE_INFO: Self = Self(13);
    pub const IMAGE_CREATE_INFO: Self = Self(14);
    pub const IMAGE_VIEW_CREATE_INFO: Self = Self(15);
    pub const SHADER_MODULE_CREATE_INFO: Self = Self(16);
    pub const PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
    pub const SAMPLER_CREATE_INFO: Self = Self(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
    pub const WRITE_DESCRIPTOR_SET: Self = Self(35);
    pub const COPY_DESCRIPTOR_SET: Self = Self(36);
    pub const FRAMEBUFFER_CREATE_INFO: Self = Self(37);
    pub const RENDER_PASS_CREATE_INFO: Self = Self(38);
    pub const COMMAND_POOL_CREATE_INFO: Self = Self(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
    pub const RENDER_PASS_BEGIN_INFO: Self = Self(43);
    pub const BUFFER_MEMORY_BARRIER: Self = Self(44);
    pub const IMAGE_MEMORY_BARRIER: Self = Self(45);
    pub const MEMORY_BARRIER: Self = Self(46);
    pub const LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
    pub const LOADER_DEVICE_CREATE_INFO: Self = Self(48);
    pub const PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: Self = Self(49);
    pub const PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: Self = Self(50);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: Self = Self(51);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: Self = Self(52);
    pub const PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: Self = Self(53);
    pub const PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: Self = Self(54);
    pub const PHYSICAL_DEVICE_VULKAN_1_4_FEATURES: Self = Self(55);
    pub const PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES: Self = Self(56);
    pub const SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000001000);
    pub const PRESENT_INFO_KHR: Self = Self(1000001001);
    pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = Self(1000002000);
    pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = Self(1000002001);
    pub const DISPLAY_PRESENT_INFO_KHR: Self = Self(1000003000);
    pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = Self(1000004000);
    pub const XCB_SURFACE_CREATE_INFO_KHR: Self = Self(1000005000);
    pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = Self(1000006000);
    pub const ANDROID_SURFACE_CREATE_INFO_KHR: Self = Self(1000008000);
    pub const WIN32_SURFACE_CREATE_INFO_KHR: Self = Self(1000009000);
    pub const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: Self = Self(1000011000);
    pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self = Self(1000018000);
    pub const DEBUG_MARKER_OBJECT_NAME_INFO_EXT: Self = Self(1000022000);
    pub const DEBUG_MARKER_OBJECT_TAG_INFO_EXT: Self = Self(1000022001);
    pub const DEBUG_MARKER_MARKER_INFO_EXT: Self = Self(1000022002);
    pub const VIDEO_PROFILE_INFO_KHR: Self = Self(1000023000);
    pub const VIDEO_CAPABILITIES_KHR: Self = Self(1000023001);
    pub const VIDEO_PICTURE_RESOURCE_INFO_KHR: Self = Self(1000023002);
    pub const VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: Self = Self(1000023003);
    pub const BIND_VIDEO_SESSION_MEMORY_INFO_KHR: Self = Self(1000023004);
    pub const VIDEO_SESSION_CREATE_INFO_KHR: Self = Self(1000023005);
    pub const VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000023006);
    pub const VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR: Self = Self(1000023007);
    pub const VIDEO_BEGIN_CODING_INFO_KHR: Self = Self(1000023008);
    pub const VIDEO_END_CODING_INFO_KHR: Self = Self(1000023009);
    pub const VIDEO_CODING_CONTROL_INFO_KHR: Self = Self(1000023010);
    pub const VIDEO_REFERENCE_SLOT_INFO_KHR: Self = Self(1000023011);
    pub const QUEUE_FAMILY_VIDEO_PROPERTIES_KHR: Self = Self(1000023012);
    pub const VIDEO_PROFILE_LIST_INFO_KHR: Self = Self(1000023013);
    pub const PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR: Self = Self(1000023014);
    pub const VIDEO_FORMAT_PROPERTIES_KHR: Self = Self(1000023015);
    pub const QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR: Self = Self(1000023016);
    pub const VIDEO_DECODE_INFO_KHR: Self = Self(1000024000);
    pub const VIDEO_DECODE_CAPABILITIES_KHR: Self = Self(1000024001);
    pub const VIDEO_DECODE_USAGE_INFO_KHR: Self = Self(1000024002);
    pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = Self(1000026000);
    pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = Self(1000026001);
    pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000026002);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: Self = Self(1000028000);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: Self = Self(1000028001);
    pub const PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: Self = Self(1000028002);
    pub const CU_MODULE_CREATE_INFO_NVX: Self = Self(1000029000);
    pub const CU_FUNCTION_CREATE_INFO_NVX: Self = Self(1000029001);
    pub const CU_LAUNCH_INFO_NVX: Self = Self(1000029002);
    pub const CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX: Self = Self(1000029004);
    pub const IMAGE_VIEW_HANDLE_INFO_NVX: Self = Self(1000030000);
    pub const IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: Self = Self(1000030001);
    pub const VIDEO_ENCODE_H264_CAPABILITIES_KHR: Self = Self(1000038000);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000038001);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000038002);
    pub const VIDEO_ENCODE_H264_PICTURE_INFO_KHR: Self = Self(1000038003);
    pub const VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000038004);
    pub const VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR: Self = Self(1000038005);
    pub const VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000038006);
    pub const VIDEO_ENCODE_H264_PROFILE_INFO_KHR: Self = Self(1000038007);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR: Self = Self(1000038008);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000038009);
    pub const VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR: Self = Self(1000038010);
    pub const VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000038011);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000038012);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000038013);
    pub const VIDEO_ENCODE_H265_CAPABILITIES_KHR: Self = Self(1000039000);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000039001);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000039002);
    pub const VIDEO_ENCODE_H265_PICTURE_INFO_KHR: Self = Self(1000039003);
    pub const VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000039004);
    pub const VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR: Self = Self(1000039005);
    pub const VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000039006);
    pub const VIDEO_ENCODE_H265_PROFILE_INFO_KHR: Self = Self(1000039007);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR: Self = Self(1000039009);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000039010);
    pub const VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR: Self = Self(1000039011);
    pub const VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000039012);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000039013);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000039014);
    pub const VIDEO_DECODE_H264_CAPABILITIES_KHR: Self = Self(1000040000);
    pub const VIDEO_DECODE_H264_PICTURE_INFO_KHR: Self = Self(1000040001);
    pub const VIDEO_DECODE_H264_PROFILE_INFO_KHR: Self = Self(1000040003);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000040004);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000040005);
    pub const VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000040006);
    pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = Self(1000041000);
    pub const RENDERING_INFO: Self = Self(1000044000);
    pub const RENDERING_ATTACHMENT_INFO: Self = Self(1000044001);
    pub const PIPELINE_RENDERING_CREATE_INFO: Self = Self(1000044002);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: Self = Self(1000044003);
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: Self = Self(1000044004);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000044006);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: Self = Self(1000044007);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_AMD: Self = Self(1000044008);
    pub const MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: Self = Self(1000044009);
    pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: Self = Self(1000049000);
    pub const PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: Self = Self(1000050000);
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: Self = Self(1000053000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: Self = Self(1000053001);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: Self = Self(1000053002);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: Self = Self(1000056000);
    pub const EXPORT_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000056001);
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057001);
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: Self = Self(1000058000);
    pub const PHYSICAL_DEVICE_FEATURES_2: Self = Self(1000059000);
    pub const PHYSICAL_DEVICE_PROPERTIES_2: Self = Self(1000059001);
    pub const FORMAT_PROPERTIES_2: Self = Self(1000059002);
    pub const IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059003);
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: Self = Self(1000059004);
    pub const QUEUE_FAMILY_PROPERTIES_2: Self = Self(1000059005);
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: Self = Self(1000059006);
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059007);
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: Self = Self(1000059008);
    pub const MEMORY_ALLOCATE_FLAGS_INFO: Self = Self(1000060000);
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: Self = Self(1000060003);
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: Self = Self(1000060004);
    pub const DEVICE_GROUP_SUBMIT_INFO: Self = Self(1000060005);
    pub const DEVICE_GROUP_BIND_SPARSE_INFO: Self = Self(1000060006);
    pub const DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: Self = Self(1000060007);
    pub const IMAGE_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060008);
    pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: Self = Self(1000060009);
    pub const ACQUIRE_NEXT_IMAGE_INFO_KHR: Self = Self(1000060010);
    pub const DEVICE_GROUP_PRESENT_INFO_KHR: Self = Self(1000060011);
    pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060012);
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060013);
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060014);
    pub const VALIDATION_FLAGS_EXT: Self = Self(1000061000);
    pub const VI_SURFACE_CREATE_INFO_NN: Self = Self(1000062000);
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: Self = Self(1000063000);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: Self = Self(1000066000);
    pub const IMAGE_VIEW_ASTC_DECODE_MODE_EXT: Self = Self(1000067000);
    pub const PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: Self = Self(1000067001);
    pub const PIPELINE_ROBUSTNESS_CREATE_INFO: Self = Self(1000068000);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES: Self = Self(1000068001);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES: Self = Self(1000068002);
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES: Self = Self(1000070000);
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO: Self = Self(1000070001);
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: Self = Self(1000071000);
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: Self = Self(1000071001);
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: Self = Self(1000071002);
    pub const EXTERNAL_BUFFER_PROPERTIES: Self = Self(1000071003);
    pub const PHYSICAL_DEVICE_ID_PROPERTIES: Self = Self(1000071004);
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: Self = Self(1000072000);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = Self(1000072001);
    pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = Self(1000072002);
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073001);
    pub const MEMORY_WIN32_HANDLE_PROPERTIES_KHR: Self = Self(1000073002);
    pub const MEMORY_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000073003);
    pub const IMPORT_MEMORY_FD_INFO_KHR: Self = Self(1000074000);
    pub const MEMORY_FD_PROPERTIES_KHR: Self = Self(1000074001);
    pub const MEMORY_GET_FD_INFO_KHR: Self = Self(1000074002);
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: Self = Self(1000075000);
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: Self = Self(1000076000);
    pub const EXTERNAL_SEMAPHORE_PROPERTIES: Self = Self(1000076001);
    pub const EXPORT_SEMAPHORE_CREATE_INFO: Self = Self(1000077000);
    pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078000);
    pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078001);
    pub const D3D12_FENCE_SUBMIT_INFO_KHR: Self = Self(1000078002);
    pub const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000078003);
    pub const IMPORT_SEMAPHORE_FD_INFO_KHR: Self = Self(1000079000);
    pub const SEMAPHORE_GET_FD_INFO_KHR: Self = Self(1000079001);
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES: Self = Self(1000080000);
    pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: Self = Self(1000081000);
    pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: Self = Self(1000081001);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_EXT: Self = Self(1000081002);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: Self = Self(1000082000);
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: Self = Self(1000083000);
    pub const PRESENT_REGIONS_KHR: Self = Self(1000084000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: Self = Self(1000085000);
    pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: Self = Self(1000087000);
    pub const SURFACE_CAPABILITIES_2_EXT: Self = Self(1000090000);
    pub const DISPLAY_POWER_INFO_EXT: Self = Self(1000091000);
    pub const DEVICE_EVENT_INFO_EXT: Self = Self(1000091001);
    pub const DISPLAY_EVENT_INFO_EXT: Self = Self(1000091002);
    pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = Self(1000091003);
    pub const PRESENT_TIMES_INFO_GOOGLE: Self = Self(1000092000);
    pub const PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: Self = Self(1000094000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: Self = Self(1000097000);
    pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: Self = Self(1000098000);
    pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: Self = Self(1000099000);
    pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: Self = Self(1000099001);
    pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1000101000);
    pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: Self = Self(1000101001);
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: Self = Self(1000102000);
    pub const PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: Self = Self(1000102001);
    pub const HDR_METADATA_EXT: Self = Self(1000105000);
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: Self = Self(1000108000);
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: Self = Self(1000108001);
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: Self = Self(1000108002);
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO: Self = Self(1000108003);
    pub const ATTACHMENT_DESCRIPTION_2: Self = Self(1000109000);
    pub const ATTACHMENT_REFERENCE_2: Self = Self(1000109001);
    pub const SUBPASS_DESCRIPTION_2: Self = Self(1000109002);
    pub const SUBPASS_DEPENDENCY_2: Self = Self(1000109003);
    pub const RENDER_PASS_CREATE_INFO_2: Self = Self(1000109004);
    pub const SUBPASS_BEGIN_INFO: Self = Self(1000109005);
    pub const SUBPASS_END_INFO: Self = Self(1000109006);
    pub const PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG: Self = Self(1000110000);
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = Self(1000111000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: Self = Self(1000112000);
    pub const EXTERNAL_FENCE_PROPERTIES: Self = Self(1000112001);
    pub const EXPORT_FENCE_CREATE_INFO: Self = Self(1000113000);
    pub const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114000);
    pub const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114001);
    pub const FENCE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000114002);
    pub const IMPORT_FENCE_FD_INFO_KHR: Self = Self(1000115000);
    pub const FENCE_GET_FD_INFO_KHR: Self = Self(1000115001);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: Self = Self(1000116000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: Self = Self(1000116001);
    pub const QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: Self = Self(1000116002);
    pub const PERFORMANCE_QUERY_SUBMIT_INFO_KHR: Self = Self(1000116003);
    pub const ACQUIRE_PROFILING_LOCK_INFO_KHR: Self = Self(1000116004);
    pub const PERFORMANCE_COUNTER_KHR: Self = Self(1000116005);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_KHR: Self = Self(1000116006);
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: Self = Self(1000117000);
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: Self = Self(1000117001);
    pub const IMAGE_VIEW_USAGE_CREATE_INFO: Self = Self(1000117002);
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: Self = Self(1000117003);
    pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = Self(1000119000);
    pub const SURFACE_CAPABILITIES_2_KHR: Self = Self(1000119001);
    pub const SURFACE_FORMAT_2_KHR: Self = Self(1000119002);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: Self = Self(1000120000);
    pub const DISPLAY_PROPERTIES_2_KHR: Self = Self(1000121000);
    pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = Self(1000121001);
    pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = Self(1000121002);
    pub const DISPLAY_PLANE_INFO_2_KHR: Self = Self(1000121003);
    pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = Self(1000121004);
    pub const IOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000122000);
    pub const MACOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000123000);
    pub const MEMORY_DEDICATED_REQUIREMENTS: Self = Self(1000127000);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO: Self = Self(1000127001);
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: Self = Self(1000128000);
    pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: Self = Self(1000128001);
    pub const DEBUG_UTILS_LABEL_EXT: Self = Self(1000128002);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: Self = Self(1000128003);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = Self(1000128004);
    pub const ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: Self = Self(1000129000);
    pub const ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: Self = Self(1000129001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: Self = Self(1000129002);
    pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129003);
    pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129004);
    pub const EXTERNAL_FORMAT_ANDROID: Self = Self(1000129005);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: Self = Self(1000129006);
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: Self = Self(1000130000);
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO: Self = Self(1000130001);
    #[cfg(feature = "beta")]
    pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX: Self = Self(1000134000);
    #[cfg(feature = "beta")]
    pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX: Self = Self(1000134001);
    #[cfg(feature = "beta")]
    pub const EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX: Self = Self(1000134002);
    #[cfg(feature = "beta")]
    pub const EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX: Self = Self(1000134003);
    #[cfg(feature = "beta")]
    pub const PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX: Self = Self(1000134004);
    pub const TEXEL_BUFFER_DESCRIPTOR_INFO_EXT: Self = Self(1000135000);
    pub const IMAGE_DESCRIPTOR_INFO_EXT: Self = Self(1000135001);
    pub const RESOURCE_DESCRIPTOR_INFO_EXT: Self = Self(1000135002);
    pub const BIND_HEAP_INFO_EXT: Self = Self(1000135003);
    pub const PUSH_DATA_INFO_EXT: Self = Self(1000135004);
    pub const DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT: Self = Self(1000135005);
    pub const SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT: Self = Self(1000135006);
    pub const OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT: Self = Self(1000135007);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT: Self = Self(1000135008);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT: Self = Self(1000135009);
    pub const COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT: Self = Self(1000135010);
    pub const SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT: Self = Self(1000135011);
    pub const INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV: Self = Self(1000135012);
    pub const SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000135013);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM: Self = Self(1000135014);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: Self = Self(1000138000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: Self = Self(1000138001);
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: Self = Self(1000138002);
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: Self = Self(1000138003);
    pub const PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR: Self = Self(1000141000);
    pub const SAMPLE_LOCATIONS_INFO_EXT: Self = Self(1000143000);
    pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: Self = Self(1000143001);
    pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: Self = Self(1000143002);
    pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: Self = Self(1000143003);
    pub const MULTISAMPLE_PROPERTIES_EXT: Self = Self(1000143004);
    pub const PROTECTED_SUBMIT_INFO: Self = Self(1000145000);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: Self = Self(1000145001);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: Self = Self(1000145002);
    pub const DEVICE_QUEUE_INFO_2: Self = Self(1000145003);
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146000);
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146001);
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146002);
    pub const MEMORY_REQUIREMENTS_2: Self = Self(1000146003);
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: Self = Self(1000146004);
    pub const IMAGE_FORMAT_LIST_CREATE_INFO: Self = Self(1000147000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self = Self(1000148000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self = Self(1000148001);
    pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = Self(1000148002);
    pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = Self(1000149000);
    pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000150002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: Self = Self(1000150003);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: Self = Self(1000150004);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: Self = Self(1000150005);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_KHR: Self = Self(1000150006);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: Self = Self(1000150007);
    pub const ACCELERATION_STRUCTURE_VERSION_INFO_KHR: Self = Self(1000150009);
    pub const COPY_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150010);
    pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: Self = Self(1000150011);
    pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150012);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: Self = Self(1000150013);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: Self = Self(1000150014);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_KHR: Self = Self(1000150015);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: Self = Self(1000150016);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_KHR: Self = Self(1000150017);
    pub const RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: Self = Self(1000150018);
    pub const ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: Self = Self(1000150020);
    pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = Self(1000152000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: Self = Self(1000154000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: Self = Self(1000154001);
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: Self = Self(1000156000);
    pub const SAMPLER_YCBCR_CONVERSION_INFO: Self = Self(1000156001);
    pub const BIND_IMAGE_PLANE_MEMORY_INFO: Self = Self(1000156002);
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: Self = Self(1000156003);
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: Self = Self(1000156004);
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: Self = Self(1000156005);
    pub const BIND_BUFFER_MEMORY_INFO: Self = Self(1000157000);
    pub const BIND_IMAGE_MEMORY_INFO: Self = Self(1000157001);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: Self = Self(1000158000);
    pub const PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: Self = Self(1000158002);
    pub const IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: Self = Self(1000158003);
    pub const IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: Self = Self(1000158004);
    pub const IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: Self = Self(1000158005);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: Self = Self(1000158006);
    pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160000);
    pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160001);
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: Self = Self(1000161000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: Self = Self(1000161001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: Self = Self(1000161002);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: Self = Self(1000161003);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: Self = Self(1000161004);
    #[cfg(feature = "beta")]
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: Self = Self(1000163000);
    #[cfg(feature = "beta")]
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: Self = Self(1000163001);
    pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: Self = Self(1000164000);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: Self = Self(1000164001);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: Self = Self(1000164002);
    pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: Self = Self(1000164005);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_NV: Self = Self(1000165000);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self = Self(1000165001);
    pub const GEOMETRY_NV: Self = Self(1000165003);
    pub const GEOMETRY_TRIANGLES_NV: Self = Self(1000165004);
    pub const GEOMETRY_AABB_NV: Self = Self(1000165005);
    pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: Self = Self(1000165006);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: Self = Self(1000165007);
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000165008);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: Self = Self(1000165009);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000165011);
    pub const ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000165012);
    pub const PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: Self = Self(1000166000);
    pub const PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: Self = Self(1000166001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: Self = Self(1000168000);
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT: Self = Self(1000168001);
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: Self = Self(1000170000);
    pub const FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000170001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM: Self = Self(1000172000);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO: Self = Self(1000174000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: Self = Self(1000175000);
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: Self = Self(1000177000);
    pub const IMPORT_MEMORY_HOST_POINTER_INFO_EXT: Self = Self(1000178000);
    pub const MEMORY_HOST_POINTER_PROPERTIES_EXT: Self = Self(1000178001);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: Self = Self(1000178002);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: Self = Self(1000180000);
    pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: Self = Self(1000181000);
    pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: Self = Self(1000183000);
    pub const CALIBRATED_TIMESTAMP_INFO_KHR: Self = Self(1000184000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: Self = Self(1000185000);
    pub const VIDEO_DECODE_H265_CAPABILITIES_KHR: Self = Self(1000187000);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000187001);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000187002);
    pub const VIDEO_DECODE_H265_PROFILE_INFO_KHR: Self = Self(1000187003);
    pub const VIDEO_DECODE_H265_PICTURE_INFO_KHR: Self = Self(1000187004);
    pub const VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000187005);
    pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: Self = Self(1000189000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: Self = Self(1000190000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO: Self = Self(1000190001);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES: Self = Self(1000190002);
    pub const PRESENT_FRAME_TOKEN_GGP: Self = Self(1000191000);
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO: Self = Self(1000192000);
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES: Self = Self(1000196000);
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: Self = Self(1000197000);
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: Self = Self(1000199000);
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: Self = Self(1000199001);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR: Self = Self(1000201000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: Self = Self(1000202000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: Self = Self(1000202001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR: Self = Self(1000203000);
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: Self = Self(1000204000);
    pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: Self = Self(1000205000);
    pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: Self = Self(1000205002);
    pub const CHECKPOINT_DATA_NV: Self = Self(1000206000);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: Self = Self(1000206001);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: Self = Self(1000207000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: Self = Self(1000207001);
    pub const SEMAPHORE_TYPE_CREATE_INFO: Self = Self(1000207002);
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO: Self = Self(1000207003);
    pub const SEMAPHORE_WAIT_INFO: Self = Self(1000207004);
    pub const SEMAPHORE_SIGNAL_INFO: Self = Self(1000207005);
    pub const PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT: Self = Self(1000208000);
    pub const SWAPCHAIN_TIMING_PROPERTIES_EXT: Self = Self(1000208001);
    pub const SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT: Self = Self(1000208002);
    pub const PRESENT_TIMINGS_INFO_EXT: Self = Self(1000208003);
    pub const PRESENT_TIMING_INFO_EXT: Self = Self(1000208004);
    pub const PAST_PRESENTATION_TIMING_INFO_EXT: Self = Self(1000208005);
    pub const PAST_PRESENTATION_TIMING_PROPERTIES_EXT: Self = Self(1000208006);
    pub const PAST_PRESENTATION_TIMING_EXT: Self = Self(1000208007);
    pub const PRESENT_TIMING_SURFACE_CAPABILITIES_EXT: Self = Self(1000208008);
    pub const SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000208009);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: Self = Self(1000209000);
    pub const QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: Self = Self(1000210000);
    pub const INITIALIZE_PERFORMANCE_API_INFO_INTEL: Self = Self(1000210001);
    pub const PERFORMANCE_MARKER_INFO_INTEL: Self = Self(1000210002);
    pub const PERFORMANCE_STREAM_MARKER_INFO_INTEL: Self = Self(1000210003);
    pub const PERFORMANCE_OVERRIDE_INFO_INTEL: Self = Self(1000210004);
    pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: Self = Self(1000210005);
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: Self = Self(1000211000);
    pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: Self = Self(1000212000);
    pub const DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: Self = Self(1000213000);
    pub const SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: Self = Self(1000213001);
    pub const IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: Self = Self(1000214000);
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: Self = Self(1000215000);
    pub const METAL_SURFACE_CREATE_INFO_EXT: Self = Self(1000217000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: Self = Self(1000218000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000218001);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: Self = Self(1000218002);
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: Self = Self(1000221000);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: Self = Self(1000225000);
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: Self = Self(1000225001);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: Self = Self(1000225002);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000226000);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: Self = Self(1000226001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: Self = Self(1000226002);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: Self = Self(1000226003);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226004);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: Self = Self(1000227000);
    pub const PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: Self = Self(1000229000);
    pub const PHYSICAL_DEVICE_SHADER_CONSTANT_DATA_FEATURES_KHR: Self = Self(1000231000);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES: Self = Self(1000232000);
    pub const RENDERING_ATTACHMENT_LOCATION_INFO: Self = Self(1000232001);
    pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO: Self = Self(1000232002);
    pub const PHYSICAL_DEVICE_SHADER_ABORT_FEATURES_KHR: Self = Self(1000233000);
    pub const DEVICE_FAULT_SHADER_ABORT_MESSAGE_INFO_KHR: Self = Self(1000233001);
    pub const PHYSICAL_DEVICE_SHADER_ABORT_PROPERTIES_KHR: Self = Self(1000233002);
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: Self = Self(1000234000);
    pub const PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR: Self = Self(1000235000);
    pub const PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: Self = Self(1000237000);
    pub const PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: Self = Self(1000238000);
    pub const MEMORY_PRIORITY_ALLOCATE_INFO_EXT: Self = Self(1000238001);
    pub const SURFACE_PROTECTED_CAPABILITIES_KHR: Self = Self(1000239000);
    pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: Self =
        Self(1000240000);
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: Self = Self(1000241000);
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT: Self = Self(1000241001);
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: Self = Self(1000241002);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: Self = Self(1000244000);
    pub const BUFFER_DEVICE_ADDRESS_INFO: Self = Self(1000244001);
    pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: Self = Self(1000244002);
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES: Self = Self(1000245000);
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO: Self = Self(1000246000);
    pub const VALIDATION_FEATURES_EXT: Self = Self(1000247000);
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: Self = Self(1000248000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: Self = Self(1000249000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249002);
    pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: Self = Self(1000250000);
    pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: Self = Self(1000250001);
    pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: Self = Self(1000250002);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: Self = Self(1000251000);
    pub const PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: Self = Self(1000252000);
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: Self = Self(1000253000);
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: Self = Self(1000254000);
    pub const PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: Self =
        Self(1000254001);
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: Self = Self(1000254002);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: Self = Self(1000255000);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: Self = Self(1000255001);
    pub const SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: Self = Self(1000255002);
    pub const HEADLESS_SURFACE_CREATE_INFO_EXT: Self = Self(1000256000);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: Self = Self(1000257000);
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: Self = Self(1000257002);
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: Self = Self(1000257003);
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: Self = Self(1000257004);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES: Self = Self(1000259000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO: Self = Self(1000259001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES: Self = Self(1000259002);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: Self = Self(1000260000);
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: Self = Self(1000261000);
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES: Self = Self(1000265000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000267000);
    pub const PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: Self = Self(1000269000);
    pub const PIPELINE_INFO_KHR: Self = Self(1000269001);
    pub const PIPELINE_EXECUTABLE_PROPERTIES_KHR: Self = Self(1000269002);
    pub const PIPELINE_EXECUTABLE_INFO_KHR: Self = Self(1000269003);
    pub const PIPELINE_EXECUTABLE_STATISTIC_KHR: Self = Self(1000269004);
    pub const PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: Self = Self(1000269005);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES: Self = Self(1000270000);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES: Self = Self(1000270001);
    pub const MEMORY_TO_IMAGE_COPY: Self = Self(1000270002);
    pub const IMAGE_TO_MEMORY_COPY: Self = Self(1000270003);
    pub const COPY_IMAGE_TO_MEMORY_INFO: Self = Self(1000270004);
    pub const COPY_MEMORY_TO_IMAGE_INFO: Self = Self(1000270005);
    pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO: Self = Self(1000270006);
    pub const COPY_IMAGE_TO_IMAGE_INFO: Self = Self(1000270007);
    pub const SUBRESOURCE_HOST_MEMCPY_SIZE: Self = Self(1000270008);
    pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY: Self = Self(1000270009);
    pub const MEMORY_MAP_INFO: Self = Self(1000271000);
    pub const MEMORY_UNMAP_INFO: Self = Self(1000271001);
    pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT: Self = Self(1000272000);
    pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT: Self = Self(1000272001);
    pub const MEMORY_MAP_PLACED_INFO_EXT: Self = Self(1000272002);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: Self = Self(1000273000);
    pub const SURFACE_PRESENT_MODE_KHR: Self = Self(1000274000);
    pub const SURFACE_PRESENT_SCALING_CAPABILITIES_KHR: Self = Self(1000274001);
    pub const SURFACE_PRESENT_MODE_COMPATIBILITY_KHR: Self = Self(1000274002);
    pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000275000);
    pub const SWAPCHAIN_PRESENT_FENCE_INFO_KHR: Self = Self(1000275001);
    pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR: Self = Self(1000275002);
    pub const SWAPCHAIN_PRESENT_MODE_INFO_KHR: Self = Self(1000275003);
    pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR: Self = Self(1000275004);
    pub const RELEASE_SWAPCHAIN_IMAGES_INFO_KHR: Self = Self(1000275005);
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: Self = Self(1000276000);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV: Self = Self(1000277000);
    pub const GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000277001);
    pub const GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV: Self = Self(1000277002);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: Self = Self(1000277003);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: Self = Self(1000277004);
    pub const GENERATED_COMMANDS_INFO_NV: Self = Self(1000277005);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000277006);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV: Self = Self(1000277007);
    pub const PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: Self = Self(1000278000);
    pub const COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: Self = Self(1000278001);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: Self = Self(1000280000);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: Self = Self(1000280001);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: Self = Self(1000281000);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: Self = Self(1000281001);
    pub const COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM: Self = Self(1000282000);
    pub const RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: Self = Self(1000282001);
    pub const PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT: Self = Self(1000283000);
    pub const DEPTH_BIAS_INFO_EXT: Self = Self(1000283001);
    pub const DEPTH_BIAS_REPRESENTATION_INFO_EXT: Self = Self(1000283002);
    pub const PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: Self = Self(1000284000);
    pub const DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: Self = Self(1000284001);
    pub const DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: Self = Self(1000284002);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR: Self = Self(1000286000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR: Self = Self(1000286001);
    pub const SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: Self = Self(1000287000);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT: Self = Self(1000287001);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT: Self = Self(1000287002);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT: Self = Self(1000288000);
    pub const PIPELINE_LIBRARY_CREATE_INFO_KHR: Self = Self(1000290000);
    pub const PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV: Self = Self(1000292000);
    pub const SURFACE_CAPABILITIES_PRESENT_BARRIER_NV: Self = Self(1000292001);
    pub const SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV: Self = Self(1000292002);
    pub const PRESENT_ID_KHR: Self = Self(1000294000);
    pub const PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: Self = Self(1000294001);
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: Self = Self(1000295000);
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO: Self = Self(1000295001);
    pub const PRIVATE_DATA_SLOT_CREATE_INFO: Self = Self(1000295002);
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: Self = Self(1000297000);
    pub const VIDEO_ENCODE_INFO_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: Self = Self(1000299001);
    pub const VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000299002);
    pub const VIDEO_ENCODE_CAPABILITIES_KHR: Self = Self(1000299003);
    pub const VIDEO_ENCODE_USAGE_INFO_KHR: Self = Self(1000299004);
    pub const QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR: Self = Self(1000299005);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299006);
    pub const VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000299007);
    pub const VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299008);
    pub const VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000299009);
    pub const VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000299010);
    pub const PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV: Self = Self(1000300000);
    pub const DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: Self = Self(1000300001);
    pub const PERF_HINT_INFO_QCOM: Self = Self(1000302000);
    pub const PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM: Self = Self(1000302001);
    pub const PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM: Self = Self(1000302002);
    #[cfg(feature = "beta")]
    pub const CUDA_MODULE_CREATE_INFO_NV: Self = Self(1000307000);
    #[cfg(feature = "beta")]
    pub const CUDA_FUNCTION_CREATE_INFO_NV: Self = Self(1000307001);
    #[cfg(feature = "beta")]
    pub const CUDA_LAUNCH_INFO_NV: Self = Self(1000307002);
    #[cfg(feature = "beta")]
    pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV: Self = Self(1000307003);
    #[cfg(feature = "beta")]
    pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV: Self = Self(1000307004);
    pub const PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM: Self = Self(1000309000);
    pub const PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM: Self = Self(1000309001);
    pub const RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM: Self = Self(1000309002);
    pub const PER_TILE_BEGIN_INFO_QCOM: Self = Self(1000309003);
    pub const PER_TILE_END_INFO_QCOM: Self = Self(1000309004);
    pub const DISPATCH_TILE_INFO_QCOM: Self = Self(1000309005);
    pub const QUERY_LOW_LATENCY_SUPPORT_NV: Self = Self(1000310000);
    pub const EXPORT_METAL_OBJECT_CREATE_INFO_EXT: Self = Self(1000311000);
    pub const EXPORT_METAL_OBJECTS_INFO_EXT: Self = Self(1000311001);
    pub const EXPORT_METAL_DEVICE_INFO_EXT: Self = Self(1000311002);
    pub const EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: Self = Self(1000311003);
    pub const EXPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311004);
    pub const IMPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311005);
    pub const EXPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311006);
    pub const IMPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311007);
    pub const EXPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311008);
    pub const IMPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311009);
    pub const EXPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311010);
    pub const IMPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311011);
    pub const MEMORY_BARRIER_2: Self = Self(1000314000);
    pub const BUFFER_MEMORY_BARRIER_2: Self = Self(1000314001);
    pub const IMAGE_MEMORY_BARRIER_2: Self = Self(1000314002);
    pub const DEPENDENCY_INFO: Self = Self(1000314003);
    pub const SUBMIT_INFO_2: Self = Self(1000314004);
    pub const SEMAPHORE_SUBMIT_INFO: Self = Self(1000314005);
    pub const COMMAND_BUFFER_SUBMIT_INFO: Self = Self(1000314006);
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: Self = Self(1000314007);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: Self = Self(1000314008);
    pub const CHECKPOINT_DATA_2_NV: Self = Self(1000314009);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT: Self = Self(1000316000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000316001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT: Self = Self(1000316002);
    pub const DESCRIPTOR_ADDRESS_INFO_EXT: Self = Self(1000316003);
    pub const DESCRIPTOR_GET_INFO_EXT: Self = Self(1000316004);
    pub const BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316005);
    pub const IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316006);
    pub const IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316007);
    pub const SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316008);
    pub const ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316009);
    pub const OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT: Self = Self(1000316010);
    pub const DESCRIPTOR_BUFFER_BINDING_INFO_EXT: Self = Self(1000316011);
    pub const DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT: Self = Self(1000316012);
    pub const DEVICE_MEMORY_COPY_KHR: Self = Self(1000318000);
    pub const COPY_DEVICE_MEMORY_INFO_KHR: Self = Self(1000318001);
    pub const DEVICE_MEMORY_IMAGE_COPY_KHR: Self = Self(1000318002);
    pub const COPY_DEVICE_MEMORY_IMAGE_INFO_KHR: Self = Self(1000318003);
    pub const MEMORY_RANGE_BARRIERS_INFO_KHR: Self = Self(1000318004);
    pub const MEMORY_RANGE_BARRIER_KHR: Self = Self(1000318005);
    pub const PHYSICAL_DEVICE_DEVICE_ADDRESS_COMMANDS_FEATURES_KHR: Self = Self(1000318006);
    pub const BIND_INDEX_BUFFER_3_INFO_KHR: Self = Self(1000318007);
    pub const BIND_VERTEX_BUFFER_3_INFO_KHR: Self = Self(1000318008);
    pub const DRAW_INDIRECT_2_INFO_KHR: Self = Self(1000318009);
    pub const DRAW_INDIRECT_COUNT_2_INFO_KHR: Self = Self(1000318010);
    pub const DISPATCH_INDIRECT_2_INFO_KHR: Self = Self(1000318011);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_2_EXT: Self = Self(1000318012);
    pub const BIND_TRANSFORM_FEEDBACK_BUFFER_2_INFO_EXT: Self = Self(1000318013);
    pub const MEMORY_MARKER_INFO_AMD: Self = Self(1000318014);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_2_KHR: Self = Self(1000318015);
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: Self = Self(1000320000);
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: Self = Self(1000320001);
    pub const GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: Self = Self(1000320002);
    pub const PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD: Self =
        Self(1000321000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR: Self = Self(1000322000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: Self =
        Self(1000323000);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: Self = Self(1000325000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV: Self = Self(1000326000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV: Self = Self(1000326001);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV: Self = Self(1000326002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: Self = Self(1000327000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: Self = Self(1000327001);
    pub const ACCELERATION_STRUCTURE_MOTION_INFO_NV: Self = Self(1000327002);
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT: Self = Self(1000328000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT: Self = Self(1000328001);
    pub const PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: Self = Self(1000330000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT: Self = Self(1000332000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT: Self = Self(1000332001);
    pub const COPY_COMMAND_TRANSFORM_INFO_QCOM: Self = Self(1000333000);
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: Self = Self(1000335000);
    pub const PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: Self =
        Self(1000336000);
    pub const COPY_BUFFER_INFO_2: Self = Self(1000337000);
    pub const COPY_IMAGE_INFO_2: Self = Self(1000337001);
    pub const COPY_BUFFER_TO_IMAGE_INFO_2: Self = Self(1000337002);
    pub const COPY_IMAGE_TO_BUFFER_INFO_2: Self = Self(1000337003);
    pub const BLIT_IMAGE_INFO_2: Self = Self(1000337004);
    pub const RESOLVE_IMAGE_INFO_2: Self = Self(1000337005);
    pub const BUFFER_COPY_2: Self = Self(1000337006);
    pub const IMAGE_COPY_2: Self = Self(1000337007);
    pub const IMAGE_BLIT_2: Self = Self(1000337008);
    pub const BUFFER_IMAGE_COPY_2: Self = Self(1000337009);
    pub const IMAGE_RESOLVE_2: Self = Self(1000337010);
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT: Self = Self(1000338000);
    pub const IMAGE_COMPRESSION_CONTROL_EXT: Self = Self(1000338001);
    pub const SUBRESOURCE_LAYOUT_2: Self = Self(1000338002);
    pub const IMAGE_SUBRESOURCE_2: Self = Self(1000338003);
    pub const IMAGE_COMPRESSION_PROPERTIES_EXT: Self = Self(1000338004);
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT: Self = Self(1000339000);
    pub const PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: Self = Self(1000340000);
    pub const PHYSICAL_DEVICE_FAULT_FEATURES_EXT: Self = Self(1000341000);
    pub const DEVICE_FAULT_COUNTS_EXT: Self = Self(1000341001);
    pub const DEVICE_FAULT_INFO_EXT: Self = Self(1000341002);
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT: Self =
        Self(1000342000);
    pub const PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: Self = Self(1000344000);
    pub const DIRECTFB_SURFACE_CREATE_INFO_EXT: Self = Self(1000346000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR: Self = Self(1000347000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR: Self = Self(1000347001);
    pub const PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: Self = Self(1000348013);
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT: Self = Self(1000351000);
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT: Self = Self(1000351002);
    pub const PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000352000);
    pub const VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: Self = Self(1000352001);
    pub const VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: Self = Self(1000352002);
    pub const PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: Self = Self(1000353000);
    pub const PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT: Self = Self(1000354000);
    pub const DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT: Self = Self(1000354001);
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: Self = Self(1000355000);
    pub const PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: Self = Self(1000355001);
    pub const PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: Self = Self(1000356000);
    pub const FORMAT_PROPERTIES_3: Self = Self(1000360000);
    pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR: Self = Self(1000361000);
    pub const IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364000);
    pub const MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: Self = Self(1000364001);
    pub const MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364002);
    pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365000);
    pub const SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365001);
    pub const BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: Self = Self(1000366000);
    pub const IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366001);
    pub const BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: Self = Self(1000366002);
    pub const BUFFER_COLLECTION_PROPERTIES_FUCHSIA: Self = Self(1000366003);
    pub const BUFFER_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366004);
    pub const BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: Self = Self(1000366005);
    pub const IMAGE_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366006);
    pub const IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366007);
    pub const SYSMEM_COLOR_SPACE_FUCHSIA: Self = Self(1000366008);
    pub const BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366009);
    pub const SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: Self = Self(1000369000);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: Self = Self(1000369001);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: Self = Self(1000369002);
    pub const PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: Self = Self(1000370000);
    pub const MEMORY_GET_REMOTE_ADDRESS_INFO_NV: Self = Self(1000371000);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: Self = Self(1000371001);
    pub const PIPELINE_PROPERTIES_IDENTIFIER_EXT: Self = Self(1000372000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT: Self = Self(1000372001);
    pub const PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT: Self = Self(1000375000);
    pub const FRAME_BOUNDARY_EXT: Self = Self(1000375001);
    pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT: Self =
        Self(1000376000);
    pub const SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT: Self = Self(1000376001);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT: Self = Self(1000376002);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(1000377000);
    pub const SCREEN_SURFACE_CREATE_INFO_QNX: Self = Self(1000378000);
    pub const PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: Self = Self(1000381000);
    pub const PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: Self = Self(1000381001);
    pub const PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT: Self = Self(1000382000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000386000);
    pub const PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR: Self = Self(1000387000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES: Self = Self(1000388000);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES: Self = Self(1000388001);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE: Self = Self(1000390000);
    pub const VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE: Self = Self(1000390001);
    pub const VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE: Self = Self(1000390002);
    pub const VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE: Self = Self(1000390003);
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: Self = Self(1000391000);
    pub const IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: Self = Self(1000391001);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: Self = Self(1000392000);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: Self = Self(1000392001);
    pub const PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000393000);
    pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT: Self = Self(1000395000);
    pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT: Self = Self(1000395001);
    pub const MICROMAP_BUILD_INFO_EXT: Self = Self(1000396000);
    pub const MICROMAP_VERSION_INFO_EXT: Self = Self(1000396001);
    pub const COPY_MICROMAP_INFO_EXT: Self = Self(1000396002);
    pub const COPY_MICROMAP_TO_MEMORY_INFO_EXT: Self = Self(1000396003);
    pub const COPY_MEMORY_TO_MICROMAP_INFO_EXT: Self = Self(1000396004);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT: Self = Self(1000396005);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT: Self = Self(1000396006);
    pub const MICROMAP_CREATE_INFO_EXT: Self = Self(1000396007);
    pub const MICROMAP_BUILD_SIZES_INFO_EXT: Self = Self(1000396008);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT: Self = Self(1000396009);
    #[cfg(feature = "beta")]
    pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV: Self = Self(1000397000);
    #[cfg(feature = "beta")]
    pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV: Self = Self(1000397001);
    #[cfg(feature = "beta")]
    pub const ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV: Self = Self(1000397002);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI: Self = Self(1000404000);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI: Self = Self(1000404001);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI: Self = Self(1000404002);
    pub const PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: Self = Self(1000411000);
    pub const SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: Self = Self(1000411001);
    pub const PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: Self = Self(1000412000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: Self = Self(1000413000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: Self = Self(1000413001);
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS: Self = Self(1000413002);
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS: Self = Self(1000413003);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM: Self = Self(1000415000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES: Self = Self(1000416000);
    pub const DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM: Self = Self(1000417000);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM: Self = Self(1000417001);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM: Self = Self(1000417002);
    pub const DISPATCH_PARAMETERS_ARM: Self = Self(1000417003);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM: Self =
        Self(1000417004);
    pub const PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000418000);
    pub const IMAGE_VIEW_SLICED_CREATE_INFO_EXT: Self = Self(1000418001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE: Self = Self(1000420000);
    pub const DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: Self = Self(1000420001);
    pub const DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: Self = Self(1000420002);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR: Self = Self(1000421000);
    pub const PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT: Self = Self(1000422000);
    pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM: Self = Self(1000424000);
    pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM: Self = Self(1000424001);
    pub const RENDER_PASS_STRIPE_BEGIN_INFO_ARM: Self = Self(1000424002);
    pub const RENDER_PASS_STRIPE_INFO_ARM: Self = Self(1000424003);
    pub const RENDER_PASS_STRIPE_SUBMIT_INFO_ARM: Self = Self(1000424004);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT: Self = Self(1000425000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT: Self = Self(1000425001);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT: Self = Self(1000425002);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV: Self = Self(1000426000);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR: Self = Self(1000426001);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT: Self = Self(1000427000);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT: Self = Self(1000427001);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV: Self =
        Self(1000428000);
    pub const COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV: Self = Self(1000428001);
    pub const PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV: Self = Self(1000428002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV: Self = Self(1000429008);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV: Self = Self(1000429009);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV: Self = Self(1000429010);
    pub const PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: Self = Self(1000430000);
    pub const PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR: Self = Self(1000434000);
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT: Self =
        Self(1000437000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM: Self = Self(1000440000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM: Self = Self(1000440001);
    pub const IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM: Self = Self(1000440002);
    pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT: Self = Self(1000451000);
    pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT: Self = Self(1000451001);
    pub const NATIVE_BUFFER_USAGE_OHOS: Self = Self(1000452000);
    pub const NATIVE_BUFFER_PROPERTIES_OHOS: Self = Self(1000452001);
    pub const NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS: Self = Self(1000452002);
    pub const IMPORT_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452003);
    pub const MEMORY_GET_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452004);
    pub const EXTERNAL_FORMAT_OHOS: Self = Self(1000452005);
    pub const EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT: Self = Self(1000453000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT: Self = Self(1000455000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT: Self = Self(1000455001);
    pub const PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT: Self = Self(1000458000);
    pub const RENDER_PASS_CREATION_CONTROL_EXT: Self = Self(1000458001);
    pub const RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458002);
    pub const RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458003);
    pub const DIRECT_DRIVER_LOADING_INFO_LUNARG: Self = Self(1000459000);
    pub const DIRECT_DRIVER_LOADING_LIST_LUNARG: Self = Self(1000459001);
    pub const TENSOR_CREATE_INFO_ARM: Self = Self(1000460000);
    pub const TENSOR_VIEW_CREATE_INFO_ARM: Self = Self(1000460001);
    pub const BIND_TENSOR_MEMORY_INFO_ARM: Self = Self(1000460002);
    pub const WRITE_DESCRIPTOR_SET_TENSOR_ARM: Self = Self(1000460003);
    pub const PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM: Self = Self(1000460004);
    pub const TENSOR_FORMAT_PROPERTIES_ARM: Self = Self(1000460005);
    pub const TENSOR_DESCRIPTION_ARM: Self = Self(1000460006);
    pub const TENSOR_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000460007);
    pub const TENSOR_MEMORY_BARRIER_ARM: Self = Self(1000460008);
    pub const PHYSICAL_DEVICE_TENSOR_FEATURES_ARM: Self = Self(1000460009);
    pub const DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM: Self = Self(1000460010);
    pub const COPY_TENSOR_INFO_ARM: Self = Self(1000460011);
    pub const TENSOR_COPY_ARM: Self = Self(1000460012);
    pub const TENSOR_DEPENDENCY_INFO_ARM: Self = Self(1000460013);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM: Self = Self(1000460014);
    pub const PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM: Self = Self(1000460015);
    pub const EXTERNAL_TENSOR_PROPERTIES_ARM: Self = Self(1000460016);
    pub const EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM: Self = Self(1000460017);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM: Self = Self(1000460018);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM: Self = Self(1000460019);
    pub const DESCRIPTOR_GET_TENSOR_INFO_ARM: Self = Self(1000460020);
    pub const TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460021);
    pub const TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460022);
    pub const FRAME_BOUNDARY_TENSORS_ARM: Self = Self(1000460023);
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT: Self = Self(1000462000);
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT: Self = Self(1000462001);
    pub const PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT: Self = Self(1000462002);
    pub const SHADER_MODULE_IDENTIFIER_EXT: Self = Self(1000462003);
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV: Self = Self(1000464000);
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV: Self = Self(1000464001);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV: Self = Self(1000464002);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV: Self = Self(1000464003);
    pub const OPTICAL_FLOW_SESSION_CREATE_INFO_NV: Self = Self(1000464004);
    pub const OPTICAL_FLOW_EXECUTE_INFO_NV: Self = Self(1000464005);
    pub const OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV: Self = Self(1000464010);
    pub const PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT: Self = Self(1000465000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES: Self = Self(1000466000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID: Self = Self(1000468000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1000468001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1000468002);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES: Self = Self(1000470000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES: Self = Self(1000470001);
    pub const RENDERING_AREA_INFO: Self = Self(1000470003);
    pub const DEVICE_IMAGE_SUBRESOURCE_INFO: Self = Self(1000470004);
    pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO: Self = Self(1000470005);
    pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO: Self = Self(1000470006);
    pub const PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD: Self = Self(1000476000);
    pub const ANTI_LAG_DATA_AMD: Self = Self(1000476001);
    pub const ANTI_LAG_PRESENTATION_INFO_AMD: Self = Self(1000476002);
    #[cfg(feature = "beta")]
    pub const PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX: Self = Self(1000478000);
    #[cfg(feature = "beta")]
    pub const ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX: Self =
        Self(1000478001);
    pub const SURFACE_CAPABILITIES_PRESENT_ID_2_KHR: Self = Self(1000479000);
    pub const PRESENT_ID_2_KHR: Self = Self(1000479001);
    pub const PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR: Self = Self(1000479002);
    pub const SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR: Self = Self(1000480000);
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR: Self = Self(1000480001);
    pub const PRESENT_WAIT_2_INFO_KHR: Self = Self(1000480002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR: Self = Self(1000481000);
    pub const PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT: Self = Self(1000482000);
    pub const PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT: Self = Self(1000482001);
    pub const SHADER_CREATE_INFO_EXT: Self = Self(1000482002);
    pub const PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR: Self = Self(1000483000);
    pub const PIPELINE_BINARY_CREATE_INFO_KHR: Self = Self(1000483001);
    pub const PIPELINE_BINARY_INFO_KHR: Self = Self(1000483002);
    pub const PIPELINE_BINARY_KEY_KHR: Self = Self(1000483003);
    pub const PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR: Self = Self(1000483004);
    pub const RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR: Self = Self(1000483005);
    pub const PIPELINE_BINARY_DATA_INFO_KHR: Self = Self(1000483006);
    pub const PIPELINE_CREATE_INFO_KHR: Self = Self(1000483007);
    pub const DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR: Self = Self(1000483008);
    pub const PIPELINE_BINARY_HANDLES_INFO_KHR: Self = Self(1000483009);
    pub const PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM: Self = Self(1000484000);
    pub const TILE_PROPERTIES_QCOM: Self = Self(1000484001);
    pub const PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC: Self = Self(1000485000);
    pub const AMIGO_PROFILING_SUBMIT_INFO_SEC: Self = Self(1000485001);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM: Self = Self(1000488000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV: Self = Self(1000490000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV: Self = Self(1000490001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV: Self = Self(1000491000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491001);
    pub const COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491002);
    pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV: Self = Self(1000491004);
    pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV: Self = Self(1000492000);
    pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV: Self = Self(1000492001);
    pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT: Self = Self(1000495000);
    pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT: Self = Self(1000495001);
    pub const LAYER_SETTINGS_CREATE_INFO_EXT: Self = Self(1000496000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM: Self = Self(1000497000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM: Self = Self(1000497001);
    pub const PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT: Self = Self(1000498000);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT: Self =
        Self(1000499000);
    pub const PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR: Self = Self(1000504000);
    pub const LATENCY_SLEEP_MODE_INFO_NV: Self = Self(1000505000);
    pub const LATENCY_SLEEP_INFO_NV: Self = Self(1000505001);
    pub const SET_LATENCY_MARKER_INFO_NV: Self = Self(1000505002);
    pub const GET_LATENCY_MARKER_INFO_NV: Self = Self(1000505003);
    pub const LATENCY_TIMINGS_FRAME_REPORT_NV: Self = Self(1000505004);
    pub const LATENCY_SUBMISSION_PRESENT_ID_NV: Self = Self(1000505005);
    pub const OUT_OF_BAND_QUEUE_TYPE_INFO_NV: Self = Self(1000505006);
    pub const SWAPCHAIN_LATENCY_CREATE_INFO_NV: Self = Self(1000505007);
    pub const LATENCY_SURFACE_CAPABILITIES_NV: Self = Self(1000505008);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR: Self = Self(1000506000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506002);
    pub const DATA_GRAPH_PIPELINE_CREATE_INFO_ARM: Self = Self(1000507000);
    pub const DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM: Self = Self(1000507001);
    pub const DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM: Self = Self(1000507002);
    pub const DATA_GRAPH_PIPELINE_CONSTANT_ARM: Self = Self(1000507003);
    pub const DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000507004);
    pub const BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM: Self = Self(1000507005);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM: Self = Self(1000507006);
    pub const DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM: Self = Self(1000507007);
    pub const DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM: Self = Self(1000507008);
    pub const DATA_GRAPH_PIPELINE_INFO_ARM: Self = Self(1000507009);
    pub const DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM: Self = Self(1000507010);
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM: Self = Self(1000507011);
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM: Self = Self(1000507012);
    pub const DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM: Self = Self(1000507013);
    pub const DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM: Self = Self(1000507014);
    pub const DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM: Self =
        Self(1000507015);
    pub const DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM: Self = Self(1000507016);
    pub const QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM: Self = Self(1000507017);
    pub const QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM: Self = Self(1000507018);
    pub const PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM: Self =
        Self(1000507019);
    pub const QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM: Self = Self(1000508000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM: Self =
        Self(1000510000);
    pub const MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM: Self = Self(1000510001);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR: Self = Self(1000511000);
    pub const VIDEO_DECODE_AV1_CAPABILITIES_KHR: Self = Self(1000512000);
    pub const VIDEO_DECODE_AV1_PICTURE_INFO_KHR: Self = Self(1000512001);
    pub const VIDEO_DECODE_AV1_PROFILE_INFO_KHR: Self = Self(1000512003);
    pub const VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000512004);
    pub const VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000512005);
    pub const VIDEO_ENCODE_AV1_CAPABILITIES_KHR: Self = Self(1000513000);
    pub const VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000513001);
    pub const VIDEO_ENCODE_AV1_PICTURE_INFO_KHR: Self = Self(1000513002);
    pub const VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000513003);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR: Self = Self(1000513004);
    pub const VIDEO_ENCODE_AV1_PROFILE_INFO_KHR: Self = Self(1000513005);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR: Self = Self(1000513006);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000513007);
    pub const VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000513008);
    pub const VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR: Self = Self(1000513009);
    pub const VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000513010);
    pub const PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR: Self = Self(1000514000);
    pub const VIDEO_DECODE_VP9_CAPABILITIES_KHR: Self = Self(1000514001);
    pub const VIDEO_DECODE_VP9_PICTURE_INFO_KHR: Self = Self(1000514002);
    pub const VIDEO_DECODE_VP9_PROFILE_INFO_KHR: Self = Self(1000514003);
    pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000515000);
    pub const VIDEO_INLINE_QUERY_INFO_KHR: Self = Self(1000515001);
    pub const PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV: Self = Self(1000516000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM: Self = Self(1000518000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM: Self = Self(1000518001);
    pub const SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM: Self = Self(1000518002);
    pub const SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM: Self = Self(1000519000);
    pub const PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM: Self = Self(1000519001);
    pub const BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM: Self = Self(1000519002);
    pub const PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM: Self = Self(1000520000);
    pub const SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM: Self = Self(1000520001);
    pub const PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM: Self = Self(1000521000);
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT: Self =
        Self(1000524000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES: Self = Self(1000525000);
    pub const PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR: Self = Self(1000527000);
    pub const ATTACHMENT_FEEDBACK_LOOP_INFO_EXT: Self = Self(1000527001);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES: Self = Self(1000528000);
    pub const SCREEN_BUFFER_PROPERTIES_QNX: Self = Self(1000529000);
    pub const SCREEN_BUFFER_FORMAT_PROPERTIES_QNX: Self = Self(1000529001);
    pub const IMPORT_SCREEN_BUFFER_INFO_QNX: Self = Self(1000529002);
    pub const EXTERNAL_FORMAT_QNX: Self = Self(1000529003);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX: Self = Self(1000529004);
    pub const PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT: Self = Self(1000530000);
    pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES: Self = Self(1000544000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES: Self = Self(1000545000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES: Self = Self(1000545001);
    pub const BIND_MEMORY_STATUS: Self = Self(1000545002);
    pub const BIND_DESCRIPTOR_SETS_INFO: Self = Self(1000545003);
    pub const PUSH_CONSTANTS_INFO: Self = Self(1000545004);
    pub const PUSH_DESCRIPTOR_SET_INFO: Self = Self(1000545005);
    pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO: Self = Self(1000545006);
    pub const SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT: Self = Self(1000545007);
    pub const BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT: Self = Self(1000545008);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV: Self = Self(1000546000);
    pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM: Self = Self(1000547000);
    pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM: Self = Self(1000547001);
    pub const TILE_MEMORY_REQUIREMENTS_QCOM: Self = Self(1000547002);
    pub const TILE_MEMORY_BIND_INFO_QCOM: Self = Self(1000547003);
    pub const TILE_MEMORY_SIZE_INFO_QCOM: Self = Self(1000547004);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR: Self = Self(1000549000);
    pub const COPY_MEMORY_INDIRECT_INFO_KHR: Self = Self(1000549002);
    pub const COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR: Self = Self(1000549003);
    pub const DECOMPRESS_MEMORY_INFO_EXT: Self = Self(1000550002);
    pub const DISPLAY_SURFACE_STEREO_CREATE_INFO_NV: Self = Self(1000551000);
    pub const DISPLAY_MODE_STEREO_PROPERTIES_NV: Self = Self(1000551001);
    pub const VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR: Self = Self(1000552000);
    pub const VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR: Self = Self(1000552001);
    pub const VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552002);
    pub const VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552003);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR: Self = Self(1000552004);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553000);
    pub const VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553001);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR: Self = Self(1000553002);
    pub const VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553003);
    pub const VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553004);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR: Self =
        Self(1000553005);
    pub const VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553006);
    pub const VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553007);
    pub const VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553008);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR: Self = Self(1000553009);
    pub const PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV: Self = Self(1000555000);
    pub const EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV: Self = Self(1000556000);
    pub const EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV: Self = Self(1000556001);
    pub const EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV: Self = Self(1000556002);
    pub const PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV: Self = Self(1000556003);
    pub const PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR: Self =
        Self(1000558000);
    pub const PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV: Self = Self(1000559000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR: Self = Self(1000562000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR: Self = Self(1000562001);
    pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR: Self = Self(1000562002);
    pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR: Self = Self(1000562003);
    pub const PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR: Self = Self(1000562004);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV: Self = Self(1000563000);
    pub const PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT: Self = Self(1000564000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT: Self = Self(1000567000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV: Self = Self(1000568000);
    pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV: Self = Self(1000569000);
    pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self = Self(1000569001);
    pub const CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV: Self =
        Self(1000569002);
    pub const CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV: Self = Self(1000569003);
    pub const CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV: Self = Self(1000569004);
    pub const CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV: Self = Self(1000569005);
    pub const CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV: Self = Self(1000569006);
    pub const RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self =
        Self(1000569007);
    pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV: Self =
        Self(1000570000);
    pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self =
        Self(1000570001);
    pub const WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570002);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV: Self = Self(1000570003);
    pub const BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000570004);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV: Self = Self(1000570005);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT: Self = Self(1000572000);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT: Self = Self(1000572001);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT: Self = Self(1000572002);
    pub const INDIRECT_EXECUTION_SET_CREATE_INFO_EXT: Self = Self(1000572003);
    pub const GENERATED_COMMANDS_INFO_EXT: Self = Self(1000572004);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT: Self = Self(1000572006);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT: Self = Self(1000572007);
    pub const WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT: Self = Self(1000572008);
    pub const WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT: Self = Self(1000572009);
    pub const INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT: Self = Self(1000572010);
    pub const INDIRECT_EXECUTION_SET_SHADER_INFO_EXT: Self = Self(1000572011);
    pub const INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT: Self = Self(1000572012);
    pub const GENERATED_COMMANDS_PIPELINE_INFO_EXT: Self = Self(1000572013);
    pub const GENERATED_COMMANDS_SHADER_INFO_EXT: Self = Self(1000572014);
    pub const PHYSICAL_DEVICE_FAULT_FEATURES_KHR: Self = Self(1000573000);
    pub const PHYSICAL_DEVICE_FAULT_PROPERTIES_KHR: Self = Self(1000573001);
    pub const DEVICE_FAULT_INFO_KHR: Self = Self(1000573002);
    pub const DEVICE_FAULT_DEBUG_INFO_KHR: Self = Self(1000573003);
    pub const PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR: Self = Self(1000574000);
    pub const MEMORY_BARRIER_ACCESS_FLAGS_3_KHR: Self = Self(1000574002);
    pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA: Self = Self(1000575000);
    pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA: Self = Self(1000575001);
    pub const IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA: Self = Self(1000575002);
    pub const PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR: Self = Self(1000579000);
    pub const PUSH_CONSTANT_BANK_INFO_NV: Self = Self(1000580000);
    pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV: Self = Self(1000580001);
    pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV: Self = Self(1000580002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT: Self = Self(1000581000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT: Self =
        Self(1000581001);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT: Self = Self(1000582000);
    pub const PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT: Self = Self(1000582001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR: Self = Self(1000584000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR: Self = Self(1000584001);
    pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR: Self = Self(1000584002);
    pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR: Self = Self(1000586000);
    pub const VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586001);
    pub const VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586002);
    pub const VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586003);
    pub const PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI: Self = Self(1000590000);
    pub const HDR_VIVID_DYNAMIC_METADATA_HUAWEI: Self = Self(1000590001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV: Self = Self(1000593000);
    pub const COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV: Self = Self(1000593001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV: Self = Self(1000593002);
    pub const PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM: Self = Self(1000596000);
    pub const IMPORT_MEMORY_METAL_HANDLE_INFO_EXT: Self = Self(1000602000);
    pub const MEMORY_METAL_HANDLE_PROPERTIES_EXT: Self = Self(1000602001);
    pub const MEMORY_GET_METAL_HANDLE_INFO_EXT: Self = Self(1000602002);
    pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM: Self = Self(1000605000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM: Self =
        Self(1000605001);
    pub const PERFORMANCE_COUNTER_ARM: Self = Self(1000605002);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_ARM: Self = Self(1000605003);
    pub const RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM: Self = Self(1000605004);
    pub const PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM: Self = Self(1000607000);
    pub const PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM: Self = Self(1000607001);
    pub const SHADER_INSTRUMENTATION_CREATE_INFO_ARM: Self = Self(1000607002);
    pub const SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM: Self = Self(1000607003);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000608000);
    pub const PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM: Self = Self(1000609000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE: Self = Self(1000611000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE: Self =
        Self(1000611001);
    pub const PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE: Self = Self(1000611002);
    pub const SET_PRESENT_CONFIG_NV: Self = Self(1000613000);
    pub const PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV: Self = Self(1000613001);
    pub const RENDERING_END_INFO_KHR: Self = Self(1000619003);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT: Self = Self(1000620000);
    pub const PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT: Self = Self(1000627000);
    pub const PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT: Self = Self(1000628000);
    pub const BEGIN_CUSTOM_RESOLVE_INFO_EXT: Self = Self(1000628001);
    pub const CUSTOM_RESOLVE_CREATE_INFO_EXT: Self = Self(1000628002);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM: Self = Self(1000629000);
    pub const DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM: Self = Self(1000629001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR: Self = Self(1000630000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR: Self = Self(1000630001);
    pub const RENDERING_ATTACHMENT_FLAGS_INFO_KHR: Self = Self(1000630002);
    pub const RESOLVE_IMAGE_MODE_INFO_KHR: Self = Self(1000630004);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM: Self = Self(1000631000);
    pub const QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM: Self = Self(1000631001);
    pub const DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM: Self = Self(1000631002);
    pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM: Self = Self(1000631003);
    pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM: Self = Self(1000631004);
    pub const DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM: Self = Self(1000631005);
    pub const DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM: Self = Self(1000631006);
    pub const DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM: Self = Self(1000631007);
    pub const DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM: Self = Self(1000631008);
    pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT: Self = Self(1000635000);
    pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT: Self = Self(1000635001);
    pub const PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC: Self = Self(1000637000);
    pub const PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT: Self =
        Self(1000642000);
    pub const COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV: Self = Self(1000645000);
    pub const PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV: Self = Self(1000645001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_11_FEATURES_KHR: Self = Self(1000657000);
    pub const QUEUE_FAMILY_OPTIMAL_IMAGE_TRANSFER_GRANULARITY_PROPERTIES_KHR: Self =
        Self(1000657001);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT: Self = Self(1000662000);
    pub const UBM_SURFACE_CREATE_INFO_SEC: Self = Self(1000664000);
    pub const PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE: Self =
        Self(1000673000);
    pub const PHYSICAL_DEVICE_THROTTLE_HINT_FEATURES_SEC: Self = Self(1000674000);
    pub const THROTTLE_HINT_SUBMIT_INFO_SEC: Self = Self(1000674001);
    pub const DATA_GRAPH_PIPELINE_NEURAL_STATISTICS_CREATE_INFO_ARM: Self = Self(1000676000);
    pub const DATA_GRAPH_PIPELINE_SESSION_NEURAL_STATISTICS_CREATE_INFO_ARM: Self =
        Self(1000676001);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_FEATURES_ARM: Self =
        Self(1000676002);
    pub const PHYSICAL_DEVICE_PRIMITIVE_RESTART_INDEX_FEATURES_EXT: Self = Self(1000678000);
    pub const SURFACE_CREATE_INFO_OHOS: Self = Self(1000685000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SubpassContents(pub i32);

impl SubpassContents {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INLINE: Self = Self(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
    pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR: Self = Self(1000451000);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SubpassMergeStatusEXT(pub i32);

impl SubpassMergeStatusEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SUBPASS_MERGE_STATUS_MERGED_EXT: Self = Self(0);
    pub const SUBPASS_MERGE_STATUS_DISALLOWED_EXT: Self = Self(1);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_SIDE_EFFECTS_EXT: Self = Self(2);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_SAMPLES_MISMATCH_EXT: Self = Self(3);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_VIEWS_MISMATCH_EXT: Self = Self(4);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_ALIASING_EXT: Self = Self(5);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_DEPENDENCIES_EXT: Self = Self(6);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT: Self = Self(7);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT: Self = Self(8);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_INSUFFICIENT_STORAGE_EXT: Self = Self(9);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_DEPTH_STENCIL_COUNT_EXT: Self = Self(10);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT: Self = Self(11);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_SINGLE_SUBPASS_EXT: Self = Self(12);
    pub const SUBPASS_MERGE_STATUS_NOT_MERGED_UNSPECIFIED_EXT: Self = Self(13);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SystemAllocationScope(pub i32);

impl SystemAllocationScope {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COMMAND: Self = Self(0);
    pub const OBJECT: Self = Self(1);
    pub const CACHE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const INSTANCE: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TensorTilingARM(pub i32);

impl TensorTilingARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TENSOR_TILING_OPTIMAL_ARM: Self = Self(0);
    pub const TENSOR_TILING_LINEAR_ARM: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TessellationDomainOrigin(pub i32);

impl TessellationDomainOrigin {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
}

pub type TessellationDomainOriginKHR = TessellationDomainOrigin;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ThrottleHintTypeSEC(pub i32);

impl ThrottleHintTypeSEC {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const THROTTLE_HINT_TYPE_DEFAULT_SEC: Self = Self(0);
    pub const THROTTLE_HINT_TYPE_LOW_SEC: Self = Self(1);
    pub const THROTTLE_HINT_TYPE_HIGH_SEC: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TimeDomainKHR(pub i32);

impl TimeDomainKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TIME_DOMAIN_DEVICE_KHR: Self = Self(0);
    pub const TIME_DOMAIN_CLOCK_MONOTONIC_KHR: Self = Self(1);
    pub const TIME_DOMAIN_CLOCK_MONOTONIC_RAW_KHR: Self = Self(2);
    pub const TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_KHR: Self = Self(3);
    pub const TIME_DOMAIN_PRESENT_STAGE_LOCAL_EXT: Self = Self(1000208000);
    pub const TIME_DOMAIN_SWAPCHAIN_LOCAL_EXT: Self = Self(1000208001);
}

pub type TimeDomainEXT = TimeDomainKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ValidationCacheHeaderVersionEXT(pub i32);

impl ValidationCacheHeaderVersionEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ValidationCheckEXT(pub i32);

impl ValidationCheckEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VALIDATION_CHECK_ALL_EXT: Self = Self(0);
    pub const VALIDATION_CHECK_SHADERS_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ValidationFeatureDisableEXT(pub i32);

impl ValidationFeatureDisableEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VALIDATION_FEATURE_DISABLE_ALL_EXT: Self = Self(0);
    pub const VALIDATION_FEATURE_DISABLE_SHADERS_EXT: Self = Self(1);
    pub const VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT: Self = Self(2);
    pub const VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT: Self = Self(3);
    pub const VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT: Self = Self(4);
    pub const VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT: Self = Self(5);
    pub const VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT: Self = Self(6);
    pub const VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT: Self = Self(7);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ValidationFeatureEnableEXT(pub i32);

impl ValidationFeatureEnableEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT: Self = Self(0);
    pub const VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: Self = Self(1);
    pub const VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT: Self = Self(2);
    pub const VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT: Self = Self(3);
    pub const VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VendorId(pub i32);

impl VendorId {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const KHRONOS: Self = Self(65536);
    pub const VIV: Self = Self(65537);
    pub const VSI: Self = Self(65538);
    pub const KAZAN: Self = Self(65539);
    pub const CODEPLAY: Self = Self(65540);
    pub const MESA: Self = Self(65541);
    pub const POCL: Self = Self(65542);
    pub const MOBILEYE: Self = Self(65543);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VertexInputRate(pub i32);

impl VertexInputRate {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VERTEX: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeAV1PredictionModeKHR(pub i32);

impl VideoEncodeAV1PredictionModeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VIDEO_ENCODE_AV1_PREDICTION_MODE_INTRA_ONLY_KHR: Self = Self(0);
    pub const VIDEO_ENCODE_AV1_PREDICTION_MODE_SINGLE_REFERENCE_KHR: Self = Self(1);
    pub const VIDEO_ENCODE_AV1_PREDICTION_MODE_UNIDIRECTIONAL_COMPOUND_KHR: Self = Self(2);
    pub const VIDEO_ENCODE_AV1_PREDICTION_MODE_BIDIRECTIONAL_COMPOUND_KHR: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeAV1RateControlGroupKHR(pub i32);

impl VideoEncodeAV1RateControlGroupKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_INTRA_KHR: Self = Self(0);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_PREDICTIVE_KHR: Self = Self(1);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_BIPREDICTIVE_KHR: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeTuningModeKHR(pub i32);

impl VideoEncodeTuningModeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VIDEO_ENCODE_TUNING_MODE_DEFAULT_KHR: Self = Self(0);
    pub const VIDEO_ENCODE_TUNING_MODE_HIGH_QUALITY_KHR: Self = Self(1);
    pub const VIDEO_ENCODE_TUNING_MODE_LOW_LATENCY_KHR: Self = Self(2);
    pub const VIDEO_ENCODE_TUNING_MODE_ULTRA_LOW_LATENCY_KHR: Self = Self(3);
    pub const VIDEO_ENCODE_TUNING_MODE_LOSSLESS_KHR: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ViewportCoordinateSwizzleNV(pub i32);

impl ViewportCoordinateSwizzleNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: Self = Self(0);
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: Self = Self(1);
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: Self = Self(2);
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: Self = Self(3);
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: Self = Self(4);
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: Self = Self(5);
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: Self = Self(6);
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: Self = Self(7);
}
