#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use crate::vk::*;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNKNOWN"),
            (Self(1), "VERTICAL"),
            (Self(2), "COLOCATED"),
            (Self(3), "RESERVED"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1ChromaSamplePosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1ChromaSamplePosition", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(1), "BT_709"),
            (Self(2), "UNSPECIFIED"),
            (Self(4), "BT_470_M"),
            (Self(5), "BT_470_B_G"),
            (Self(6), "BT_601"),
            (Self(7), "SMPTE_240"),
            (Self(8), "GENERIC_FILM"),
            (Self(9), "BT_2020"),
            (Self(10), "XYZ"),
            (Self(11), "SMPTE_431"),
            (Self(12), "SMPTE_432"),
            (Self(22), "EBU_3213"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1ColorPrimaries {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1ColorPrimaries", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "NONE"),
            (Self(1), "WIENER"),
            (Self(2), "SGRPROJ"),
            (Self(3), "SWITCHABLE"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1FrameRestorationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1FrameRestorationType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "KEY"),
            (Self(1), "INTER"),
            (Self(2), "INTRA_ONLY"),
            (Self(3), "SWITCH"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1FrameType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1FrameType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "EIGHTTAP"),
            (Self(1), "EIGHTTAP_SMOOTH"),
            (Self(2), "EIGHTTAP_SHARP"),
            (Self(3), "BILINEAR"),
            (Self(4), "SWITCHABLE"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1InterpolationFilter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1InterpolationFilter", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_2_0"),
            (Self(1), "_2_1"),
            (Self(2), "_2_2"),
            (Self(3), "_2_3"),
            (Self(4), "_3_0"),
            (Self(5), "_3_1"),
            (Self(6), "_3_2"),
            (Self(7), "_3_3"),
            (Self(8), "_4_0"),
            (Self(9), "_4_1"),
            (Self(10), "_4_2"),
            (Self(11), "_4_3"),
            (Self(12), "_5_0"),
            (Self(13), "_5_1"),
            (Self(14), "_5_2"),
            (Self(15), "_5_3"),
            (Self(16), "_6_0"),
            (Self(17), "_6_1"),
            (Self(18), "_6_2"),
            (Self(19), "_6_3"),
            (Self(20), "_7_0"),
            (Self(21), "_7_1"),
            (Self(22), "_7_2"),
            (Self(23), "_7_3"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1Level {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1Level", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "IDENTITY"),
            (Self(1), "BT_709"),
            (Self(2), "UNSPECIFIED"),
            (Self(3), "RESERVED_3"),
            (Self(4), "FCC"),
            (Self(5), "BT_470_B_G"),
            (Self(6), "BT_601"),
            (Self(7), "SMPTE_240"),
            (Self(8), "SMPTE_YCGCO"),
            (Self(9), "BT_2020_NCL"),
            (Self(10), "BT_2020_CL"),
            (Self(11), "SMPTE_2085"),
            (Self(12), "CHROMAT_NCL"),
            (Self(13), "CHROMAT_CL"),
            (Self(14), "ICTCP"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1MatrixCoefficients {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1MatrixCoefficients", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "MAIN"),
            (Self(1), "HIGH"),
            (Self(2), "PROFESSIONAL"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1Profile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1Profile", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "INTRA_FRAME"),
            (Self(1), "LAST_FRAME"),
            (Self(2), "LAST2_FRAME"),
            (Self(3), "LAST3_FRAME"),
            (Self(4), "GOLDEN_FRAME"),
            (Self(5), "BWDREF_FRAME"),
            (Self(6), "ALTREF2_FRAME"),
            (Self(7), "ALTREF_FRAME"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1ReferenceName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1ReferenceName", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "RESERVED_0"),
            (Self(1), "BT_709"),
            (Self(2), "UNSPECIFIED"),
            (Self(3), "RESERVED_3"),
            (Self(4), "BT_470_M"),
            (Self(5), "BT_470_B_G"),
            (Self(6), "BT_601"),
            (Self(7), "SMPTE_240"),
            (Self(8), "LINEAR"),
            (Self(9), "LOG_100"),
            (Self(10), "LOG_100_SQRT10"),
            (Self(11), "IEC_61966"),
            (Self(12), "BT_1361"),
            (Self(13), "SRGB"),
            (Self(14), "BT_2020_10_BIT"),
            (Self(15), "BT_2020_12_BIT"),
            (Self(16), "SMPTE_2084"),
            (Self(17), "SMPTE_428"),
            (Self(18), "HLG"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1TransferCharacteristics {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1TransferCharacteristics", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "ONLY_4X4"),
            (Self(1), "LARGEST"),
            (Self(2), "SELECT"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoAV1TxMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoAV1TxMode", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "TOP"),
            (Self(1), "BOTTOM"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoDecodeH264FieldOrderCount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoDecodeH264FieldOrderCount", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNSPECIFIED"),
            (Self(1), "SQUARE"),
            (Self(2), "_12_11"),
            (Self(3), "_10_11"),
            (Self(4), "_16_11"),
            (Self(5), "_40_33"),
            (Self(6), "_24_11"),
            (Self(7), "_20_11"),
            (Self(8), "_32_11"),
            (Self(9), "_80_33"),
            (Self(10), "_18_11"),
            (Self(11), "_15_11"),
            (Self(12), "_64_33"),
            (Self(13), "_160_99"),
            (Self(14), "_4_3"),
            (Self(15), "_3_2"),
            (Self(16), "_2_1"),
            (Self(255), "EXTENDED_SAR"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264AspectRatioIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264AspectRatioIdc", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_0"),
            (Self(1), "_1"),
            (Self(2), "_2"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264CabacInitIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264CabacInitIdc", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "MONOCHROME"),
            (Self(1), "_420"),
            (Self(2), "_422"),
            (Self(3), "_444"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264ChromaFormatIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264ChromaFormatIdc", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DISABLED"),
            (Self(1), "ENABLED"),
            (Self(2), "PARTIAL"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264DisableDeblockingFilterIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "StdVideoH264DisableDeblockingFilterIdc",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_1_0"),
            (Self(1), "_1_1"),
            (Self(2), "_1_2"),
            (Self(3), "_1_3"),
            (Self(4), "_2_0"),
            (Self(5), "_2_1"),
            (Self(6), "_2_2"),
            (Self(7), "_3_0"),
            (Self(8), "_3_1"),
            (Self(9), "_3_2"),
            (Self(10), "_4_0"),
            (Self(11), "_4_1"),
            (Self(12), "_4_2"),
            (Self(13), "_5_0"),
            (Self(14), "_5_1"),
            (Self(15), "_5_2"),
            (Self(16), "_6_0"),
            (Self(17), "_6_1"),
            (Self(18), "_6_2"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264LevelIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264LevelIdc", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "END"),
            (Self(1), "UNMARK_SHORT_TERM"),
            (Self(2), "UNMARK_LONG_TERM"),
            (Self(3), "MARK_LONG_TERM"),
            (Self(4), "SET_MAX_LONG_TERM_INDEX"),
            (Self(5), "UNMARK_ALL"),
            (Self(6), "MARK_CURRENT_AS_LONG_TERM"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264MemMgmtControlOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264MemMgmtControlOp", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "SHORT_TERM_SUBTRACT"),
            (Self(1), "SHORT_TERM_ADD"),
            (Self(2), "LONG_TERM"),
            (Self(3), "END"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264ModificationOfPicNumsIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "StdVideoH264ModificationOfPicNumsIdc",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "SPS"),
            (Self(1), "PPS"),
            (Self(2), "AUD"),
            (Self(3), "PREFIX"),
            (Self(4), "END_OF_SEQUENCE"),
            (Self(5), "END_OF_STREAM"),
            (Self(6), "PRECODED"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264NonVclNaluType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264NonVclNaluType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "P"),
            (Self(1), "B"),
            (Self(2), "I"),
            (Self(5), "IDR"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264PictureType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264PictureType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_0"),
            (Self(1), "_1"),
            (Self(2), "_2"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264PocType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264PocType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const HIGH_10: Self = Self(110);
    pub const HIGH_422: Self = Self(122);
    pub const HIGH_444_PREDICTIVE: Self = Self(244);
    pub const INVALID: Self = Self(2147483647);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(66), "BASELINE"),
            (Self(77), "MAIN"),
            (Self(100), "HIGH"),
            (Self(110), "HIGH_10"),
            (Self(122), "HIGH_422"),
            (Self(244), "HIGH_444_PREDICTIVE"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264ProfileIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264ProfileIdc", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "P"),
            (Self(1), "B"),
            (Self(2), "I"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264SliceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264SliceType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT"),
            (Self(1), "EXPLICIT"),
            (Self(2), "IMPLICIT"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH264WeightedBipredIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH264WeightedBipredIdc", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNSPECIFIED"),
            (Self(1), "SQUARE"),
            (Self(2), "_12_11"),
            (Self(3), "_10_11"),
            (Self(4), "_16_11"),
            (Self(5), "_40_33"),
            (Self(6), "_24_11"),
            (Self(7), "_20_11"),
            (Self(8), "_32_11"),
            (Self(9), "_80_33"),
            (Self(10), "_18_11"),
            (Self(11), "_15_11"),
            (Self(12), "_64_33"),
            (Self(13), "_160_99"),
            (Self(14), "_4_3"),
            (Self(15), "_3_2"),
            (Self(16), "_2_1"),
            (Self(255), "EXTENDED_SAR"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH265AspectRatioIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH265AspectRatioIdc", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "MONOCHROME"),
            (Self(1), "_420"),
            (Self(2), "_422"),
            (Self(3), "_444"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH265ChromaFormatIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH265ChromaFormatIdc", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_1_0"),
            (Self(1), "_2_0"),
            (Self(2), "_2_1"),
            (Self(3), "_3_0"),
            (Self(4), "_3_1"),
            (Self(5), "_4_0"),
            (Self(6), "_4_1"),
            (Self(7), "_5_0"),
            (Self(8), "_5_1"),
            (Self(9), "_5_2"),
            (Self(10), "_6_0"),
            (Self(11), "_6_1"),
            (Self(12), "_6_2"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH265LevelIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH265LevelIdc", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "P"),
            (Self(1), "B"),
            (Self(2), "I"),
            (Self(3), "IDR"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH265PictureType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH265PictureType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(1), "MAIN"),
            (Self(2), "MAIN_10"),
            (Self(3), "MAIN_STILL_PICTURE"),
            (Self(4), "FORMAT_RANGE_EXTENSIONS"),
            (Self(9), "SCC_EXTENSIONS"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH265ProfileIdc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH265ProfileIdc", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "B"),
            (Self(1), "P"),
            (Self(2), "I"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoH265SliceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoH265SliceType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNKNOWN"),
            (Self(1), "BT_601"),
            (Self(2), "BT_709"),
            (Self(3), "SMPTE_170"),
            (Self(4), "SMPTE_240"),
            (Self(5), "BT_2020"),
            (Self(6), "RESERVED"),
            (Self(7), "RGB"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoVP9ColorSpace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoVP9ColorSpace", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "KEY"),
            (Self(1), "NON_KEY"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoVP9FrameType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoVP9FrameType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "EIGHTTAP"),
            (Self(1), "EIGHTTAP_SMOOTH"),
            (Self(2), "EIGHTTAP_SHARP"),
            (Self(3), "BILINEAR"),
            (Self(4), "SWITCHABLE"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoVP9InterpolationFilter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoVP9InterpolationFilter", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_1_0"),
            (Self(1), "_1_1"),
            (Self(2), "_2_0"),
            (Self(3), "_2_1"),
            (Self(4), "_3_0"),
            (Self(5), "_3_1"),
            (Self(6), "_4_0"),
            (Self(7), "_4_1"),
            (Self(8), "_5_0"),
            (Self(9), "_5_1"),
            (Self(10), "_5_2"),
            (Self(11), "_6_0"),
            (Self(12), "_6_1"),
            (Self(13), "_6_2"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoVP9Level {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoVP9Level", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_0"),
            (Self(1), "_1"),
            (Self(2), "_2"),
            (Self(3), "_3"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoVP9Profile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoVP9Profile", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "INTRA_FRAME"),
            (Self(1), "LAST_FRAME"),
            (Self(2), "GOLDEN_FRAME"),
            (Self(3), "ALTREF_FRAME"),
            (Self(2147483647), "INVALID"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StdVideoVP9ReferenceName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StdVideoVP9ReferenceName", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccelerationStructureBuildTypeKHR(pub i32);

impl AccelerationStructureBuildTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const HOST: Self = Self(0);
    pub const DEVICE: Self = Self(1);
    pub const HOST_OR_DEVICE: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "HOST"),
            (Self(1), "DEVICE"),
            (Self(2), "HOST_OR_DEVICE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for AccelerationStructureBuildTypeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "AccelerationStructureBuildTypeKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccelerationStructureCompatibilityKHR(pub i32);

impl AccelerationStructureCompatibilityKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COMPATIBLE: Self = Self(0);
    pub const INCOMPATIBLE: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "COMPATIBLE"), (Self(1), "INCOMPATIBLE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for AccelerationStructureCompatibilityKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "AccelerationStructureCompatibilityKHR",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccelerationStructureMemoryRequirementsTypeNV(pub i32);

impl AccelerationStructureMemoryRequirementsTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OBJECT: Self = Self(0);
    pub const BUILD_SCRATCH: Self = Self(1);
    pub const UPDATE_SCRATCH: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "OBJECT"),
            (Self(1), "BUILD_SCRATCH"),
            (Self(2), "UPDATE_SCRATCH"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for AccelerationStructureMemoryRequirementsTypeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "AccelerationStructureMemoryRequirementsTypeNV",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccelerationStructureMotionInstanceTypeNV(pub i32);

impl AccelerationStructureMotionInstanceTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const STATIC: Self = Self(0);
    pub const MATRIX_MOTION: Self = Self(1);
    pub const SRT_MOTION: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "STATIC"),
            (Self(1), "MATRIX_MOTION"),
            (Self(2), "SRT_MOTION"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for AccelerationStructureMotionInstanceTypeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "AccelerationStructureMotionInstanceTypeNV",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccelerationStructureSerializedBlockTypeKHR(pub i32);

impl AccelerationStructureSerializedBlockTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPACITY_MICROMAP: Self = Self(0);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "OPACITY_MICROMAP")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for AccelerationStructureSerializedBlockTypeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "AccelerationStructureSerializedBlockTypeKHR",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccelerationStructureTypeKHR(pub i32);

impl AccelerationStructureTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TOP_LEVEL: Self = Self(0);
    pub const BOTTOM_LEVEL: Self = Self(1);
    pub const GENERIC: Self = Self(2);
    pub const OPACITY_MICROMAP: Self = Self(1000623000);
    pub const TOP_LEVEL_NV: Self = Self(0);
    pub const BOTTOM_LEVEL_NV: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "TOP_LEVEL"),
            (Self(1), "BOTTOM_LEVEL"),
            (Self(2), "GENERIC"),
            (Self(1000623000), "OPACITY_MICROMAP"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(0), "TOP_LEVEL_NV"), (Self(1), "BOTTOM_LEVEL_NV")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for AccelerationStructureTypeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "AccelerationStructureTypeKHR", self.name(), self.0)
    }
}

pub type AccelerationStructureTypeNV = AccelerationStructureTypeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AntiLagModeAMD(pub i32);

impl AntiLagModeAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DRIVER_CONTROL: Self = Self(0);
    pub const ON: Self = Self(1);
    pub const OFF: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DRIVER_CONTROL"),
            (Self(1), "ON"),
            (Self(2), "OFF"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for AntiLagModeAMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "AntiLagModeAMD", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AntiLagStageAMD(pub i32);

impl AntiLagStageAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INPUT: Self = Self(0);
    pub const PRESENT: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "INPUT"), (Self(1), "PRESENT")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for AntiLagStageAMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "AntiLagStageAMD", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const NONE_EXT: Self = Self(1000400000);
    pub const NONE_KHR: Self = Self(1000400000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "LOAD"),
            (Self(1), "CLEAR"),
            (Self(2), "DONT_CARE"),
            (Self(1000400000), "NONE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(1000400000), "NONE_EXT"),
            (Self(1000400000), "NONE_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for AttachmentLoadOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "AttachmentLoadOp", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const NONE_EXT: Self = Self(1000301000);
    pub const NONE_KHR: Self = Self(1000301000);
    pub const NONE_QCOM: Self = Self(1000301000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "STORE"),
            (Self(1), "DONT_CARE"),
            (Self(1000301000), "NONE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(1000301000), "NONE_EXT"),
            (Self(1000301000), "NONE_KHR"),
            (Self(1000301000), "NONE_QCOM"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for AttachmentStoreOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "AttachmentStoreOp", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "ZERO"),
            (Self(1), "ONE"),
            (Self(2), "SRC_COLOR"),
            (Self(3), "ONE_MINUS_SRC_COLOR"),
            (Self(4), "DST_COLOR"),
            (Self(5), "ONE_MINUS_DST_COLOR"),
            (Self(6), "SRC_ALPHA"),
            (Self(7), "ONE_MINUS_SRC_ALPHA"),
            (Self(8), "DST_ALPHA"),
            (Self(9), "ONE_MINUS_DST_ALPHA"),
            (Self(10), "CONSTANT_COLOR"),
            (Self(11), "ONE_MINUS_CONSTANT_COLOR"),
            (Self(12), "CONSTANT_ALPHA"),
            (Self(13), "ONE_MINUS_CONSTANT_ALPHA"),
            (Self(14), "SRC_ALPHA_SATURATE"),
            (Self(15), "SRC1_COLOR"),
            (Self(16), "ONE_MINUS_SRC1_COLOR"),
            (Self(17), "SRC1_ALPHA"),
            (Self(18), "ONE_MINUS_SRC1_ALPHA"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for BlendFactor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "BlendFactor", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "ADD"),
            (Self(1), "SUBTRACT"),
            (Self(2), "REVERSE_SUBTRACT"),
            (Self(3), "MIN"),
            (Self(4), "MAX"),
            (Self(1000148000), "ZERO_EXT"),
            (Self(1000148001), "SRC_EXT"),
            (Self(1000148002), "DST_EXT"),
            (Self(1000148003), "SRC_OVER_EXT"),
            (Self(1000148004), "DST_OVER_EXT"),
            (Self(1000148005), "SRC_IN_EXT"),
            (Self(1000148006), "DST_IN_EXT"),
            (Self(1000148007), "SRC_OUT_EXT"),
            (Self(1000148008), "DST_OUT_EXT"),
            (Self(1000148009), "SRC_ATOP_EXT"),
            (Self(1000148010), "DST_ATOP_EXT"),
            (Self(1000148011), "XOR_EXT"),
            (Self(1000148012), "MULTIPLY_EXT"),
            (Self(1000148013), "SCREEN_EXT"),
            (Self(1000148014), "OVERLAY_EXT"),
            (Self(1000148015), "DARKEN_EXT"),
            (Self(1000148016), "LIGHTEN_EXT"),
            (Self(1000148017), "COLORDODGE_EXT"),
            (Self(1000148018), "COLORBURN_EXT"),
            (Self(1000148019), "HARDLIGHT_EXT"),
            (Self(1000148020), "SOFTLIGHT_EXT"),
            (Self(1000148021), "DIFFERENCE_EXT"),
            (Self(1000148022), "EXCLUSION_EXT"),
            (Self(1000148023), "INVERT_EXT"),
            (Self(1000148024), "INVERT_RGB_EXT"),
            (Self(1000148025), "LINEARDODGE_EXT"),
            (Self(1000148026), "LINEARBURN_EXT"),
            (Self(1000148027), "VIVIDLIGHT_EXT"),
            (Self(1000148028), "LINEARLIGHT_EXT"),
            (Self(1000148029), "PINLIGHT_EXT"),
            (Self(1000148030), "HARDMIX_EXT"),
            (Self(1000148031), "HSL_HUE_EXT"),
            (Self(1000148032), "HSL_SATURATION_EXT"),
            (Self(1000148033), "HSL_COLOR_EXT"),
            (Self(1000148034), "HSL_LUMINOSITY_EXT"),
            (Self(1000148035), "PLUS_EXT"),
            (Self(1000148036), "PLUS_CLAMPED_EXT"),
            (Self(1000148037), "PLUS_CLAMPED_ALPHA_EXT"),
            (Self(1000148038), "PLUS_DARKER_EXT"),
            (Self(1000148039), "MINUS_EXT"),
            (Self(1000148040), "MINUS_CLAMPED_EXT"),
            (Self(1000148041), "CONTRAST_EXT"),
            (Self(1000148042), "INVERT_OVG_EXT"),
            (Self(1000148043), "RED_EXT"),
            (Self(1000148044), "GREEN_EXT"),
            (Self(1000148045), "BLUE_EXT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for BlendOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "BlendOp", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlendOverlapEXT(pub i32);

impl BlendOverlapEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNCORRELATED: Self = Self(0);
    pub const DISJOINT: Self = Self(1);
    pub const CONJOINT: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNCORRELATED"),
            (Self(1), "DISJOINT"),
            (Self(2), "CONJOINT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for BlendOverlapEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "BlendOverlapEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlockMatchWindowCompareModeQCOM(pub i32);

impl BlockMatchWindowCompareModeQCOM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "MIN"), (Self(1), "MAX")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for BlockMatchWindowCompareModeQCOM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "BlockMatchWindowCompareModeQCOM", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "FLOAT_TRANSPARENT_BLACK"),
            (Self(1), "INT_TRANSPARENT_BLACK"),
            (Self(2), "FLOAT_OPAQUE_BLACK"),
            (Self(3), "INT_OPAQUE_BLACK"),
            (Self(4), "FLOAT_OPAQUE_WHITE"),
            (Self(5), "INT_OPAQUE_WHITE"),
            (Self(1000287003), "FLOAT_CUSTOM_EXT"),
            (Self(1000287004), "INT_CUSTOM_EXT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for BorderColor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "BorderColor", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BuildAccelerationStructureModeKHR(pub i32);

impl BuildAccelerationStructureModeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BUILD: Self = Self(0);
    pub const UPDATE: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "BUILD"), (Self(1), "UPDATE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for BuildAccelerationStructureModeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "BuildAccelerationStructureModeKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BuildMicromapModeEXT(pub i32);

impl BuildMicromapModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BUILD: Self = Self(0);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "BUILD")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for BuildMicromapModeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "BuildMicromapModeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const COSITED_EVEN_KHR: Self = Self(0);
    pub const MIDPOINT_KHR: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "COSITED_EVEN"), (Self(1), "MIDPOINT")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(0), "COSITED_EVEN_KHR"), (Self(1), "MIDPOINT_KHR")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ChromaLocation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ChromaLocation", self.name(), self.0)
    }
}

pub type ChromaLocationKHR = ChromaLocation;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ClusterAccelerationStructureOpModeNV(pub i32);

impl ClusterAccelerationStructureOpModeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const IMPLICIT_DESTINATIONS: Self = Self(0);
    pub const EXPLICIT_DESTINATIONS: Self = Self(1);
    pub const COMPUTE_SIZES: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "IMPLICIT_DESTINATIONS"),
            (Self(1), "EXPLICIT_DESTINATIONS"),
            (Self(2), "COMPUTE_SIZES"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ClusterAccelerationStructureOpModeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "ClusterAccelerationStructureOpModeNV",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ClusterAccelerationStructureOpTypeNV(pub i32);

impl ClusterAccelerationStructureOpTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MOVE_OBJECTS: Self = Self(0);
    pub const BUILD_CLUSTERS_BOTTOM_LEVEL: Self = Self(1);
    pub const BUILD_TRIANGLE_CLUSTER: Self = Self(2);
    pub const BUILD_TRIANGLE_CLUSTER_TEMPLATE: Self = Self(3);
    pub const INSTANTIATE_TRIANGLE_CLUSTER: Self = Self(4);
    pub const GET_CLUSTER_TEMPLATE_INDICES: Self = Self(5);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "MOVE_OBJECTS"),
            (Self(1), "BUILD_CLUSTERS_BOTTOM_LEVEL"),
            (Self(2), "BUILD_TRIANGLE_CLUSTER"),
            (Self(3), "BUILD_TRIANGLE_CLUSTER_TEMPLATE"),
            (Self(4), "INSTANTIATE_TRIANGLE_CLUSTER"),
            (Self(5), "GET_CLUSTER_TEMPLATE_INDICES"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ClusterAccelerationStructureOpTypeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "ClusterAccelerationStructureOpTypeNV",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ClusterAccelerationStructureTypeNV(pub i32);

impl ClusterAccelerationStructureTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CLUSTERS_BOTTOM_LEVEL: Self = Self(0);
    pub const TRIANGLE_CLUSTER: Self = Self(1);
    pub const TRIANGLE_CLUSTER_TEMPLATE: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "CLUSTERS_BOTTOM_LEVEL"),
            (Self(1), "TRIANGLE_CLUSTER"),
            (Self(2), "TRIANGLE_CLUSTER_TEMPLATE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ClusterAccelerationStructureTypeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ClusterAccelerationStructureTypeNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CoarseSampleOrderTypeNV(pub i32);

impl CoarseSampleOrderTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const CUSTOM: Self = Self(1);
    pub const PIXEL_MAJOR: Self = Self(2);
    pub const SAMPLE_MAJOR: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT"),
            (Self(1), "CUSTOM"),
            (Self(2), "PIXEL_MAJOR"),
            (Self(3), "SAMPLE_MAJOR"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for CoarseSampleOrderTypeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "CoarseSampleOrderTypeNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ColorSpaceKHR(pub i32);

impl ColorSpaceKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SRGB_NONLINEAR: Self = Self(0);
    pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    pub const DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    pub const BT709_LINEAR_EXT: Self = Self(1000104005);
    pub const BT709_NONLINEAR_EXT: Self = Self(1000104006);
    pub const BT2020_LINEAR_EXT: Self = Self(1000104007);
    pub const HDR10_ST2084_EXT: Self = Self(1000104008);
    pub const DOLBYVISION_EXT: Self = Self(1000104009);
    pub const HDR10_HLG_EXT: Self = Self(1000104010);
    pub const ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    pub const PASS_THROUGH_EXT: Self = Self(1000104013);
    pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    pub const DISPLAY_NATIVE_AMD: Self = Self(1000213000);
    pub const COLORSPACE_SRGB_NONLINEAR: Self = Self(0);
    pub const DCI_P3_LINEAR_EXT: Self = Self(1000104003);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "SRGB_NONLINEAR"),
            (Self(1000104001), "DISPLAY_P3_NONLINEAR_EXT"),
            (Self(1000104002), "EXTENDED_SRGB_LINEAR_EXT"),
            (Self(1000104003), "DISPLAY_P3_LINEAR_EXT"),
            (Self(1000104004), "DCI_P3_NONLINEAR_EXT"),
            (Self(1000104005), "BT709_LINEAR_EXT"),
            (Self(1000104006), "BT709_NONLINEAR_EXT"),
            (Self(1000104007), "BT2020_LINEAR_EXT"),
            (Self(1000104008), "HDR10_ST2084_EXT"),
            (Self(1000104009), "DOLBYVISION_EXT"),
            (Self(1000104010), "HDR10_HLG_EXT"),
            (Self(1000104011), "ADOBERGB_LINEAR_EXT"),
            (Self(1000104012), "ADOBERGB_NONLINEAR_EXT"),
            (Self(1000104013), "PASS_THROUGH_EXT"),
            (Self(1000104014), "EXTENDED_SRGB_NONLINEAR_EXT"),
            (Self(1000213000), "DISPLAY_NATIVE_AMD"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "COLORSPACE_SRGB_NONLINEAR"),
            (Self(1000104003), "DCI_P3_LINEAR_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ColorSpaceKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ColorSpaceKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "PRIMARY"), (Self(1), "SECONDARY")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for CommandBufferLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "CommandBufferLevel", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "NEVER"),
            (Self(1), "LESS"),
            (Self(2), "EQUAL"),
            (Self(3), "LESS_OR_EQUAL"),
            (Self(4), "GREATER"),
            (Self(5), "NOT_EQUAL"),
            (Self(6), "GREATER_OR_EQUAL"),
            (Self(7), "ALWAYS"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for CompareOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "CompareOp", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "IDENTITY"),
            (Self(1), "ZERO"),
            (Self(2), "ONE"),
            (Self(3), "R"),
            (Self(4), "G"),
            (Self(5), "B"),
            (Self(6), "A"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ComponentSwizzle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ComponentSwizzle", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ComponentTypeKHR(pub i32);

impl ComponentTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FLOAT16: Self = Self(0);
    pub const FLOAT32: Self = Self(1);
    pub const FLOAT64: Self = Self(2);
    pub const SINT8: Self = Self(3);
    pub const SINT16: Self = Self(4);
    pub const SINT32: Self = Self(5);
    pub const SINT64: Self = Self(6);
    pub const UINT8: Self = Self(7);
    pub const UINT16: Self = Self(8);
    pub const UINT32: Self = Self(9);
    pub const UINT64: Self = Self(10);
    pub const BFLOAT16: Self = Self(1000141000);
    pub const SINT8_PACKED_NV: Self = Self(1000491000);
    pub const UINT8_PACKED_NV: Self = Self(1000491001);
    pub const FLOAT8_E4M3_EXT: Self = Self(1000491002);
    pub const FLOAT8_E5M2_EXT: Self = Self(1000491003);
    pub const FLOAT6_E2M3_EXT: Self = Self(1000672000);
    pub const FLOAT6_E3M2_EXT: Self = Self(1000672001);
    pub const FLOAT4_E2M1_EXT: Self = Self(1000672002);
    pub const FLOAT8_UNSIGNED_E8M0_EXT: Self = Self(1000672003);
    pub const MXINT8_EXT: Self = Self(1000672004);
    pub const FLOAT16_NV: Self = Self(0);
    pub const FLOAT32_NV: Self = Self(1);
    pub const FLOAT64_NV: Self = Self(2);
    pub const SINT8_NV: Self = Self(3);
    pub const SINT16_NV: Self = Self(4);
    pub const SINT32_NV: Self = Self(5);
    pub const SINT64_NV: Self = Self(6);
    pub const UINT8_NV: Self = Self(7);
    pub const UINT16_NV: Self = Self(8);
    pub const UINT32_NV: Self = Self(9);
    pub const UINT64_NV: Self = Self(10);
    pub const FLOAT_E4M3_NV: Self = Self(1000491002);
    pub const FLOAT_E5M2_NV: Self = Self(1000491003);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "FLOAT16"),
            (Self(1), "FLOAT32"),
            (Self(2), "FLOAT64"),
            (Self(3), "SINT8"),
            (Self(4), "SINT16"),
            (Self(5), "SINT32"),
            (Self(6), "SINT64"),
            (Self(7), "UINT8"),
            (Self(8), "UINT16"),
            (Self(9), "UINT32"),
            (Self(10), "UINT64"),
            (Self(1000141000), "BFLOAT16"),
            (Self(1000491000), "SINT8_PACKED_NV"),
            (Self(1000491001), "UINT8_PACKED_NV"),
            (Self(1000491002), "FLOAT8_E4M3_EXT"),
            (Self(1000491003), "FLOAT8_E5M2_EXT"),
            (Self(1000672000), "FLOAT6_E2M3_EXT"),
            (Self(1000672001), "FLOAT6_E3M2_EXT"),
            (Self(1000672002), "FLOAT4_E2M1_EXT"),
            (Self(1000672003), "FLOAT8_UNSIGNED_E8M0_EXT"),
            (Self(1000672004), "MXINT8_EXT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "FLOAT16_NV"),
            (Self(1), "FLOAT32_NV"),
            (Self(2), "FLOAT64_NV"),
            (Self(3), "SINT8_NV"),
            (Self(4), "SINT16_NV"),
            (Self(5), "SINT32_NV"),
            (Self(6), "SINT64_NV"),
            (Self(7), "UINT8_NV"),
            (Self(8), "UINT16_NV"),
            (Self(9), "UINT32_NV"),
            (Self(10), "UINT64_NV"),
            (Self(1000491002), "FLOAT_E4M3_NV"),
            (Self(1000491003), "FLOAT_E5M2_NV"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ComponentTypeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ComponentTypeKHR", self.name(), self.0)
    }
}

pub type ComponentTypeNV = ComponentTypeKHR;

#[cfg(feature = "beta")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CompressedTriangleFormatAMDX(pub i32);

#[cfg(feature = "beta")]
impl CompressedTriangleFormatAMDX {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DGF1: Self = Self(0);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "DGF1")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

#[cfg(feature = "beta")]
impl core::fmt::Debug for CompressedTriangleFormatAMDX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "CompressedTriangleFormatAMDX", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ConservativeRasterizationModeEXT(pub i32);

impl ConservativeRasterizationModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DISABLED: Self = Self(0);
    pub const OVERESTIMATE: Self = Self(1);
    pub const UNDERESTIMATE: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DISABLED"),
            (Self(1), "OVERESTIMATE"),
            (Self(2), "UNDERESTIMATE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ConservativeRasterizationModeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ConservativeRasterizationModeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CooperativeVectorMatrixLayoutNV(pub i32);

impl CooperativeVectorMatrixLayoutNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ROW_MAJOR: Self = Self(0);
    pub const COLUMN_MAJOR: Self = Self(1);
    pub const INFERENCING_OPTIMAL: Self = Self(2);
    pub const TRAINING_OPTIMAL: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "ROW_MAJOR"),
            (Self(1), "COLUMN_MAJOR"),
            (Self(2), "INFERENCING_OPTIMAL"),
            (Self(3), "TRAINING_OPTIMAL"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for CooperativeVectorMatrixLayoutNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "CooperativeVectorMatrixLayoutNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CopyAccelerationStructureModeKHR(pub i32);

impl CopyAccelerationStructureModeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CLONE: Self = Self(0);
    pub const COMPACT: Self = Self(1);
    pub const SERIALIZE: Self = Self(2);
    pub const DESERIALIZE: Self = Self(3);
    pub const CLONE_NV: Self = Self(0);
    pub const COMPACT_NV: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "CLONE"),
            (Self(1), "COMPACT"),
            (Self(2), "SERIALIZE"),
            (Self(3), "DESERIALIZE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(0), "CLONE_NV"), (Self(1), "COMPACT_NV")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for CopyAccelerationStructureModeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "CopyAccelerationStructureModeKHR", self.name(), self.0)
    }
}

pub type CopyAccelerationStructureModeNV = CopyAccelerationStructureModeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CopyMicromapModeEXT(pub i32);

impl CopyMicromapModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CLONE: Self = Self(0);
    pub const SERIALIZE: Self = Self(1);
    pub const DESERIALIZE: Self = Self(2);
    pub const COMPACT: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "CLONE"),
            (Self(1), "SERIALIZE"),
            (Self(2), "DESERIALIZE"),
            (Self(3), "COMPACT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for CopyMicromapModeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "CopyMicromapModeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CoverageModulationModeNV(pub i32);

impl CoverageModulationModeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NONE: Self = Self(0);
    pub const RGB: Self = Self(1);
    pub const ALPHA: Self = Self(2);
    pub const RGBA: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "NONE"),
            (Self(1), "RGB"),
            (Self(2), "ALPHA"),
            (Self(3), "RGBA"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for CoverageModulationModeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "CoverageModulationModeNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CoverageReductionModeNV(pub i32);

impl CoverageReductionModeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MERGE: Self = Self(0);
    pub const TRUNCATE: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "MERGE"), (Self(1), "TRUNCATE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for CoverageReductionModeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "CoverageReductionModeNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CubicFilterWeightsQCOM(pub i32);

impl CubicFilterWeightsQCOM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CATMULL_ROM: Self = Self(0);
    pub const ZERO_TANGENT_CARDINAL: Self = Self(1);
    pub const B_SPLINE: Self = Self(2);
    pub const MITCHELL_NETRAVALI: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "CATMULL_ROM"),
            (Self(1), "ZERO_TANGENT_CARDINAL"),
            (Self(2), "B_SPLINE"),
            (Self(3), "MITCHELL_NETRAVALI"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for CubicFilterWeightsQCOM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "CubicFilterWeightsQCOM", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataGraphModelCacheTypeQCOM(pub i32);

impl DataGraphModelCacheTypeQCOM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const GENERIC_BINARY: Self = Self(0);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "GENERIC_BINARY")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DataGraphModelCacheTypeQCOM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DataGraphModelCacheTypeQCOM", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataGraphOpticalFlowPerformanceLevelARM(pub i32);

impl DataGraphOpticalFlowPerformanceLevelARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNKNOWN: Self = Self(0);
    pub const SLOW: Self = Self(1);
    pub const MEDIUM: Self = Self(2);
    pub const FAST: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNKNOWN"),
            (Self(1), "SLOW"),
            (Self(2), "MEDIUM"),
            (Self(3), "FAST"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DataGraphOpticalFlowPerformanceLevelARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "DataGraphOpticalFlowPerformanceLevelARM",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataGraphPipelineNodeConnectionTypeARM(pub i32);

impl DataGraphPipelineNodeConnectionTypeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPTICAL_FLOW_INPUT: Self = Self(1000631000);
    pub const OPTICAL_FLOW_REFERENCE: Self = Self(1000631001);
    pub const OPTICAL_FLOW_HINT: Self = Self(1000631002);
    pub const OPTICAL_FLOW_FLOW_VECTOR: Self = Self(1000631003);
    pub const OPTICAL_FLOW_COST: Self = Self(1000631004);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(1000631000), "OPTICAL_FLOW_INPUT"),
            (Self(1000631001), "OPTICAL_FLOW_REFERENCE"),
            (Self(1000631002), "OPTICAL_FLOW_HINT"),
            (Self(1000631003), "OPTICAL_FLOW_FLOW_VECTOR"),
            (Self(1000631004), "OPTICAL_FLOW_COST"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DataGraphPipelineNodeConnectionTypeARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "DataGraphPipelineNodeConnectionTypeARM",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataGraphPipelineNodeTypeARM(pub i32);

impl DataGraphPipelineNodeTypeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPTICAL_FLOW: Self = Self(1000631000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(1000631000), "OPTICAL_FLOW")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DataGraphPipelineNodeTypeARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DataGraphPipelineNodeTypeARM", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataGraphPipelinePropertyARM(pub i32);

impl DataGraphPipelinePropertyARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CREATION_LOG: Self = Self(0);
    pub const IDENTIFIER: Self = Self(1);
    pub const NEURAL_ACCELERATOR_DEBUG_DATABASE: Self = Self(1000676000);
    pub const NEURAL_ACCELERATOR_STATISTICS_INFO: Self = Self(1000676001);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "CREATION_LOG"),
            (Self(1), "IDENTIFIER"),
            (Self(1000676000), "NEURAL_ACCELERATOR_DEBUG_DATABASE"),
            (Self(1000676001), "NEURAL_ACCELERATOR_STATISTICS_INFO"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DataGraphPipelinePropertyARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DataGraphPipelinePropertyARM", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataGraphPipelineSessionBindPointARM(pub i32);

impl DataGraphPipelineSessionBindPointARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TRANSIENT: Self = Self(0);
    pub const OPTICAL_FLOW_CACHE: Self = Self(1000631001);
    pub const NEURAL_ACCELERATOR_STATISTICS: Self = Self(1000676000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "TRANSIENT"),
            (Self(1000631001), "OPTICAL_FLOW_CACHE"),
            (Self(1000676000), "NEURAL_ACCELERATOR_STATISTICS"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DataGraphPipelineSessionBindPointARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "DataGraphPipelineSessionBindPointARM",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataGraphPipelineSessionBindPointTypeARM(pub i32);

impl DataGraphPipelineSessionBindPointTypeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MEMORY: Self = Self(0);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "MEMORY")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DataGraphPipelineSessionBindPointTypeARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "DataGraphPipelineSessionBindPointTypeARM",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataGraphTOSALevelARM(pub i32);

impl DataGraphTOSALevelARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NONE: Self = Self(0);
    pub const _8K: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "NONE"), (Self(1), "_8K")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DataGraphTOSALevelARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DataGraphTOSALevelARM", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DebugReportObjectTypeEXT(pub i32);

impl DebugReportObjectTypeEXT {
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
    pub const SURFACE_KHR: Self = Self(26);
    pub const SWAPCHAIN_KHR: Self = Self(27);
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(28);
    pub const DISPLAY_KHR: Self = Self(29);
    pub const DISPLAY_MODE_KHR: Self = Self(30);
    pub const VALIDATION_CACHE_EXT: Self = Self(33);
    pub const CU_MODULE_NVX: Self = Self(1000029000);
    pub const CU_FUNCTION_NVX: Self = Self(1000029001);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    pub const CUDA_MODULE_NV: Self = Self(1000307000);
    pub const CUDA_FUNCTION_NV: Self = Self(1000307001);
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
    pub const DEBUG_REPORT: Self = Self(28);
    pub const VALIDATION_CACHE: Self = Self(33);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self(1000085000);
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self(1000156000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNKNOWN"),
            (Self(1), "INSTANCE"),
            (Self(2), "PHYSICAL_DEVICE"),
            (Self(3), "DEVICE"),
            (Self(4), "QUEUE"),
            (Self(5), "SEMAPHORE"),
            (Self(6), "COMMAND_BUFFER"),
            (Self(7), "FENCE"),
            (Self(8), "DEVICE_MEMORY"),
            (Self(9), "BUFFER"),
            (Self(10), "IMAGE"),
            (Self(11), "EVENT"),
            (Self(12), "QUERY_POOL"),
            (Self(13), "BUFFER_VIEW"),
            (Self(14), "IMAGE_VIEW"),
            (Self(15), "SHADER_MODULE"),
            (Self(16), "PIPELINE_CACHE"),
            (Self(17), "PIPELINE_LAYOUT"),
            (Self(18), "RENDER_PASS"),
            (Self(19), "PIPELINE"),
            (Self(20), "DESCRIPTOR_SET_LAYOUT"),
            (Self(21), "SAMPLER"),
            (Self(22), "DESCRIPTOR_POOL"),
            (Self(23), "DESCRIPTOR_SET"),
            (Self(24), "FRAMEBUFFER"),
            (Self(25), "COMMAND_POOL"),
            (Self(26), "SURFACE_KHR"),
            (Self(27), "SWAPCHAIN_KHR"),
            (Self(28), "DEBUG_REPORT_CALLBACK_EXT"),
            (Self(29), "DISPLAY_KHR"),
            (Self(30), "DISPLAY_MODE_KHR"),
            (Self(33), "VALIDATION_CACHE_EXT"),
            (Self(1000029000), "CU_MODULE_NVX"),
            (Self(1000029001), "CU_FUNCTION_NVX"),
            (Self(1000085000), "DESCRIPTOR_UPDATE_TEMPLATE"),
            (Self(1000150000), "ACCELERATION_STRUCTURE_KHR"),
            (Self(1000156000), "SAMPLER_YCBCR_CONVERSION"),
            (Self(1000165000), "ACCELERATION_STRUCTURE_NV"),
            (Self(1000307000), "CUDA_MODULE_NV"),
            (Self(1000307001), "CUDA_FUNCTION_NV"),
            (Self(1000366000), "BUFFER_COLLECTION_FUCHSIA"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(28), "DEBUG_REPORT"),
            (Self(33), "VALIDATION_CACHE"),
            (Self(1000085000), "DESCRIPTOR_UPDATE_TEMPLATE_KHR"),
            (Self(1000156000), "SAMPLER_YCBCR_CONVERSION_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DebugReportObjectTypeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DebugReportObjectTypeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DefaultVertexAttributeValueKHR(pub i32);

impl DefaultVertexAttributeValueKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ZERO_ZERO_ZERO_ZERO: Self = Self(0);
    pub const ZERO_ZERO_ZERO_ONE: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "ZERO_ZERO_ZERO_ZERO"),
            (Self(1), "ZERO_ZERO_ZERO_ONE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DefaultVertexAttributeValueKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DefaultVertexAttributeValueKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DepthBiasRepresentationEXT(pub i32);

impl DepthBiasRepresentationEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const LEAST_REPRESENTABLE_VALUE_FORMAT: Self = Self(0);
    pub const LEAST_REPRESENTABLE_VALUE_FORCE_UNORM: Self = Self(1);
    pub const FLOAT: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "LEAST_REPRESENTABLE_VALUE_FORMAT"),
            (Self(1), "LEAST_REPRESENTABLE_VALUE_FORCE_UNORM"),
            (Self(2), "FLOAT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DepthBiasRepresentationEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DepthBiasRepresentationEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DepthClampModeEXT(pub i32);

impl DepthClampModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VIEWPORT_RANGE: Self = Self(0);
    pub const USER_DEFINED_RANGE: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "VIEWPORT_RANGE"), (Self(1), "USER_DEFINED_RANGE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DepthClampModeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DepthClampModeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DescriptorMappingSourceEXT(pub i32);

impl DescriptorMappingSourceEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const HEAP_WITH_CONSTANT_OFFSET: Self = Self(0);
    pub const HEAP_WITH_PUSH_INDEX: Self = Self(1);
    pub const HEAP_WITH_INDIRECT_INDEX: Self = Self(2);
    pub const HEAP_WITH_INDIRECT_INDEX_ARRAY: Self = Self(3);
    pub const RESOURCE_HEAP_DATA: Self = Self(4);
    pub const PUSH_DATA: Self = Self(5);
    pub const PUSH_ADDRESS: Self = Self(6);
    pub const INDIRECT_ADDRESS: Self = Self(7);
    pub const HEAP_WITH_SHADER_RECORD_INDEX: Self = Self(8);
    pub const SHADER_RECORD_DATA: Self = Self(9);
    pub const SHADER_RECORD_ADDRESS: Self = Self(10);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "HEAP_WITH_CONSTANT_OFFSET"),
            (Self(1), "HEAP_WITH_PUSH_INDEX"),
            (Self(2), "HEAP_WITH_INDIRECT_INDEX"),
            (Self(3), "HEAP_WITH_INDIRECT_INDEX_ARRAY"),
            (Self(4), "RESOURCE_HEAP_DATA"),
            (Self(5), "PUSH_DATA"),
            (Self(6), "PUSH_ADDRESS"),
            (Self(7), "INDIRECT_ADDRESS"),
            (Self(8), "HEAP_WITH_SHADER_RECORD_INDEX"),
            (Self(9), "SHADER_RECORD_DATA"),
            (Self(10), "SHADER_RECORD_ADDRESS"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DescriptorMappingSourceEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DescriptorMappingSourceEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const INLINE_UNIFORM_BLOCK_EXT: Self = Self(1000138000);
    pub const MUTABLE_VALVE: Self = Self(1000351000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "SAMPLER"),
            (Self(1), "COMBINED_IMAGE_SAMPLER"),
            (Self(2), "SAMPLED_IMAGE"),
            (Self(3), "STORAGE_IMAGE"),
            (Self(4), "UNIFORM_TEXEL_BUFFER"),
            (Self(5), "STORAGE_TEXEL_BUFFER"),
            (Self(6), "UNIFORM_BUFFER"),
            (Self(7), "STORAGE_BUFFER"),
            (Self(8), "UNIFORM_BUFFER_DYNAMIC"),
            (Self(9), "STORAGE_BUFFER_DYNAMIC"),
            (Self(10), "INPUT_ATTACHMENT"),
            (Self(1000138000), "INLINE_UNIFORM_BLOCK"),
            (Self(1000150000), "ACCELERATION_STRUCTURE_KHR"),
            (Self(1000165000), "ACCELERATION_STRUCTURE_NV"),
            (Self(1000351000), "MUTABLE_EXT"),
            (Self(1000440000), "SAMPLE_WEIGHT_IMAGE_QCOM"),
            (Self(1000440001), "BLOCK_MATCH_IMAGE_QCOM"),
            (Self(1000460000), "TENSOR_ARM"),
            (Self(1000570000), "PARTITIONED_ACCELERATION_STRUCTURE_NV"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(1000138000), "INLINE_UNIFORM_BLOCK_EXT"),
            (Self(1000351000), "MUTABLE_VALVE"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DescriptorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DescriptorType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const DESCRIPTOR_SET_KHR: Self = Self(0);
    pub const PUSH_DESCRIPTORS_KHR: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "DESCRIPTOR_SET"), (Self(1), "PUSH_DESCRIPTORS")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DESCRIPTOR_SET_KHR"),
            (Self(1), "PUSH_DESCRIPTORS_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DescriptorUpdateTemplateType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DescriptorUpdateTemplateType", self.name(), self.0)
    }
}

pub type DescriptorUpdateTemplateTypeKHR = DescriptorUpdateTemplateType;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceAddressBindingTypeEXT(pub i32);

impl DeviceAddressBindingTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BIND: Self = Self(0);
    pub const UNBIND: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "BIND"), (Self(1), "UNBIND")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DeviceAddressBindingTypeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DeviceAddressBindingTypeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceEventTypeEXT(pub i32);

impl DeviceEventTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DISPLAY_HOTPLUG: Self = Self(0);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "DISPLAY_HOTPLUG")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DeviceEventTypeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DeviceEventTypeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceFaultAddressTypeKHR(pub i32);

impl DeviceFaultAddressTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NONE: Self = Self(0);
    pub const READ_INVALID: Self = Self(1);
    pub const WRITE_INVALID: Self = Self(2);
    pub const EXECUTE_INVALID: Self = Self(3);
    pub const INSTRUCTION_POINTER_UNKNOWN: Self = Self(4);
    pub const INSTRUCTION_POINTER_INVALID: Self = Self(5);
    pub const INSTRUCTION_POINTER_FAULT: Self = Self(6);
    pub const NONE_EXT: Self = Self(0);
    pub const READ_INVALID_EXT: Self = Self(1);
    pub const WRITE_INVALID_EXT: Self = Self(2);
    pub const EXECUTE_INVALID_EXT: Self = Self(3);
    pub const INSTRUCTION_POINTER_UNKNOWN_EXT: Self = Self(4);
    pub const INSTRUCTION_POINTER_INVALID_EXT: Self = Self(5);
    pub const INSTRUCTION_POINTER_FAULT_EXT: Self = Self(6);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "NONE"),
            (Self(1), "READ_INVALID"),
            (Self(2), "WRITE_INVALID"),
            (Self(3), "EXECUTE_INVALID"),
            (Self(4), "INSTRUCTION_POINTER_UNKNOWN"),
            (Self(5), "INSTRUCTION_POINTER_INVALID"),
            (Self(6), "INSTRUCTION_POINTER_FAULT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "NONE_EXT"),
            (Self(1), "READ_INVALID_EXT"),
            (Self(2), "WRITE_INVALID_EXT"),
            (Self(3), "EXECUTE_INVALID_EXT"),
            (Self(4), "INSTRUCTION_POINTER_UNKNOWN_EXT"),
            (Self(5), "INSTRUCTION_POINTER_INVALID_EXT"),
            (Self(6), "INSTRUCTION_POINTER_FAULT_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DeviceFaultAddressTypeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DeviceFaultAddressTypeKHR", self.name(), self.0)
    }
}

pub type DeviceFaultAddressTypeEXT = DeviceFaultAddressTypeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceFaultVendorBinaryHeaderVersionKHR(pub i32);

impl DeviceFaultVendorBinaryHeaderVersionKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ONE: Self = Self(1);
    pub const ONE_EXT: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(1), "ONE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(1), "ONE_EXT")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DeviceFaultVendorBinaryHeaderVersionKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "DeviceFaultVendorBinaryHeaderVersionKHR",
            self.name(),
            self.0,
        )
    }
}

pub type DeviceFaultVendorBinaryHeaderVersionEXT = DeviceFaultVendorBinaryHeaderVersionKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceMemoryReportEventTypeEXT(pub i32);

impl DeviceMemoryReportEventTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ALLOCATE: Self = Self(0);
    pub const FREE: Self = Self(1);
    pub const IMPORT: Self = Self(2);
    pub const UNIMPORT: Self = Self(3);
    pub const ALLOCATION_FAILED: Self = Self(4);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "ALLOCATE"),
            (Self(1), "FREE"),
            (Self(2), "IMPORT"),
            (Self(3), "UNIMPORT"),
            (Self(4), "ALLOCATION_FAILED"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DeviceMemoryReportEventTypeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DeviceMemoryReportEventTypeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DirectDriverLoadingModeLUNARG(pub i32);

impl DirectDriverLoadingModeLUNARG {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const EXCLUSIVE: Self = Self(0);
    pub const INCLUSIVE: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "EXCLUSIVE"), (Self(1), "INCLUSIVE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DirectDriverLoadingModeLUNARG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DirectDriverLoadingModeLUNARG", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DiscardRectangleModeEXT(pub i32);

impl DiscardRectangleModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INCLUSIVE: Self = Self(0);
    pub const EXCLUSIVE: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "INCLUSIVE"), (Self(1), "EXCLUSIVE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DiscardRectangleModeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DiscardRectangleModeEXT", self.name(), self.0)
    }
}

#[cfg(feature = "beta")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DisplacementMicromapFormatNV(pub i32);

#[cfg(feature = "beta")]
impl DisplacementMicromapFormatNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _64_TRIANGLES_64_BYTES: Self = Self(1);
    pub const _256_TRIANGLES_128_BYTES: Self = Self(2);
    pub const _1024_TRIANGLES_128_BYTES: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(1), "_64_TRIANGLES_64_BYTES"),
            (Self(2), "_256_TRIANGLES_128_BYTES"),
            (Self(3), "_1024_TRIANGLES_128_BYTES"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

#[cfg(feature = "beta")]
impl core::fmt::Debug for DisplacementMicromapFormatNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DisplacementMicromapFormatNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DisplayEventTypeEXT(pub i32);

impl DisplayEventTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FIRST_PIXEL_OUT: Self = Self(0);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "FIRST_PIXEL_OUT")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DisplayEventTypeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DisplayEventTypeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DisplayPowerStateEXT(pub i32);

impl DisplayPowerStateEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OFF: Self = Self(0);
    pub const SUSPEND: Self = Self(1);
    pub const ON: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "OFF"), (Self(1), "SUSPEND"), (Self(2), "ON")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DisplayPowerStateEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DisplayPowerStateEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DisplaySurfaceStereoTypeNV(pub i32);

impl DisplaySurfaceStereoTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NONE: Self = Self(0);
    pub const ONBOARD_DIN: Self = Self(1);
    pub const HDMI_3D: Self = Self(2);
    pub const INBAND_DISPLAYPORT: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "NONE"),
            (Self(1), "ONBOARD_DIN"),
            (Self(2), "HDMI_3D"),
            (Self(3), "INBAND_DISPLAYPORT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DisplaySurfaceStereoTypeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DisplaySurfaceStereoTypeNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const MESA_GFXSTREAM: Self = Self(29);
    pub const APE_SOFT: Self = Self(30);
    pub const RESERVED_31: Self = Self(31);
    pub const AMD_PROPRIETARY_KHR: Self = Self(1);
    pub const AMD_OPEN_SOURCE_KHR: Self = Self(2);
    pub const MESA_RADV_KHR: Self = Self(3);
    pub const NVIDIA_PROPRIETARY_KHR: Self = Self(4);
    pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self(5);
    pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self(6);
    pub const IMAGINATION_PROPRIETARY_KHR: Self = Self(7);
    pub const QUALCOMM_PROPRIETARY_KHR: Self = Self(8);
    pub const ARM_PROPRIETARY_KHR: Self = Self(9);
    pub const GOOGLE_SWIFTSHADER_KHR: Self = Self(10);
    pub const GGP_PROPRIETARY_KHR: Self = Self(11);
    pub const BROADCOM_PROPRIETARY_KHR: Self = Self(12);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(1), "AMD_PROPRIETARY"),
            (Self(2), "AMD_OPEN_SOURCE"),
            (Self(3), "MESA_RADV"),
            (Self(4), "NVIDIA_PROPRIETARY"),
            (Self(5), "INTEL_PROPRIETARY_WINDOWS"),
            (Self(6), "INTEL_OPEN_SOURCE_MESA"),
            (Self(7), "IMAGINATION_PROPRIETARY"),
            (Self(8), "QUALCOMM_PROPRIETARY"),
            (Self(9), "ARM_PROPRIETARY"),
            (Self(10), "GOOGLE_SWIFTSHADER"),
            (Self(11), "GGP_PROPRIETARY"),
            (Self(12), "BROADCOM_PROPRIETARY"),
            (Self(13), "MESA_LLVMPIPE"),
            (Self(14), "MOLTENVK"),
            (Self(15), "COREAVI_PROPRIETARY"),
            (Self(16), "JUICE_PROPRIETARY"),
            (Self(17), "VERISILICON_PROPRIETARY"),
            (Self(18), "MESA_TURNIP"),
            (Self(19), "MESA_V3DV"),
            (Self(20), "MESA_PANVK"),
            (Self(21), "SAMSUNG_PROPRIETARY"),
            (Self(22), "MESA_VENUS"),
            (Self(23), "MESA_DOZEN"),
            (Self(24), "MESA_NVK"),
            (Self(25), "IMAGINATION_OPEN_SOURCE_MESA"),
            (Self(26), "MESA_HONEYKRISP"),
            (Self(27), "VULKAN_SC_EMULATION_ON_VULKAN"),
            (Self(28), "MESA_KOSMICKRISP"),
            (Self(29), "MESA_GFXSTREAM"),
            (Self(30), "APE_SOFT"),
            (Self(31), "RESERVED_31"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(1), "AMD_PROPRIETARY_KHR"),
            (Self(2), "AMD_OPEN_SOURCE_KHR"),
            (Self(3), "MESA_RADV_KHR"),
            (Self(4), "NVIDIA_PROPRIETARY_KHR"),
            (Self(5), "INTEL_PROPRIETARY_WINDOWS_KHR"),
            (Self(6), "INTEL_OPEN_SOURCE_MESA_KHR"),
            (Self(7), "IMAGINATION_PROPRIETARY_KHR"),
            (Self(8), "QUALCOMM_PROPRIETARY_KHR"),
            (Self(9), "ARM_PROPRIETARY_KHR"),
            (Self(10), "GOOGLE_SWIFTSHADER_KHR"),
            (Self(11), "GGP_PROPRIETARY_KHR"),
            (Self(12), "BROADCOM_PROPRIETARY_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DriverId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DriverId", self.name(), self.0)
    }
}

pub type DriverIdKHR = DriverId;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const LINE_STIPPLE_EXT: Self = Self(1000259000);
    pub const LINE_STIPPLE_KHR: Self = Self(1000259000);
    pub const CULL_MODE_EXT: Self = Self(1000267000);
    pub const FRONT_FACE_EXT: Self = Self(1000267001);
    pub const PRIMITIVE_TOPOLOGY_EXT: Self = Self(1000267002);
    pub const VIEWPORT_WITH_COUNT_EXT: Self = Self(1000267003);
    pub const SCISSOR_WITH_COUNT_EXT: Self = Self(1000267004);
    pub const VERTEX_INPUT_BINDING_STRIDE_EXT: Self = Self(1000267005);
    pub const DEPTH_TEST_ENABLE_EXT: Self = Self(1000267006);
    pub const DEPTH_WRITE_ENABLE_EXT: Self = Self(1000267007);
    pub const DEPTH_COMPARE_OP_EXT: Self = Self(1000267008);
    pub const DEPTH_BOUNDS_TEST_ENABLE_EXT: Self = Self(1000267009);
    pub const STENCIL_TEST_ENABLE_EXT: Self = Self(1000267010);
    pub const STENCIL_OP_EXT: Self = Self(1000267011);
    pub const RASTERIZER_DISCARD_ENABLE_EXT: Self = Self(1000377001);
    pub const DEPTH_BIAS_ENABLE_EXT: Self = Self(1000377002);
    pub const PRIMITIVE_RESTART_ENABLE_EXT: Self = Self(1000377004);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "VIEWPORT"),
            (Self(1), "SCISSOR"),
            (Self(2), "LINE_WIDTH"),
            (Self(3), "DEPTH_BIAS"),
            (Self(4), "BLEND_CONSTANTS"),
            (Self(5), "DEPTH_BOUNDS"),
            (Self(6), "STENCIL_COMPARE_MASK"),
            (Self(7), "STENCIL_WRITE_MASK"),
            (Self(8), "STENCIL_REFERENCE"),
            (Self(1000087000), "VIEWPORT_W_SCALING_NV"),
            (Self(1000099000), "DISCARD_RECTANGLE_EXT"),
            (Self(1000099001), "DISCARD_RECTANGLE_ENABLE_EXT"),
            (Self(1000099002), "DISCARD_RECTANGLE_MODE_EXT"),
            (Self(1000143000), "SAMPLE_LOCATIONS_EXT"),
            (Self(1000164004), "VIEWPORT_SHADING_RATE_PALETTE_NV"),
            (Self(1000164006), "VIEWPORT_COARSE_SAMPLE_ORDER_NV"),
            (Self(1000205000), "EXCLUSIVE_SCISSOR_ENABLE_NV"),
            (Self(1000205001), "EXCLUSIVE_SCISSOR_NV"),
            (Self(1000226000), "FRAGMENT_SHADING_RATE_KHR"),
            (Self(1000259000), "LINE_STIPPLE"),
            (Self(1000267000), "CULL_MODE"),
            (Self(1000267001), "FRONT_FACE"),
            (Self(1000267002), "PRIMITIVE_TOPOLOGY"),
            (Self(1000267003), "VIEWPORT_WITH_COUNT"),
            (Self(1000267004), "SCISSOR_WITH_COUNT"),
            (Self(1000267005), "VERTEX_INPUT_BINDING_STRIDE"),
            (Self(1000267006), "DEPTH_TEST_ENABLE"),
            (Self(1000267007), "DEPTH_WRITE_ENABLE"),
            (Self(1000267008), "DEPTH_COMPARE_OP"),
            (Self(1000267009), "DEPTH_BOUNDS_TEST_ENABLE"),
            (Self(1000267010), "STENCIL_TEST_ENABLE"),
            (Self(1000267011), "STENCIL_OP"),
            (Self(1000347000), "RAY_TRACING_PIPELINE_STACK_SIZE_KHR"),
            (Self(1000352000), "VERTEX_INPUT_EXT"),
            (Self(1000377000), "PATCH_CONTROL_POINTS_EXT"),
            (Self(1000377001), "RASTERIZER_DISCARD_ENABLE"),
            (Self(1000377002), "DEPTH_BIAS_ENABLE"),
            (Self(1000377003), "LOGIC_OP_EXT"),
            (Self(1000377004), "PRIMITIVE_RESTART_ENABLE"),
            (Self(1000381000), "COLOR_WRITE_ENABLE_EXT"),
            (Self(1000455002), "TESSELLATION_DOMAIN_ORIGIN_EXT"),
            (Self(1000455003), "DEPTH_CLAMP_ENABLE_EXT"),
            (Self(1000455004), "POLYGON_MODE_EXT"),
            (Self(1000455005), "RASTERIZATION_SAMPLES_EXT"),
            (Self(1000455006), "SAMPLE_MASK_EXT"),
            (Self(1000455007), "ALPHA_TO_COVERAGE_ENABLE_EXT"),
            (Self(1000455008), "ALPHA_TO_ONE_ENABLE_EXT"),
            (Self(1000455009), "LOGIC_OP_ENABLE_EXT"),
            (Self(1000455010), "COLOR_BLEND_ENABLE_EXT"),
            (Self(1000455011), "COLOR_BLEND_EQUATION_EXT"),
            (Self(1000455012), "COLOR_WRITE_MASK_EXT"),
            (Self(1000455013), "RASTERIZATION_STREAM_EXT"),
            (Self(1000455014), "CONSERVATIVE_RASTERIZATION_MODE_EXT"),
            (Self(1000455015), "EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT"),
            (Self(1000455016), "DEPTH_CLIP_ENABLE_EXT"),
            (Self(1000455017), "SAMPLE_LOCATIONS_ENABLE_EXT"),
            (Self(1000455018), "COLOR_BLEND_ADVANCED_EXT"),
            (Self(1000455019), "PROVOKING_VERTEX_MODE_EXT"),
            (Self(1000455020), "LINE_RASTERIZATION_MODE_EXT"),
            (Self(1000455021), "LINE_STIPPLE_ENABLE_EXT"),
            (Self(1000455022), "DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT"),
            (Self(1000455023), "VIEWPORT_W_SCALING_ENABLE_NV"),
            (Self(1000455024), "VIEWPORT_SWIZZLE_NV"),
            (Self(1000455025), "COVERAGE_TO_COLOR_ENABLE_NV"),
            (Self(1000455026), "COVERAGE_TO_COLOR_LOCATION_NV"),
            (Self(1000455027), "COVERAGE_MODULATION_MODE_NV"),
            (Self(1000455028), "COVERAGE_MODULATION_TABLE_ENABLE_NV"),
            (Self(1000455029), "COVERAGE_MODULATION_TABLE_NV"),
            (Self(1000455030), "SHADING_RATE_IMAGE_ENABLE_NV"),
            (Self(1000455031), "REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV"),
            (Self(1000455032), "COVERAGE_REDUCTION_MODE_NV"),
            (Self(1000524000), "ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT"),
            (Self(1000582000), "DEPTH_CLAMP_RANGE_EXT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(1000259000), "LINE_STIPPLE_EXT"),
            (Self(1000259000), "LINE_STIPPLE_KHR"),
            (Self(1000267000), "CULL_MODE_EXT"),
            (Self(1000267001), "FRONT_FACE_EXT"),
            (Self(1000267002), "PRIMITIVE_TOPOLOGY_EXT"),
            (Self(1000267003), "VIEWPORT_WITH_COUNT_EXT"),
            (Self(1000267004), "SCISSOR_WITH_COUNT_EXT"),
            (Self(1000267005), "VERTEX_INPUT_BINDING_STRIDE_EXT"),
            (Self(1000267006), "DEPTH_TEST_ENABLE_EXT"),
            (Self(1000267007), "DEPTH_WRITE_ENABLE_EXT"),
            (Self(1000267008), "DEPTH_COMPARE_OP_EXT"),
            (Self(1000267009), "DEPTH_BOUNDS_TEST_ENABLE_EXT"),
            (Self(1000267010), "STENCIL_TEST_ENABLE_EXT"),
            (Self(1000267011), "STENCIL_OP_EXT"),
            (Self(1000377001), "RASTERIZER_DISCARD_ENABLE_EXT"),
            (Self(1000377002), "DEPTH_BIAS_ENABLE_EXT"),
            (Self(1000377004), "PRIMITIVE_RESTART_ENABLE_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for DynamicState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "DynamicState", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const CUBIC_IMG: Self = Self(1000015000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "NEAREST"),
            (Self(1), "LINEAR"),
            (Self(1000015000), "CUBIC_EXT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(1000015000), "CUBIC_IMG")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for Filter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "Filter", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const ASTC_4x4_SFLOAT_BLOCK_EXT: Self = Self(1000066000);
    pub const ASTC_5x4_SFLOAT_BLOCK_EXT: Self = Self(1000066001);
    pub const ASTC_5x5_SFLOAT_BLOCK_EXT: Self = Self(1000066002);
    pub const ASTC_6x5_SFLOAT_BLOCK_EXT: Self = Self(1000066003);
    pub const ASTC_6x6_SFLOAT_BLOCK_EXT: Self = Self(1000066004);
    pub const ASTC_8x5_SFLOAT_BLOCK_EXT: Self = Self(1000066005);
    pub const ASTC_8x6_SFLOAT_BLOCK_EXT: Self = Self(1000066006);
    pub const ASTC_8x8_SFLOAT_BLOCK_EXT: Self = Self(1000066007);
    pub const ASTC_10x5_SFLOAT_BLOCK_EXT: Self = Self(1000066008);
    pub const ASTC_10x6_SFLOAT_BLOCK_EXT: Self = Self(1000066009);
    pub const ASTC_10x8_SFLOAT_BLOCK_EXT: Self = Self(1000066010);
    pub const ASTC_10x10_SFLOAT_BLOCK_EXT: Self = Self(1000066011);
    pub const ASTC_12x10_SFLOAT_BLOCK_EXT: Self = Self(1000066012);
    pub const ASTC_12x12_SFLOAT_BLOCK_EXT: Self = Self(1000066013);
    pub const G8B8G8R8_422_UNORM_KHR: Self = Self(1000156000);
    pub const B8G8R8G8_422_UNORM_KHR: Self = Self(1000156001);
    pub const G8_B8_R8_3PLANE_420_UNORM_KHR: Self = Self(1000156002);
    pub const G8_B8R8_2PLANE_420_UNORM_KHR: Self = Self(1000156003);
    pub const G8_B8_R8_3PLANE_422_UNORM_KHR: Self = Self(1000156004);
    pub const G8_B8R8_2PLANE_422_UNORM_KHR: Self = Self(1000156005);
    pub const G8_B8_R8_3PLANE_444_UNORM_KHR: Self = Self(1000156006);
    pub const R10X6_UNORM_PACK16_KHR: Self = Self(1000156007);
    pub const R10X6G10X6_UNORM_2PACK16_KHR: Self = Self(1000156008);
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: Self = Self(1000156009);
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: Self = Self(1000156010);
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: Self = Self(1000156011);
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156012);
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156013);
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156014);
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156015);
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: Self = Self(1000156016);
    pub const R12X4_UNORM_PACK16_KHR: Self = Self(1000156017);
    pub const R12X4G12X4_UNORM_2PACK16_KHR: Self = Self(1000156018);
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: Self = Self(1000156019);
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: Self = Self(1000156020);
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: Self = Self(1000156021);
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156022);
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156023);
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156024);
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156025);
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: Self = Self(1000156026);
    pub const G16B16G16R16_422_UNORM_KHR: Self = Self(1000156027);
    pub const B16G16R16G16_422_UNORM_KHR: Self = Self(1000156028);
    pub const G16_B16_R16_3PLANE_420_UNORM_KHR: Self = Self(1000156029);
    pub const G16_B16R16_2PLANE_420_UNORM_KHR: Self = Self(1000156030);
    pub const G16_B16_R16_3PLANE_422_UNORM_KHR: Self = Self(1000156031);
    pub const G16_B16R16_2PLANE_422_UNORM_KHR: Self = Self(1000156032);
    pub const G16_B16_R16_3PLANE_444_UNORM_KHR: Self = Self(1000156033);
    pub const G8_B8R8_2PLANE_444_UNORM_EXT: Self = Self(1000330000);
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: Self = Self(1000330001);
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: Self = Self(1000330002);
    pub const G16_B16R16_2PLANE_444_UNORM_EXT: Self = Self(1000330003);
    pub const A4R4G4B4_UNORM_PACK16_EXT: Self = Self(1000340000);
    pub const A4B4G4R4_UNORM_PACK16_EXT: Self = Self(1000340001);
    pub const R16G16_S10_5_NV: Self = Self(1000464000);
    pub const A1B5G5R5_UNORM_PACK16_KHR: Self = Self(1000470000);
    pub const A8_UNORM_KHR: Self = Self(1000470001);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNDEFINED"),
            (Self(1), "R4G4_UNORM_PACK8"),
            (Self(2), "R4G4B4A4_UNORM_PACK16"),
            (Self(3), "B4G4R4A4_UNORM_PACK16"),
            (Self(4), "R5G6B5_UNORM_PACK16"),
            (Self(5), "B5G6R5_UNORM_PACK16"),
            (Self(6), "R5G5B5A1_UNORM_PACK16"),
            (Self(7), "B5G5R5A1_UNORM_PACK16"),
            (Self(8), "A1R5G5B5_UNORM_PACK16"),
            (Self(9), "R8_UNORM"),
            (Self(10), "R8_SNORM"),
            (Self(11), "R8_USCALED"),
            (Self(12), "R8_SSCALED"),
            (Self(13), "R8_UINT"),
            (Self(14), "R8_SINT"),
            (Self(15), "R8_SRGB"),
            (Self(16), "R8G8_UNORM"),
            (Self(17), "R8G8_SNORM"),
            (Self(18), "R8G8_USCALED"),
            (Self(19), "R8G8_SSCALED"),
            (Self(20), "R8G8_UINT"),
            (Self(21), "R8G8_SINT"),
            (Self(22), "R8G8_SRGB"),
            (Self(23), "R8G8B8_UNORM"),
            (Self(24), "R8G8B8_SNORM"),
            (Self(25), "R8G8B8_USCALED"),
            (Self(26), "R8G8B8_SSCALED"),
            (Self(27), "R8G8B8_UINT"),
            (Self(28), "R8G8B8_SINT"),
            (Self(29), "R8G8B8_SRGB"),
            (Self(30), "B8G8R8_UNORM"),
            (Self(31), "B8G8R8_SNORM"),
            (Self(32), "B8G8R8_USCALED"),
            (Self(33), "B8G8R8_SSCALED"),
            (Self(34), "B8G8R8_UINT"),
            (Self(35), "B8G8R8_SINT"),
            (Self(36), "B8G8R8_SRGB"),
            (Self(37), "R8G8B8A8_UNORM"),
            (Self(38), "R8G8B8A8_SNORM"),
            (Self(39), "R8G8B8A8_USCALED"),
            (Self(40), "R8G8B8A8_SSCALED"),
            (Self(41), "R8G8B8A8_UINT"),
            (Self(42), "R8G8B8A8_SINT"),
            (Self(43), "R8G8B8A8_SRGB"),
            (Self(44), "B8G8R8A8_UNORM"),
            (Self(45), "B8G8R8A8_SNORM"),
            (Self(46), "B8G8R8A8_USCALED"),
            (Self(47), "B8G8R8A8_SSCALED"),
            (Self(48), "B8G8R8A8_UINT"),
            (Self(49), "B8G8R8A8_SINT"),
            (Self(50), "B8G8R8A8_SRGB"),
            (Self(51), "A8B8G8R8_UNORM_PACK32"),
            (Self(52), "A8B8G8R8_SNORM_PACK32"),
            (Self(53), "A8B8G8R8_USCALED_PACK32"),
            (Self(54), "A8B8G8R8_SSCALED_PACK32"),
            (Self(55), "A8B8G8R8_UINT_PACK32"),
            (Self(56), "A8B8G8R8_SINT_PACK32"),
            (Self(57), "A8B8G8R8_SRGB_PACK32"),
            (Self(58), "A2R10G10B10_UNORM_PACK32"),
            (Self(59), "A2R10G10B10_SNORM_PACK32"),
            (Self(60), "A2R10G10B10_USCALED_PACK32"),
            (Self(61), "A2R10G10B10_SSCALED_PACK32"),
            (Self(62), "A2R10G10B10_UINT_PACK32"),
            (Self(63), "A2R10G10B10_SINT_PACK32"),
            (Self(64), "A2B10G10R10_UNORM_PACK32"),
            (Self(65), "A2B10G10R10_SNORM_PACK32"),
            (Self(66), "A2B10G10R10_USCALED_PACK32"),
            (Self(67), "A2B10G10R10_SSCALED_PACK32"),
            (Self(68), "A2B10G10R10_UINT_PACK32"),
            (Self(69), "A2B10G10R10_SINT_PACK32"),
            (Self(70), "R16_UNORM"),
            (Self(71), "R16_SNORM"),
            (Self(72), "R16_USCALED"),
            (Self(73), "R16_SSCALED"),
            (Self(74), "R16_UINT"),
            (Self(75), "R16_SINT"),
            (Self(76), "R16_SFLOAT"),
            (Self(77), "R16G16_UNORM"),
            (Self(78), "R16G16_SNORM"),
            (Self(79), "R16G16_USCALED"),
            (Self(80), "R16G16_SSCALED"),
            (Self(81), "R16G16_UINT"),
            (Self(82), "R16G16_SINT"),
            (Self(83), "R16G16_SFLOAT"),
            (Self(84), "R16G16B16_UNORM"),
            (Self(85), "R16G16B16_SNORM"),
            (Self(86), "R16G16B16_USCALED"),
            (Self(87), "R16G16B16_SSCALED"),
            (Self(88), "R16G16B16_UINT"),
            (Self(89), "R16G16B16_SINT"),
            (Self(90), "R16G16B16_SFLOAT"),
            (Self(91), "R16G16B16A16_UNORM"),
            (Self(92), "R16G16B16A16_SNORM"),
            (Self(93), "R16G16B16A16_USCALED"),
            (Self(94), "R16G16B16A16_SSCALED"),
            (Self(95), "R16G16B16A16_UINT"),
            (Self(96), "R16G16B16A16_SINT"),
            (Self(97), "R16G16B16A16_SFLOAT"),
            (Self(98), "R32_UINT"),
            (Self(99), "R32_SINT"),
            (Self(100), "R32_SFLOAT"),
            (Self(101), "R32G32_UINT"),
            (Self(102), "R32G32_SINT"),
            (Self(103), "R32G32_SFLOAT"),
            (Self(104), "R32G32B32_UINT"),
            (Self(105), "R32G32B32_SINT"),
            (Self(106), "R32G32B32_SFLOAT"),
            (Self(107), "R32G32B32A32_UINT"),
            (Self(108), "R32G32B32A32_SINT"),
            (Self(109), "R32G32B32A32_SFLOAT"),
            (Self(110), "R64_UINT"),
            (Self(111), "R64_SINT"),
            (Self(112), "R64_SFLOAT"),
            (Self(113), "R64G64_UINT"),
            (Self(114), "R64G64_SINT"),
            (Self(115), "R64G64_SFLOAT"),
            (Self(116), "R64G64B64_UINT"),
            (Self(117), "R64G64B64_SINT"),
            (Self(118), "R64G64B64_SFLOAT"),
            (Self(119), "R64G64B64A64_UINT"),
            (Self(120), "R64G64B64A64_SINT"),
            (Self(121), "R64G64B64A64_SFLOAT"),
            (Self(122), "B10G11R11_UFLOAT_PACK32"),
            (Self(123), "E5B9G9R9_UFLOAT_PACK32"),
            (Self(124), "D16_UNORM"),
            (Self(125), "X8_D24_UNORM_PACK32"),
            (Self(126), "D32_SFLOAT"),
            (Self(127), "S8_UINT"),
            (Self(128), "D16_UNORM_S8_UINT"),
            (Self(129), "D24_UNORM_S8_UINT"),
            (Self(130), "D32_SFLOAT_S8_UINT"),
            (Self(131), "BC1_RGB_UNORM_BLOCK"),
            (Self(132), "BC1_RGB_SRGB_BLOCK"),
            (Self(133), "BC1_RGBA_UNORM_BLOCK"),
            (Self(134), "BC1_RGBA_SRGB_BLOCK"),
            (Self(135), "BC2_UNORM_BLOCK"),
            (Self(136), "BC2_SRGB_BLOCK"),
            (Self(137), "BC3_UNORM_BLOCK"),
            (Self(138), "BC3_SRGB_BLOCK"),
            (Self(139), "BC4_UNORM_BLOCK"),
            (Self(140), "BC4_SNORM_BLOCK"),
            (Self(141), "BC5_UNORM_BLOCK"),
            (Self(142), "BC5_SNORM_BLOCK"),
            (Self(143), "BC6H_UFLOAT_BLOCK"),
            (Self(144), "BC6H_SFLOAT_BLOCK"),
            (Self(145), "BC7_UNORM_BLOCK"),
            (Self(146), "BC7_SRGB_BLOCK"),
            (Self(147), "ETC2_R8G8B8_UNORM_BLOCK"),
            (Self(148), "ETC2_R8G8B8_SRGB_BLOCK"),
            (Self(149), "ETC2_R8G8B8A1_UNORM_BLOCK"),
            (Self(150), "ETC2_R8G8B8A1_SRGB_BLOCK"),
            (Self(151), "ETC2_R8G8B8A8_UNORM_BLOCK"),
            (Self(152), "ETC2_R8G8B8A8_SRGB_BLOCK"),
            (Self(153), "EAC_R11_UNORM_BLOCK"),
            (Self(154), "EAC_R11_SNORM_BLOCK"),
            (Self(155), "EAC_R11G11_UNORM_BLOCK"),
            (Self(156), "EAC_R11G11_SNORM_BLOCK"),
            (Self(157), "ASTC_4x4_UNORM_BLOCK"),
            (Self(158), "ASTC_4x4_SRGB_BLOCK"),
            (Self(159), "ASTC_5x4_UNORM_BLOCK"),
            (Self(160), "ASTC_5x4_SRGB_BLOCK"),
            (Self(161), "ASTC_5x5_UNORM_BLOCK"),
            (Self(162), "ASTC_5x5_SRGB_BLOCK"),
            (Self(163), "ASTC_6x5_UNORM_BLOCK"),
            (Self(164), "ASTC_6x5_SRGB_BLOCK"),
            (Self(165), "ASTC_6x6_UNORM_BLOCK"),
            (Self(166), "ASTC_6x6_SRGB_BLOCK"),
            (Self(167), "ASTC_8x5_UNORM_BLOCK"),
            (Self(168), "ASTC_8x5_SRGB_BLOCK"),
            (Self(169), "ASTC_8x6_UNORM_BLOCK"),
            (Self(170), "ASTC_8x6_SRGB_BLOCK"),
            (Self(171), "ASTC_8x8_UNORM_BLOCK"),
            (Self(172), "ASTC_8x8_SRGB_BLOCK"),
            (Self(173), "ASTC_10x5_UNORM_BLOCK"),
            (Self(174), "ASTC_10x5_SRGB_BLOCK"),
            (Self(175), "ASTC_10x6_UNORM_BLOCK"),
            (Self(176), "ASTC_10x6_SRGB_BLOCK"),
            (Self(177), "ASTC_10x8_UNORM_BLOCK"),
            (Self(178), "ASTC_10x8_SRGB_BLOCK"),
            (Self(179), "ASTC_10x10_UNORM_BLOCK"),
            (Self(180), "ASTC_10x10_SRGB_BLOCK"),
            (Self(181), "ASTC_12x10_UNORM_BLOCK"),
            (Self(182), "ASTC_12x10_SRGB_BLOCK"),
            (Self(183), "ASTC_12x12_UNORM_BLOCK"),
            (Self(184), "ASTC_12x12_SRGB_BLOCK"),
            (Self(1000054000), "PVRTC1_2BPP_UNORM_BLOCK_IMG"),
            (Self(1000054001), "PVRTC1_4BPP_UNORM_BLOCK_IMG"),
            (Self(1000054002), "PVRTC2_2BPP_UNORM_BLOCK_IMG"),
            (Self(1000054003), "PVRTC2_4BPP_UNORM_BLOCK_IMG"),
            (Self(1000054004), "PVRTC1_2BPP_SRGB_BLOCK_IMG"),
            (Self(1000054005), "PVRTC1_4BPP_SRGB_BLOCK_IMG"),
            (Self(1000054006), "PVRTC2_2BPP_SRGB_BLOCK_IMG"),
            (Self(1000054007), "PVRTC2_4BPP_SRGB_BLOCK_IMG"),
            (Self(1000066000), "ASTC_4x4_SFLOAT_BLOCK"),
            (Self(1000066001), "ASTC_5x4_SFLOAT_BLOCK"),
            (Self(1000066002), "ASTC_5x5_SFLOAT_BLOCK"),
            (Self(1000066003), "ASTC_6x5_SFLOAT_BLOCK"),
            (Self(1000066004), "ASTC_6x6_SFLOAT_BLOCK"),
            (Self(1000066005), "ASTC_8x5_SFLOAT_BLOCK"),
            (Self(1000066006), "ASTC_8x6_SFLOAT_BLOCK"),
            (Self(1000066007), "ASTC_8x8_SFLOAT_BLOCK"),
            (Self(1000066008), "ASTC_10x5_SFLOAT_BLOCK"),
            (Self(1000066009), "ASTC_10x6_SFLOAT_BLOCK"),
            (Self(1000066010), "ASTC_10x8_SFLOAT_BLOCK"),
            (Self(1000066011), "ASTC_10x10_SFLOAT_BLOCK"),
            (Self(1000066012), "ASTC_12x10_SFLOAT_BLOCK"),
            (Self(1000066013), "ASTC_12x12_SFLOAT_BLOCK"),
            (Self(1000156000), "G8B8G8R8_422_UNORM"),
            (Self(1000156001), "B8G8R8G8_422_UNORM"),
            (Self(1000156002), "G8_B8_R8_3PLANE_420_UNORM"),
            (Self(1000156003), "G8_B8R8_2PLANE_420_UNORM"),
            (Self(1000156004), "G8_B8_R8_3PLANE_422_UNORM"),
            (Self(1000156005), "G8_B8R8_2PLANE_422_UNORM"),
            (Self(1000156006), "G8_B8_R8_3PLANE_444_UNORM"),
            (Self(1000156007), "R10X6_UNORM_PACK16"),
            (Self(1000156008), "R10X6G10X6_UNORM_2PACK16"),
            (Self(1000156009), "R10X6G10X6B10X6A10X6_UNORM_4PACK16"),
            (Self(1000156010), "G10X6B10X6G10X6R10X6_422_UNORM_4PACK16"),
            (Self(1000156011), "B10X6G10X6R10X6G10X6_422_UNORM_4PACK16"),
            (
                Self(1000156012),
                "G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16",
            ),
            (
                Self(1000156013),
                "G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16",
            ),
            (
                Self(1000156014),
                "G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16",
            ),
            (
                Self(1000156015),
                "G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16",
            ),
            (
                Self(1000156016),
                "G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16",
            ),
            (Self(1000156017), "R12X4_UNORM_PACK16"),
            (Self(1000156018), "R12X4G12X4_UNORM_2PACK16"),
            (Self(1000156019), "R12X4G12X4B12X4A12X4_UNORM_4PACK16"),
            (Self(1000156020), "G12X4B12X4G12X4R12X4_422_UNORM_4PACK16"),
            (Self(1000156021), "B12X4G12X4R12X4G12X4_422_UNORM_4PACK16"),
            (
                Self(1000156022),
                "G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16",
            ),
            (
                Self(1000156023),
                "G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16",
            ),
            (
                Self(1000156024),
                "G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16",
            ),
            (
                Self(1000156025),
                "G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16",
            ),
            (
                Self(1000156026),
                "G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16",
            ),
            (Self(1000156027), "G16B16G16R16_422_UNORM"),
            (Self(1000156028), "B16G16R16G16_422_UNORM"),
            (Self(1000156029), "G16_B16_R16_3PLANE_420_UNORM"),
            (Self(1000156030), "G16_B16R16_2PLANE_420_UNORM"),
            (Self(1000156031), "G16_B16_R16_3PLANE_422_UNORM"),
            (Self(1000156032), "G16_B16R16_2PLANE_422_UNORM"),
            (Self(1000156033), "G16_B16_R16_3PLANE_444_UNORM"),
            (Self(1000288000), "ASTC_3x3x3_UNORM_BLOCK_EXT"),
            (Self(1000288001), "ASTC_3x3x3_SRGB_BLOCK_EXT"),
            (Self(1000288002), "ASTC_3x3x3_SFLOAT_BLOCK_EXT"),
            (Self(1000288003), "ASTC_4x3x3_UNORM_BLOCK_EXT"),
            (Self(1000288004), "ASTC_4x3x3_SRGB_BLOCK_EXT"),
            (Self(1000288005), "ASTC_4x3x3_SFLOAT_BLOCK_EXT"),
            (Self(1000288006), "ASTC_4x4x3_UNORM_BLOCK_EXT"),
            (Self(1000288007), "ASTC_4x4x3_SRGB_BLOCK_EXT"),
            (Self(1000288008), "ASTC_4x4x3_SFLOAT_BLOCK_EXT"),
            (Self(1000288009), "ASTC_4x4x4_UNORM_BLOCK_EXT"),
            (Self(1000288010), "ASTC_4x4x4_SRGB_BLOCK_EXT"),
            (Self(1000288011), "ASTC_4x4x4_SFLOAT_BLOCK_EXT"),
            (Self(1000288012), "ASTC_5x4x4_UNORM_BLOCK_EXT"),
            (Self(1000288013), "ASTC_5x4x4_SRGB_BLOCK_EXT"),
            (Self(1000288014), "ASTC_5x4x4_SFLOAT_BLOCK_EXT"),
            (Self(1000288015), "ASTC_5x5x4_UNORM_BLOCK_EXT"),
            (Self(1000288016), "ASTC_5x5x4_SRGB_BLOCK_EXT"),
            (Self(1000288017), "ASTC_5x5x4_SFLOAT_BLOCK_EXT"),
            (Self(1000288018), "ASTC_5x5x5_UNORM_BLOCK_EXT"),
            (Self(1000288019), "ASTC_5x5x5_SRGB_BLOCK_EXT"),
            (Self(1000288020), "ASTC_5x5x5_SFLOAT_BLOCK_EXT"),
            (Self(1000288021), "ASTC_6x5x5_UNORM_BLOCK_EXT"),
            (Self(1000288022), "ASTC_6x5x5_SRGB_BLOCK_EXT"),
            (Self(1000288023), "ASTC_6x5x5_SFLOAT_BLOCK_EXT"),
            (Self(1000288024), "ASTC_6x6x5_UNORM_BLOCK_EXT"),
            (Self(1000288025), "ASTC_6x6x5_SRGB_BLOCK_EXT"),
            (Self(1000288026), "ASTC_6x6x5_SFLOAT_BLOCK_EXT"),
            (Self(1000288027), "ASTC_6x6x6_UNORM_BLOCK_EXT"),
            (Self(1000288028), "ASTC_6x6x6_SRGB_BLOCK_EXT"),
            (Self(1000288029), "ASTC_6x6x6_SFLOAT_BLOCK_EXT"),
            (Self(1000330000), "G8_B8R8_2PLANE_444_UNORM"),
            (
                Self(1000330001),
                "G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16",
            ),
            (
                Self(1000330002),
                "G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16",
            ),
            (Self(1000330003), "G16_B16R16_2PLANE_444_UNORM"),
            (Self(1000340000), "A4R4G4B4_UNORM_PACK16"),
            (Self(1000340001), "A4B4G4R4_UNORM_PACK16"),
            (Self(1000460000), "R8_BOOL_ARM"),
            (Self(1000460001), "R16_SFLOAT_FPENCODING_BFLOAT16_ARM"),
            (Self(1000460002), "R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM"),
            (Self(1000460003), "R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM"),
            (Self(1000464000), "R16G16_SFIXED5_NV"),
            (Self(1000470000), "A1B5G5R5_UNORM_PACK16"),
            (Self(1000470001), "A8_UNORM"),
            (Self(1000609000), "R10X6_UINT_PACK16_ARM"),
            (Self(1000609001), "R10X6G10X6_UINT_2PACK16_ARM"),
            (Self(1000609002), "R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM"),
            (Self(1000609003), "R12X4_UINT_PACK16_ARM"),
            (Self(1000609004), "R12X4G12X4_UINT_2PACK16_ARM"),
            (Self(1000609005), "R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM"),
            (Self(1000609006), "R14X2_UINT_PACK16_ARM"),
            (Self(1000609007), "R14X2G14X2_UINT_2PACK16_ARM"),
            (Self(1000609008), "R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM"),
            (Self(1000609009), "R14X2_UNORM_PACK16_ARM"),
            (Self(1000609010), "R14X2G14X2_UNORM_2PACK16_ARM"),
            (Self(1000609011), "R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM"),
            (
                Self(1000609012),
                "G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM",
            ),
            (
                Self(1000609013),
                "G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM",
            ),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(1000066000), "ASTC_4x4_SFLOAT_BLOCK_EXT"),
            (Self(1000066001), "ASTC_5x4_SFLOAT_BLOCK_EXT"),
            (Self(1000066002), "ASTC_5x5_SFLOAT_BLOCK_EXT"),
            (Self(1000066003), "ASTC_6x5_SFLOAT_BLOCK_EXT"),
            (Self(1000066004), "ASTC_6x6_SFLOAT_BLOCK_EXT"),
            (Self(1000066005), "ASTC_8x5_SFLOAT_BLOCK_EXT"),
            (Self(1000066006), "ASTC_8x6_SFLOAT_BLOCK_EXT"),
            (Self(1000066007), "ASTC_8x8_SFLOAT_BLOCK_EXT"),
            (Self(1000066008), "ASTC_10x5_SFLOAT_BLOCK_EXT"),
            (Self(1000066009), "ASTC_10x6_SFLOAT_BLOCK_EXT"),
            (Self(1000066010), "ASTC_10x8_SFLOAT_BLOCK_EXT"),
            (Self(1000066011), "ASTC_10x10_SFLOAT_BLOCK_EXT"),
            (Self(1000066012), "ASTC_12x10_SFLOAT_BLOCK_EXT"),
            (Self(1000066013), "ASTC_12x12_SFLOAT_BLOCK_EXT"),
            (Self(1000156000), "G8B8G8R8_422_UNORM_KHR"),
            (Self(1000156001), "B8G8R8G8_422_UNORM_KHR"),
            (Self(1000156002), "G8_B8_R8_3PLANE_420_UNORM_KHR"),
            (Self(1000156003), "G8_B8R8_2PLANE_420_UNORM_KHR"),
            (Self(1000156004), "G8_B8_R8_3PLANE_422_UNORM_KHR"),
            (Self(1000156005), "G8_B8R8_2PLANE_422_UNORM_KHR"),
            (Self(1000156006), "G8_B8_R8_3PLANE_444_UNORM_KHR"),
            (Self(1000156007), "R10X6_UNORM_PACK16_KHR"),
            (Self(1000156008), "R10X6G10X6_UNORM_2PACK16_KHR"),
            (Self(1000156009), "R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR"),
            (
                Self(1000156010),
                "G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR",
            ),
            (
                Self(1000156011),
                "B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR",
            ),
            (
                Self(1000156012),
                "G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR",
            ),
            (
                Self(1000156013),
                "G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR",
            ),
            (
                Self(1000156014),
                "G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR",
            ),
            (
                Self(1000156015),
                "G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR",
            ),
            (
                Self(1000156016),
                "G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR",
            ),
            (Self(1000156017), "R12X4_UNORM_PACK16_KHR"),
            (Self(1000156018), "R12X4G12X4_UNORM_2PACK16_KHR"),
            (Self(1000156019), "R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR"),
            (
                Self(1000156020),
                "G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR",
            ),
            (
                Self(1000156021),
                "B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR",
            ),
            (
                Self(1000156022),
                "G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR",
            ),
            (
                Self(1000156023),
                "G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR",
            ),
            (
                Self(1000156024),
                "G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR",
            ),
            (
                Self(1000156025),
                "G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR",
            ),
            (
                Self(1000156026),
                "G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR",
            ),
            (Self(1000156027), "G16B16G16R16_422_UNORM_KHR"),
            (Self(1000156028), "B16G16R16G16_422_UNORM_KHR"),
            (Self(1000156029), "G16_B16_R16_3PLANE_420_UNORM_KHR"),
            (Self(1000156030), "G16_B16R16_2PLANE_420_UNORM_KHR"),
            (Self(1000156031), "G16_B16_R16_3PLANE_422_UNORM_KHR"),
            (Self(1000156032), "G16_B16R16_2PLANE_422_UNORM_KHR"),
            (Self(1000156033), "G16_B16_R16_3PLANE_444_UNORM_KHR"),
            (Self(1000330000), "G8_B8R8_2PLANE_444_UNORM_EXT"),
            (
                Self(1000330001),
                "G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT",
            ),
            (
                Self(1000330002),
                "G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT",
            ),
            (Self(1000330003), "G16_B16R16_2PLANE_444_UNORM_EXT"),
            (Self(1000340000), "A4R4G4B4_UNORM_PACK16_EXT"),
            (Self(1000340001), "A4B4G4R4_UNORM_PACK16_EXT"),
            (Self(1000464000), "R16G16_S10_5_NV"),
            (Self(1000470000), "A1B5G5R5_UNORM_PACK16_KHR"),
            (Self(1000470001), "A8_UNORM_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for Format {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "Format", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FragmentShadingRateCombinerOpKHR(pub i32);

impl FragmentShadingRateCombinerOpKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const KEEP: Self = Self(0);
    pub const REPLACE: Self = Self(1);
    pub const MIN: Self = Self(2);
    pub const MAX: Self = Self(3);
    pub const MUL: Self = Self(4);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "KEEP"),
            (Self(1), "REPLACE"),
            (Self(2), "MIN"),
            (Self(3), "MAX"),
            (Self(4), "MUL"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for FragmentShadingRateCombinerOpKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "FragmentShadingRateCombinerOpKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FragmentShadingRateNV(pub i32);

impl FragmentShadingRateNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _1_INVOCATION_PER_PIXEL: Self = Self(0);
    pub const _1_INVOCATION_PER_1X2_PIXELS: Self = Self(1);
    pub const _1_INVOCATION_PER_2X1_PIXELS: Self = Self(4);
    pub const _1_INVOCATION_PER_2X2_PIXELS: Self = Self(5);
    pub const _1_INVOCATION_PER_2X4_PIXELS: Self = Self(6);
    pub const _1_INVOCATION_PER_4X2_PIXELS: Self = Self(9);
    pub const _1_INVOCATION_PER_4X4_PIXELS: Self = Self(10);
    pub const _2_INVOCATIONS_PER_PIXEL: Self = Self(11);
    pub const _4_INVOCATIONS_PER_PIXEL: Self = Self(12);
    pub const _8_INVOCATIONS_PER_PIXEL: Self = Self(13);
    pub const _16_INVOCATIONS_PER_PIXEL: Self = Self(14);
    pub const NO_INVOCATIONS: Self = Self(15);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_1_INVOCATION_PER_PIXEL"),
            (Self(1), "_1_INVOCATION_PER_1X2_PIXELS"),
            (Self(4), "_1_INVOCATION_PER_2X1_PIXELS"),
            (Self(5), "_1_INVOCATION_PER_2X2_PIXELS"),
            (Self(6), "_1_INVOCATION_PER_2X4_PIXELS"),
            (Self(9), "_1_INVOCATION_PER_4X2_PIXELS"),
            (Self(10), "_1_INVOCATION_PER_4X4_PIXELS"),
            (Self(11), "_2_INVOCATIONS_PER_PIXEL"),
            (Self(12), "_4_INVOCATIONS_PER_PIXEL"),
            (Self(13), "_8_INVOCATIONS_PER_PIXEL"),
            (Self(14), "_16_INVOCATIONS_PER_PIXEL"),
            (Self(15), "NO_INVOCATIONS"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for FragmentShadingRateNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "FragmentShadingRateNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FragmentShadingRateTypeNV(pub i32);

impl FragmentShadingRateTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FRAGMENT_SIZE: Self = Self(0);
    pub const ENUMS: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "FRAGMENT_SIZE"), (Self(1), "ENUMS")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for FragmentShadingRateTypeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "FragmentShadingRateTypeNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "COUNTER_CLOCKWISE"), (Self(1), "CLOCKWISE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for FrontFace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "FrontFace", self.name(), self.0)
    }
}

#[cfg(feature = "win32")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FullScreenExclusiveEXT(pub i32);

#[cfg(feature = "win32")]
impl FullScreenExclusiveEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const ALLOWED: Self = Self(1);
    pub const DISALLOWED: Self = Self(2);
    pub const APPLICATION_CONTROLLED: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT"),
            (Self(1), "ALLOWED"),
            (Self(2), "DISALLOWED"),
            (Self(3), "APPLICATION_CONTROLLED"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

#[cfg(feature = "win32")]
impl core::fmt::Debug for FullScreenExclusiveEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "FullScreenExclusiveEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GeometryTypeKHR(pub i32);

impl GeometryTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TRIANGLES: Self = Self(0);
    pub const AABBS: Self = Self(1);
    pub const INSTANCES: Self = Self(2);
    pub const SPHERES_NV: Self = Self(1000429004);
    pub const LINEAR_SWEPT_SPHERES_NV: Self = Self(1000429005);
    #[cfg(feature = "beta")]
    pub const DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX: Self = Self(1000478000);
    pub const MICROMAP: Self = Self(1000623000);
    pub const TRIANGLES_NV: Self = Self(0);
    pub const AABBS_NV: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "TRIANGLES"),
            (Self(1), "AABBS"),
            (Self(2), "INSTANCES"),
            (Self(1000429004), "SPHERES_NV"),
            (Self(1000429005), "LINEAR_SWEPT_SPHERES_NV"),
            #[cfg(feature = "beta")]
            (Self(1000478000), "DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX"),
            (Self(1000623000), "MICROMAP"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(0), "TRIANGLES_NV"), (Self(1), "AABBS_NV")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for GeometryTypeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "GeometryTypeKHR", self.name(), self.0)
    }
}

pub type GeometryTypeNV = GeometryTypeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GpaDeviceClockModeAMD(pub i32);

impl GpaDeviceClockModeAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const QUERY: Self = Self(1);
    pub const PROFILING: Self = Self(2);
    pub const MIN_MEMORY: Self = Self(3);
    pub const MIN_ENGINE: Self = Self(4);
    pub const PEAK: Self = Self(5);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT"),
            (Self(1), "QUERY"),
            (Self(2), "PROFILING"),
            (Self(3), "MIN_MEMORY"),
            (Self(4), "MIN_ENGINE"),
            (Self(5), "PEAK"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for GpaDeviceClockModeAMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "GpaDeviceClockModeAMD", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GpaPerfBlockAMD(pub i32);

impl GpaPerfBlockAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CPF: Self = Self(0);
    pub const IA: Self = Self(1);
    pub const VGT: Self = Self(2);
    pub const PA: Self = Self(3);
    pub const SC: Self = Self(4);
    pub const SPI: Self = Self(5);
    pub const SQ: Self = Self(6);
    pub const SX: Self = Self(7);
    pub const TA: Self = Self(8);
    pub const TD: Self = Self(9);
    pub const TCP: Self = Self(10);
    pub const TCC: Self = Self(11);
    pub const TCA: Self = Self(12);
    pub const DB: Self = Self(13);
    pub const CB: Self = Self(14);
    pub const GDS: Self = Self(15);
    pub const SRBM: Self = Self(16);
    pub const GRBM: Self = Self(17);
    pub const GRBM_SE: Self = Self(18);
    pub const RLC: Self = Self(19);
    pub const DMA: Self = Self(20);
    pub const MC: Self = Self(21);
    pub const CPG: Self = Self(22);
    pub const CPC: Self = Self(23);
    pub const WD: Self = Self(24);
    pub const TCS: Self = Self(25);
    pub const ATC: Self = Self(26);
    pub const ATC_L2: Self = Self(27);
    pub const MC_VM_L2: Self = Self(28);
    pub const EA: Self = Self(29);
    pub const RPB: Self = Self(30);
    pub const RMI: Self = Self(31);
    pub const UMCCH: Self = Self(32);
    pub const GE: Self = Self(33);
    pub const GL1A: Self = Self(34);
    pub const GL1C: Self = Self(35);
    pub const GL1CG: Self = Self(36);
    pub const GL2A: Self = Self(37);
    pub const GL2C: Self = Self(38);
    pub const CHA: Self = Self(39);
    pub const CHC: Self = Self(40);
    pub const CHCG: Self = Self(41);
    pub const GUS: Self = Self(42);
    pub const GCR: Self = Self(43);
    pub const PH: Self = Self(44);
    pub const UTCL1: Self = Self(45);
    pub const GE_DIST: Self = Self(46);
    pub const GE_SE: Self = Self(47);
    pub const DF_MALL: Self = Self(48);
    pub const SQ_WGP: Self = Self(49);
    pub const PC: Self = Self(50);
    pub const GL1XA: Self = Self(51);
    pub const GL1XC: Self = Self(52);
    pub const WGS: Self = Self(53);
    pub const EACPWD: Self = Self(54);
    pub const EASE: Self = Self(55);
    pub const RLCUSER: Self = Self(56);
    pub const GE1: Self = Self(33);
    pub const RLCLOCAL: Self = Self(56);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "CPF"),
            (Self(1), "IA"),
            (Self(2), "VGT"),
            (Self(3), "PA"),
            (Self(4), "SC"),
            (Self(5), "SPI"),
            (Self(6), "SQ"),
            (Self(7), "SX"),
            (Self(8), "TA"),
            (Self(9), "TD"),
            (Self(10), "TCP"),
            (Self(11), "TCC"),
            (Self(12), "TCA"),
            (Self(13), "DB"),
            (Self(14), "CB"),
            (Self(15), "GDS"),
            (Self(16), "SRBM"),
            (Self(17), "GRBM"),
            (Self(18), "GRBM_SE"),
            (Self(19), "RLC"),
            (Self(20), "DMA"),
            (Self(21), "MC"),
            (Self(22), "CPG"),
            (Self(23), "CPC"),
            (Self(24), "WD"),
            (Self(25), "TCS"),
            (Self(26), "ATC"),
            (Self(27), "ATC_L2"),
            (Self(28), "MC_VM_L2"),
            (Self(29), "EA"),
            (Self(30), "RPB"),
            (Self(31), "RMI"),
            (Self(32), "UMCCH"),
            (Self(33), "GE"),
            (Self(34), "GL1A"),
            (Self(35), "GL1C"),
            (Self(36), "GL1CG"),
            (Self(37), "GL2A"),
            (Self(38), "GL2C"),
            (Self(39), "CHA"),
            (Self(40), "CHC"),
            (Self(41), "CHCG"),
            (Self(42), "GUS"),
            (Self(43), "GCR"),
            (Self(44), "PH"),
            (Self(45), "UTCL1"),
            (Self(46), "GE_DIST"),
            (Self(47), "GE_SE"),
            (Self(48), "DF_MALL"),
            (Self(49), "SQ_WGP"),
            (Self(50), "PC"),
            (Self(51), "GL1XA"),
            (Self(52), "GL1XC"),
            (Self(53), "WGS"),
            (Self(54), "EACPWD"),
            (Self(55), "EASE"),
            (Self(56), "RLCUSER"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(33), "GE1"), (Self(56), "RLCLOCAL")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for GpaPerfBlockAMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "GpaPerfBlockAMD", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GpaSampleTypeAMD(pub i32);

impl GpaSampleTypeAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CUMULATIVE: Self = Self(0);
    pub const TRACE: Self = Self(1);
    pub const TIMING: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "CUMULATIVE"),
            (Self(1), "TRACE"),
            (Self(2), "TIMING"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for GpaSampleTypeAMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "GpaSampleTypeAMD", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000117000);
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self(1000117001);
    pub const SHADING_RATE_OPTIMAL_NV: Self = Self(1000164003);
    pub const RENDERING_LOCAL_READ_KHR: Self = Self(1000232000);
    pub const DEPTH_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000241000);
    pub const DEPTH_READ_ONLY_OPTIMAL_KHR: Self = Self(1000241001);
    pub const STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000241002);
    pub const STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self(1000241003);
    pub const READ_ONLY_OPTIMAL_KHR: Self = Self(1000314000);
    pub const ATTACHMENT_OPTIMAL_KHR: Self = Self(1000314001);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNDEFINED"),
            (Self(1), "GENERAL"),
            (Self(2), "COLOR_ATTACHMENT_OPTIMAL"),
            (Self(3), "DEPTH_STENCIL_ATTACHMENT_OPTIMAL"),
            (Self(4), "DEPTH_STENCIL_READ_ONLY_OPTIMAL"),
            (Self(5), "SHADER_READ_ONLY_OPTIMAL"),
            (Self(6), "TRANSFER_SRC_OPTIMAL"),
            (Self(7), "TRANSFER_DST_OPTIMAL"),
            (Self(8), "PREINITIALIZED"),
            (Self(1000001002), "PRESENT_SRC_KHR"),
            (Self(1000024000), "VIDEO_DECODE_DST_KHR"),
            (Self(1000024001), "VIDEO_DECODE_SRC_KHR"),
            (Self(1000024002), "VIDEO_DECODE_DPB_KHR"),
            (Self(1000111000), "SHARED_PRESENT_KHR"),
            (
                Self(1000117000),
                "DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL",
            ),
            (
                Self(1000117001),
                "DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL",
            ),
            (
                Self(1000164003),
                "FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR",
            ),
            (Self(1000218000), "FRAGMENT_DENSITY_MAP_OPTIMAL_EXT"),
            (Self(1000232000), "RENDERING_LOCAL_READ"),
            (Self(1000241000), "DEPTH_ATTACHMENT_OPTIMAL"),
            (Self(1000241001), "DEPTH_READ_ONLY_OPTIMAL"),
            (Self(1000241002), "STENCIL_ATTACHMENT_OPTIMAL"),
            (Self(1000241003), "STENCIL_READ_ONLY_OPTIMAL"),
            (Self(1000299000), "VIDEO_ENCODE_DST_KHR"),
            (Self(1000299001), "VIDEO_ENCODE_SRC_KHR"),
            (Self(1000299002), "VIDEO_ENCODE_DPB_KHR"),
            (Self(1000314000), "READ_ONLY_OPTIMAL"),
            (Self(1000314001), "ATTACHMENT_OPTIMAL"),
            (Self(1000339000), "ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT"),
            (Self(1000460000), "TENSOR_ALIASING_ARM"),
            (Self(1000553000), "VIDEO_ENCODE_QUANTIZATION_MAP_KHR"),
            (Self(1000620000), "ZERO_INITIALIZED_EXT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (
                Self(1000117000),
                "DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR",
            ),
            (
                Self(1000117001),
                "DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR",
            ),
            (Self(1000164003), "SHADING_RATE_OPTIMAL_NV"),
            (Self(1000232000), "RENDERING_LOCAL_READ_KHR"),
            (Self(1000241000), "DEPTH_ATTACHMENT_OPTIMAL_KHR"),
            (Self(1000241001), "DEPTH_READ_ONLY_OPTIMAL_KHR"),
            (Self(1000241002), "STENCIL_ATTACHMENT_OPTIMAL_KHR"),
            (Self(1000241003), "STENCIL_READ_ONLY_OPTIMAL_KHR"),
            (Self(1000314000), "READ_ONLY_OPTIMAL_KHR"),
            (Self(1000314001), "ATTACHMENT_OPTIMAL_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ImageLayout {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ImageLayout", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "OPTIMAL"),
            (Self(1), "LINEAR"),
            (Self(1000158000), "DRM_FORMAT_MODIFIER_EXT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ImageTiling {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ImageTiling", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImageTilingControlEXT(pub i32);

impl ImageTilingControlEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const MIN_SIZE: Self = Self(1);
    pub const MAX_PERFORMANCE: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT"),
            (Self(1), "MIN_SIZE"),
            (Self(2), "MAX_PERFORMANCE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ImageTilingControlEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ImageTilingControlEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "_1D"), (Self(1), "_2D"), (Self(2), "_3D")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ImageType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ImageType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_1D"),
            (Self(1), "_2D"),
            (Self(2), "_3D"),
            (Self(3), "CUBE"),
            (Self(4), "_1D_ARRAY"),
            (Self(5), "_2D_ARRAY"),
            (Self(6), "CUBE_ARRAY"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ImageViewType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ImageViewType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const NONE_NV: Self = Self(1000165000);
    pub const UINT8_EXT: Self = Self(1000265000);
    pub const UINT8_KHR: Self = Self(1000265000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UINT16"),
            (Self(1), "UINT32"),
            (Self(1000165000), "NONE_KHR"),
            (Self(1000265000), "UINT8"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(1000165000), "NONE_NV"),
            (Self(1000265000), "UINT8_EXT"),
            (Self(1000265000), "UINT8_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for IndexType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "IndexType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectCommandsTokenTypeEXT(pub i32);

impl IndirectCommandsTokenTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const EXECUTION_SET: Self = Self(0);
    pub const PUSH_CONSTANT: Self = Self(1);
    pub const SEQUENCE_INDEX: Self = Self(2);
    pub const INDEX_BUFFER: Self = Self(3);
    pub const VERTEX_BUFFER: Self = Self(4);
    pub const DRAW_INDEXED: Self = Self(5);
    pub const DRAW: Self = Self(6);
    pub const DRAW_INDEXED_COUNT: Self = Self(7);
    pub const DRAW_COUNT: Self = Self(8);
    pub const DISPATCH: Self = Self(9);
    pub const PUSH_DATA: Self = Self(1000135000);
    pub const PUSH_DATA_SEQUENCE_INDEX: Self = Self(1000135001);
    pub const DRAW_MESH_TASKS_NV: Self = Self(1000202002);
    pub const DRAW_MESH_TASKS_COUNT_NV: Self = Self(1000202003);
    pub const DRAW_MESH_TASKS: Self = Self(1000328000);
    pub const DRAW_MESH_TASKS_COUNT: Self = Self(1000328001);
    pub const TRACE_RAYS2: Self = Self(1000386004);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "EXECUTION_SET"),
            (Self(1), "PUSH_CONSTANT"),
            (Self(2), "SEQUENCE_INDEX"),
            (Self(3), "INDEX_BUFFER"),
            (Self(4), "VERTEX_BUFFER"),
            (Self(5), "DRAW_INDEXED"),
            (Self(6), "DRAW"),
            (Self(7), "DRAW_INDEXED_COUNT"),
            (Self(8), "DRAW_COUNT"),
            (Self(9), "DISPATCH"),
            (Self(1000135000), "PUSH_DATA"),
            (Self(1000135001), "PUSH_DATA_SEQUENCE_INDEX"),
            (Self(1000202002), "DRAW_MESH_TASKS_NV"),
            (Self(1000202003), "DRAW_MESH_TASKS_COUNT_NV"),
            (Self(1000328000), "DRAW_MESH_TASKS"),
            (Self(1000328001), "DRAW_MESH_TASKS_COUNT"),
            (Self(1000386004), "TRACE_RAYS2"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for IndirectCommandsTokenTypeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "IndirectCommandsTokenTypeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectCommandsTokenTypeNV(pub i32);

impl IndirectCommandsTokenTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SHADER_GROUP: Self = Self(0);
    pub const STATE_FLAGS: Self = Self(1);
    pub const INDEX_BUFFER: Self = Self(2);
    pub const VERTEX_BUFFER: Self = Self(3);
    pub const PUSH_CONSTANT: Self = Self(4);
    pub const DRAW_INDEXED: Self = Self(5);
    pub const DRAW: Self = Self(6);
    pub const DRAW_TASKS: Self = Self(7);
    pub const PUSH_DATA: Self = Self(1000135000);
    pub const DRAW_MESH_TASKS: Self = Self(1000328000);
    pub const PIPELINE: Self = Self(1000428003);
    pub const DISPATCH: Self = Self(1000428004);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "SHADER_GROUP"),
            (Self(1), "STATE_FLAGS"),
            (Self(2), "INDEX_BUFFER"),
            (Self(3), "VERTEX_BUFFER"),
            (Self(4), "PUSH_CONSTANT"),
            (Self(5), "DRAW_INDEXED"),
            (Self(6), "DRAW"),
            (Self(7), "DRAW_TASKS"),
            (Self(1000135000), "PUSH_DATA"),
            (Self(1000328000), "DRAW_MESH_TASKS"),
            (Self(1000428003), "PIPELINE"),
            (Self(1000428004), "DISPATCH"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for IndirectCommandsTokenTypeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "IndirectCommandsTokenTypeNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectExecutionSetInfoTypeEXT(pub i32);

impl IndirectExecutionSetInfoTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PIPELINES: Self = Self(0);
    pub const SHADER_OBJECTS: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "PIPELINES"), (Self(1), "SHADER_OBJECTS")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for IndirectExecutionSetInfoTypeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "IndirectExecutionSetInfoTypeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InternalAllocationType(pub i32);

impl InternalAllocationType {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const EXECUTABLE: Self = Self(0);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "EXECUTABLE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for InternalAllocationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "InternalAllocationType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LatencyMarkerNV(pub i32);

impl LatencyMarkerNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SIMULATION_START: Self = Self(0);
    pub const SIMULATION_END: Self = Self(1);
    pub const RENDERSUBMIT_START: Self = Self(2);
    pub const RENDERSUBMIT_END: Self = Self(3);
    pub const PRESENT_START: Self = Self(4);
    pub const PRESENT_END: Self = Self(5);
    pub const INPUT_SAMPLE: Self = Self(6);
    pub const TRIGGER_FLASH: Self = Self(7);
    pub const OUT_OF_BAND_RENDERSUBMIT_START: Self = Self(8);
    pub const OUT_OF_BAND_RENDERSUBMIT_END: Self = Self(9);
    pub const OUT_OF_BAND_PRESENT_START: Self = Self(10);
    pub const OUT_OF_BAND_PRESENT_END: Self = Self(11);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "SIMULATION_START"),
            (Self(1), "SIMULATION_END"),
            (Self(2), "RENDERSUBMIT_START"),
            (Self(3), "RENDERSUBMIT_END"),
            (Self(4), "PRESENT_START"),
            (Self(5), "PRESENT_END"),
            (Self(6), "INPUT_SAMPLE"),
            (Self(7), "TRIGGER_FLASH"),
            (Self(8), "OUT_OF_BAND_RENDERSUBMIT_START"),
            (Self(9), "OUT_OF_BAND_RENDERSUBMIT_END"),
            (Self(10), "OUT_OF_BAND_PRESENT_START"),
            (Self(11), "OUT_OF_BAND_PRESENT_END"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for LatencyMarkerNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "LatencyMarkerNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LayerSettingTypeEXT(pub i32);

impl LayerSettingTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BOOL32: Self = Self(0);
    pub const INT32: Self = Self(1);
    pub const INT64: Self = Self(2);
    pub const UINT32: Self = Self(3);
    pub const UINT64: Self = Self(4);
    pub const FLOAT32: Self = Self(5);
    pub const FLOAT64: Self = Self(6);
    pub const STRING: Self = Self(7);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "BOOL32"),
            (Self(1), "INT32"),
            (Self(2), "INT64"),
            (Self(3), "UINT32"),
            (Self(4), "UINT64"),
            (Self(5), "FLOAT32"),
            (Self(6), "FLOAT64"),
            (Self(7), "STRING"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for LayerSettingTypeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "LayerSettingTypeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LayeredDriverUnderlyingApiMSFT(pub i32);

impl LayeredDriverUnderlyingApiMSFT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NONE: Self = Self(0);
    pub const D3D12: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "NONE"), (Self(1), "D3D12")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for LayeredDriverUnderlyingApiMSFT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "LayeredDriverUnderlyingApiMSFT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const DEFAULT_EXT: Self = Self(0);
    pub const DEFAULT_KHR: Self = Self(0);
    pub const RECTANGULAR_EXT: Self = Self(1);
    pub const RECTANGULAR_KHR: Self = Self(1);
    pub const BRESENHAM_EXT: Self = Self(2);
    pub const BRESENHAM_KHR: Self = Self(2);
    pub const RECTANGULAR_SMOOTH_EXT: Self = Self(3);
    pub const RECTANGULAR_SMOOTH_KHR: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT"),
            (Self(1), "RECTANGULAR"),
            (Self(2), "BRESENHAM"),
            (Self(3), "RECTANGULAR_SMOOTH"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT_EXT"),
            (Self(0), "DEFAULT_KHR"),
            (Self(1), "RECTANGULAR_EXT"),
            (Self(1), "RECTANGULAR_KHR"),
            (Self(2), "BRESENHAM_EXT"),
            (Self(2), "BRESENHAM_KHR"),
            (Self(3), "RECTANGULAR_SMOOTH_EXT"),
            (Self(3), "RECTANGULAR_SMOOTH_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for LineRasterizationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "LineRasterizationMode", self.name(), self.0)
    }
}

pub type LineRasterizationModeEXT = LineRasterizationMode;

pub type LineRasterizationModeKHR = LineRasterizationMode;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "CLEAR"),
            (Self(1), "AND"),
            (Self(2), "AND_REVERSE"),
            (Self(3), "COPY"),
            (Self(4), "AND_INVERTED"),
            (Self(5), "NO_OP"),
            (Self(6), "XOR"),
            (Self(7), "OR"),
            (Self(8), "NOR"),
            (Self(9), "EQUIVALENT"),
            (Self(10), "INVERT"),
            (Self(11), "OR_REVERSE"),
            (Self(12), "COPY_INVERTED"),
            (Self(13), "OR_INVERTED"),
            (Self(14), "NAND"),
            (Self(15), "SET"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for LogicOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "LogicOp", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MemoryOverallocationBehaviorAMD(pub i32);

impl MemoryOverallocationBehaviorAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const ALLOWED: Self = Self(1);
    pub const DISALLOWED: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT"),
            (Self(1), "ALLOWED"),
            (Self(2), "DISALLOWED"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for MemoryOverallocationBehaviorAMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "MemoryOverallocationBehaviorAMD", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MicromapTypeEXT(pub i32);

impl MicromapTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPACITY_MICROMAP: Self = Self(0);
    #[cfg(feature = "beta")]
    pub const DISPLACEMENT_MICROMAP_NV: Self = Self(1000397000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "OPACITY_MICROMAP"),
            #[cfg(feature = "beta")]
            (Self(1000397000), "DISPLACEMENT_MICROMAP_NV"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for MicromapTypeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "MicromapTypeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NeuralAcceleratorStatisticsModeARM(pub i32);

impl NeuralAcceleratorStatisticsModeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DISABLED: Self = Self(0);
    pub const STATISTICS0: Self = Self(1);
    pub const STATISTICS1: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DISABLED"),
            (Self(1), "STATISTICS0"),
            (Self(2), "STATISTICS1"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for NeuralAcceleratorStatisticsModeARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "NeuralAcceleratorStatisticsModeARM", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const GPA_SESSION_AMD: Self = Self(1000133000);
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
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self(1000085000);
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self(1000156000);
    pub const PRIVATE_DATA_SLOT_EXT: Self = Self(1000295000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNKNOWN"),
            (Self(1), "INSTANCE"),
            (Self(2), "PHYSICAL_DEVICE"),
            (Self(3), "DEVICE"),
            (Self(4), "QUEUE"),
            (Self(5), "SEMAPHORE"),
            (Self(6), "COMMAND_BUFFER"),
            (Self(7), "FENCE"),
            (Self(8), "DEVICE_MEMORY"),
            (Self(9), "BUFFER"),
            (Self(10), "IMAGE"),
            (Self(11), "EVENT"),
            (Self(12), "QUERY_POOL"),
            (Self(13), "BUFFER_VIEW"),
            (Self(14), "IMAGE_VIEW"),
            (Self(15), "SHADER_MODULE"),
            (Self(16), "PIPELINE_CACHE"),
            (Self(17), "PIPELINE_LAYOUT"),
            (Self(18), "RENDER_PASS"),
            (Self(19), "PIPELINE"),
            (Self(20), "DESCRIPTOR_SET_LAYOUT"),
            (Self(21), "SAMPLER"),
            (Self(22), "DESCRIPTOR_POOL"),
            (Self(23), "DESCRIPTOR_SET"),
            (Self(24), "FRAMEBUFFER"),
            (Self(25), "COMMAND_POOL"),
            (Self(1000000000), "SURFACE_KHR"),
            (Self(1000001000), "SWAPCHAIN_KHR"),
            (Self(1000002000), "DISPLAY_KHR"),
            (Self(1000002001), "DISPLAY_MODE_KHR"),
            (Self(1000011000), "DEBUG_REPORT_CALLBACK_EXT"),
            (Self(1000023000), "VIDEO_SESSION_KHR"),
            (Self(1000023001), "VIDEO_SESSION_PARAMETERS_KHR"),
            (Self(1000029000), "CU_MODULE_NVX"),
            (Self(1000029001), "CU_FUNCTION_NVX"),
            (Self(1000085000), "DESCRIPTOR_UPDATE_TEMPLATE"),
            (Self(1000128000), "DEBUG_UTILS_MESSENGER_EXT"),
            (Self(1000133000), "GPA_SESSION_AMD"),
            (Self(1000150000), "ACCELERATION_STRUCTURE_KHR"),
            (Self(1000156000), "SAMPLER_YCBCR_CONVERSION"),
            (Self(1000160000), "VALIDATION_CACHE_EXT"),
            (Self(1000165000), "ACCELERATION_STRUCTURE_NV"),
            (Self(1000210000), "PERFORMANCE_CONFIGURATION_INTEL"),
            (Self(1000268000), "DEFERRED_OPERATION_KHR"),
            (Self(1000277000), "INDIRECT_COMMANDS_LAYOUT_NV"),
            (Self(1000295000), "PRIVATE_DATA_SLOT"),
            #[cfg(feature = "beta")]
            (Self(1000307000), "CUDA_MODULE_NV"),
            #[cfg(feature = "beta")]
            (Self(1000307001), "CUDA_FUNCTION_NV"),
            (Self(1000366000), "BUFFER_COLLECTION_FUCHSIA"),
            (Self(1000396000), "MICROMAP_EXT"),
            (Self(1000460000), "TENSOR_ARM"),
            (Self(1000460001), "TENSOR_VIEW_ARM"),
            (Self(1000464000), "OPTICAL_FLOW_SESSION_NV"),
            (Self(1000482000), "SHADER_EXT"),
            (Self(1000483000), "PIPELINE_BINARY_KHR"),
            (Self(1000507000), "DATA_GRAPH_PIPELINE_SESSION_ARM"),
            (Self(1000556000), "EXTERNAL_COMPUTE_QUEUE_NV"),
            (Self(1000572000), "INDIRECT_COMMANDS_LAYOUT_EXT"),
            (Self(1000572001), "INDIRECT_EXECUTION_SET_EXT"),
            (Self(1000607000), "SHADER_INSTRUMENTATION_ARM"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(1000085000), "DESCRIPTOR_UPDATE_TEMPLATE_KHR"),
            (Self(1000156000), "SAMPLER_YCBCR_CONVERSION_KHR"),
            (Self(1000295000), "PRIVATE_DATA_SLOT_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ObjectType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ObjectType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OpacityMicromapFormatKHR(pub i32);

impl OpacityMicromapFormatKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const _2_STATE: Self = Self(1);
    pub const _4_STATE: Self = Self(2);
    pub const _2_STATE_EXT: Self = Self(1);
    pub const _4_STATE_EXT: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(1), "_2_STATE"), (Self(2), "_4_STATE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(1), "_2_STATE_EXT"), (Self(2), "_4_STATE_EXT")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for OpacityMicromapFormatKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "OpacityMicromapFormatKHR", self.name(), self.0)
    }
}

pub type OpacityMicromapFormatEXT = OpacityMicromapFormatKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OpacityMicromapSpecialIndexKHR(pub i32);

impl OpacityMicromapSpecialIndexKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV: Self = Self(-5);
    pub const FULLY_UNKNOWN_OPAQUE: Self = Self(-4);
    pub const FULLY_UNKNOWN_TRANSPARENT: Self = Self(-3);
    pub const FULLY_OPAQUE: Self = Self(-2);
    pub const FULLY_TRANSPARENT: Self = Self(-1);
    pub const FULLY_UNKNOWN_OPAQUE_EXT: Self = Self(-4);
    pub const FULLY_UNKNOWN_TRANSPARENT_EXT: Self = Self(-3);
    pub const FULLY_OPAQUE_EXT: Self = Self(-2);
    pub const FULLY_TRANSPARENT_EXT: Self = Self(-1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(-5), "CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV"),
            (Self(-4), "FULLY_UNKNOWN_OPAQUE"),
            (Self(-3), "FULLY_UNKNOWN_TRANSPARENT"),
            (Self(-2), "FULLY_OPAQUE"),
            (Self(-1), "FULLY_TRANSPARENT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(-4), "FULLY_UNKNOWN_OPAQUE_EXT"),
            (Self(-3), "FULLY_UNKNOWN_TRANSPARENT_EXT"),
            (Self(-2), "FULLY_OPAQUE_EXT"),
            (Self(-1), "FULLY_TRANSPARENT_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for OpacityMicromapSpecialIndexKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "OpacityMicromapSpecialIndexKHR", self.name(), self.0)
    }
}

pub type OpacityMicromapSpecialIndexEXT = OpacityMicromapSpecialIndexKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OpticalFlowPerformanceLevelNV(pub i32);

impl OpticalFlowPerformanceLevelNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNKNOWN: Self = Self(0);
    pub const SLOW: Self = Self(1);
    pub const MEDIUM: Self = Self(2);
    pub const FAST: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNKNOWN"),
            (Self(1), "SLOW"),
            (Self(2), "MEDIUM"),
            (Self(3), "FAST"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for OpticalFlowPerformanceLevelNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "OpticalFlowPerformanceLevelNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OpticalFlowSessionBindingPointNV(pub i32);

impl OpticalFlowSessionBindingPointNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNKNOWN: Self = Self(0);
    pub const INPUT: Self = Self(1);
    pub const REFERENCE: Self = Self(2);
    pub const HINT: Self = Self(3);
    pub const FLOW_VECTOR: Self = Self(4);
    pub const BACKWARD_FLOW_VECTOR: Self = Self(5);
    pub const COST: Self = Self(6);
    pub const BACKWARD_COST: Self = Self(7);
    pub const GLOBAL_FLOW: Self = Self(8);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UNKNOWN"),
            (Self(1), "INPUT"),
            (Self(2), "REFERENCE"),
            (Self(3), "HINT"),
            (Self(4), "FLOW_VECTOR"),
            (Self(5), "BACKWARD_FLOW_VECTOR"),
            (Self(6), "COST"),
            (Self(7), "BACKWARD_COST"),
            (Self(8), "GLOBAL_FLOW"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for OpticalFlowSessionBindingPointNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "OpticalFlowSessionBindingPointNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OutOfBandQueueTypeNV(pub i32);

impl OutOfBandQueueTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const RENDER: Self = Self(0);
    pub const PRESENT: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "RENDER"), (Self(1), "PRESENT")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for OutOfBandQueueTypeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "OutOfBandQueueTypeNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PartitionedAccelerationStructureOpTypeNV(pub i32);

impl PartitionedAccelerationStructureOpTypeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const WRITE_INSTANCE: Self = Self(0);
    pub const UPDATE_INSTANCE: Self = Self(1);
    pub const WRITE_PARTITION_TRANSLATION: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "WRITE_INSTANCE"),
            (Self(1), "UPDATE_INSTANCE"),
            (Self(2), "WRITE_PARTITION_TRANSLATION"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PartitionedAccelerationStructureOpTypeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "PartitionedAccelerationStructureOpTypeNV",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerfHintTypeQCOM(pub i32);

impl PerfHintTypeQCOM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const FREQUENCY_MIN: Self = Self(1);
    pub const FREQUENCY_MAX: Self = Self(2);
    pub const FREQUENCY_SCALED: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT"),
            (Self(1), "FREQUENCY_MIN"),
            (Self(2), "FREQUENCY_MAX"),
            (Self(3), "FREQUENCY_SCALED"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PerfHintTypeQCOM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PerfHintTypeQCOM", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceConfigurationTypeINTEL(pub i32);

impl PerformanceConfigurationTypeINTEL {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED: Self = Self(0);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PerformanceConfigurationTypeINTEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PerformanceConfigurationTypeINTEL", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceCounterScopeKHR(pub i32);

impl PerformanceCounterScopeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COMMAND_BUFFER: Self = Self(0);
    pub const RENDER_PASS: Self = Self(1);
    pub const COMMAND: Self = Self(2);
    pub const QUERY_SCOPE_COMMAND_BUFFER: Self = Self(0);
    pub const QUERY_SCOPE_RENDER_PASS: Self = Self(1);
    pub const QUERY_SCOPE_COMMAND: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "COMMAND_BUFFER"),
            (Self(1), "RENDER_PASS"),
            (Self(2), "COMMAND"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "QUERY_SCOPE_COMMAND_BUFFER"),
            (Self(1), "QUERY_SCOPE_RENDER_PASS"),
            (Self(2), "QUERY_SCOPE_COMMAND"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PerformanceCounterScopeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PerformanceCounterScopeKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceCounterStorageKHR(pub i32);

impl PerformanceCounterStorageKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INT32: Self = Self(0);
    pub const INT64: Self = Self(1);
    pub const UINT32: Self = Self(2);
    pub const UINT64: Self = Self(3);
    pub const FLOAT32: Self = Self(4);
    pub const FLOAT64: Self = Self(5);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "INT32"),
            (Self(1), "INT64"),
            (Self(2), "UINT32"),
            (Self(3), "UINT64"),
            (Self(4), "FLOAT32"),
            (Self(5), "FLOAT64"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PerformanceCounterStorageKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PerformanceCounterStorageKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceCounterUnitKHR(pub i32);

impl PerformanceCounterUnitKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const GENERIC: Self = Self(0);
    pub const PERCENTAGE: Self = Self(1);
    pub const NANOSECONDS: Self = Self(2);
    pub const BYTES: Self = Self(3);
    pub const BYTES_PER_SECOND: Self = Self(4);
    pub const KELVIN: Self = Self(5);
    pub const WATTS: Self = Self(6);
    pub const VOLTS: Self = Self(7);
    pub const AMPS: Self = Self(8);
    pub const HERTZ: Self = Self(9);
    pub const CYCLES: Self = Self(10);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "GENERIC"),
            (Self(1), "PERCENTAGE"),
            (Self(2), "NANOSECONDS"),
            (Self(3), "BYTES"),
            (Self(4), "BYTES_PER_SECOND"),
            (Self(5), "KELVIN"),
            (Self(6), "WATTS"),
            (Self(7), "VOLTS"),
            (Self(8), "AMPS"),
            (Self(9), "HERTZ"),
            (Self(10), "CYCLES"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PerformanceCounterUnitKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PerformanceCounterUnitKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceOverrideTypeINTEL(pub i32);

impl PerformanceOverrideTypeINTEL {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NULL_HARDWARE: Self = Self(0);
    pub const FLUSH_GPU_CACHES: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "NULL_HARDWARE"), (Self(1), "FLUSH_GPU_CACHES")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PerformanceOverrideTypeINTEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PerformanceOverrideTypeINTEL", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceParameterTypeINTEL(pub i32);

impl PerformanceParameterTypeINTEL {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const HW_COUNTERS_SUPPORTED: Self = Self(0);
    pub const STREAM_MARKER_VALID_BITS: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "HW_COUNTERS_SUPPORTED"),
            (Self(1), "STREAM_MARKER_VALID_BITS"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PerformanceParameterTypeINTEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PerformanceParameterTypeINTEL", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceValueTypeINTEL(pub i32);

impl PerformanceValueTypeINTEL {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UINT32: Self = Self(0);
    pub const UINT64: Self = Self(1);
    pub const FLOAT: Self = Self(2);
    pub const BOOL: Self = Self(3);
    pub const STRING: Self = Self(4);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "UINT32"),
            (Self(1), "UINT64"),
            (Self(2), "FLOAT"),
            (Self(3), "BOOL"),
            (Self(4), "STRING"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PerformanceValueTypeINTEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PerformanceValueTypeINTEL", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PhysicalDeviceDataGraphOperationTypeARM(pub i32);

impl PhysicalDeviceDataGraphOperationTypeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SPIRV_EXTENDED_INSTRUCTION_SET: Self = Self(0);
    pub const NEURAL_MODEL_QCOM: Self = Self(1000629000);
    pub const BUILTIN_MODEL_QCOM: Self = Self(1000629001);
    pub const OPTICAL_FLOW: Self = Self(1000631000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "SPIRV_EXTENDED_INSTRUCTION_SET"),
            (Self(1000629000), "NEURAL_MODEL_QCOM"),
            (Self(1000629001), "BUILTIN_MODEL_QCOM"),
            (Self(1000631000), "OPTICAL_FLOW"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PhysicalDeviceDataGraphOperationTypeARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "PhysicalDeviceDataGraphOperationTypeARM",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PhysicalDeviceDataGraphProcessingEngineTypeARM(pub i32);

impl PhysicalDeviceDataGraphProcessingEngineTypeARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const NEURAL_QCOM: Self = Self(1000629000);
    pub const COMPUTE_QCOM: Self = Self(1000629001);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT"),
            (Self(1000629000), "NEURAL_QCOM"),
            (Self(1000629001), "COMPUTE_QCOM"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PhysicalDeviceDataGraphProcessingEngineTypeARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "PhysicalDeviceDataGraphProcessingEngineTypeARM",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PhysicalDeviceLayeredApiKHR(pub i32);

impl PhysicalDeviceLayeredApiKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VULKAN: Self = Self(0);
    pub const D3D12: Self = Self(1);
    pub const METAL: Self = Self(2);
    pub const OPENGL: Self = Self(3);
    pub const OPENGLES: Self = Self(4);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "VULKAN"),
            (Self(1), "D3D12"),
            (Self(2), "METAL"),
            (Self(3), "OPENGL"),
            (Self(4), "OPENGLES"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PhysicalDeviceLayeredApiKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PhysicalDeviceLayeredApiKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "OTHER"),
            (Self(1), "INTEGRATED_GPU"),
            (Self(2), "DISCRETE_GPU"),
            (Self(3), "VIRTUAL_GPU"),
            (Self(4), "CPU"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PhysicalDeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PhysicalDeviceType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const RAY_TRACING_NV: Self = Self(1000165000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "GRAPHICS"),
            (Self(1), "COMPUTE"),
            #[cfg(feature = "beta")]
            (Self(1000134000), "EXECUTION_GRAPH_AMDX"),
            (Self(1000165000), "RAY_TRACING_KHR"),
            (Self(1000369003), "SUBPASS_SHADING_HUAWEI"),
            (Self(1000507000), "DATA_GRAPH_ARM"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(1000165000), "RAY_TRACING_NV")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PipelineBindPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PipelineBindPoint", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(1), "ONE"), (Self(1000629000), "DATA_GRAPH_QCOM")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PipelineCacheHeaderVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PipelineCacheHeaderVersion", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PipelineExecutableStatisticFormatKHR(pub i32);

impl PipelineExecutableStatisticFormatKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BOOL32: Self = Self(0);
    pub const INT64: Self = Self(1);
    pub const UINT64: Self = Self(2);
    pub const FLOAT64: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "BOOL32"),
            (Self(1), "INT64"),
            (Self(2), "UINT64"),
            (Self(3), "FLOAT64"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PipelineExecutableStatisticFormatKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "PipelineExecutableStatisticFormatKHR",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const DEVICE_DEFAULT_EXT: Self = Self(0);
    pub const DISABLED_EXT: Self = Self(1);
    pub const ROBUST_BUFFER_ACCESS_EXT: Self = Self(2);
    pub const ROBUST_BUFFER_ACCESS_2_EXT: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEVICE_DEFAULT"),
            (Self(1), "DISABLED"),
            (Self(2), "ROBUST_BUFFER_ACCESS"),
            (Self(3), "ROBUST_BUFFER_ACCESS_2"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEVICE_DEFAULT_EXT"),
            (Self(1), "DISABLED_EXT"),
            (Self(2), "ROBUST_BUFFER_ACCESS_EXT"),
            (Self(3), "ROBUST_BUFFER_ACCESS_2_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PipelineRobustnessBufferBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PipelineRobustnessBufferBehavior", self.name(), self.0)
    }
}

pub type PipelineRobustnessBufferBehaviorEXT = PipelineRobustnessBufferBehavior;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const DEVICE_DEFAULT_EXT: Self = Self(0);
    pub const DISABLED_EXT: Self = Self(1);
    pub const ROBUST_IMAGE_ACCESS_EXT: Self = Self(2);
    pub const ROBUST_IMAGE_ACCESS_2_EXT: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEVICE_DEFAULT"),
            (Self(1), "DISABLED"),
            (Self(2), "ROBUST_IMAGE_ACCESS"),
            (Self(3), "ROBUST_IMAGE_ACCESS_2"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEVICE_DEFAULT_EXT"),
            (Self(1), "DISABLED_EXT"),
            (Self(2), "ROBUST_IMAGE_ACCESS_EXT"),
            (Self(3), "ROBUST_IMAGE_ACCESS_2_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PipelineRobustnessImageBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PipelineRobustnessImageBehavior", self.name(), self.0)
    }
}

pub type PipelineRobustnessImageBehaviorEXT = PipelineRobustnessImageBehavior;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const ALL_CLIP_PLANES_KHR: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "ALL_CLIP_PLANES"),
            (Self(1), "USER_CLIP_PLANES_ONLY"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "ALL_CLIP_PLANES_KHR"),
            (Self(1), "USER_CLIP_PLANES_ONLY_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PointClippingBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PointClippingBehavior", self.name(), self.0)
    }
}

pub type PointClippingBehaviorKHR = PointClippingBehavior;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "FILL"),
            (Self(1), "LINE"),
            (Self(2), "POINT"),
            (Self(1000153000), "FILL_RECTANGLE_NV"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PolygonMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PolygonMode", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PresentModeKHR(pub i32);

impl PresentModeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const IMMEDIATE: Self = Self(0);
    pub const MAILBOX: Self = Self(1);
    pub const FIFO: Self = Self(2);
    pub const FIFO_RELAXED: Self = Self(3);
    pub const SHARED_DEMAND_REFRESH: Self = Self(1000111000);
    pub const SHARED_CONTINUOUS_REFRESH: Self = Self(1000111001);
    pub const FIFO_LATEST_READY: Self = Self(1000361000);
    pub const FIFO_LATEST_READY_EXT: Self = Self(1000361000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "IMMEDIATE"),
            (Self(1), "MAILBOX"),
            (Self(2), "FIFO"),
            (Self(3), "FIFO_RELAXED"),
            (Self(1000111000), "SHARED_DEMAND_REFRESH"),
            (Self(1000111001), "SHARED_CONTINUOUS_REFRESH"),
            (Self(1000361000), "FIFO_LATEST_READY"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(1000361000), "FIFO_LATEST_READY_EXT")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PresentModeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PresentModeKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "POINT_LIST"),
            (Self(1), "LINE_LIST"),
            (Self(2), "LINE_STRIP"),
            (Self(3), "TRIANGLE_LIST"),
            (Self(4), "TRIANGLE_STRIP"),
            (Self(5), "TRIANGLE_FAN"),
            (Self(6), "LINE_LIST_WITH_ADJACENCY"),
            (Self(7), "LINE_STRIP_WITH_ADJACENCY"),
            (Self(8), "TRIANGLE_LIST_WITH_ADJACENCY"),
            (Self(9), "TRIANGLE_STRIP_WITH_ADJACENCY"),
            (Self(10), "PATCH_LIST"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for PrimitiveTopology {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "PrimitiveTopology", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ProvokingVertexModeEXT(pub i32);

impl ProvokingVertexModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FIRST_VERTEX: Self = Self(0);
    pub const LAST_VERTEX: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "FIRST_VERTEX"), (Self(1), "LAST_VERTEX")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ProvokingVertexModeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ProvokingVertexModeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct QueryPoolSamplingModeINTEL(pub i32);

impl QueryPoolSamplingModeINTEL {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MANUAL: Self = Self(0);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "MANUAL")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for QueryPoolSamplingModeINTEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "QueryPoolSamplingModeINTEL", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct QueryResultStatusKHR(pub i32);

impl QueryResultStatusKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INSUFFICIENT_BITSTREAM_BUFFER_RANGE: Self = Self(-1000299000);
    pub const ERROR: Self = Self(-1);
    pub const NOT_READY: Self = Self(0);
    pub const COMPLETE: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(-1000299000), "INSUFFICIENT_BITSTREAM_BUFFER_RANGE"),
            (Self(-1), "ERROR"),
            (Self(0), "NOT_READY"),
            (Self(1), "COMPLETE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for QueryResultStatusKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "QueryResultStatusKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const TIME_ELAPSED_QCOM: Self = Self(1000173000);
    pub const PERFORMANCE_QUERY_INTEL: Self = Self(1000210000);
    pub const VIDEO_ENCODE_FEEDBACK_KHR: Self = Self(1000299000);
    pub const MESH_PRIMITIVES_GENERATED_EXT: Self = Self(1000328000);
    pub const PRIMITIVES_GENERATED_EXT: Self = Self(1000382000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR: Self =
        Self(1000386000);
    pub const ACCELERATION_STRUCTURE_SIZE_KHR: Self = Self(1000386001);
    pub const MICROMAP_SERIALIZATION_SIZE_EXT: Self = Self(1000396000);
    pub const MICROMAP_COMPACTED_SIZE_EXT: Self = Self(1000396001);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "OCCLUSION"),
            (Self(1), "PIPELINE_STATISTICS"),
            (Self(2), "TIMESTAMP"),
            (Self(1000023000), "RESULT_STATUS_ONLY_KHR"),
            (Self(1000028004), "TRANSFORM_FEEDBACK_STREAM_EXT"),
            (Self(1000116000), "PERFORMANCE_QUERY_KHR"),
            (
                Self(1000150000),
                "ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR",
            ),
            (
                Self(1000150001),
                "ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR",
            ),
            (Self(1000165000), "ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV"),
            (Self(1000173000), "TIME_ELAPSED_QCOM"),
            (Self(1000210000), "PERFORMANCE_QUERY_INTEL"),
            (Self(1000299000), "VIDEO_ENCODE_FEEDBACK_KHR"),
            (Self(1000328000), "MESH_PRIMITIVES_GENERATED_EXT"),
            (Self(1000382000), "PRIMITIVES_GENERATED_EXT"),
            (
                Self(1000386000),
                "ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR",
            ),
            (Self(1000386001), "ACCELERATION_STRUCTURE_SIZE_KHR"),
            (Self(1000396000), "MICROMAP_SERIALIZATION_SIZE_EXT"),
            (Self(1000396001), "MICROMAP_COMPACTED_SIZE_EXT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for QueryType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "QueryType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const LOW_EXT: Self = Self(128);
    pub const LOW_KHR: Self = Self(128);
    pub const MEDIUM_EXT: Self = Self(256);
    pub const MEDIUM_KHR: Self = Self(256);
    pub const HIGH_EXT: Self = Self(512);
    pub const HIGH_KHR: Self = Self(512);
    pub const REALTIME_EXT: Self = Self(1024);
    pub const REALTIME_KHR: Self = Self(1024);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(128), "LOW"),
            (Self(256), "MEDIUM"),
            (Self(512), "HIGH"),
            (Self(1024), "REALTIME"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(128), "LOW_EXT"),
            (Self(128), "LOW_KHR"),
            (Self(256), "MEDIUM_EXT"),
            (Self(256), "MEDIUM_KHR"),
            (Self(512), "HIGH_EXT"),
            (Self(512), "HIGH_KHR"),
            (Self(1024), "REALTIME_EXT"),
            (Self(1024), "REALTIME_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for QueueGlobalPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "QueueGlobalPriority", self.name(), self.0)
    }
}

pub type QueueGlobalPriorityEXT = QueueGlobalPriority;

pub type QueueGlobalPriorityKHR = QueueGlobalPriority;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RasterizationOrderAMD(pub i32);

impl RasterizationOrderAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const STRICT: Self = Self(0);
    pub const RELAXED: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "STRICT"), (Self(1), "RELAXED")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for RasterizationOrderAMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "RasterizationOrderAMD", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RayTracingInvocationReorderModeEXT(pub i32);

impl RayTracingInvocationReorderModeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NONE: Self = Self(0);
    pub const REORDER: Self = Self(1);
    pub const NONE_NV: Self = Self(0);
    pub const REORDER_NV: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "NONE"), (Self(1), "REORDER")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(0), "NONE_NV"), (Self(1), "REORDER_NV")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for RayTracingInvocationReorderModeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "RayTracingInvocationReorderModeEXT", self.name(), self.0)
    }
}

pub type RayTracingInvocationReorderModeNV = RayTracingInvocationReorderModeEXT;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RayTracingLssIndexingModeNV(pub i32);

impl RayTracingLssIndexingModeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const LIST: Self = Self(0);
    pub const SUCCESSIVE: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "LIST"), (Self(1), "SUCCESSIVE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for RayTracingLssIndexingModeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "RayTracingLssIndexingModeNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RayTracingLssPrimitiveEndCapsModeNV(pub i32);

impl RayTracingLssPrimitiveEndCapsModeNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NONE: Self = Self(0);
    pub const CHAINED: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "NONE"), (Self(1), "CHAINED")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for RayTracingLssPrimitiveEndCapsModeNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(
            f,
            "RayTracingLssPrimitiveEndCapsModeNV",
            self.name(),
            self.0,
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RayTracingShaderGroupTypeKHR(pub i32);

impl RayTracingShaderGroupTypeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const GENERAL: Self = Self(0);
    pub const TRIANGLES_HIT_GROUP: Self = Self(1);
    pub const PROCEDURAL_HIT_GROUP: Self = Self(2);
    pub const GENERAL_NV: Self = Self(0);
    pub const TRIANGLES_HIT_GROUP_NV: Self = Self(1);
    pub const PROCEDURAL_HIT_GROUP_NV: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "GENERAL"),
            (Self(1), "TRIANGLES_HIT_GROUP"),
            (Self(2), "PROCEDURAL_HIT_GROUP"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "GENERAL_NV"),
            (Self(1), "TRIANGLES_HIT_GROUP_NV"),
            (Self(2), "PROCEDURAL_HIT_GROUP_NV"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for RayTracingShaderGroupTypeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "RayTracingShaderGroupTypeKHR", self.name(), self.0)
    }
}

pub type RayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VkResult(pub i32);

impl VkResult {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ERROR_NOT_ENOUGH_SPACE_KHR: Self = Self(-1000483000);
    pub const ERROR_COMPRESSION_EXHAUSTED_EXT: Self = Self(-1000338000);
    pub const ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: Self = Self(-1000299000);
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: Self = Self(-1000257000);
    pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: Self = Self(-1000255000);
    pub const ERROR_PRESENT_TIMING_QUEUE_FULL_EXT: Self = Self(-1000208000);
    pub const ERROR_NOT_PERMITTED: Self = Self(-1000174001);
    pub const ERROR_FRAGMENTATION: Self = Self(-1000161000);
    pub const ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: Self = Self(-1000158000);
    pub const ERROR_INVALID_EXTERNAL_HANDLE: Self = Self(-1000072003);
    pub const ERROR_OUT_OF_POOL_MEMORY: Self = Self(-1000069000);
    pub const ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: Self = Self(-1000023005);
    pub const ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: Self = Self(-1000023004);
    pub const ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: Self = Self(-1000023003);
    pub const ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: Self = Self(-1000023002);
    pub const ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: Self = Self(-1000023001);
    pub const ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: Self = Self(-1000023000);
    pub const ERROR_INVALID_SHADER_NV: Self = Self(-1000012000);
    pub const ERROR_VALIDATION_FAILED: Self = Self(-1000011001);
    pub const ERROR_INCOMPATIBLE_DISPLAY_KHR: Self = Self(-1000003001);
    pub const ERROR_OUT_OF_DATE_KHR: Self = Self(-1000001004);
    pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1000000001);
    pub const ERROR_SURFACE_LOST_KHR: Self = Self(-1000000000);
    pub const ERROR_UNKNOWN: Self = Self(-13);
    pub const ERROR_FRAGMENTED_POOL: Self = Self(-12);
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
    pub const ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
    pub const ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
    pub const ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
    pub const ERROR_DEVICE_LOST: Self = Self(-4);
    pub const ERROR_INITIALIZATION_FAILED: Self = Self(-3);
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
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
    pub const ERROR_INVALID_DEVICE_ADDRESS_EXT: Self = Self(-1000257000);
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: Self = Self(-1000257000);
    pub const ERROR_NOT_PERMITTED_EXT: Self = Self(-1000174001);
    pub const ERROR_NOT_PERMITTED_KHR: Self = Self(-1000174001);
    pub const ERROR_FRAGMENTATION_EXT: Self = Self(-1000161000);
    pub const ERROR_INVALID_EXTERNAL_HANDLE_KHR: Self = Self(-1000072003);
    pub const ERROR_OUT_OF_POOL_MEMORY_KHR: Self = Self(-1000069000);
    pub const ERROR_VALIDATION_FAILED_EXT: Self = Self(-1000011001);
    pub const ERROR_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self(1000297000);
    pub const PIPELINE_COMPILE_REQUIRED_EXT: Self = Self(1000297000);
    pub const ERROR_INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1000482000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(-1000483000), "ERROR_NOT_ENOUGH_SPACE_KHR"),
            (Self(-1000338000), "ERROR_COMPRESSION_EXHAUSTED_EXT"),
            (Self(-1000299000), "ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR"),
            (Self(-1000257000), "ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS"),
            (
                Self(-1000255000),
                "ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT",
            ),
            (Self(-1000208000), "ERROR_PRESENT_TIMING_QUEUE_FULL_EXT"),
            (Self(-1000174001), "ERROR_NOT_PERMITTED"),
            (Self(-1000161000), "ERROR_FRAGMENTATION"),
            (
                Self(-1000158000),
                "ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT",
            ),
            (Self(-1000072003), "ERROR_INVALID_EXTERNAL_HANDLE"),
            (Self(-1000069000), "ERROR_OUT_OF_POOL_MEMORY"),
            (
                Self(-1000023005),
                "ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR",
            ),
            (
                Self(-1000023004),
                "ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR",
            ),
            (
                Self(-1000023003),
                "ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR",
            ),
            (
                Self(-1000023002),
                "ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR",
            ),
            (
                Self(-1000023001),
                "ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR",
            ),
            (Self(-1000023000), "ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR"),
            (Self(-1000012000), "ERROR_INVALID_SHADER_NV"),
            (Self(-1000011001), "ERROR_VALIDATION_FAILED"),
            (Self(-1000003001), "ERROR_INCOMPATIBLE_DISPLAY_KHR"),
            (Self(-1000001004), "ERROR_OUT_OF_DATE_KHR"),
            (Self(-1000000001), "ERROR_NATIVE_WINDOW_IN_USE_KHR"),
            (Self(-1000000000), "ERROR_SURFACE_LOST_KHR"),
            (Self(-13), "ERROR_UNKNOWN"),
            (Self(-12), "ERROR_FRAGMENTED_POOL"),
            (Self(-11), "ERROR_FORMAT_NOT_SUPPORTED"),
            (Self(-10), "ERROR_TOO_MANY_OBJECTS"),
            (Self(-9), "ERROR_INCOMPATIBLE_DRIVER"),
            (Self(-8), "ERROR_FEATURE_NOT_PRESENT"),
            (Self(-7), "ERROR_EXTENSION_NOT_PRESENT"),
            (Self(-6), "ERROR_LAYER_NOT_PRESENT"),
            (Self(-5), "ERROR_MEMORY_MAP_FAILED"),
            (Self(-4), "ERROR_DEVICE_LOST"),
            (Self(-3), "ERROR_INITIALIZATION_FAILED"),
            (Self(-2), "ERROR_OUT_OF_DEVICE_MEMORY"),
            (Self(-1), "ERROR_OUT_OF_HOST_MEMORY"),
            (Self(0), "SUCCESS"),
            (Self(1), "NOT_READY"),
            (Self(2), "TIMEOUT"),
            (Self(3), "EVENT_SET"),
            (Self(4), "EVENT_RESET"),
            (Self(5), "INCOMPLETE"),
            (Self(1000001003), "SUBOPTIMAL_KHR"),
            (Self(1000268000), "THREAD_IDLE_KHR"),
            (Self(1000268001), "THREAD_DONE_KHR"),
            (Self(1000268002), "OPERATION_DEFERRED_KHR"),
            (Self(1000268003), "OPERATION_NOT_DEFERRED_KHR"),
            (Self(1000297000), "PIPELINE_COMPILE_REQUIRED"),
            (Self(1000482000), "INCOMPATIBLE_SHADER_BINARY_EXT"),
            (Self(1000483000), "PIPELINE_BINARY_MISSING_KHR"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(-1000257000), "ERROR_INVALID_DEVICE_ADDRESS_EXT"),
            (
                Self(-1000257000),
                "ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR",
            ),
            (Self(-1000174001), "ERROR_NOT_PERMITTED_EXT"),
            (Self(-1000174001), "ERROR_NOT_PERMITTED_KHR"),
            (Self(-1000161000), "ERROR_FRAGMENTATION_EXT"),
            (Self(-1000072003), "ERROR_INVALID_EXTERNAL_HANDLE_KHR"),
            (Self(-1000069000), "ERROR_OUT_OF_POOL_MEMORY_KHR"),
            (Self(-1000011001), "ERROR_VALIDATION_FAILED_EXT"),
            (Self(1000297000), "ERROR_PIPELINE_COMPILE_REQUIRED_EXT"),
            (Self(1000297000), "PIPELINE_COMPILE_REQUIRED_EXT"),
            (Self(1000482000), "ERROR_INCOMPATIBLE_SHADER_BINARY_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for VkResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "VkResult", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const MIRROR_CLAMP_TO_EDGE_KHR: Self = Self(4);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "REPEAT"),
            (Self(1), "MIRRORED_REPEAT"),
            (Self(2), "CLAMP_TO_EDGE"),
            (Self(3), "CLAMP_TO_BORDER"),
            (Self(4), "MIRROR_CLAMP_TO_EDGE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(4), "MIRROR_CLAMP_TO_EDGE_KHR")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for SamplerAddressMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "SamplerAddressMode", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "NEAREST"), (Self(1), "LINEAR")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for SamplerMipmapMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "SamplerMipmapMode", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const WEIGHTED_AVERAGE_EXT: Self = Self(0);
    pub const MIN_EXT: Self = Self(1);
    pub const MAX_EXT: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "WEIGHTED_AVERAGE"),
            (Self(1), "MIN"),
            (Self(2), "MAX"),
            (Self(1000521000), "WEIGHTED_AVERAGE_RANGECLAMP_QCOM"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "WEIGHTED_AVERAGE_EXT"),
            (Self(1), "MIN_EXT"),
            (Self(2), "MAX_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for SamplerReductionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "SamplerReductionMode", self.name(), self.0)
    }
}

pub type SamplerReductionModeEXT = SamplerReductionMode;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const RGB_IDENTITY_KHR: Self = Self(0);
    pub const YCBCR_IDENTITY_KHR: Self = Self(1);
    pub const YCBCR_709_KHR: Self = Self(2);
    pub const YCBCR_601_KHR: Self = Self(3);
    pub const YCBCR_2020_KHR: Self = Self(4);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "RGB_IDENTITY"),
            (Self(1), "YCBCR_IDENTITY"),
            (Self(2), "YCBCR_709"),
            (Self(3), "YCBCR_601"),
            (Self(4), "YCBCR_2020"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "RGB_IDENTITY_KHR"),
            (Self(1), "YCBCR_IDENTITY_KHR"),
            (Self(2), "YCBCR_709_KHR"),
            (Self(3), "YCBCR_601_KHR"),
            (Self(4), "YCBCR_2020_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for SamplerYcbcrModelConversion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "SamplerYcbcrModelConversion", self.name(), self.0)
    }
}

pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const ITU_FULL_KHR: Self = Self(0);
    pub const ITU_NARROW_KHR: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "ITU_FULL"), (Self(1), "ITU_NARROW")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(0), "ITU_FULL_KHR"), (Self(1), "ITU_NARROW_KHR")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for SamplerYcbcrRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "SamplerYcbcrRange", self.name(), self.0)
    }
}

pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ScopeKHR(pub i32);

impl ScopeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE: Self = Self(1);
    pub const WORKGROUP: Self = Self(2);
    pub const SUBGROUP: Self = Self(3);
    pub const QUEUE_FAMILY: Self = Self(5);
    pub const DEVICE_NV: Self = Self(1);
    pub const WORKGROUP_NV: Self = Self(2);
    pub const SUBGROUP_NV: Self = Self(3);
    pub const QUEUE_FAMILY_NV: Self = Self(5);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(1), "DEVICE"),
            (Self(2), "WORKGROUP"),
            (Self(3), "SUBGROUP"),
            (Self(5), "QUEUE_FAMILY"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(1), "DEVICE_NV"),
            (Self(2), "WORKGROUP_NV"),
            (Self(3), "SUBGROUP_NV"),
            (Self(5), "QUEUE_FAMILY_NV"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ScopeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ScopeKHR", self.name(), self.0)
    }
}

pub type ScopeNV = ScopeKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const BINARY_KHR: Self = Self(0);
    pub const TIMELINE_KHR: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "BINARY"), (Self(1), "TIMELINE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(0), "BINARY_KHR"), (Self(1), "TIMELINE_KHR")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for SemaphoreType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "SemaphoreType", self.name(), self.0)
    }
}

pub type SemaphoreTypeKHR = SemaphoreType;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ShaderCodeTypeEXT(pub i32);

impl ShaderCodeTypeEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BINARY: Self = Self(0);
    pub const SPIRV: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "BINARY"), (Self(1), "SPIRV")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ShaderCodeTypeEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ShaderCodeTypeEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const _32_BIT_ONLY_KHR: Self = Self(0);
    pub const ALL_KHR: Self = Self(1);
    pub const NONE_KHR: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_32_BIT_ONLY"),
            (Self(1), "ALL"),
            (Self(2), "NONE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "_32_BIT_ONLY_KHR"),
            (Self(1), "ALL_KHR"),
            (Self(2), "NONE_KHR"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ShaderFloatControlsIndependence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ShaderFloatControlsIndependence", self.name(), self.0)
    }
}

pub type ShaderFloatControlsIndependenceKHR = ShaderFloatControlsIndependence;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ShaderGroupShaderKHR(pub i32);

impl ShaderGroupShaderKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const GENERAL: Self = Self(0);
    pub const CLOSEST_HIT: Self = Self(1);
    pub const ANY_HIT: Self = Self(2);
    pub const INTERSECTION: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "GENERAL"),
            (Self(1), "CLOSEST_HIT"),
            (Self(2), "ANY_HIT"),
            (Self(3), "INTERSECTION"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ShaderGroupShaderKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ShaderGroupShaderKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ShaderInfoTypeAMD(pub i32);

impl ShaderInfoTypeAMD {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const STATISTICS: Self = Self(0);
    pub const BINARY: Self = Self(1);
    pub const DISASSEMBLY: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "STATISTICS"),
            (Self(1), "BINARY"),
            (Self(2), "DISASSEMBLY"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ShaderInfoTypeAMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ShaderInfoTypeAMD", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ShadingRatePaletteEntryNV(pub i32);

impl ShadingRatePaletteEntryNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NO_INVOCATIONS: Self = Self(0);
    pub const _16_INVOCATIONS_PER_PIXEL: Self = Self(1);
    pub const _8_INVOCATIONS_PER_PIXEL: Self = Self(2);
    pub const _4_INVOCATIONS_PER_PIXEL: Self = Self(3);
    pub const _2_INVOCATIONS_PER_PIXEL: Self = Self(4);
    pub const _1_INVOCATION_PER_PIXEL: Self = Self(5);
    pub const _1_INVOCATION_PER_2X1_PIXELS: Self = Self(6);
    pub const _1_INVOCATION_PER_1X2_PIXELS: Self = Self(7);
    pub const _1_INVOCATION_PER_2X2_PIXELS: Self = Self(8);
    pub const _1_INVOCATION_PER_4X2_PIXELS: Self = Self(9);
    pub const _1_INVOCATION_PER_2X4_PIXELS: Self = Self(10);
    pub const _1_INVOCATION_PER_4X4_PIXELS: Self = Self(11);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "NO_INVOCATIONS"),
            (Self(1), "_16_INVOCATIONS_PER_PIXEL"),
            (Self(2), "_8_INVOCATIONS_PER_PIXEL"),
            (Self(3), "_4_INVOCATIONS_PER_PIXEL"),
            (Self(4), "_2_INVOCATIONS_PER_PIXEL"),
            (Self(5), "_1_INVOCATION_PER_PIXEL"),
            (Self(6), "_1_INVOCATION_PER_2X1_PIXELS"),
            (Self(7), "_1_INVOCATION_PER_1X2_PIXELS"),
            (Self(8), "_1_INVOCATION_PER_2X2_PIXELS"),
            (Self(9), "_1_INVOCATION_PER_4X2_PIXELS"),
            (Self(10), "_1_INVOCATION_PER_2X4_PIXELS"),
            (Self(11), "_1_INVOCATION_PER_4X4_PIXELS"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ShadingRatePaletteEntryNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ShadingRatePaletteEntryNV", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "EXCLUSIVE"), (Self(1), "CONCURRENT")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for SharingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "SharingMode", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "KEEP"),
            (Self(1), "ZERO"),
            (Self(2), "REPLACE"),
            (Self(3), "INCREMENT_AND_CLAMP"),
            (Self(4), "DECREMENT_AND_CLAMP"),
            (Self(5), "INVERT"),
            (Self(6), "INCREMENT_AND_WRAP"),
            (Self(7), "DECREMENT_AND_WRAP"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StencilOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StencilOp", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const PHYSICAL_DEVICE_GPA_FEATURES_AMD: Self = Self(1000133000);
    pub const PHYSICAL_DEVICE_GPA_PROPERTIES_AMD: Self = Self(1000133001);
    pub const GPA_SAMPLE_BEGIN_INFO_AMD: Self = Self(1000133002);
    pub const GPA_SESSION_CREATE_INFO_AMD: Self = Self(1000133003);
    pub const GPA_DEVICE_CLOCK_MODE_INFO_AMD: Self = Self(1000133004);
    pub const PHYSICAL_DEVICE_GPA_PROPERTIES_2_AMD: Self = Self(1000133005);
    pub const GPA_DEVICE_GET_CLOCK_INFO_AMD: Self = Self(1000133006);
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
    pub const PHYSICAL_DEVICE_ELAPSED_TIMER_QUERY_FEATURES_QCOM: Self = Self(1000173000);
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
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_3_FEATURES_QCOM: Self = Self(1000303000);
    pub const PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_FEATURES_QCOM: Self = Self(1000304000);
    pub const PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_PROPERTIES_QCOM: Self = Self(1000304001);
    pub const PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_FEATURES_EXT: Self = Self(1000305000);
    pub const PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_PROPERTIES_EXT: Self = Self(1000305001);
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
    pub const PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_KHR: Self = Self(1000498000);
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
    pub const TENSOR_EXPLICIT_TILING_FORMAT_PROPERTIES_ARM: Self = Self(1000565000);
    pub const TENSOR_ROLLING_BACKING_CREATE_INFO_ARM: Self = Self(1000565001);
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
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_FEEDBACK_2_FEATURES_KHR: Self = Self(1000598000);
    pub const VIDEO_ENCODE_FEEDBACK_2_CAPABILITIES_KHR: Self = Self(1000598001);
    pub const QUERY_POOL_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_CREATE_INFO_KHR: Self =
        Self(1000598002);
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
    pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SWAPCHAIN_FEATURES_EXT: Self =
        Self(1000616000);
    pub const SWAPCHAIN_FLAGS_SURFACE_CAPABILITIES_EXT: Self = Self(1000616001);
    pub const RENDERING_END_INFO_KHR: Self = Self(1000619003);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT: Self = Self(1000620000);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_KHR: Self = Self(1000623000);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_KHR: Self = Self(1000623001);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_MICROMAP_DATA_KHR: Self = Self(1000623002);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_KHR: Self = Self(1000623003);
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
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_MAINTENANCE_1_FEATURES_EXT: Self =
        Self(1000659000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_INFO_2_EXT: Self = Self(1000659001);
    pub const COOPERATIVE_MATRIX_PROPERTIES_2_EXT: Self = Self(1000659002);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT: Self = Self(1000662000);
    pub const UBM_SURFACE_CREATE_INFO_SEC: Self = Self(1000664000);
    pub const FORMAT_PROPERTIES_4_KHR: Self = Self(1000668000);
    pub const IMAGE_CREATE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000668001);
    pub const IMAGE_USAGE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000668002);
    pub const IMAGE_VIEW_USAGE_2_CREATE_INFO_KHR: Self = Self(1000668003);
    pub const PHYSICAL_DEVICE_EXTENDED_FLAGS_FEATURES_KHR: Self = Self(1000668004);
    pub const IMAGE_STENCIL_USAGE_2_CREATE_INFO_KHR: Self = Self(1000668005);
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_2_KHR: Self = Self(1000668006);
    pub const PHYSICAL_DEVICE_SHADER_OCP_MICROSCALING_TYPES_FEATURES_EXT: Self = Self(1000672000);
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
    pub const PHYSICAL_DEVICE_IMAGE_TILING_CONTROL_FEATURES_EXT: Self = Self(1000687000);
    pub const IMAGE_TILING_CONTROL_CREATE_INFO_EXT: Self = Self(1000687001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_DECODE_VECTOR_FEATURES_NV: Self = Self(1000689000);
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_BASE_HANDLE_FEATURES_NV: Self = Self(1000707000);
    pub const PHYSICAL_DEVICE_INFO_PROPERTIES_INTEL: Self = Self(1000708000);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_FEATURES_VALVE: Self =
        Self(1000709000);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_PROPERTIES_VALVE: Self =
        Self(1000709001);
    pub const BUFFER_DEVICE_ADDRESS_ALIGNMENT_ALLOCATE_INFO_VALVE: Self = Self(1000709002);
    pub const DEBUG_REPORT_CREATE_INFO_EXT: Self = Self(1000011000);
    pub const RENDERING_INFO_KHR: Self = Self(1000044000);
    pub const RENDERING_ATTACHMENT_INFO_KHR: Self = Self(1000044001);
    pub const PIPELINE_RENDERING_CREATE_INFO_KHR: Self = Self(1000044002);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: Self = Self(1000044003);
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: Self = Self(1000044004);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_NV: Self = Self(1000044008);
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: Self = Self(1000053000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: Self = Self(1000053001);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: Self = Self(1000053002);
    pub const PHYSICAL_DEVICE_FEATURES_2_KHR: Self = Self(1000059000);
    pub const PHYSICAL_DEVICE_PROPERTIES_2_KHR: Self = Self(1000059001);
    pub const FORMAT_PROPERTIES_2_KHR: Self = Self(1000059002);
    pub const IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self(1000059003);
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: Self = Self(1000059004);
    pub const QUEUE_FAMILY_PROPERTIES_2_KHR: Self = Self(1000059005);
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: Self = Self(1000059006);
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self(1000059007);
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: Self = Self(1000059008);
    pub const MEMORY_ALLOCATE_FLAGS_INFO_KHR: Self = Self(1000060000);
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: Self = Self(1000060003);
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: Self = Self(1000060004);
    pub const DEVICE_GROUP_SUBMIT_INFO_KHR: Self = Self(1000060005);
    pub const DEVICE_GROUP_BIND_SPARSE_INFO_KHR: Self = Self(1000060006);
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: Self = Self(1000060013);
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: Self = Self(1000060014);
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: Self = Self(1000063000);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: Self = Self(1000066000);
    pub const PIPELINE_ROBUSTNESS_CREATE_INFO_EXT: Self = Self(1000068000);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000068001);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT: Self = Self(1000068002);
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: Self = Self(1000070000);
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: Self = Self(1000070001);
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: Self = Self(1000071000);
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self(1000071001);
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: Self = Self(1000071002);
    pub const EXTERNAL_BUFFER_PROPERTIES_KHR: Self = Self(1000071003);
    pub const PHYSICAL_DEVICE_ID_PROPERTIES_KHR: Self = Self(1000071004);
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: Self = Self(1000072000);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: Self = Self(1000072001);
    pub const EXPORT_MEMORY_ALLOCATE_INFO_KHR: Self = Self(1000072002);
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: Self = Self(1000076000);
    pub const EXTERNAL_SEMAPHORE_PROPERTIES_KHR: Self = Self(1000076001);
    pub const EXPORT_SEMAPHORE_CREATE_INFO_KHR: Self = Self(1000077000);
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self = Self(1000080000);
    pub const PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: Self = Self(1000082000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: Self = Self(1000082000);
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: Self = Self(1000083000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: Self = Self(1000085000);
    pub const SURFACE_CAPABILITIES2_EXT: Self = Self(1000090000);
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: Self = Self(1000108000);
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: Self = Self(1000108001);
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: Self = Self(1000108002);
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: Self = Self(1000108003);
    pub const ATTACHMENT_DESCRIPTION_2_KHR: Self = Self(1000109000);
    pub const ATTACHMENT_REFERENCE_2_KHR: Self = Self(1000109001);
    pub const SUBPASS_DESCRIPTION_2_KHR: Self = Self(1000109002);
    pub const SUBPASS_DEPENDENCY_2_KHR: Self = Self(1000109003);
    pub const RENDER_PASS_CREATE_INFO_2_KHR: Self = Self(1000109004);
    pub const SUBPASS_BEGIN_INFO_KHR: Self = Self(1000109005);
    pub const SUBPASS_END_INFO_KHR: Self = Self(1000109006);
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: Self = Self(1000112000);
    pub const EXTERNAL_FENCE_PROPERTIES_KHR: Self = Self(1000112001);
    pub const EXPORT_FENCE_CREATE_INFO_KHR: Self = Self(1000113000);
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: Self = Self(1000117000);
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: Self = Self(1000117001);
    pub const IMAGE_VIEW_USAGE_CREATE_INFO_KHR: Self = Self(1000117002);
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: Self = Self(1000117003);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: Self = Self(1000120000);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: Self = Self(1000120000);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: Self = Self(1000120000);
    pub const MEMORY_DEDICATED_REQUIREMENTS_KHR: Self = Self(1000127000);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_KHR: Self = Self(1000127001);
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self = Self(1000130000);
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self = Self(1000130001);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: Self = Self(1000138000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: Self = Self(1000138001);
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: Self = Self(1000138002);
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: Self = Self(1000138003);
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self(1000146000);
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self(1000146001);
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self(1000146002);
    pub const MEMORY_REQUIREMENTS_2_KHR: Self = Self(1000146003);
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: Self = Self(1000146004);
    pub const IMAGE_FORMAT_LIST_CREATE_INFO_KHR: Self = Self(1000147000);
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: Self = Self(1000156000);
    pub const SAMPLER_YCBCR_CONVERSION_INFO_KHR: Self = Self(1000156001);
    pub const BIND_IMAGE_PLANE_MEMORY_INFO_KHR: Self = Self(1000156002);
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: Self = Self(1000156003);
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: Self = Self(1000156004);
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self(1000156005);
    pub const BIND_BUFFER_MEMORY_INFO_KHR: Self = Self(1000157000);
    pub const BIND_IMAGE_MEMORY_INFO_KHR: Self = Self(1000157001);
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self = Self(1000161000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self = Self(1000161001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self = Self(1000161002);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self = Self(1000161003);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self = Self(1000161004);
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: Self = Self(1000168000);
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: Self = Self(1000168001);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self = Self(1000174000);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: Self = Self(1000174000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: Self = Self(1000175000);
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: Self = Self(1000177000);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: Self = Self(1000180000);
    pub const CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000184000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: Self = Self(1000190001);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR: Self = Self(1000190001);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: Self = Self(1000190002);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR: Self = Self(1000190002);
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000192000);
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: Self = Self(1000196000);
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: Self = Self(1000197000);
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: Self = Self(1000199000);
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: Self = Self(1000199001);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: Self = Self(1000201000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: Self = Self(1000203000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: Self = Self(1000207000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: Self = Self(1000207001);
    pub const SEMAPHORE_TYPE_CREATE_INFO_KHR: Self = Self(1000207002);
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: Self = Self(1000207003);
    pub const SEMAPHORE_WAIT_INFO_KHR: Self = Self(1000207004);
    pub const SEMAPHORE_SIGNAL_INFO_KHR: Self = Self(1000207005);
    pub const QUERY_POOL_CREATE_INFO_INTEL: Self = Self(1000210000);
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: Self = Self(1000211000);
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR: Self = Self(1000215000);
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: Self = Self(1000221000);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: Self = Self(1000225000);
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self = Self(1000225001);
    pub const SHADER_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self = Self(1000225001);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: Self = Self(1000225002);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR: Self = Self(1000232000);
    pub const RENDERING_ATTACHMENT_LOCATION_INFO_KHR: Self = Self(1000232001);
    pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR: Self = Self(1000232002);
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: Self = Self(1000241000);
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: Self = Self(1000241001);
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: Self = Self(1000241002);
    pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: Self = Self(1000244000);
    pub const BUFFER_DEVICE_ADDRESS_INFO_EXT: Self = Self(1000244001);
    pub const BUFFER_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000244001);
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: Self = Self(1000245000);
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: Self = Self(1000246000);
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: Self = Self(1000253000);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: Self = Self(1000257000);
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: Self = Self(1000257002);
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: Self = Self(1000257003);
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: Self = Self(1000257004);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: Self = Self(1000259000);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR: Self = Self(1000259000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: Self = Self(1000259001);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR: Self = Self(1000259001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1000259002);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR: Self = Self(1000259002);
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: Self = Self(1000261000);
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: Self = Self(1000265000);
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR: Self = Self(1000265000);
    pub const PIPELINE_INFO_EXT: Self = Self(1000269001);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT: Self = Self(1000270000);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT: Self = Self(1000270001);
    pub const MEMORY_TO_IMAGE_COPY_EXT: Self = Self(1000270002);
    pub const IMAGE_TO_MEMORY_COPY_EXT: Self = Self(1000270003);
    pub const COPY_IMAGE_TO_MEMORY_INFO_EXT: Self = Self(1000270004);
    pub const COPY_MEMORY_TO_IMAGE_INFO_EXT: Self = Self(1000270005);
    pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO_EXT: Self = Self(1000270006);
    pub const COPY_IMAGE_TO_IMAGE_INFO_EXT: Self = Self(1000270007);
    pub const SUBRESOURCE_HOST_MEMCPY_SIZE_EXT: Self = Self(1000270008);
    pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT: Self = Self(1000270009);
    pub const MEMORY_MAP_INFO_KHR: Self = Self(1000271000);
    pub const MEMORY_UNMAP_INFO_KHR: Self = Self(1000271001);
    pub const SURFACE_PRESENT_MODE_EXT: Self = Self(1000274000);
    pub const SURFACE_PRESENT_SCALING_CAPABILITIES_EXT: Self = Self(1000274001);
    pub const SURFACE_PRESENT_MODE_COMPATIBILITY_EXT: Self = Self(1000274002);
    pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT: Self = Self(1000275000);
    pub const SWAPCHAIN_PRESENT_FENCE_INFO_EXT: Self = Self(1000275001);
    pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT: Self = Self(1000275002);
    pub const SWAPCHAIN_PRESENT_MODE_INFO_EXT: Self = Self(1000275003);
    pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT: Self = Self(1000275004);
    pub const RELEASE_SWAPCHAIN_IMAGES_INFO_EXT: Self = Self(1000275005);
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: Self =
        Self(1000276000);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: Self = Self(1000280000);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: Self = Self(1000280001);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: Self = Self(1000281001);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: Self = Self(1000286000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: Self = Self(1000286001);
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: Self = Self(1000295000);
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: Self = Self(1000295001);
    pub const PRIVATE_DATA_SLOT_CREATE_INFO_EXT: Self = Self(1000295002);
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: Self = Self(1000297000);
    pub const MEMORY_BARRIER_2_KHR: Self = Self(1000314000);
    pub const BUFFER_MEMORY_BARRIER_2_KHR: Self = Self(1000314001);
    pub const IMAGE_MEMORY_BARRIER_2_KHR: Self = Self(1000314002);
    pub const DEPENDENCY_INFO_KHR: Self = Self(1000314003);
    pub const SUBMIT_INFO_2_KHR: Self = Self(1000314004);
    pub const SEMAPHORE_SUBMIT_INFO_KHR: Self = Self(1000314005);
    pub const COMMAND_BUFFER_SUBMIT_INFO_KHR: Self = Self(1000314006);
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: Self = Self(1000314007);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: Self =
        Self(1000325000);
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000335000);
    pub const COPY_BUFFER_INFO_2_KHR: Self = Self(1000337000);
    pub const COPY_IMAGE_INFO_2_KHR: Self = Self(1000337001);
    pub const COPY_BUFFER_TO_IMAGE_INFO_2_KHR: Self = Self(1000337002);
    pub const COPY_IMAGE_TO_BUFFER_INFO_2_KHR: Self = Self(1000337003);
    pub const BLIT_IMAGE_INFO_2_KHR: Self = Self(1000337004);
    pub const RESOLVE_IMAGE_INFO_2_KHR: Self = Self(1000337005);
    pub const BUFFER_COPY_2_KHR: Self = Self(1000337006);
    pub const IMAGE_COPY_2_KHR: Self = Self(1000337007);
    pub const IMAGE_BLIT_2_KHR: Self = Self(1000337008);
    pub const BUFFER_IMAGE_COPY_2_KHR: Self = Self(1000337009);
    pub const IMAGE_RESOLVE_2_KHR: Self = Self(1000337010);
    pub const SUBRESOURCE_LAYOUT_2_EXT: Self = Self(1000338002);
    pub const SUBRESOURCE_LAYOUT_2_KHR: Self = Self(1000338002);
    pub const IMAGE_SUBRESOURCE_2_EXT: Self = Self(1000338003);
    pub const IMAGE_SUBRESOURCE_2_KHR: Self = Self(1000338003);
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: Self =
        Self(1000342000);
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: Self = Self(1000351000);
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: Self = Self(1000351002);
    pub const FORMAT_PROPERTIES_3_KHR: Self = Self(1000360000);
    pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_EXT: Self = Self(1000361000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self = Self(1000388000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: Self = Self(1000388000);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self = Self(1000388001);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: Self = Self(1000388001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: Self = Self(1000413000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: Self = Self(1000413001);
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: Self = Self(1000413002);
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: Self = Self(1000413003);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR: Self = Self(1000416000);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT: Self = Self(1000421000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: Self = Self(1000425000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: Self = Self(1000425001);
    pub const SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: Self = Self(1000425002);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV: Self = Self(1000426001);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV: Self = Self(1000427000);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV: Self = Self(1000427001);
    pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT: Self = Self(1000466000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR: Self = Self(1000470000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR: Self = Self(1000470001);
    pub const RENDERING_AREA_INFO_KHR: Self = Self(1000470003);
    pub const DEVICE_IMAGE_SUBRESOURCE_INFO_KHR: Self = Self(1000470004);
    pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000470005);
    pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000470006);
    pub const PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT: Self = Self(1000498000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR: Self = Self(1000525000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR: Self = Self(1000528000);
    pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR: Self = Self(1000544000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR: Self = Self(1000545000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR: Self = Self(1000545001);
    pub const BIND_MEMORY_STATUS_KHR: Self = Self(1000545002);
    pub const BIND_DESCRIPTOR_SETS_INFO_KHR: Self = Self(1000545003);
    pub const PUSH_CONSTANTS_INFO_KHR: Self = Self(1000545004);
    pub const PUSH_DESCRIPTOR_SET_INFO_KHR: Self = Self(1000545005);
    pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR: Self = Self(1000545006);
    pub const RENDERING_END_INFO_EXT: Self = Self(1000619003);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "APPLICATION_INFO"),
            (Self(1), "INSTANCE_CREATE_INFO"),
            (Self(2), "DEVICE_QUEUE_CREATE_INFO"),
            (Self(3), "DEVICE_CREATE_INFO"),
            (Self(4), "SUBMIT_INFO"),
            (Self(5), "MEMORY_ALLOCATE_INFO"),
            (Self(6), "MAPPED_MEMORY_RANGE"),
            (Self(7), "BIND_SPARSE_INFO"),
            (Self(8), "FENCE_CREATE_INFO"),
            (Self(9), "SEMAPHORE_CREATE_INFO"),
            (Self(10), "EVENT_CREATE_INFO"),
            (Self(11), "QUERY_POOL_CREATE_INFO"),
            (Self(12), "BUFFER_CREATE_INFO"),
            (Self(13), "BUFFER_VIEW_CREATE_INFO"),
            (Self(14), "IMAGE_CREATE_INFO"),
            (Self(15), "IMAGE_VIEW_CREATE_INFO"),
            (Self(16), "SHADER_MODULE_CREATE_INFO"),
            (Self(17), "PIPELINE_CACHE_CREATE_INFO"),
            (Self(18), "PIPELINE_SHADER_STAGE_CREATE_INFO"),
            (Self(19), "PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO"),
            (Self(20), "PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO"),
            (Self(21), "PIPELINE_TESSELLATION_STATE_CREATE_INFO"),
            (Self(22), "PIPELINE_VIEWPORT_STATE_CREATE_INFO"),
            (Self(23), "PIPELINE_RASTERIZATION_STATE_CREATE_INFO"),
            (Self(24), "PIPELINE_MULTISAMPLE_STATE_CREATE_INFO"),
            (Self(25), "PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO"),
            (Self(26), "PIPELINE_COLOR_BLEND_STATE_CREATE_INFO"),
            (Self(27), "PIPELINE_DYNAMIC_STATE_CREATE_INFO"),
            (Self(28), "GRAPHICS_PIPELINE_CREATE_INFO"),
            (Self(29), "COMPUTE_PIPELINE_CREATE_INFO"),
            (Self(30), "PIPELINE_LAYOUT_CREATE_INFO"),
            (Self(31), "SAMPLER_CREATE_INFO"),
            (Self(32), "DESCRIPTOR_SET_LAYOUT_CREATE_INFO"),
            (Self(33), "DESCRIPTOR_POOL_CREATE_INFO"),
            (Self(34), "DESCRIPTOR_SET_ALLOCATE_INFO"),
            (Self(35), "WRITE_DESCRIPTOR_SET"),
            (Self(36), "COPY_DESCRIPTOR_SET"),
            (Self(37), "FRAMEBUFFER_CREATE_INFO"),
            (Self(38), "RENDER_PASS_CREATE_INFO"),
            (Self(39), "COMMAND_POOL_CREATE_INFO"),
            (Self(40), "COMMAND_BUFFER_ALLOCATE_INFO"),
            (Self(41), "COMMAND_BUFFER_INHERITANCE_INFO"),
            (Self(42), "COMMAND_BUFFER_BEGIN_INFO"),
            (Self(43), "RENDER_PASS_BEGIN_INFO"),
            (Self(44), "BUFFER_MEMORY_BARRIER"),
            (Self(45), "IMAGE_MEMORY_BARRIER"),
            (Self(46), "MEMORY_BARRIER"),
            (Self(47), "LOADER_INSTANCE_CREATE_INFO"),
            (Self(48), "LOADER_DEVICE_CREATE_INFO"),
            (Self(49), "PHYSICAL_DEVICE_VULKAN_1_1_FEATURES"),
            (Self(50), "PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES"),
            (Self(51), "PHYSICAL_DEVICE_VULKAN_1_2_FEATURES"),
            (Self(52), "PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES"),
            (Self(53), "PHYSICAL_DEVICE_VULKAN_1_3_FEATURES"),
            (Self(54), "PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES"),
            (Self(55), "PHYSICAL_DEVICE_VULKAN_1_4_FEATURES"),
            (Self(56), "PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES"),
            (Self(1000001000), "SWAPCHAIN_CREATE_INFO_KHR"),
            (Self(1000001001), "PRESENT_INFO_KHR"),
            (Self(1000002000), "DISPLAY_MODE_CREATE_INFO_KHR"),
            (Self(1000002001), "DISPLAY_SURFACE_CREATE_INFO_KHR"),
            (Self(1000003000), "DISPLAY_PRESENT_INFO_KHR"),
            (Self(1000004000), "XLIB_SURFACE_CREATE_INFO_KHR"),
            (Self(1000005000), "XCB_SURFACE_CREATE_INFO_KHR"),
            (Self(1000006000), "WAYLAND_SURFACE_CREATE_INFO_KHR"),
            (Self(1000008000), "ANDROID_SURFACE_CREATE_INFO_KHR"),
            (Self(1000009000), "WIN32_SURFACE_CREATE_INFO_KHR"),
            (Self(1000011000), "DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT"),
            (
                Self(1000018000),
                "PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD",
            ),
            (Self(1000022000), "DEBUG_MARKER_OBJECT_NAME_INFO_EXT"),
            (Self(1000022001), "DEBUG_MARKER_OBJECT_TAG_INFO_EXT"),
            (Self(1000022002), "DEBUG_MARKER_MARKER_INFO_EXT"),
            (Self(1000023000), "VIDEO_PROFILE_INFO_KHR"),
            (Self(1000023001), "VIDEO_CAPABILITIES_KHR"),
            (Self(1000023002), "VIDEO_PICTURE_RESOURCE_INFO_KHR"),
            (Self(1000023003), "VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR"),
            (Self(1000023004), "BIND_VIDEO_SESSION_MEMORY_INFO_KHR"),
            (Self(1000023005), "VIDEO_SESSION_CREATE_INFO_KHR"),
            (Self(1000023006), "VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR"),
            (Self(1000023007), "VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR"),
            (Self(1000023008), "VIDEO_BEGIN_CODING_INFO_KHR"),
            (Self(1000023009), "VIDEO_END_CODING_INFO_KHR"),
            (Self(1000023010), "VIDEO_CODING_CONTROL_INFO_KHR"),
            (Self(1000023011), "VIDEO_REFERENCE_SLOT_INFO_KHR"),
            (Self(1000023012), "QUEUE_FAMILY_VIDEO_PROPERTIES_KHR"),
            (Self(1000023013), "VIDEO_PROFILE_LIST_INFO_KHR"),
            (Self(1000023014), "PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR"),
            (Self(1000023015), "VIDEO_FORMAT_PROPERTIES_KHR"),
            (
                Self(1000023016),
                "QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR",
            ),
            (Self(1000024000), "VIDEO_DECODE_INFO_KHR"),
            (Self(1000024001), "VIDEO_DECODE_CAPABILITIES_KHR"),
            (Self(1000024002), "VIDEO_DECODE_USAGE_INFO_KHR"),
            (
                Self(1000026000),
                "DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV",
            ),
            (
                Self(1000026001),
                "DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV",
            ),
            (
                Self(1000026002),
                "DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV",
            ),
            (
                Self(1000028000),
                "PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT",
            ),
            (
                Self(1000028001),
                "PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT",
            ),
            (
                Self(1000028002),
                "PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT",
            ),
            (Self(1000029000), "CU_MODULE_CREATE_INFO_NVX"),
            (Self(1000029001), "CU_FUNCTION_CREATE_INFO_NVX"),
            (Self(1000029002), "CU_LAUNCH_INFO_NVX"),
            (Self(1000029004), "CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX"),
            (Self(1000030000), "IMAGE_VIEW_HANDLE_INFO_NVX"),
            (Self(1000030001), "IMAGE_VIEW_ADDRESS_PROPERTIES_NVX"),
            (Self(1000038000), "VIDEO_ENCODE_H264_CAPABILITIES_KHR"),
            (
                Self(1000038001),
                "VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR",
            ),
            (
                Self(1000038002),
                "VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR",
            ),
            (Self(1000038003), "VIDEO_ENCODE_H264_PICTURE_INFO_KHR"),
            (Self(1000038004), "VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR"),
            (Self(1000038005), "VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR"),
            (
                Self(1000038006),
                "VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR",
            ),
            (Self(1000038007), "VIDEO_ENCODE_H264_PROFILE_INFO_KHR"),
            (Self(1000038008), "VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR"),
            (
                Self(1000038009),
                "VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR",
            ),
            (
                Self(1000038010),
                "VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR",
            ),
            (
                Self(1000038011),
                "VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR",
            ),
            (
                Self(1000038012),
                "VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR",
            ),
            (
                Self(1000038013),
                "VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR",
            ),
            (Self(1000039000), "VIDEO_ENCODE_H265_CAPABILITIES_KHR"),
            (
                Self(1000039001),
                "VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR",
            ),
            (
                Self(1000039002),
                "VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR",
            ),
            (Self(1000039003), "VIDEO_ENCODE_H265_PICTURE_INFO_KHR"),
            (Self(1000039004), "VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR"),
            (
                Self(1000039005),
                "VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR",
            ),
            (
                Self(1000039006),
                "VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR",
            ),
            (Self(1000039007), "VIDEO_ENCODE_H265_PROFILE_INFO_KHR"),
            (Self(1000039009), "VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR"),
            (
                Self(1000039010),
                "VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR",
            ),
            (
                Self(1000039011),
                "VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR",
            ),
            (
                Self(1000039012),
                "VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR",
            ),
            (
                Self(1000039013),
                "VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR",
            ),
            (
                Self(1000039014),
                "VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR",
            ),
            (Self(1000040000), "VIDEO_DECODE_H264_CAPABILITIES_KHR"),
            (Self(1000040001), "VIDEO_DECODE_H264_PICTURE_INFO_KHR"),
            (Self(1000040003), "VIDEO_DECODE_H264_PROFILE_INFO_KHR"),
            (
                Self(1000040004),
                "VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR",
            ),
            (
                Self(1000040005),
                "VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR",
            ),
            (Self(1000040006), "VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR"),
            (Self(1000041000), "TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD"),
            (Self(1000044000), "RENDERING_INFO"),
            (Self(1000044001), "RENDERING_ATTACHMENT_INFO"),
            (Self(1000044002), "PIPELINE_RENDERING_CREATE_INFO"),
            (
                Self(1000044003),
                "PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES",
            ),
            (
                Self(1000044004),
                "COMMAND_BUFFER_INHERITANCE_RENDERING_INFO",
            ),
            (
                Self(1000044006),
                "RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR",
            ),
            (
                Self(1000044007),
                "RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT",
            ),
            (Self(1000044008), "ATTACHMENT_SAMPLE_COUNT_INFO_AMD"),
            (Self(1000044009), "MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX"),
            (
                Self(1000049000),
                "STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP",
            ),
            (
                Self(1000050000),
                "PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV",
            ),
            (Self(1000053000), "RENDER_PASS_MULTIVIEW_CREATE_INFO"),
            (Self(1000053001), "PHYSICAL_DEVICE_MULTIVIEW_FEATURES"),
            (Self(1000053002), "PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES"),
            (Self(1000056000), "EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV"),
            (Self(1000056001), "EXPORT_MEMORY_ALLOCATE_INFO_NV"),
            (Self(1000057000), "IMPORT_MEMORY_WIN32_HANDLE_INFO_NV"),
            (Self(1000057001), "EXPORT_MEMORY_WIN32_HANDLE_INFO_NV"),
            (
                Self(1000058000),
                "WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV",
            ),
            (Self(1000059000), "PHYSICAL_DEVICE_FEATURES_2"),
            (Self(1000059001), "PHYSICAL_DEVICE_PROPERTIES_2"),
            (Self(1000059002), "FORMAT_PROPERTIES_2"),
            (Self(1000059003), "IMAGE_FORMAT_PROPERTIES_2"),
            (Self(1000059004), "PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2"),
            (Self(1000059005), "QUEUE_FAMILY_PROPERTIES_2"),
            (Self(1000059006), "PHYSICAL_DEVICE_MEMORY_PROPERTIES_2"),
            (Self(1000059007), "SPARSE_IMAGE_FORMAT_PROPERTIES_2"),
            (
                Self(1000059008),
                "PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2",
            ),
            (Self(1000060000), "MEMORY_ALLOCATE_FLAGS_INFO"),
            (Self(1000060003), "DEVICE_GROUP_RENDER_PASS_BEGIN_INFO"),
            (Self(1000060004), "DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO"),
            (Self(1000060005), "DEVICE_GROUP_SUBMIT_INFO"),
            (Self(1000060006), "DEVICE_GROUP_BIND_SPARSE_INFO"),
            (Self(1000060007), "DEVICE_GROUP_PRESENT_CAPABILITIES_KHR"),
            (Self(1000060008), "IMAGE_SWAPCHAIN_CREATE_INFO_KHR"),
            (Self(1000060009), "BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR"),
            (Self(1000060010), "ACQUIRE_NEXT_IMAGE_INFO_KHR"),
            (Self(1000060011), "DEVICE_GROUP_PRESENT_INFO_KHR"),
            (Self(1000060012), "DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR"),
            (Self(1000060013), "BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO"),
            (Self(1000060014), "BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO"),
            (Self(1000061000), "VALIDATION_FLAGS_EXT"),
            (Self(1000062000), "VI_SURFACE_CREATE_INFO_NN"),
            (
                Self(1000063000),
                "PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES",
            ),
            (
                Self(1000066000),
                "PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES",
            ),
            (Self(1000067000), "IMAGE_VIEW_ASTC_DECODE_MODE_EXT"),
            (Self(1000067001), "PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT"),
            (Self(1000068000), "PIPELINE_ROBUSTNESS_CREATE_INFO"),
            (
                Self(1000068001),
                "PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES",
            ),
            (
                Self(1000068002),
                "PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES",
            ),
            (Self(1000070000), "PHYSICAL_DEVICE_GROUP_PROPERTIES"),
            (Self(1000070001), "DEVICE_GROUP_DEVICE_CREATE_INFO"),
            (
                Self(1000071000),
                "PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO",
            ),
            (Self(1000071001), "EXTERNAL_IMAGE_FORMAT_PROPERTIES"),
            (Self(1000071002), "PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO"),
            (Self(1000071003), "EXTERNAL_BUFFER_PROPERTIES"),
            (Self(1000071004), "PHYSICAL_DEVICE_ID_PROPERTIES"),
            (Self(1000072000), "EXTERNAL_MEMORY_BUFFER_CREATE_INFO"),
            (Self(1000072001), "EXTERNAL_MEMORY_IMAGE_CREATE_INFO"),
            (Self(1000072002), "EXPORT_MEMORY_ALLOCATE_INFO"),
            (Self(1000073000), "IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR"),
            (Self(1000073001), "EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR"),
            (Self(1000073002), "MEMORY_WIN32_HANDLE_PROPERTIES_KHR"),
            (Self(1000073003), "MEMORY_GET_WIN32_HANDLE_INFO_KHR"),
            (Self(1000074000), "IMPORT_MEMORY_FD_INFO_KHR"),
            (Self(1000074001), "MEMORY_FD_PROPERTIES_KHR"),
            (Self(1000074002), "MEMORY_GET_FD_INFO_KHR"),
            (
                Self(1000075000),
                "WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR",
            ),
            (Self(1000076000), "PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO"),
            (Self(1000076001), "EXTERNAL_SEMAPHORE_PROPERTIES"),
            (Self(1000077000), "EXPORT_SEMAPHORE_CREATE_INFO"),
            (Self(1000078000), "IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR"),
            (Self(1000078001), "EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR"),
            (Self(1000078002), "D3D12_FENCE_SUBMIT_INFO_KHR"),
            (Self(1000078003), "SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR"),
            (Self(1000079000), "IMPORT_SEMAPHORE_FD_INFO_KHR"),
            (Self(1000079001), "SEMAPHORE_GET_FD_INFO_KHR"),
            (
                Self(1000080000),
                "PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES",
            ),
            (
                Self(1000081000),
                "COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT",
            ),
            (
                Self(1000081001),
                "PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT",
            ),
            (Self(1000081002), "CONDITIONAL_RENDERING_BEGIN_INFO_EXT"),
            (
                Self(1000082000),
                "PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES",
            ),
            (Self(1000083000), "PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES"),
            (Self(1000084000), "PRESENT_REGIONS_KHR"),
            (Self(1000085000), "DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO"),
            (
                Self(1000087000),
                "PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV",
            ),
            (Self(1000090000), "SURFACE_CAPABILITIES_2_EXT"),
            (Self(1000091000), "DISPLAY_POWER_INFO_EXT"),
            (Self(1000091001), "DEVICE_EVENT_INFO_EXT"),
            (Self(1000091002), "DISPLAY_EVENT_INFO_EXT"),
            (Self(1000091003), "SWAPCHAIN_COUNTER_CREATE_INFO_EXT"),
            (Self(1000092000), "PRESENT_TIMES_INFO_GOOGLE"),
            (Self(1000094000), "PHYSICAL_DEVICE_SUBGROUP_PROPERTIES"),
            (
                Self(1000097000),
                "PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX",
            ),
            (
                Self(1000098000),
                "PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV",
            ),
            (
                Self(1000099000),
                "PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT",
            ),
            (
                Self(1000099001),
                "PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT",
            ),
            (
                Self(1000101000),
                "PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT",
            ),
            (
                Self(1000101001),
                "PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT",
            ),
            (
                Self(1000102000),
                "PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT",
            ),
            (
                Self(1000102001),
                "PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT",
            ),
            (Self(1000105000), "HDR_METADATA_EXT"),
            (
                Self(1000108000),
                "PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES",
            ),
            (Self(1000108001), "FRAMEBUFFER_ATTACHMENTS_CREATE_INFO"),
            (Self(1000108002), "FRAMEBUFFER_ATTACHMENT_IMAGE_INFO"),
            (Self(1000108003), "RENDER_PASS_ATTACHMENT_BEGIN_INFO"),
            (Self(1000109000), "ATTACHMENT_DESCRIPTION_2"),
            (Self(1000109001), "ATTACHMENT_REFERENCE_2"),
            (Self(1000109002), "SUBPASS_DESCRIPTION_2"),
            (Self(1000109003), "SUBPASS_DEPENDENCY_2"),
            (Self(1000109004), "RENDER_PASS_CREATE_INFO_2"),
            (Self(1000109005), "SUBPASS_BEGIN_INFO"),
            (Self(1000109006), "SUBPASS_END_INFO"),
            (
                Self(1000110000),
                "PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG",
            ),
            (Self(1000111000), "SHARED_PRESENT_SURFACE_CAPABILITIES_KHR"),
            (Self(1000112000), "PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO"),
            (Self(1000112001), "EXTERNAL_FENCE_PROPERTIES"),
            (Self(1000113000), "EXPORT_FENCE_CREATE_INFO"),
            (Self(1000114000), "IMPORT_FENCE_WIN32_HANDLE_INFO_KHR"),
            (Self(1000114001), "EXPORT_FENCE_WIN32_HANDLE_INFO_KHR"),
            (Self(1000114002), "FENCE_GET_WIN32_HANDLE_INFO_KHR"),
            (Self(1000115000), "IMPORT_FENCE_FD_INFO_KHR"),
            (Self(1000115001), "FENCE_GET_FD_INFO_KHR"),
            (
                Self(1000116000),
                "PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR",
            ),
            (
                Self(1000116001),
                "PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR",
            ),
            (Self(1000116002), "QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR"),
            (Self(1000116003), "PERFORMANCE_QUERY_SUBMIT_INFO_KHR"),
            (Self(1000116004), "ACQUIRE_PROFILING_LOCK_INFO_KHR"),
            (Self(1000116005), "PERFORMANCE_COUNTER_KHR"),
            (Self(1000116006), "PERFORMANCE_COUNTER_DESCRIPTION_KHR"),
            (
                Self(1000117000),
                "PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES",
            ),
            (
                Self(1000117001),
                "RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO",
            ),
            (Self(1000117002), "IMAGE_VIEW_USAGE_CREATE_INFO"),
            (
                Self(1000117003),
                "PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO",
            ),
            (Self(1000119000), "PHYSICAL_DEVICE_SURFACE_INFO_2_KHR"),
            (Self(1000119001), "SURFACE_CAPABILITIES_2_KHR"),
            (Self(1000119002), "SURFACE_FORMAT_2_KHR"),
            (
                Self(1000120000),
                "PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES",
            ),
            (Self(1000121000), "DISPLAY_PROPERTIES_2_KHR"),
            (Self(1000121001), "DISPLAY_PLANE_PROPERTIES_2_KHR"),
            (Self(1000121002), "DISPLAY_MODE_PROPERTIES_2_KHR"),
            (Self(1000121003), "DISPLAY_PLANE_INFO_2_KHR"),
            (Self(1000121004), "DISPLAY_PLANE_CAPABILITIES_2_KHR"),
            (Self(1000122000), "IOS_SURFACE_CREATE_INFO_MVK"),
            (Self(1000123000), "MACOS_SURFACE_CREATE_INFO_MVK"),
            (Self(1000127000), "MEMORY_DEDICATED_REQUIREMENTS"),
            (Self(1000127001), "MEMORY_DEDICATED_ALLOCATE_INFO"),
            (Self(1000128000), "DEBUG_UTILS_OBJECT_NAME_INFO_EXT"),
            (Self(1000128001), "DEBUG_UTILS_OBJECT_TAG_INFO_EXT"),
            (Self(1000128002), "DEBUG_UTILS_LABEL_EXT"),
            (Self(1000128003), "DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT"),
            (Self(1000128004), "DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT"),
            (Self(1000129000), "ANDROID_HARDWARE_BUFFER_USAGE_ANDROID"),
            (
                Self(1000129001),
                "ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID",
            ),
            (
                Self(1000129002),
                "ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID",
            ),
            (
                Self(1000129003),
                "IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID",
            ),
            (
                Self(1000129004),
                "MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID",
            ),
            (Self(1000129005), "EXTERNAL_FORMAT_ANDROID"),
            (
                Self(1000129006),
                "ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID",
            ),
            (
                Self(1000130000),
                "PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES",
            ),
            (Self(1000130001), "SAMPLER_REDUCTION_MODE_CREATE_INFO"),
            (Self(1000133000), "PHYSICAL_DEVICE_GPA_FEATURES_AMD"),
            (Self(1000133001), "PHYSICAL_DEVICE_GPA_PROPERTIES_AMD"),
            (Self(1000133002), "GPA_SAMPLE_BEGIN_INFO_AMD"),
            (Self(1000133003), "GPA_SESSION_CREATE_INFO_AMD"),
            (Self(1000133004), "GPA_DEVICE_CLOCK_MODE_INFO_AMD"),
            (Self(1000133005), "PHYSICAL_DEVICE_GPA_PROPERTIES_2_AMD"),
            (Self(1000133006), "GPA_DEVICE_GET_CLOCK_INFO_AMD"),
            #[cfg(feature = "beta")]
            (
                Self(1000134000),
                "PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000134001),
                "PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000134002),
                "EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000134003),
                "EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000134004),
                "PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX",
            ),
            (Self(1000135000), "TEXEL_BUFFER_DESCRIPTOR_INFO_EXT"),
            (Self(1000135001), "IMAGE_DESCRIPTOR_INFO_EXT"),
            (Self(1000135002), "RESOURCE_DESCRIPTOR_INFO_EXT"),
            (Self(1000135003), "BIND_HEAP_INFO_EXT"),
            (Self(1000135004), "PUSH_DATA_INFO_EXT"),
            (Self(1000135005), "DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT"),
            (
                Self(1000135006),
                "SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT",
            ),
            (Self(1000135007), "OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT"),
            (
                Self(1000135008),
                "PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT",
            ),
            (
                Self(1000135009),
                "PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT",
            ),
            (
                Self(1000135010),
                "COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT",
            ),
            (
                Self(1000135011),
                "SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT",
            ),
            (
                Self(1000135012),
                "INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV",
            ),
            (Self(1000135013), "SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT"),
            (
                Self(1000135014),
                "PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM",
            ),
            (
                Self(1000138000),
                "PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES",
            ),
            (
                Self(1000138001),
                "PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES",
            ),
            (
                Self(1000138002),
                "WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK",
            ),
            (
                Self(1000138003),
                "DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO",
            ),
            (
                Self(1000141000),
                "PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR",
            ),
            (Self(1000143000), "SAMPLE_LOCATIONS_INFO_EXT"),
            (
                Self(1000143001),
                "RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT",
            ),
            (
                Self(1000143002),
                "PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT",
            ),
            (
                Self(1000143003),
                "PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT",
            ),
            (Self(1000143004), "MULTISAMPLE_PROPERTIES_EXT"),
            (Self(1000145000), "PROTECTED_SUBMIT_INFO"),
            (
                Self(1000145001),
                "PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES",
            ),
            (
                Self(1000145002),
                "PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES",
            ),
            (Self(1000145003), "DEVICE_QUEUE_INFO_2"),
            (Self(1000146000), "BUFFER_MEMORY_REQUIREMENTS_INFO_2"),
            (Self(1000146001), "IMAGE_MEMORY_REQUIREMENTS_INFO_2"),
            (Self(1000146002), "IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2"),
            (Self(1000146003), "MEMORY_REQUIREMENTS_2"),
            (Self(1000146004), "SPARSE_IMAGE_MEMORY_REQUIREMENTS_2"),
            (Self(1000147000), "IMAGE_FORMAT_LIST_CREATE_INFO"),
            (
                Self(1000148000),
                "PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT",
            ),
            (
                Self(1000148001),
                "PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT",
            ),
            (
                Self(1000148002),
                "PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT",
            ),
            (
                Self(1000149000),
                "PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV",
            ),
            (
                Self(1000150000),
                "ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR",
            ),
            (
                Self(1000150002),
                "ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR",
            ),
            (
                Self(1000150003),
                "ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR",
            ),
            (
                Self(1000150004),
                "ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR",
            ),
            (
                Self(1000150005),
                "ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR",
            ),
            (Self(1000150006), "ACCELERATION_STRUCTURE_GEOMETRY_KHR"),
            (
                Self(1000150007),
                "WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR",
            ),
            (Self(1000150009), "ACCELERATION_STRUCTURE_VERSION_INFO_KHR"),
            (Self(1000150010), "COPY_ACCELERATION_STRUCTURE_INFO_KHR"),
            (
                Self(1000150011),
                "COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR",
            ),
            (
                Self(1000150012),
                "COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR",
            ),
            (
                Self(1000150013),
                "PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR",
            ),
            (
                Self(1000150014),
                "PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR",
            ),
            (Self(1000150015), "RAY_TRACING_PIPELINE_CREATE_INFO_KHR"),
            (Self(1000150016), "RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR"),
            (Self(1000150017), "ACCELERATION_STRUCTURE_CREATE_INFO_KHR"),
            (
                Self(1000150018),
                "RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR",
            ),
            (
                Self(1000150020),
                "ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR",
            ),
            (
                Self(1000152000),
                "PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV",
            ),
            (
                Self(1000154000),
                "PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV",
            ),
            (
                Self(1000154001),
                "PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV",
            ),
            (Self(1000156000), "SAMPLER_YCBCR_CONVERSION_CREATE_INFO"),
            (Self(1000156001), "SAMPLER_YCBCR_CONVERSION_INFO"),
            (Self(1000156002), "BIND_IMAGE_PLANE_MEMORY_INFO"),
            (Self(1000156003), "IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO"),
            (
                Self(1000156004),
                "PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES",
            ),
            (
                Self(1000156005),
                "SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES",
            ),
            (Self(1000157000), "BIND_BUFFER_MEMORY_INFO"),
            (Self(1000157001), "BIND_IMAGE_MEMORY_INFO"),
            (Self(1000158000), "DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT"),
            (
                Self(1000158002),
                "PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT",
            ),
            (
                Self(1000158003),
                "IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT",
            ),
            (
                Self(1000158004),
                "IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT",
            ),
            (Self(1000158005), "IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT"),
            (
                Self(1000158006),
                "DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT",
            ),
            (Self(1000160000), "VALIDATION_CACHE_CREATE_INFO_EXT"),
            (
                Self(1000160001),
                "SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT",
            ),
            (
                Self(1000161000),
                "DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO",
            ),
            (
                Self(1000161001),
                "PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES",
            ),
            (
                Self(1000161002),
                "PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES",
            ),
            (
                Self(1000161003),
                "DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO",
            ),
            (
                Self(1000161004),
                "DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000163000),
                "PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000163001),
                "PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR",
            ),
            (
                Self(1000164000),
                "PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV",
            ),
            (
                Self(1000164001),
                "PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV",
            ),
            (
                Self(1000164002),
                "PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV",
            ),
            (
                Self(1000164005),
                "PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV",
            ),
            (Self(1000165000), "RAY_TRACING_PIPELINE_CREATE_INFO_NV"),
            (Self(1000165001), "ACCELERATION_STRUCTURE_CREATE_INFO_NV"),
            (Self(1000165003), "GEOMETRY_NV"),
            (Self(1000165004), "GEOMETRY_TRIANGLES_NV"),
            (Self(1000165005), "GEOMETRY_AABB_NV"),
            (
                Self(1000165006),
                "BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV",
            ),
            (
                Self(1000165007),
                "WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV",
            ),
            (
                Self(1000165008),
                "ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV",
            ),
            (
                Self(1000165009),
                "PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV",
            ),
            (Self(1000165011), "RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV"),
            (Self(1000165012), "ACCELERATION_STRUCTURE_INFO_NV"),
            (
                Self(1000166000),
                "PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV",
            ),
            (
                Self(1000166001),
                "PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV",
            ),
            (Self(1000168000), "PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES"),
            (Self(1000168001), "DESCRIPTOR_SET_LAYOUT_SUPPORT"),
            (
                Self(1000170000),
                "PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT",
            ),
            (
                Self(1000170001),
                "FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT",
            ),
            (
                Self(1000172000),
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM",
            ),
            (
                Self(1000173000),
                "PHYSICAL_DEVICE_ELAPSED_TIMER_QUERY_FEATURES_QCOM",
            ),
            (Self(1000174000), "DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO"),
            (
                Self(1000175000),
                "PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES",
            ),
            (Self(1000177000), "PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES"),
            (Self(1000178000), "IMPORT_MEMORY_HOST_POINTER_INFO_EXT"),
            (Self(1000178001), "MEMORY_HOST_POINTER_PROPERTIES_EXT"),
            (
                Self(1000178002),
                "PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT",
            ),
            (
                Self(1000180000),
                "PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES",
            ),
            (
                Self(1000181000),
                "PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR",
            ),
            (
                Self(1000183000),
                "PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD",
            ),
            (Self(1000184000), "CALIBRATED_TIMESTAMP_INFO_KHR"),
            (
                Self(1000185000),
                "PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD",
            ),
            (Self(1000187000), "VIDEO_DECODE_H265_CAPABILITIES_KHR"),
            (
                Self(1000187001),
                "VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR",
            ),
            (
                Self(1000187002),
                "VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR",
            ),
            (Self(1000187003), "VIDEO_DECODE_H265_PROFILE_INFO_KHR"),
            (Self(1000187004), "VIDEO_DECODE_H265_PICTURE_INFO_KHR"),
            (Self(1000187005), "VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR"),
            (
                Self(1000189000),
                "DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD",
            ),
            (
                Self(1000190000),
                "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT",
            ),
            (
                Self(1000190001),
                "PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO",
            ),
            (
                Self(1000190002),
                "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES",
            ),
            (Self(1000191000), "PRESENT_FRAME_TOKEN_GGP"),
            (Self(1000192000), "PIPELINE_CREATION_FEEDBACK_CREATE_INFO"),
            (Self(1000196000), "PHYSICAL_DEVICE_DRIVER_PROPERTIES"),
            (
                Self(1000197000),
                "PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES",
            ),
            (
                Self(1000199000),
                "PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES",
            ),
            (
                Self(1000199001),
                "SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE",
            ),
            (
                Self(1000201000),
                "PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR",
            ),
            (Self(1000202000), "PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV"),
            (
                Self(1000202001),
                "PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV",
            ),
            (
                Self(1000203000),
                "PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR",
            ),
            (
                Self(1000204000),
                "PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV",
            ),
            (
                Self(1000205000),
                "PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV",
            ),
            (
                Self(1000205002),
                "PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV",
            ),
            (Self(1000206000), "CHECKPOINT_DATA_NV"),
            (Self(1000206001), "QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV"),
            (
                Self(1000207000),
                "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES",
            ),
            (
                Self(1000207001),
                "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES",
            ),
            (Self(1000207002), "SEMAPHORE_TYPE_CREATE_INFO"),
            (Self(1000207003), "TIMELINE_SEMAPHORE_SUBMIT_INFO"),
            (Self(1000207004), "SEMAPHORE_WAIT_INFO"),
            (Self(1000207005), "SEMAPHORE_SIGNAL_INFO"),
            (
                Self(1000208000),
                "PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT",
            ),
            (Self(1000208001), "SWAPCHAIN_TIMING_PROPERTIES_EXT"),
            (Self(1000208002), "SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT"),
            (Self(1000208003), "PRESENT_TIMINGS_INFO_EXT"),
            (Self(1000208004), "PRESENT_TIMING_INFO_EXT"),
            (Self(1000208005), "PAST_PRESENTATION_TIMING_INFO_EXT"),
            (Self(1000208006), "PAST_PRESENTATION_TIMING_PROPERTIES_EXT"),
            (Self(1000208007), "PAST_PRESENTATION_TIMING_EXT"),
            (Self(1000208008), "PRESENT_TIMING_SURFACE_CAPABILITIES_EXT"),
            (Self(1000208009), "SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT"),
            (
                Self(1000209000),
                "PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL",
            ),
            (
                Self(1000210000),
                "QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL",
            ),
            (Self(1000210001), "INITIALIZE_PERFORMANCE_API_INFO_INTEL"),
            (Self(1000210002), "PERFORMANCE_MARKER_INFO_INTEL"),
            (Self(1000210003), "PERFORMANCE_STREAM_MARKER_INFO_INTEL"),
            (Self(1000210004), "PERFORMANCE_OVERRIDE_INFO_INTEL"),
            (
                Self(1000210005),
                "PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL",
            ),
            (
                Self(1000211000),
                "PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES",
            ),
            (
                Self(1000212000),
                "PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT",
            ),
            (
                Self(1000213000),
                "DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD",
            ),
            (
                Self(1000213001),
                "SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD",
            ),
            (Self(1000214000), "IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA"),
            (
                Self(1000215000),
                "PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES",
            ),
            (Self(1000217000), "METAL_SURFACE_CREATE_INFO_EXT"),
            (
                Self(1000218000),
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT",
            ),
            (
                Self(1000218001),
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT",
            ),
            (
                Self(1000218002),
                "RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT",
            ),
            (
                Self(1000221000),
                "PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES",
            ),
            (
                Self(1000225000),
                "PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES",
            ),
            (
                Self(1000225001),
                "PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO",
            ),
            (
                Self(1000225002),
                "PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES",
            ),
            (
                Self(1000226000),
                "FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR",
            ),
            (
                Self(1000226001),
                "PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR",
            ),
            (
                Self(1000226002),
                "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR",
            ),
            (
                Self(1000226003),
                "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR",
            ),
            (
                Self(1000226004),
                "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR",
            ),
            (
                Self(1000227000),
                "PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD",
            ),
            (
                Self(1000229000),
                "PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD",
            ),
            (
                Self(1000231000),
                "PHYSICAL_DEVICE_SHADER_CONSTANT_DATA_FEATURES_KHR",
            ),
            (
                Self(1000232000),
                "PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES",
            ),
            (Self(1000232001), "RENDERING_ATTACHMENT_LOCATION_INFO"),
            (Self(1000232002), "RENDERING_INPUT_ATTACHMENT_INDEX_INFO"),
            (
                Self(1000233000),
                "PHYSICAL_DEVICE_SHADER_ABORT_FEATURES_KHR",
            ),
            (
                Self(1000233001),
                "DEVICE_FAULT_SHADER_ABORT_MESSAGE_INFO_KHR",
            ),
            (
                Self(1000233002),
                "PHYSICAL_DEVICE_SHADER_ABORT_PROPERTIES_KHR",
            ),
            (
                Self(1000234000),
                "PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT",
            ),
            (
                Self(1000235000),
                "PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR",
            ),
            (
                Self(1000237000),
                "PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT",
            ),
            (
                Self(1000238000),
                "PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT",
            ),
            (Self(1000238001), "MEMORY_PRIORITY_ALLOCATE_INFO_EXT"),
            (Self(1000239000), "SURFACE_PROTECTED_CAPABILITIES_KHR"),
            (
                Self(1000240000),
                "PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV",
            ),
            (
                Self(1000241000),
                "PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES",
            ),
            (Self(1000241001), "ATTACHMENT_REFERENCE_STENCIL_LAYOUT"),
            (Self(1000241002), "ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT"),
            (
                Self(1000244000),
                "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT",
            ),
            (Self(1000244001), "BUFFER_DEVICE_ADDRESS_INFO"),
            (Self(1000244002), "BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT"),
            (Self(1000245000), "PHYSICAL_DEVICE_TOOL_PROPERTIES"),
            (Self(1000246000), "IMAGE_STENCIL_USAGE_CREATE_INFO"),
            (Self(1000247000), "VALIDATION_FEATURES_EXT"),
            (
                Self(1000248000),
                "PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR",
            ),
            (
                Self(1000249000),
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV",
            ),
            (Self(1000249001), "COOPERATIVE_MATRIX_PROPERTIES_NV"),
            (
                Self(1000249002),
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV",
            ),
            (
                Self(1000250000),
                "PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV",
            ),
            (
                Self(1000250001),
                "PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV",
            ),
            (Self(1000250002), "FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV"),
            (
                Self(1000251000),
                "PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT",
            ),
            (
                Self(1000252000),
                "PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT",
            ),
            (
                Self(1000253000),
                "PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES",
            ),
            (
                Self(1000254000),
                "PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT",
            ),
            (
                Self(1000254001),
                "PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT",
            ),
            (
                Self(1000254002),
                "PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT",
            ),
            (Self(1000255000), "SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT"),
            (
                Self(1000255001),
                "SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT",
            ),
            (
                Self(1000255002),
                "SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT",
            ),
            (Self(1000256000), "HEADLESS_SURFACE_CREATE_INFO_EXT"),
            (
                Self(1000257000),
                "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES",
            ),
            (
                Self(1000257002),
                "BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO",
            ),
            (
                Self(1000257003),
                "MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO",
            ),
            (
                Self(1000257004),
                "DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO",
            ),
            (
                Self(1000259000),
                "PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES",
            ),
            (
                Self(1000259001),
                "PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO",
            ),
            (
                Self(1000259002),
                "PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES",
            ),
            (
                Self(1000260000),
                "PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT",
            ),
            (
                Self(1000261000),
                "PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES",
            ),
            (
                Self(1000265000),
                "PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES",
            ),
            (
                Self(1000267000),
                "PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT",
            ),
            (
                Self(1000269000),
                "PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR",
            ),
            (Self(1000269001), "PIPELINE_INFO_KHR"),
            (Self(1000269002), "PIPELINE_EXECUTABLE_PROPERTIES_KHR"),
            (Self(1000269003), "PIPELINE_EXECUTABLE_INFO_KHR"),
            (Self(1000269004), "PIPELINE_EXECUTABLE_STATISTIC_KHR"),
            (
                Self(1000269005),
                "PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR",
            ),
            (Self(1000270000), "PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES"),
            (
                Self(1000270001),
                "PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES",
            ),
            (Self(1000270002), "MEMORY_TO_IMAGE_COPY"),
            (Self(1000270003), "IMAGE_TO_MEMORY_COPY"),
            (Self(1000270004), "COPY_IMAGE_TO_MEMORY_INFO"),
            (Self(1000270005), "COPY_MEMORY_TO_IMAGE_INFO"),
            (Self(1000270006), "HOST_IMAGE_LAYOUT_TRANSITION_INFO"),
            (Self(1000270007), "COPY_IMAGE_TO_IMAGE_INFO"),
            (Self(1000270008), "SUBRESOURCE_HOST_MEMCPY_SIZE"),
            (Self(1000270009), "HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY"),
            (Self(1000271000), "MEMORY_MAP_INFO"),
            (Self(1000271001), "MEMORY_UNMAP_INFO"),
            (
                Self(1000272000),
                "PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT",
            ),
            (
                Self(1000272001),
                "PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT",
            ),
            (Self(1000272002), "MEMORY_MAP_PLACED_INFO_EXT"),
            (
                Self(1000273000),
                "PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT",
            ),
            (Self(1000274000), "SURFACE_PRESENT_MODE_KHR"),
            (Self(1000274001), "SURFACE_PRESENT_SCALING_CAPABILITIES_KHR"),
            (Self(1000274002), "SURFACE_PRESENT_MODE_COMPATIBILITY_KHR"),
            (
                Self(1000275000),
                "PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR",
            ),
            (Self(1000275001), "SWAPCHAIN_PRESENT_FENCE_INFO_KHR"),
            (Self(1000275002), "SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR"),
            (Self(1000275003), "SWAPCHAIN_PRESENT_MODE_INFO_KHR"),
            (
                Self(1000275004),
                "SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR",
            ),
            (Self(1000275005), "RELEASE_SWAPCHAIN_IMAGES_INFO_KHR"),
            (
                Self(1000276000),
                "PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES",
            ),
            (
                Self(1000277000),
                "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV",
            ),
            (Self(1000277001), "GRAPHICS_SHADER_GROUP_CREATE_INFO_NV"),
            (
                Self(1000277002),
                "GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV",
            ),
            (Self(1000277003), "INDIRECT_COMMANDS_LAYOUT_TOKEN_NV"),
            (Self(1000277004), "INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV"),
            (Self(1000277005), "GENERATED_COMMANDS_INFO_NV"),
            (
                Self(1000277006),
                "GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV",
            ),
            (
                Self(1000277007),
                "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV",
            ),
            (
                Self(1000278000),
                "PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV",
            ),
            (
                Self(1000278001),
                "COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV",
            ),
            (
                Self(1000280000),
                "PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES",
            ),
            (
                Self(1000280001),
                "PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES",
            ),
            (
                Self(1000281000),
                "PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT",
            ),
            (
                Self(1000281001),
                "PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES",
            ),
            (
                Self(1000282000),
                "COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM",
            ),
            (Self(1000282001), "RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM"),
            (
                Self(1000283000),
                "PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT",
            ),
            (Self(1000283001), "DEPTH_BIAS_INFO_EXT"),
            (Self(1000283002), "DEPTH_BIAS_REPRESENTATION_INFO_EXT"),
            (
                Self(1000284000),
                "PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT",
            ),
            (
                Self(1000284001),
                "DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT",
            ),
            (Self(1000284002), "DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT"),
            (
                Self(1000286000),
                "PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR",
            ),
            (
                Self(1000286001),
                "PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR",
            ),
            (
                Self(1000287000),
                "SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT",
            ),
            (
                Self(1000287001),
                "PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT",
            ),
            (
                Self(1000287002),
                "PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT",
            ),
            (
                Self(1000288000),
                "PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT",
            ),
            (Self(1000290000), "PIPELINE_LIBRARY_CREATE_INFO_KHR"),
            (
                Self(1000292000),
                "PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV",
            ),
            (Self(1000292001), "SURFACE_CAPABILITIES_PRESENT_BARRIER_NV"),
            (Self(1000292002), "SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV"),
            (Self(1000294000), "PRESENT_ID_KHR"),
            (Self(1000294001), "PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR"),
            (Self(1000295000), "PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES"),
            (Self(1000295001), "DEVICE_PRIVATE_DATA_CREATE_INFO"),
            (Self(1000295002), "PRIVATE_DATA_SLOT_CREATE_INFO"),
            (
                Self(1000297000),
                "PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES",
            ),
            (Self(1000299000), "VIDEO_ENCODE_INFO_KHR"),
            (Self(1000299001), "VIDEO_ENCODE_RATE_CONTROL_INFO_KHR"),
            (Self(1000299002), "VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR"),
            (Self(1000299003), "VIDEO_ENCODE_CAPABILITIES_KHR"),
            (Self(1000299004), "VIDEO_ENCODE_USAGE_INFO_KHR"),
            (
                Self(1000299005),
                "QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR",
            ),
            (
                Self(1000299006),
                "PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR",
            ),
            (
                Self(1000299007),
                "VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR",
            ),
            (Self(1000299008), "VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR"),
            (
                Self(1000299009),
                "VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR",
            ),
            (
                Self(1000299010),
                "VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR",
            ),
            (
                Self(1000300000),
                "PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV",
            ),
            (Self(1000300001), "DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV"),
            (Self(1000302000), "PERF_HINT_INFO_QCOM"),
            (
                Self(1000302001),
                "PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM",
            ),
            (
                Self(1000302002),
                "PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM",
            ),
            (
                Self(1000303000),
                "PHYSICAL_DEVICE_IMAGE_PROCESSING_3_FEATURES_QCOM",
            ),
            (
                Self(1000304000),
                "PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_FEATURES_QCOM",
            ),
            (
                Self(1000304001),
                "PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_PROPERTIES_QCOM",
            ),
            (
                Self(1000305000),
                "PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_FEATURES_EXT",
            ),
            (
                Self(1000305001),
                "PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_PROPERTIES_EXT",
            ),
            #[cfg(feature = "beta")]
            (Self(1000307000), "CUDA_MODULE_CREATE_INFO_NV"),
            #[cfg(feature = "beta")]
            (Self(1000307001), "CUDA_FUNCTION_CREATE_INFO_NV"),
            #[cfg(feature = "beta")]
            (Self(1000307002), "CUDA_LAUNCH_INFO_NV"),
            #[cfg(feature = "beta")]
            (
                Self(1000307003),
                "PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000307004),
                "PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV",
            ),
            (
                Self(1000309000),
                "PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM",
            ),
            (
                Self(1000309001),
                "PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM",
            ),
            (
                Self(1000309002),
                "RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM",
            ),
            (Self(1000309003), "PER_TILE_BEGIN_INFO_QCOM"),
            (Self(1000309004), "PER_TILE_END_INFO_QCOM"),
            (Self(1000309005), "DISPATCH_TILE_INFO_QCOM"),
            (Self(1000310000), "QUERY_LOW_LATENCY_SUPPORT_NV"),
            (Self(1000311000), "EXPORT_METAL_OBJECT_CREATE_INFO_EXT"),
            (Self(1000311001), "EXPORT_METAL_OBJECTS_INFO_EXT"),
            (Self(1000311002), "EXPORT_METAL_DEVICE_INFO_EXT"),
            (Self(1000311003), "EXPORT_METAL_COMMAND_QUEUE_INFO_EXT"),
            (Self(1000311004), "EXPORT_METAL_BUFFER_INFO_EXT"),
            (Self(1000311005), "IMPORT_METAL_BUFFER_INFO_EXT"),
            (Self(1000311006), "EXPORT_METAL_TEXTURE_INFO_EXT"),
            (Self(1000311007), "IMPORT_METAL_TEXTURE_INFO_EXT"),
            (Self(1000311008), "EXPORT_METAL_IO_SURFACE_INFO_EXT"),
            (Self(1000311009), "IMPORT_METAL_IO_SURFACE_INFO_EXT"),
            (Self(1000311010), "EXPORT_METAL_SHARED_EVENT_INFO_EXT"),
            (Self(1000311011), "IMPORT_METAL_SHARED_EVENT_INFO_EXT"),
            (Self(1000314000), "MEMORY_BARRIER_2"),
            (Self(1000314001), "BUFFER_MEMORY_BARRIER_2"),
            (Self(1000314002), "IMAGE_MEMORY_BARRIER_2"),
            (Self(1000314003), "DEPENDENCY_INFO"),
            (Self(1000314004), "SUBMIT_INFO_2"),
            (Self(1000314005), "SEMAPHORE_SUBMIT_INFO"),
            (Self(1000314006), "COMMAND_BUFFER_SUBMIT_INFO"),
            (
                Self(1000314007),
                "PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES",
            ),
            (Self(1000314008), "QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV"),
            (Self(1000314009), "CHECKPOINT_DATA_2_NV"),
            (
                Self(1000316000),
                "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT",
            ),
            (
                Self(1000316001),
                "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT",
            ),
            (
                Self(1000316002),
                "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT",
            ),
            (Self(1000316003), "DESCRIPTOR_ADDRESS_INFO_EXT"),
            (Self(1000316004), "DESCRIPTOR_GET_INFO_EXT"),
            (Self(1000316005), "BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT"),
            (Self(1000316006), "IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT"),
            (
                Self(1000316007),
                "IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT",
            ),
            (Self(1000316008), "SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT"),
            (
                Self(1000316009),
                "ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT",
            ),
            (
                Self(1000316010),
                "OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT",
            ),
            (Self(1000316011), "DESCRIPTOR_BUFFER_BINDING_INFO_EXT"),
            (
                Self(1000316012),
                "DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT",
            ),
            (Self(1000318000), "DEVICE_MEMORY_COPY_KHR"),
            (Self(1000318001), "COPY_DEVICE_MEMORY_INFO_KHR"),
            (Self(1000318002), "DEVICE_MEMORY_IMAGE_COPY_KHR"),
            (Self(1000318003), "COPY_DEVICE_MEMORY_IMAGE_INFO_KHR"),
            (Self(1000318004), "MEMORY_RANGE_BARRIERS_INFO_KHR"),
            (Self(1000318005), "MEMORY_RANGE_BARRIER_KHR"),
            (
                Self(1000318006),
                "PHYSICAL_DEVICE_DEVICE_ADDRESS_COMMANDS_FEATURES_KHR",
            ),
            (Self(1000318007), "BIND_INDEX_BUFFER_3_INFO_KHR"),
            (Self(1000318008), "BIND_VERTEX_BUFFER_3_INFO_KHR"),
            (Self(1000318009), "DRAW_INDIRECT_2_INFO_KHR"),
            (Self(1000318010), "DRAW_INDIRECT_COUNT_2_INFO_KHR"),
            (Self(1000318011), "DISPATCH_INDIRECT_2_INFO_KHR"),
            (Self(1000318012), "CONDITIONAL_RENDERING_BEGIN_INFO_2_EXT"),
            (
                Self(1000318013),
                "BIND_TRANSFORM_FEEDBACK_BUFFER_2_INFO_EXT",
            ),
            (Self(1000318014), "MEMORY_MARKER_INFO_AMD"),
            (Self(1000318015), "ACCELERATION_STRUCTURE_CREATE_INFO_2_KHR"),
            (
                Self(1000320000),
                "PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT",
            ),
            (
                Self(1000320001),
                "PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT",
            ),
            (
                Self(1000320002),
                "GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT",
            ),
            (
                Self(1000321000),
                "PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD",
            ),
            (
                Self(1000322000),
                "PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR",
            ),
            (
                Self(1000323000),
                "PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR",
            ),
            (
                Self(1000325000),
                "PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES",
            ),
            (
                Self(1000326000),
                "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV",
            ),
            (
                Self(1000326001),
                "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV",
            ),
            (
                Self(1000326002),
                "PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV",
            ),
            (
                Self(1000327000),
                "ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV",
            ),
            (
                Self(1000327001),
                "PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV",
            ),
            (Self(1000327002), "ACCELERATION_STRUCTURE_MOTION_INFO_NV"),
            (Self(1000328000), "PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT"),
            (
                Self(1000328001),
                "PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT",
            ),
            (
                Self(1000330000),
                "PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT",
            ),
            (
                Self(1000332000),
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT",
            ),
            (
                Self(1000332001),
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT",
            ),
            (Self(1000333000), "COPY_COMMAND_TRANSFORM_INFO_QCOM"),
            (
                Self(1000335000),
                "PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES",
            ),
            (
                Self(1000336000),
                "PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR",
            ),
            (Self(1000337000), "COPY_BUFFER_INFO_2"),
            (Self(1000337001), "COPY_IMAGE_INFO_2"),
            (Self(1000337002), "COPY_BUFFER_TO_IMAGE_INFO_2"),
            (Self(1000337003), "COPY_IMAGE_TO_BUFFER_INFO_2"),
            (Self(1000337004), "BLIT_IMAGE_INFO_2"),
            (Self(1000337005), "RESOLVE_IMAGE_INFO_2"),
            (Self(1000337006), "BUFFER_COPY_2"),
            (Self(1000337007), "IMAGE_COPY_2"),
            (Self(1000337008), "IMAGE_BLIT_2"),
            (Self(1000337009), "BUFFER_IMAGE_COPY_2"),
            (Self(1000337010), "IMAGE_RESOLVE_2"),
            (
                Self(1000338000),
                "PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT",
            ),
            (Self(1000338001), "IMAGE_COMPRESSION_CONTROL_EXT"),
            (Self(1000338002), "SUBRESOURCE_LAYOUT_2"),
            (Self(1000338003), "IMAGE_SUBRESOURCE_2"),
            (Self(1000338004), "IMAGE_COMPRESSION_PROPERTIES_EXT"),
            (
                Self(1000339000),
                "PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT",
            ),
            (
                Self(1000340000),
                "PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT",
            ),
            (Self(1000341000), "PHYSICAL_DEVICE_FAULT_FEATURES_EXT"),
            (Self(1000341001), "DEVICE_FAULT_COUNTS_EXT"),
            (Self(1000341002), "DEVICE_FAULT_INFO_EXT"),
            (
                Self(1000342000),
                "PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT",
            ),
            (
                Self(1000344000),
                "PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT",
            ),
            (Self(1000346000), "DIRECTFB_SURFACE_CREATE_INFO_EXT"),
            (
                Self(1000347000),
                "PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR",
            ),
            (
                Self(1000347001),
                "PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR",
            ),
            (Self(1000348013), "PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR"),
            (
                Self(1000351000),
                "PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT",
            ),
            (Self(1000351002), "MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT"),
            (
                Self(1000352000),
                "PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT",
            ),
            (Self(1000352001), "VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT"),
            (Self(1000352002), "VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT"),
            (Self(1000353000), "PHYSICAL_DEVICE_DRM_PROPERTIES_EXT"),
            (
                Self(1000354000),
                "PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT",
            ),
            (Self(1000354001), "DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT"),
            (
                Self(1000355000),
                "PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT",
            ),
            (
                Self(1000355001),
                "PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT",
            ),
            (
                Self(1000356000),
                "PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT",
            ),
            (Self(1000360000), "FORMAT_PROPERTIES_3"),
            (
                Self(1000361000),
                "PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR",
            ),
            (Self(1000364000), "IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA"),
            (Self(1000364001), "MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA"),
            (Self(1000364002), "MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA"),
            (
                Self(1000365000),
                "IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA",
            ),
            (Self(1000365001), "SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA"),
            (Self(1000366000), "BUFFER_COLLECTION_CREATE_INFO_FUCHSIA"),
            (Self(1000366001), "IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA"),
            (
                Self(1000366002),
                "BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA",
            ),
            (Self(1000366003), "BUFFER_COLLECTION_PROPERTIES_FUCHSIA"),
            (Self(1000366004), "BUFFER_CONSTRAINTS_INFO_FUCHSIA"),
            (
                Self(1000366005),
                "BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA",
            ),
            (Self(1000366006), "IMAGE_CONSTRAINTS_INFO_FUCHSIA"),
            (Self(1000366007), "IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA"),
            (Self(1000366008), "SYSMEM_COLOR_SPACE_FUCHSIA"),
            (
                Self(1000366009),
                "BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA",
            ),
            (
                Self(1000369000),
                "SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI",
            ),
            (
                Self(1000369001),
                "PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI",
            ),
            (
                Self(1000369002),
                "PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI",
            ),
            (
                Self(1000370000),
                "PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI",
            ),
            (Self(1000371000), "MEMORY_GET_REMOTE_ADDRESS_INFO_NV"),
            (
                Self(1000371001),
                "PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV",
            ),
            (Self(1000372000), "PIPELINE_PROPERTIES_IDENTIFIER_EXT"),
            (
                Self(1000372001),
                "PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT",
            ),
            (
                Self(1000375000),
                "PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT",
            ),
            (Self(1000375001), "FRAME_BOUNDARY_EXT"),
            (
                Self(1000376000),
                "PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT",
            ),
            (Self(1000376001), "SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT"),
            (
                Self(1000376002),
                "MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT",
            ),
            (
                Self(1000377000),
                "PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT",
            ),
            (Self(1000378000), "SCREEN_SURFACE_CREATE_INFO_QNX"),
            (
                Self(1000381000),
                "PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT",
            ),
            (Self(1000381001), "PIPELINE_COLOR_WRITE_CREATE_INFO_EXT"),
            (
                Self(1000382000),
                "PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT",
            ),
            (
                Self(1000386000),
                "PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR",
            ),
            (
                Self(1000387000),
                "PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR",
            ),
            (
                Self(1000388000),
                "PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES",
            ),
            (Self(1000388001), "QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES"),
            (
                Self(1000390000),
                "PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE",
            ),
            (
                Self(1000390001),
                "VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE",
            ),
            (
                Self(1000390002),
                "VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE",
            ),
            (
                Self(1000390003),
                "VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE",
            ),
            (
                Self(1000391000),
                "PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT",
            ),
            (Self(1000391001), "IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT"),
            (Self(1000392000), "PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT"),
            (
                Self(1000392001),
                "PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT",
            ),
            (
                Self(1000393000),
                "PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT",
            ),
            (
                Self(1000395000),
                "PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT",
            ),
            (
                Self(1000395001),
                "PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT",
            ),
            (Self(1000396000), "MICROMAP_BUILD_INFO_EXT"),
            (Self(1000396001), "MICROMAP_VERSION_INFO_EXT"),
            (Self(1000396002), "COPY_MICROMAP_INFO_EXT"),
            (Self(1000396003), "COPY_MICROMAP_TO_MEMORY_INFO_EXT"),
            (Self(1000396004), "COPY_MEMORY_TO_MICROMAP_INFO_EXT"),
            (
                Self(1000396005),
                "PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT",
            ),
            (
                Self(1000396006),
                "PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT",
            ),
            (Self(1000396007), "MICROMAP_CREATE_INFO_EXT"),
            (Self(1000396008), "MICROMAP_BUILD_SIZES_INFO_EXT"),
            (
                Self(1000396009),
                "ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000397000),
                "PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000397001),
                "PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000397002),
                "ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV",
            ),
            (
                Self(1000404000),
                "PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI",
            ),
            (
                Self(1000404001),
                "PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI",
            ),
            (
                Self(1000404002),
                "PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI",
            ),
            (
                Self(1000411000),
                "PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT",
            ),
            (
                Self(1000411001),
                "SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT",
            ),
            (
                Self(1000412000),
                "PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT",
            ),
            (Self(1000413000), "PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES"),
            (Self(1000413001), "PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES"),
            (Self(1000413002), "DEVICE_BUFFER_MEMORY_REQUIREMENTS"),
            (Self(1000413003), "DEVICE_IMAGE_MEMORY_REQUIREMENTS"),
            (
                Self(1000415000),
                "PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM",
            ),
            (
                Self(1000416000),
                "PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES",
            ),
            (
                Self(1000417000),
                "DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM",
            ),
            (
                Self(1000417001),
                "PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM",
            ),
            (
                Self(1000417002),
                "PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM",
            ),
            (Self(1000417003), "DISPATCH_PARAMETERS_ARM"),
            (
                Self(1000417004),
                "PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM",
            ),
            (
                Self(1000418000),
                "PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT",
            ),
            (Self(1000418001), "IMAGE_VIEW_SLICED_CREATE_INFO_EXT"),
            (
                Self(1000420000),
                "PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE",
            ),
            (Self(1000420001), "DESCRIPTOR_SET_BINDING_REFERENCE_VALVE"),
            (
                Self(1000420002),
                "DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE",
            ),
            (
                Self(1000421000),
                "PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR",
            ),
            (
                Self(1000422000),
                "PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT",
            ),
            (
                Self(1000424000),
                "PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM",
            ),
            (
                Self(1000424001),
                "PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM",
            ),
            (Self(1000424002), "RENDER_PASS_STRIPE_BEGIN_INFO_ARM"),
            (Self(1000424003), "RENDER_PASS_STRIPE_INFO_ARM"),
            (Self(1000424004), "RENDER_PASS_STRIPE_SUBMIT_INFO_ARM"),
            (
                Self(1000425000),
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT",
            ),
            (
                Self(1000425001),
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT",
            ),
            (
                Self(1000425002),
                "RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT",
            ),
            (
                Self(1000426000),
                "PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV",
            ),
            (
                Self(1000426001),
                "PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR",
            ),
            (
                Self(1000427000),
                "PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT",
            ),
            (
                Self(1000427001),
                "PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT",
            ),
            (
                Self(1000428000),
                "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV",
            ),
            (Self(1000428001), "COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV"),
            (Self(1000428002), "PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV"),
            (
                Self(1000429008),
                "PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV",
            ),
            (
                Self(1000429009),
                "ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV",
            ),
            (
                Self(1000429010),
                "ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV",
            ),
            (
                Self(1000430000),
                "PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV",
            ),
            (
                Self(1000434000),
                "PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR",
            ),
            (
                Self(1000437000),
                "PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT",
            ),
            (
                Self(1000440000),
                "PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM",
            ),
            (
                Self(1000440001),
                "PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM",
            ),
            (
                Self(1000440002),
                "IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM",
            ),
            (
                Self(1000451000),
                "PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT",
            ),
            (
                Self(1000451001),
                "PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT",
            ),
            (Self(1000452000), "NATIVE_BUFFER_USAGE_OHOS"),
            (Self(1000452001), "NATIVE_BUFFER_PROPERTIES_OHOS"),
            (Self(1000452002), "NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS"),
            (Self(1000452003), "IMPORT_NATIVE_BUFFER_INFO_OHOS"),
            (Self(1000452004), "MEMORY_GET_NATIVE_BUFFER_INFO_OHOS"),
            (Self(1000452005), "EXTERNAL_FORMAT_OHOS"),
            (Self(1000453000), "EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT"),
            (
                Self(1000455000),
                "PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT",
            ),
            (
                Self(1000455001),
                "PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT",
            ),
            (
                Self(1000458000),
                "PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT",
            ),
            (Self(1000458001), "RENDER_PASS_CREATION_CONTROL_EXT"),
            (
                Self(1000458002),
                "RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT",
            ),
            (
                Self(1000458003),
                "RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT",
            ),
            (Self(1000459000), "DIRECT_DRIVER_LOADING_INFO_LUNARG"),
            (Self(1000459001), "DIRECT_DRIVER_LOADING_LIST_LUNARG"),
            (Self(1000460000), "TENSOR_CREATE_INFO_ARM"),
            (Self(1000460001), "TENSOR_VIEW_CREATE_INFO_ARM"),
            (Self(1000460002), "BIND_TENSOR_MEMORY_INFO_ARM"),
            (Self(1000460003), "WRITE_DESCRIPTOR_SET_TENSOR_ARM"),
            (Self(1000460004), "PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM"),
            (Self(1000460005), "TENSOR_FORMAT_PROPERTIES_ARM"),
            (Self(1000460006), "TENSOR_DESCRIPTION_ARM"),
            (Self(1000460007), "TENSOR_MEMORY_REQUIREMENTS_INFO_ARM"),
            (Self(1000460008), "TENSOR_MEMORY_BARRIER_ARM"),
            (Self(1000460009), "PHYSICAL_DEVICE_TENSOR_FEATURES_ARM"),
            (Self(1000460010), "DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM"),
            (Self(1000460011), "COPY_TENSOR_INFO_ARM"),
            (Self(1000460012), "TENSOR_COPY_ARM"),
            (Self(1000460013), "TENSOR_DEPENDENCY_INFO_ARM"),
            (
                Self(1000460014),
                "MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM",
            ),
            (Self(1000460015), "PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM"),
            (Self(1000460016), "EXTERNAL_TENSOR_PROPERTIES_ARM"),
            (Self(1000460017), "EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM"),
            (
                Self(1000460018),
                "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM",
            ),
            (
                Self(1000460019),
                "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM",
            ),
            (Self(1000460020), "DESCRIPTOR_GET_TENSOR_INFO_ARM"),
            (Self(1000460021), "TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM"),
            (
                Self(1000460022),
                "TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM",
            ),
            (Self(1000460023), "FRAME_BOUNDARY_TENSORS_ARM"),
            (
                Self(1000462000),
                "PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT",
            ),
            (
                Self(1000462001),
                "PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT",
            ),
            (
                Self(1000462002),
                "PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT",
            ),
            (Self(1000462003), "SHADER_MODULE_IDENTIFIER_EXT"),
            (Self(1000464000), "PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV"),
            (
                Self(1000464001),
                "PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV",
            ),
            (Self(1000464002), "OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV"),
            (Self(1000464003), "OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV"),
            (Self(1000464004), "OPTICAL_FLOW_SESSION_CREATE_INFO_NV"),
            (Self(1000464005), "OPTICAL_FLOW_EXECUTE_INFO_NV"),
            (
                Self(1000464010),
                "OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV",
            ),
            (
                Self(1000465000),
                "PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT",
            ),
            (
                Self(1000466000),
                "PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES",
            ),
            (
                Self(1000468000),
                "PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID",
            ),
            (
                Self(1000468001),
                "PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID",
            ),
            (
                Self(1000468002),
                "ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID",
            ),
            (Self(1000470000), "PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES"),
            (Self(1000470001), "PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES"),
            (Self(1000470003), "RENDERING_AREA_INFO"),
            (Self(1000470004), "DEVICE_IMAGE_SUBRESOURCE_INFO"),
            (Self(1000470005), "PIPELINE_CREATE_FLAGS_2_CREATE_INFO"),
            (Self(1000470006), "BUFFER_USAGE_FLAGS_2_CREATE_INFO"),
            (Self(1000476000), "PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD"),
            (Self(1000476001), "ANTI_LAG_DATA_AMD"),
            (Self(1000476002), "ANTI_LAG_PRESENTATION_INFO_AMD"),
            #[cfg(feature = "beta")]
            (
                Self(1000478000),
                "PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX",
            ),
            #[cfg(feature = "beta")]
            (
                Self(1000478001),
                "ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX",
            ),
            (Self(1000479000), "SURFACE_CAPABILITIES_PRESENT_ID_2_KHR"),
            (Self(1000479001), "PRESENT_ID_2_KHR"),
            (
                Self(1000479002),
                "PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR",
            ),
            (Self(1000480000), "SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR"),
            (
                Self(1000480001),
                "PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR",
            ),
            (Self(1000480002), "PRESENT_WAIT_2_INFO_KHR"),
            (
                Self(1000481000),
                "PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR",
            ),
            (
                Self(1000482000),
                "PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT",
            ),
            (
                Self(1000482001),
                "PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT",
            ),
            (Self(1000482002), "SHADER_CREATE_INFO_EXT"),
            (
                Self(1000483000),
                "PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR",
            ),
            (Self(1000483001), "PIPELINE_BINARY_CREATE_INFO_KHR"),
            (Self(1000483002), "PIPELINE_BINARY_INFO_KHR"),
            (Self(1000483003), "PIPELINE_BINARY_KEY_KHR"),
            (
                Self(1000483004),
                "PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR",
            ),
            (Self(1000483005), "RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR"),
            (Self(1000483006), "PIPELINE_BINARY_DATA_INFO_KHR"),
            (Self(1000483007), "PIPELINE_CREATE_INFO_KHR"),
            (
                Self(1000483008),
                "DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR",
            ),
            (Self(1000483009), "PIPELINE_BINARY_HANDLES_INFO_KHR"),
            (
                Self(1000484000),
                "PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM",
            ),
            (Self(1000484001), "TILE_PROPERTIES_QCOM"),
            (
                Self(1000485000),
                "PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC",
            ),
            (Self(1000485001), "AMIGO_PROFILING_SUBMIT_INFO_SEC"),
            (
                Self(1000488000),
                "PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM",
            ),
            (
                Self(1000490000),
                "PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV",
            ),
            (
                Self(1000490001),
                "PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV",
            ),
            (
                Self(1000491000),
                "PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV",
            ),
            (
                Self(1000491001),
                "PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV",
            ),
            (Self(1000491002), "COOPERATIVE_VECTOR_PROPERTIES_NV"),
            (
                Self(1000491004),
                "CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV",
            ),
            (
                Self(1000492000),
                "PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV",
            ),
            (
                Self(1000492001),
                "PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV",
            ),
            (
                Self(1000495000),
                "PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT",
            ),
            (
                Self(1000495001),
                "PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT",
            ),
            (Self(1000496000), "LAYER_SETTINGS_CREATE_INFO_EXT"),
            (
                Self(1000497000),
                "PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM",
            ),
            (
                Self(1000497001),
                "PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM",
            ),
            (
                Self(1000498000),
                "PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_KHR",
            ),
            (
                Self(1000499000),
                "PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT",
            ),
            (
                Self(1000504000),
                "PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR",
            ),
            (Self(1000505000), "LATENCY_SLEEP_MODE_INFO_NV"),
            (Self(1000505001), "LATENCY_SLEEP_INFO_NV"),
            (Self(1000505002), "SET_LATENCY_MARKER_INFO_NV"),
            (Self(1000505003), "GET_LATENCY_MARKER_INFO_NV"),
            (Self(1000505004), "LATENCY_TIMINGS_FRAME_REPORT_NV"),
            (Self(1000505005), "LATENCY_SUBMISSION_PRESENT_ID_NV"),
            (Self(1000505006), "OUT_OF_BAND_QUEUE_TYPE_INFO_NV"),
            (Self(1000505007), "SWAPCHAIN_LATENCY_CREATE_INFO_NV"),
            (Self(1000505008), "LATENCY_SURFACE_CAPABILITIES_NV"),
            (
                Self(1000506000),
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR",
            ),
            (Self(1000506001), "COOPERATIVE_MATRIX_PROPERTIES_KHR"),
            (
                Self(1000506002),
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR",
            ),
            (Self(1000507000), "DATA_GRAPH_PIPELINE_CREATE_INFO_ARM"),
            (
                Self(1000507001),
                "DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM",
            ),
            (Self(1000507002), "DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM"),
            (Self(1000507003), "DATA_GRAPH_PIPELINE_CONSTANT_ARM"),
            (
                Self(1000507004),
                "DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM",
            ),
            (
                Self(1000507005),
                "BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM",
            ),
            (Self(1000507006), "PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM"),
            (
                Self(1000507007),
                "DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM",
            ),
            (
                Self(1000507008),
                "DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM",
            ),
            (Self(1000507009), "DATA_GRAPH_PIPELINE_INFO_ARM"),
            (
                Self(1000507010),
                "DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM",
            ),
            (
                Self(1000507011),
                "DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM",
            ),
            (
                Self(1000507012),
                "DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM",
            ),
            (
                Self(1000507013),
                "DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM",
            ),
            (Self(1000507014), "DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM"),
            (
                Self(1000507015),
                "DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM",
            ),
            (
                Self(1000507016),
                "DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM",
            ),
            (
                Self(1000507017),
                "QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM",
            ),
            (Self(1000507018), "QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM"),
            (
                Self(1000507019),
                "PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM",
            ),
            (
                Self(1000508000),
                "QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM",
            ),
            (
                Self(1000510000),
                "PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM",
            ),
            (
                Self(1000510001),
                "MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM",
            ),
            (
                Self(1000511000),
                "PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR",
            ),
            (Self(1000512000), "VIDEO_DECODE_AV1_CAPABILITIES_KHR"),
            (Self(1000512001), "VIDEO_DECODE_AV1_PICTURE_INFO_KHR"),
            (Self(1000512003), "VIDEO_DECODE_AV1_PROFILE_INFO_KHR"),
            (
                Self(1000512004),
                "VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR",
            ),
            (Self(1000512005), "VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR"),
            (Self(1000513000), "VIDEO_ENCODE_AV1_CAPABILITIES_KHR"),
            (
                Self(1000513001),
                "VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR",
            ),
            (Self(1000513002), "VIDEO_ENCODE_AV1_PICTURE_INFO_KHR"),
            (Self(1000513003), "VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR"),
            (
                Self(1000513004),
                "PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR",
            ),
            (Self(1000513005), "VIDEO_ENCODE_AV1_PROFILE_INFO_KHR"),
            (Self(1000513006), "VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR"),
            (
                Self(1000513007),
                "VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR",
            ),
            (
                Self(1000513008),
                "VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR",
            ),
            (Self(1000513009), "VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR"),
            (
                Self(1000513010),
                "VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR",
            ),
            (
                Self(1000514000),
                "PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR",
            ),
            (Self(1000514001), "VIDEO_DECODE_VP9_CAPABILITIES_KHR"),
            (Self(1000514002), "VIDEO_DECODE_VP9_PICTURE_INFO_KHR"),
            (Self(1000514003), "VIDEO_DECODE_VP9_PROFILE_INFO_KHR"),
            (
                Self(1000515000),
                "PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR",
            ),
            (Self(1000515001), "VIDEO_INLINE_QUERY_INFO_KHR"),
            (
                Self(1000516000),
                "PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV",
            ),
            (
                Self(1000518000),
                "PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM",
            ),
            (
                Self(1000518001),
                "PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM",
            ),
            (
                Self(1000518002),
                "SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM",
            ),
            (Self(1000519000), "SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM"),
            (
                Self(1000519001),
                "PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM",
            ),
            (Self(1000519002), "BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM"),
            (
                Self(1000520000),
                "PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM",
            ),
            (
                Self(1000520001),
                "SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM",
            ),
            (
                Self(1000521000),
                "PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM",
            ),
            (
                Self(1000524000),
                "PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT",
            ),
            (
                Self(1000525000),
                "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES",
            ),
            (
                Self(1000527000),
                "PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR",
            ),
            (Self(1000527001), "ATTACHMENT_FEEDBACK_LOOP_INFO_EXT"),
            (
                Self(1000528000),
                "PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES",
            ),
            (Self(1000529000), "SCREEN_BUFFER_PROPERTIES_QNX"),
            (Self(1000529001), "SCREEN_BUFFER_FORMAT_PROPERTIES_QNX"),
            (Self(1000529002), "IMPORT_SCREEN_BUFFER_INFO_QNX"),
            (Self(1000529003), "EXTERNAL_FORMAT_QNX"),
            (
                Self(1000529004),
                "PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX",
            ),
            (
                Self(1000530000),
                "PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT",
            ),
            (
                Self(1000544000),
                "PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES",
            ),
            (Self(1000545000), "PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES"),
            (Self(1000545001), "PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES"),
            (Self(1000545002), "BIND_MEMORY_STATUS"),
            (Self(1000545003), "BIND_DESCRIPTOR_SETS_INFO"),
            (Self(1000545004), "PUSH_CONSTANTS_INFO"),
            (Self(1000545005), "PUSH_DESCRIPTOR_SET_INFO"),
            (Self(1000545006), "PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO"),
            (Self(1000545007), "SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT"),
            (
                Self(1000545008),
                "BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT",
            ),
            (
                Self(1000546000),
                "PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV",
            ),
            (
                Self(1000547000),
                "PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM",
            ),
            (
                Self(1000547001),
                "PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM",
            ),
            (Self(1000547002), "TILE_MEMORY_REQUIREMENTS_QCOM"),
            (Self(1000547003), "TILE_MEMORY_BIND_INFO_QCOM"),
            (Self(1000547004), "TILE_MEMORY_SIZE_INFO_QCOM"),
            (
                Self(1000549000),
                "PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR",
            ),
            (Self(1000549002), "COPY_MEMORY_INDIRECT_INFO_KHR"),
            (Self(1000549003), "COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR"),
            (Self(1000550002), "DECOMPRESS_MEMORY_INFO_EXT"),
            (Self(1000551000), "DISPLAY_SURFACE_STEREO_CREATE_INFO_NV"),
            (Self(1000551001), "DISPLAY_MODE_STEREO_PROPERTIES_NV"),
            (
                Self(1000552000),
                "VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR",
            ),
            (
                Self(1000552001),
                "VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR",
            ),
            (Self(1000552002), "VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR"),
            (Self(1000552003), "VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR"),
            (
                Self(1000552004),
                "PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR",
            ),
            (
                Self(1000553000),
                "VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR",
            ),
            (
                Self(1000553001),
                "VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR",
            ),
            (Self(1000553002), "VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR"),
            (
                Self(1000553003),
                "VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR",
            ),
            (
                Self(1000553004),
                "VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR",
            ),
            (
                Self(1000553005),
                "VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR",
            ),
            (
                Self(1000553006),
                "VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR",
            ),
            (
                Self(1000553007),
                "VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR",
            ),
            (
                Self(1000553008),
                "VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR",
            ),
            (
                Self(1000553009),
                "PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR",
            ),
            (
                Self(1000555000),
                "PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV",
            ),
            (
                Self(1000556000),
                "EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV",
            ),
            (Self(1000556001), "EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV"),
            (Self(1000556002), "EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV"),
            (
                Self(1000556003),
                "PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV",
            ),
            (
                Self(1000558000),
                "PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR",
            ),
            (
                Self(1000559000),
                "PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV",
            ),
            (
                Self(1000562000),
                "PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR",
            ),
            (
                Self(1000562001),
                "PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR",
            ),
            (
                Self(1000562002),
                "PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR",
            ),
            (
                Self(1000562003),
                "PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR",
            ),
            (
                Self(1000562004),
                "PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR",
            ),
            (
                Self(1000563000),
                "PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV",
            ),
            (
                Self(1000564000),
                "PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT",
            ),
            (
                Self(1000565000),
                "TENSOR_EXPLICIT_TILING_FORMAT_PROPERTIES_ARM",
            ),
            (Self(1000565001), "TENSOR_ROLLING_BACKING_CREATE_INFO_ARM"),
            (
                Self(1000567000),
                "PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT",
            ),
            (
                Self(1000568000),
                "PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV",
            ),
            (
                Self(1000569000),
                "PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV",
            ),
            (
                Self(1000569001),
                "PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV",
            ),
            (
                Self(1000569002),
                "CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV",
            ),
            (
                Self(1000569003),
                "CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV",
            ),
            (
                Self(1000569004),
                "CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV",
            ),
            (
                Self(1000569005),
                "CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV",
            ),
            (
                Self(1000569006),
                "CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV",
            ),
            (
                Self(1000569007),
                "RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV",
            ),
            (
                Self(1000570000),
                "PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV",
            ),
            (
                Self(1000570001),
                "PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV",
            ),
            (
                Self(1000570002),
                "WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV",
            ),
            (
                Self(1000570003),
                "PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV",
            ),
            (
                Self(1000570004),
                "BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV",
            ),
            (
                Self(1000570005),
                "PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV",
            ),
            (
                Self(1000572000),
                "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT",
            ),
            (
                Self(1000572001),
                "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT",
            ),
            (
                Self(1000572002),
                "GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT",
            ),
            (Self(1000572003), "INDIRECT_EXECUTION_SET_CREATE_INFO_EXT"),
            (Self(1000572004), "GENERATED_COMMANDS_INFO_EXT"),
            (Self(1000572006), "INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT"),
            (Self(1000572007), "INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT"),
            (
                Self(1000572008),
                "WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT",
            ),
            (Self(1000572009), "WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT"),
            (Self(1000572010), "INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT"),
            (Self(1000572011), "INDIRECT_EXECUTION_SET_SHADER_INFO_EXT"),
            (
                Self(1000572012),
                "INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT",
            ),
            (Self(1000572013), "GENERATED_COMMANDS_PIPELINE_INFO_EXT"),
            (Self(1000572014), "GENERATED_COMMANDS_SHADER_INFO_EXT"),
            (Self(1000573000), "PHYSICAL_DEVICE_FAULT_FEATURES_KHR"),
            (Self(1000573001), "PHYSICAL_DEVICE_FAULT_PROPERTIES_KHR"),
            (Self(1000573002), "DEVICE_FAULT_INFO_KHR"),
            (Self(1000573003), "DEVICE_FAULT_DEBUG_INFO_KHR"),
            (
                Self(1000574000),
                "PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR",
            ),
            (Self(1000574002), "MEMORY_BARRIER_ACCESS_FLAGS_3_KHR"),
            (
                Self(1000575000),
                "PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA",
            ),
            (
                Self(1000575001),
                "PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA",
            ),
            (Self(1000575002), "IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA"),
            (Self(1000579000), "PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR"),
            (Self(1000580000), "PUSH_CONSTANT_BANK_INFO_NV"),
            (
                Self(1000580001),
                "PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV",
            ),
            (
                Self(1000580002),
                "PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV",
            ),
            (
                Self(1000581000),
                "PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT",
            ),
            (
                Self(1000581001),
                "PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT",
            ),
            (
                Self(1000582000),
                "PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT",
            ),
            (
                Self(1000582001),
                "PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT",
            ),
            (
                Self(1000584000),
                "PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR",
            ),
            (
                Self(1000584001),
                "PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR",
            ),
            (
                Self(1000584002),
                "QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR",
            ),
            (
                Self(1000586000),
                "PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR",
            ),
            (
                Self(1000586001),
                "VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR",
            ),
            (
                Self(1000586002),
                "VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR",
            ),
            (
                Self(1000586003),
                "VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR",
            ),
            (
                Self(1000590000),
                "PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI",
            ),
            (Self(1000590001), "HDR_VIVID_DYNAMIC_METADATA_HUAWEI"),
            (
                Self(1000593000),
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV",
            ),
            (
                Self(1000593001),
                "COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV",
            ),
            (
                Self(1000593002),
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV",
            ),
            (
                Self(1000596000),
                "PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM",
            ),
            (
                Self(1000598000),
                "PHYSICAL_DEVICE_VIDEO_ENCODE_FEEDBACK_2_FEATURES_KHR",
            ),
            (Self(1000598001), "VIDEO_ENCODE_FEEDBACK_2_CAPABILITIES_KHR"),
            (
                Self(1000598002),
                "QUERY_POOL_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_CREATE_INFO_KHR",
            ),
            (Self(1000602000), "IMPORT_MEMORY_METAL_HANDLE_INFO_EXT"),
            (Self(1000602001), "MEMORY_METAL_HANDLE_PROPERTIES_EXT"),
            (Self(1000602002), "MEMORY_GET_METAL_HANDLE_INFO_EXT"),
            (
                Self(1000605000),
                "PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM",
            ),
            (
                Self(1000605001),
                "PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM",
            ),
            (Self(1000605002), "PERFORMANCE_COUNTER_ARM"),
            (Self(1000605003), "PERFORMANCE_COUNTER_DESCRIPTION_ARM"),
            (
                Self(1000605004),
                "RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM",
            ),
            (
                Self(1000607000),
                "PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM",
            ),
            (
                Self(1000607001),
                "PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM",
            ),
            (Self(1000607002), "SHADER_INSTRUMENTATION_CREATE_INFO_ARM"),
            (
                Self(1000607003),
                "SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM",
            ),
            (
                Self(1000608000),
                "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT",
            ),
            (Self(1000609000), "PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM"),
            (
                Self(1000611000),
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE",
            ),
            (
                Self(1000611001),
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE",
            ),
            (
                Self(1000611002),
                "PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE",
            ),
            (Self(1000613000), "SET_PRESENT_CONFIG_NV"),
            (
                Self(1000613001),
                "PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV",
            ),
            (
                Self(1000616000),
                "PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SWAPCHAIN_FEATURES_EXT",
            ),
            (Self(1000616001), "SWAPCHAIN_FLAGS_SURFACE_CAPABILITIES_EXT"),
            (Self(1000619003), "RENDERING_END_INFO_KHR"),
            (
                Self(1000620000),
                "PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT",
            ),
            (
                Self(1000623000),
                "PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_KHR",
            ),
            (
                Self(1000623001),
                "PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_KHR",
            ),
            (
                Self(1000623002),
                "ACCELERATION_STRUCTURE_GEOMETRY_MICROMAP_DATA_KHR",
            ),
            (
                Self(1000623003),
                "ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_KHR",
            ),
            (
                Self(1000627000),
                "PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT",
            ),
            (
                Self(1000628000),
                "PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT",
            ),
            (Self(1000628001), "BEGIN_CUSTOM_RESOLVE_INFO_EXT"),
            (Self(1000628002), "CUSTOM_RESOLVE_CREATE_INFO_EXT"),
            (
                Self(1000629000),
                "PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM",
            ),
            (
                Self(1000629001),
                "DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM",
            ),
            (
                Self(1000630000),
                "PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR",
            ),
            (
                Self(1000630001),
                "PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR",
            ),
            (Self(1000630002), "RENDERING_ATTACHMENT_FLAGS_INFO_KHR"),
            (Self(1000630004), "RESOLVE_IMAGE_MODE_INFO_KHR"),
            (
                Self(1000631000),
                "PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM",
            ),
            (
                Self(1000631001),
                "QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM",
            ),
            (
                Self(1000631002),
                "DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM",
            ),
            (
                Self(1000631003),
                "DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM",
            ),
            (
                Self(1000631004),
                "DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM",
            ),
            (
                Self(1000631005),
                "DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM",
            ),
            (
                Self(1000631006),
                "DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM",
            ),
            (
                Self(1000631007),
                "DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM",
            ),
            (
                Self(1000631008),
                "DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM",
            ),
            (
                Self(1000635000),
                "PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT",
            ),
            (
                Self(1000635001),
                "PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT",
            ),
            (
                Self(1000637000),
                "PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC",
            ),
            (
                Self(1000642000),
                "PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT",
            ),
            (Self(1000645000), "COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV"),
            (
                Self(1000645001),
                "PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV",
            ),
            (
                Self(1000657000),
                "PHYSICAL_DEVICE_MAINTENANCE_11_FEATURES_KHR",
            ),
            (
                Self(1000657001),
                "QUEUE_FAMILY_OPTIMAL_IMAGE_TRANSFER_GRANULARITY_PROPERTIES_KHR",
            ),
            (
                Self(1000659000),
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_MAINTENANCE_1_FEATURES_EXT",
            ),
            (
                Self(1000659001),
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_INFO_2_EXT",
            ),
            (Self(1000659002), "COOPERATIVE_MATRIX_PROPERTIES_2_EXT"),
            (
                Self(1000662000),
                "PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT",
            ),
            (Self(1000664000), "UBM_SURFACE_CREATE_INFO_SEC"),
            (Self(1000668000), "FORMAT_PROPERTIES_4_KHR"),
            (Self(1000668001), "IMAGE_CREATE_FLAGS_2_CREATE_INFO_KHR"),
            (Self(1000668002), "IMAGE_USAGE_FLAGS_2_CREATE_INFO_KHR"),
            (Self(1000668003), "IMAGE_VIEW_USAGE_2_CREATE_INFO_KHR"),
            (
                Self(1000668004),
                "PHYSICAL_DEVICE_EXTENDED_FLAGS_FEATURES_KHR",
            ),
            (Self(1000668005), "IMAGE_STENCIL_USAGE_2_CREATE_INFO_KHR"),
            (
                Self(1000668006),
                "SHARED_PRESENT_SURFACE_CAPABILITIES_2_KHR",
            ),
            (
                Self(1000672000),
                "PHYSICAL_DEVICE_SHADER_OCP_MICROSCALING_TYPES_FEATURES_EXT",
            ),
            (
                Self(1000673000),
                "PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE",
            ),
            (
                Self(1000674000),
                "PHYSICAL_DEVICE_THROTTLE_HINT_FEATURES_SEC",
            ),
            (Self(1000674001), "THROTTLE_HINT_SUBMIT_INFO_SEC"),
            (
                Self(1000676000),
                "DATA_GRAPH_PIPELINE_NEURAL_STATISTICS_CREATE_INFO_ARM",
            ),
            (
                Self(1000676001),
                "DATA_GRAPH_PIPELINE_SESSION_NEURAL_STATISTICS_CREATE_INFO_ARM",
            ),
            (
                Self(1000676002),
                "PHYSICAL_DEVICE_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_FEATURES_ARM",
            ),
            (
                Self(1000678000),
                "PHYSICAL_DEVICE_PRIMITIVE_RESTART_INDEX_FEATURES_EXT",
            ),
            (Self(1000685000), "SURFACE_CREATE_INFO_OHOS"),
            (
                Self(1000687000),
                "PHYSICAL_DEVICE_IMAGE_TILING_CONTROL_FEATURES_EXT",
            ),
            (Self(1000687001), "IMAGE_TILING_CONTROL_CREATE_INFO_EXT"),
            (
                Self(1000689000),
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_DECODE_VECTOR_FEATURES_NV",
            ),
            (
                Self(1000707000),
                "PHYSICAL_DEVICE_PRIVATE_DATA_BASE_HANDLE_FEATURES_NV",
            ),
            (Self(1000708000), "PHYSICAL_DEVICE_INFO_PROPERTIES_INTEL"),
            (
                Self(1000709000),
                "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_FEATURES_VALVE",
            ),
            (
                Self(1000709001),
                "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_PROPERTIES_VALVE",
            ),
            (
                Self(1000709002),
                "BUFFER_DEVICE_ADDRESS_ALIGNMENT_ALLOCATE_INFO_VALVE",
            ),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(1000011000), "DEBUG_REPORT_CREATE_INFO_EXT"),
            (Self(1000044000), "RENDERING_INFO_KHR"),
            (Self(1000044001), "RENDERING_ATTACHMENT_INFO_KHR"),
            (Self(1000044002), "PIPELINE_RENDERING_CREATE_INFO_KHR"),
            (
                Self(1000044003),
                "PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR",
            ),
            (
                Self(1000044004),
                "COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR",
            ),
            (Self(1000044008), "ATTACHMENT_SAMPLE_COUNT_INFO_NV"),
            (Self(1000053000), "RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR"),
            (Self(1000053001), "PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR"),
            (Self(1000053002), "PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR"),
            (Self(1000059000), "PHYSICAL_DEVICE_FEATURES_2_KHR"),
            (Self(1000059001), "PHYSICAL_DEVICE_PROPERTIES_2_KHR"),
            (Self(1000059002), "FORMAT_PROPERTIES_2_KHR"),
            (Self(1000059003), "IMAGE_FORMAT_PROPERTIES_2_KHR"),
            (Self(1000059004), "PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR"),
            (Self(1000059005), "QUEUE_FAMILY_PROPERTIES_2_KHR"),
            (Self(1000059006), "PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR"),
            (Self(1000059007), "SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR"),
            (
                Self(1000059008),
                "PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR",
            ),
            (Self(1000060000), "MEMORY_ALLOCATE_FLAGS_INFO_KHR"),
            (Self(1000060003), "DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR"),
            (
                Self(1000060004),
                "DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR",
            ),
            (Self(1000060005), "DEVICE_GROUP_SUBMIT_INFO_KHR"),
            (Self(1000060006), "DEVICE_GROUP_BIND_SPARSE_INFO_KHR"),
            (Self(1000060013), "BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR"),
            (Self(1000060014), "BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR"),
            (
                Self(1000063000),
                "PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES",
            ),
            (
                Self(1000066000),
                "PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT",
            ),
            (Self(1000068000), "PIPELINE_ROBUSTNESS_CREATE_INFO_EXT"),
            (
                Self(1000068001),
                "PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT",
            ),
            (
                Self(1000068002),
                "PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT",
            ),
            (Self(1000070000), "PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR"),
            (Self(1000070001), "DEVICE_GROUP_DEVICE_CREATE_INFO_KHR"),
            (
                Self(1000071000),
                "PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR",
            ),
            (Self(1000071001), "EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR"),
            (Self(1000071002), "PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR"),
            (Self(1000071003), "EXTERNAL_BUFFER_PROPERTIES_KHR"),
            (Self(1000071004), "PHYSICAL_DEVICE_ID_PROPERTIES_KHR"),
            (Self(1000072000), "EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR"),
            (Self(1000072001), "EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR"),
            (Self(1000072002), "EXPORT_MEMORY_ALLOCATE_INFO_KHR"),
            (
                Self(1000076000),
                "PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR",
            ),
            (Self(1000076001), "EXTERNAL_SEMAPHORE_PROPERTIES_KHR"),
            (Self(1000077000), "EXPORT_SEMAPHORE_CREATE_INFO_KHR"),
            (
                Self(1000080000),
                "PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR",
            ),
            (
                Self(1000082000),
                "PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR",
            ),
            (
                Self(1000082000),
                "PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR",
            ),
            (
                Self(1000083000),
                "PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR",
            ),
            (
                Self(1000085000),
                "DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR",
            ),
            (Self(1000090000), "SURFACE_CAPABILITIES2_EXT"),
            (
                Self(1000108000),
                "PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR",
            ),
            (Self(1000108001), "FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR"),
            (Self(1000108002), "FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR"),
            (Self(1000108003), "RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR"),
            (Self(1000109000), "ATTACHMENT_DESCRIPTION_2_KHR"),
            (Self(1000109001), "ATTACHMENT_REFERENCE_2_KHR"),
            (Self(1000109002), "SUBPASS_DESCRIPTION_2_KHR"),
            (Self(1000109003), "SUBPASS_DEPENDENCY_2_KHR"),
            (Self(1000109004), "RENDER_PASS_CREATE_INFO_2_KHR"),
            (Self(1000109005), "SUBPASS_BEGIN_INFO_KHR"),
            (Self(1000109006), "SUBPASS_END_INFO_KHR"),
            (Self(1000112000), "PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR"),
            (Self(1000112001), "EXTERNAL_FENCE_PROPERTIES_KHR"),
            (Self(1000113000), "EXPORT_FENCE_CREATE_INFO_KHR"),
            (
                Self(1000117000),
                "PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR",
            ),
            (
                Self(1000117001),
                "RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR",
            ),
            (Self(1000117002), "IMAGE_VIEW_USAGE_CREATE_INFO_KHR"),
            (
                Self(1000117003),
                "PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR",
            ),
            (
                Self(1000120000),
                "PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR",
            ),
            (
                Self(1000120000),
                "PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES",
            ),
            (
                Self(1000120000),
                "PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR",
            ),
            (Self(1000127000), "MEMORY_DEDICATED_REQUIREMENTS_KHR"),
            (Self(1000127001), "MEMORY_DEDICATED_ALLOCATE_INFO_KHR"),
            (
                Self(1000130000),
                "PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT",
            ),
            (Self(1000130001), "SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT"),
            (
                Self(1000138000),
                "PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT",
            ),
            (
                Self(1000138001),
                "PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT",
            ),
            (
                Self(1000138002),
                "WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT",
            ),
            (
                Self(1000138003),
                "DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT",
            ),
            (Self(1000146000), "BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR"),
            (Self(1000146001), "IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR"),
            (
                Self(1000146002),
                "IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR",
            ),
            (Self(1000146003), "MEMORY_REQUIREMENTS_2_KHR"),
            (Self(1000146004), "SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR"),
            (Self(1000147000), "IMAGE_FORMAT_LIST_CREATE_INFO_KHR"),
            (Self(1000156000), "SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR"),
            (Self(1000156001), "SAMPLER_YCBCR_CONVERSION_INFO_KHR"),
            (Self(1000156002), "BIND_IMAGE_PLANE_MEMORY_INFO_KHR"),
            (Self(1000156003), "IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR"),
            (
                Self(1000156004),
                "PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR",
            ),
            (
                Self(1000156005),
                "SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR",
            ),
            (Self(1000157000), "BIND_BUFFER_MEMORY_INFO_KHR"),
            (Self(1000157001), "BIND_IMAGE_MEMORY_INFO_KHR"),
            (
                Self(1000161000),
                "DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT",
            ),
            (
                Self(1000161001),
                "PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT",
            ),
            (
                Self(1000161002),
                "PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT",
            ),
            (
                Self(1000161003),
                "DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT",
            ),
            (
                Self(1000161004),
                "DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT",
            ),
            (
                Self(1000168000),
                "PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR",
            ),
            (Self(1000168001), "DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR"),
            (
                Self(1000174000),
                "DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT",
            ),
            (
                Self(1000174000),
                "DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR",
            ),
            (
                Self(1000175000),
                "PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR",
            ),
            (
                Self(1000177000),
                "PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR",
            ),
            (
                Self(1000180000),
                "PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR",
            ),
            (Self(1000184000), "CALIBRATED_TIMESTAMP_INFO_EXT"),
            (
                Self(1000190001),
                "PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT",
            ),
            (
                Self(1000190001),
                "PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR",
            ),
            (
                Self(1000190002),
                "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT",
            ),
            (
                Self(1000190002),
                "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR",
            ),
            (
                Self(1000192000),
                "PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT",
            ),
            (Self(1000196000), "PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR"),
            (
                Self(1000197000),
                "PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR",
            ),
            (
                Self(1000199000),
                "PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR",
            ),
            (
                Self(1000199001),
                "SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR",
            ),
            (
                Self(1000201000),
                "PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV",
            ),
            (
                Self(1000203000),
                "PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV",
            ),
            (
                Self(1000207000),
                "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR",
            ),
            (
                Self(1000207001),
                "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR",
            ),
            (Self(1000207002), "SEMAPHORE_TYPE_CREATE_INFO_KHR"),
            (Self(1000207003), "TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR"),
            (Self(1000207004), "SEMAPHORE_WAIT_INFO_KHR"),
            (Self(1000207005), "SEMAPHORE_SIGNAL_INFO_KHR"),
            (Self(1000210000), "QUERY_POOL_CREATE_INFO_INTEL"),
            (
                Self(1000211000),
                "PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR",
            ),
            (
                Self(1000215000),
                "PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR",
            ),
            (
                Self(1000221000),
                "PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT",
            ),
            (
                Self(1000225000),
                "PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT",
            ),
            (
                Self(1000225001),
                "PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT",
            ),
            (
                Self(1000225001),
                "SHADER_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT",
            ),
            (
                Self(1000225002),
                "PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT",
            ),
            (
                Self(1000232000),
                "PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR",
            ),
            (Self(1000232001), "RENDERING_ATTACHMENT_LOCATION_INFO_KHR"),
            (
                Self(1000232002),
                "RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR",
            ),
            (
                Self(1000241000),
                "PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR",
            ),
            (Self(1000241001), "ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR"),
            (
                Self(1000241002),
                "ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR",
            ),
            (
                Self(1000244000),
                "PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT",
            ),
            (Self(1000244001), "BUFFER_DEVICE_ADDRESS_INFO_EXT"),
            (Self(1000244001), "BUFFER_DEVICE_ADDRESS_INFO_KHR"),
            (Self(1000245000), "PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT"),
            (Self(1000246000), "IMAGE_STENCIL_USAGE_CREATE_INFO_EXT"),
            (
                Self(1000253000),
                "PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR",
            ),
            (
                Self(1000257000),
                "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR",
            ),
            (
                Self(1000257002),
                "BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR",
            ),
            (
                Self(1000257003),
                "MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR",
            ),
            (
                Self(1000257004),
                "DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR",
            ),
            (
                Self(1000259000),
                "PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT",
            ),
            (
                Self(1000259000),
                "PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR",
            ),
            (
                Self(1000259001),
                "PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT",
            ),
            (
                Self(1000259001),
                "PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR",
            ),
            (
                Self(1000259002),
                "PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT",
            ),
            (
                Self(1000259002),
                "PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR",
            ),
            (
                Self(1000261000),
                "PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT",
            ),
            (
                Self(1000265000),
                "PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT",
            ),
            (
                Self(1000265000),
                "PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR",
            ),
            (Self(1000269001), "PIPELINE_INFO_EXT"),
            (
                Self(1000270000),
                "PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT",
            ),
            (
                Self(1000270001),
                "PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT",
            ),
            (Self(1000270002), "MEMORY_TO_IMAGE_COPY_EXT"),
            (Self(1000270003), "IMAGE_TO_MEMORY_COPY_EXT"),
            (Self(1000270004), "COPY_IMAGE_TO_MEMORY_INFO_EXT"),
            (Self(1000270005), "COPY_MEMORY_TO_IMAGE_INFO_EXT"),
            (Self(1000270006), "HOST_IMAGE_LAYOUT_TRANSITION_INFO_EXT"),
            (Self(1000270007), "COPY_IMAGE_TO_IMAGE_INFO_EXT"),
            (Self(1000270008), "SUBRESOURCE_HOST_MEMCPY_SIZE_EXT"),
            (
                Self(1000270009),
                "HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT",
            ),
            (Self(1000271000), "MEMORY_MAP_INFO_KHR"),
            (Self(1000271001), "MEMORY_UNMAP_INFO_KHR"),
            (Self(1000274000), "SURFACE_PRESENT_MODE_EXT"),
            (Self(1000274001), "SURFACE_PRESENT_SCALING_CAPABILITIES_EXT"),
            (Self(1000274002), "SURFACE_PRESENT_MODE_COMPATIBILITY_EXT"),
            (
                Self(1000275000),
                "PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT",
            ),
            (Self(1000275001), "SWAPCHAIN_PRESENT_FENCE_INFO_EXT"),
            (Self(1000275002), "SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT"),
            (Self(1000275003), "SWAPCHAIN_PRESENT_MODE_INFO_EXT"),
            (
                Self(1000275004),
                "SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT",
            ),
            (Self(1000275005), "RELEASE_SWAPCHAIN_IMAGES_INFO_EXT"),
            (
                Self(1000276000),
                "PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT",
            ),
            (
                Self(1000280000),
                "PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR",
            ),
            (
                Self(1000280001),
                "PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR",
            ),
            (
                Self(1000281001),
                "PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT",
            ),
            (
                Self(1000286000),
                "PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT",
            ),
            (
                Self(1000286001),
                "PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT",
            ),
            (
                Self(1000295000),
                "PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT",
            ),
            (Self(1000295001), "DEVICE_PRIVATE_DATA_CREATE_INFO_EXT"),
            (Self(1000295002), "PRIVATE_DATA_SLOT_CREATE_INFO_EXT"),
            (
                Self(1000297000),
                "PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT",
            ),
            (Self(1000314000), "MEMORY_BARRIER_2_KHR"),
            (Self(1000314001), "BUFFER_MEMORY_BARRIER_2_KHR"),
            (Self(1000314002), "IMAGE_MEMORY_BARRIER_2_KHR"),
            (Self(1000314003), "DEPENDENCY_INFO_KHR"),
            (Self(1000314004), "SUBMIT_INFO_2_KHR"),
            (Self(1000314005), "SEMAPHORE_SUBMIT_INFO_KHR"),
            (Self(1000314006), "COMMAND_BUFFER_SUBMIT_INFO_KHR"),
            (
                Self(1000314007),
                "PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR",
            ),
            (
                Self(1000325000),
                "PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR",
            ),
            (
                Self(1000335000),
                "PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT",
            ),
            (Self(1000337000), "COPY_BUFFER_INFO_2_KHR"),
            (Self(1000337001), "COPY_IMAGE_INFO_2_KHR"),
            (Self(1000337002), "COPY_BUFFER_TO_IMAGE_INFO_2_KHR"),
            (Self(1000337003), "COPY_IMAGE_TO_BUFFER_INFO_2_KHR"),
            (Self(1000337004), "BLIT_IMAGE_INFO_2_KHR"),
            (Self(1000337005), "RESOLVE_IMAGE_INFO_2_KHR"),
            (Self(1000337006), "BUFFER_COPY_2_KHR"),
            (Self(1000337007), "IMAGE_COPY_2_KHR"),
            (Self(1000337008), "IMAGE_BLIT_2_KHR"),
            (Self(1000337009), "BUFFER_IMAGE_COPY_2_KHR"),
            (Self(1000337010), "IMAGE_RESOLVE_2_KHR"),
            (Self(1000338002), "SUBRESOURCE_LAYOUT_2_EXT"),
            (Self(1000338002), "SUBRESOURCE_LAYOUT_2_KHR"),
            (Self(1000338003), "IMAGE_SUBRESOURCE_2_EXT"),
            (Self(1000338003), "IMAGE_SUBRESOURCE_2_KHR"),
            (
                Self(1000342000),
                "PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM",
            ),
            (
                Self(1000351000),
                "PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE",
            ),
            (
                Self(1000351002),
                "MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE",
            ),
            (Self(1000360000), "FORMAT_PROPERTIES_3_KHR"),
            (
                Self(1000361000),
                "PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_EXT",
            ),
            (
                Self(1000388000),
                "PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT",
            ),
            (
                Self(1000388000),
                "PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR",
            ),
            (
                Self(1000388001),
                "QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT",
            ),
            (
                Self(1000388001),
                "QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR",
            ),
            (
                Self(1000413000),
                "PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR",
            ),
            (
                Self(1000413001),
                "PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR",
            ),
            (Self(1000413002), "DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR"),
            (Self(1000413003), "DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR"),
            (
                Self(1000416000),
                "PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR",
            ),
            (
                Self(1000421000),
                "PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT",
            ),
            (
                Self(1000425000),
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM",
            ),
            (
                Self(1000425001),
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM",
            ),
            (
                Self(1000425002),
                "SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM",
            ),
            (
                Self(1000426001),
                "PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV",
            ),
            (
                Self(1000427000),
                "PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV",
            ),
            (
                Self(1000427001),
                "PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV",
            ),
            (
                Self(1000466000),
                "PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT",
            ),
            (
                Self(1000470000),
                "PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR",
            ),
            (
                Self(1000470001),
                "PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR",
            ),
            (Self(1000470003), "RENDERING_AREA_INFO_KHR"),
            (Self(1000470004), "DEVICE_IMAGE_SUBRESOURCE_INFO_KHR"),
            (Self(1000470005), "PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR"),
            (Self(1000470006), "BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR"),
            (
                Self(1000498000),
                "PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT",
            ),
            (
                Self(1000525000),
                "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR",
            ),
            (
                Self(1000528000),
                "PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR",
            ),
            (
                Self(1000544000),
                "PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR",
            ),
            (
                Self(1000545000),
                "PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR",
            ),
            (
                Self(1000545001),
                "PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR",
            ),
            (Self(1000545002), "BIND_MEMORY_STATUS_KHR"),
            (Self(1000545003), "BIND_DESCRIPTOR_SETS_INFO_KHR"),
            (Self(1000545004), "PUSH_CONSTANTS_INFO_KHR"),
            (Self(1000545005), "PUSH_DESCRIPTOR_SET_INFO_KHR"),
            (
                Self(1000545006),
                "PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR",
            ),
            (Self(1000619003), "RENDERING_END_INFO_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for StructureType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "StructureType", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_EXT: Self = Self(1000451000);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "INLINE"),
            (Self(1), "SECONDARY_COMMAND_BUFFERS"),
            (Self(1000451000), "INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(1000451000), "INLINE_AND_SECONDARY_COMMAND_BUFFERS_EXT")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for SubpassContents {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "SubpassContents", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SubpassMergeStatusEXT(pub i32);

impl SubpassMergeStatusEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MERGED: Self = Self(0);
    pub const DISALLOWED: Self = Self(1);
    pub const NOT_MERGED_SIDE_EFFECTS: Self = Self(2);
    pub const NOT_MERGED_SAMPLES_MISMATCH: Self = Self(3);
    pub const NOT_MERGED_VIEWS_MISMATCH: Self = Self(4);
    pub const NOT_MERGED_ALIASING: Self = Self(5);
    pub const NOT_MERGED_DEPENDENCIES: Self = Self(6);
    pub const NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT: Self = Self(7);
    pub const NOT_MERGED_TOO_MANY_ATTACHMENTS: Self = Self(8);
    pub const NOT_MERGED_INSUFFICIENT_STORAGE: Self = Self(9);
    pub const NOT_MERGED_DEPTH_STENCIL_COUNT: Self = Self(10);
    pub const NOT_MERGED_RESOLVE_ATTACHMENT_REUSE: Self = Self(11);
    pub const NOT_MERGED_SINGLE_SUBPASS: Self = Self(12);
    pub const NOT_MERGED_UNSPECIFIED: Self = Self(13);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "MERGED"),
            (Self(1), "DISALLOWED"),
            (Self(2), "NOT_MERGED_SIDE_EFFECTS"),
            (Self(3), "NOT_MERGED_SAMPLES_MISMATCH"),
            (Self(4), "NOT_MERGED_VIEWS_MISMATCH"),
            (Self(5), "NOT_MERGED_ALIASING"),
            (Self(6), "NOT_MERGED_DEPENDENCIES"),
            (Self(7), "NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT"),
            (Self(8), "NOT_MERGED_TOO_MANY_ATTACHMENTS"),
            (Self(9), "NOT_MERGED_INSUFFICIENT_STORAGE"),
            (Self(10), "NOT_MERGED_DEPTH_STENCIL_COUNT"),
            (Self(11), "NOT_MERGED_RESOLVE_ATTACHMENT_REUSE"),
            (Self(12), "NOT_MERGED_SINGLE_SUBPASS"),
            (Self(13), "NOT_MERGED_UNSPECIFIED"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for SubpassMergeStatusEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "SubpassMergeStatusEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "COMMAND"),
            (Self(1), "OBJECT"),
            (Self(2), "CACHE"),
            (Self(3), "DEVICE"),
            (Self(4), "INSTANCE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for SystemAllocationScope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "SystemAllocationScope", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TensorTilingARM(pub i32);

impl TensorTilingARM {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPTIMAL: Self = Self(0);
    pub const LINEAR: Self = Self(1);
    pub const BRICK_16_WIDE: Self = Self(1000565000);
    pub const BRICK_8_WIDE: Self = Self(1000565001);
    pub const BRICK_4_WIDE: Self = Self(1000565002);
    pub const BLOCK_U_INTERLEAVED: Self = Self(1000565003);
    pub const BLOCK_U_INTERLEAVED_64K: Self = Self(1000565004);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "OPTIMAL"),
            (Self(1), "LINEAR"),
            (Self(1000565000), "BRICK_16_WIDE"),
            (Self(1000565001), "BRICK_8_WIDE"),
            (Self(1000565002), "BRICK_4_WIDE"),
            (Self(1000565003), "BLOCK_U_INTERLEAVED"),
            (Self(1000565004), "BLOCK_U_INTERLEAVED_64K"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for TensorTilingARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "TensorTilingARM", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const UPPER_LEFT_KHR: Self = Self(0);
    pub const LOWER_LEFT_KHR: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "UPPER_LEFT"), (Self(1), "LOWER_LEFT")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[(Self(0), "UPPER_LEFT_KHR"), (Self(1), "LOWER_LEFT_KHR")]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for TessellationDomainOrigin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "TessellationDomainOrigin", self.name(), self.0)
    }
}

pub type TessellationDomainOriginKHR = TessellationDomainOrigin;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ThrottleHintTypeSEC(pub i32);

impl ThrottleHintTypeSEC {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const LOW: Self = Self(1);
    pub const HIGH: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "DEFAULT"), (Self(1), "LOW"), (Self(2), "HIGH")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ThrottleHintTypeSEC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ThrottleHintTypeSEC", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TimeDomainKHR(pub i32);

impl TimeDomainKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE: Self = Self(0);
    pub const CLOCK_MONOTONIC: Self = Self(1);
    pub const CLOCK_MONOTONIC_RAW: Self = Self(2);
    pub const QUERY_PERFORMANCE_COUNTER: Self = Self(3);
    pub const PRESENT_STAGE_LOCAL_EXT: Self = Self(1000208000);
    pub const SWAPCHAIN_LOCAL_EXT: Self = Self(1000208001);
    pub const DEVICE_EXT: Self = Self(0);
    pub const CLOCK_MONOTONIC_EXT: Self = Self(1);
    pub const CLOCK_MONOTONIC_RAW_EXT: Self = Self(2);
    pub const QUERY_PERFORMANCE_COUNTER_EXT: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEVICE"),
            (Self(1), "CLOCK_MONOTONIC"),
            (Self(2), "CLOCK_MONOTONIC_RAW"),
            (Self(3), "QUERY_PERFORMANCE_COUNTER"),
            (Self(1000208000), "PRESENT_STAGE_LOCAL_EXT"),
            (Self(1000208001), "SWAPCHAIN_LOCAL_EXT"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEVICE_EXT"),
            (Self(1), "CLOCK_MONOTONIC_EXT"),
            (Self(2), "CLOCK_MONOTONIC_RAW_EXT"),
            (Self(3), "QUERY_PERFORMANCE_COUNTER_EXT"),
        ]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for TimeDomainKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "TimeDomainKHR", self.name(), self.0)
    }
}

pub type TimeDomainEXT = TimeDomainKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidationCacheHeaderVersionEXT(pub i32);

impl ValidationCacheHeaderVersionEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ONE: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(1), "ONE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ValidationCacheHeaderVersionEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ValidationCacheHeaderVersionEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidationCheckEXT(pub i32);

impl ValidationCheckEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ALL: Self = Self(0);
    pub const SHADERS: Self = Self(1);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "ALL"), (Self(1), "SHADERS")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ValidationCheckEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ValidationCheckEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidationFeatureDisableEXT(pub i32);

impl ValidationFeatureDisableEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ALL: Self = Self(0);
    pub const SHADERS: Self = Self(1);
    pub const THREAD_SAFETY: Self = Self(2);
    pub const API_PARAMETERS: Self = Self(3);
    pub const OBJECT_LIFETIMES: Self = Self(4);
    pub const CORE_CHECKS: Self = Self(5);
    pub const UNIQUE_HANDLES: Self = Self(6);
    pub const SHADER_VALIDATION_CACHE: Self = Self(7);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "ALL"),
            (Self(1), "SHADERS"),
            (Self(2), "THREAD_SAFETY"),
            (Self(3), "API_PARAMETERS"),
            (Self(4), "OBJECT_LIFETIMES"),
            (Self(5), "CORE_CHECKS"),
            (Self(6), "UNIQUE_HANDLES"),
            (Self(7), "SHADER_VALIDATION_CACHE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ValidationFeatureDisableEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ValidationFeatureDisableEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidationFeatureEnableEXT(pub i32);

impl ValidationFeatureEnableEXT {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const GPU_ASSISTED: Self = Self(0);
    pub const GPU_ASSISTED_RESERVE_BINDING_SLOT: Self = Self(1);
    pub const BEST_PRACTICES: Self = Self(2);
    pub const DEBUG_PRINTF: Self = Self(3);
    pub const SYNCHRONIZATION_VALIDATION: Self = Self(4);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "GPU_ASSISTED"),
            (Self(1), "GPU_ASSISTED_RESERVE_BINDING_SLOT"),
            (Self(2), "BEST_PRACTICES"),
            (Self(3), "DEBUG_PRINTF"),
            (Self(4), "SYNCHRONIZATION_VALIDATION"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ValidationFeatureEnableEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ValidationFeatureEnableEXT", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const APE: Self = Self(65544);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(65536), "KHRONOS"),
            (Self(65537), "VIV"),
            (Self(65538), "VSI"),
            (Self(65539), "KAZAN"),
            (Self(65540), "CODEPLAY"),
            (Self(65541), "MESA"),
            (Self(65542), "POCL"),
            (Self(65543), "MOBILEYE"),
            (Self(65544), "APE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for VendorId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "VendorId", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[(Self(0), "VERTEX"), (Self(1), "INSTANCE")]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for VertexInputRate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "VertexInputRate", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VideoEncodeAV1PredictionModeKHR(pub i32);

impl VideoEncodeAV1PredictionModeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INTRA_ONLY: Self = Self(0);
    pub const SINGLE_REFERENCE: Self = Self(1);
    pub const UNIDIRECTIONAL_COMPOUND: Self = Self(2);
    pub const BIDIRECTIONAL_COMPOUND: Self = Self(3);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "INTRA_ONLY"),
            (Self(1), "SINGLE_REFERENCE"),
            (Self(2), "UNIDIRECTIONAL_COMPOUND"),
            (Self(3), "BIDIRECTIONAL_COMPOUND"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for VideoEncodeAV1PredictionModeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "VideoEncodeAV1PredictionModeKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VideoEncodeAV1RateControlGroupKHR(pub i32);

impl VideoEncodeAV1RateControlGroupKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INTRA: Self = Self(0);
    pub const PREDICTIVE: Self = Self(1);
    pub const BIPREDICTIVE: Self = Self(2);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "INTRA"),
            (Self(1), "PREDICTIVE"),
            (Self(2), "BIPREDICTIVE"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for VideoEncodeAV1RateControlGroupKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "VideoEncodeAV1RateControlGroupKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VideoEncodeTuningModeKHR(pub i32);

impl VideoEncodeTuningModeKHR {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const HIGH_QUALITY: Self = Self(1);
    pub const LOW_LATENCY: Self = Self(2);
    pub const ULTRA_LOW_LATENCY: Self = Self(3);
    pub const LOSSLESS: Self = Self(4);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "DEFAULT"),
            (Self(1), "HIGH_QUALITY"),
            (Self(2), "LOW_LATENCY"),
            (Self(3), "ULTRA_LOW_LATENCY"),
            (Self(4), "LOSSLESS"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for VideoEncodeTuningModeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "VideoEncodeTuningModeKHR", self.name(), self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ViewportCoordinateSwizzleNV(pub i32);

impl ViewportCoordinateSwizzleNV {
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const POSITIVE_X: Self = Self(0);
    pub const NEGATIVE_X: Self = Self(1);
    pub const POSITIVE_Y: Self = Self(2);
    pub const NEGATIVE_Y: Self = Self(3);
    pub const POSITIVE_Z: Self = Self(4);
    pub const NEGATIVE_Z: Self = Self(5);
    pub const POSITIVE_W: Self = Self(6);
    pub const NEGATIVE_W: Self = Self(7);
    pub const fn variants() -> &'static [(Self, &'static str)] {
        &[
            (Self(0), "POSITIVE_X"),
            (Self(1), "NEGATIVE_X"),
            (Self(2), "POSITIVE_Y"),
            (Self(3), "NEGATIVE_Y"),
            (Self(4), "POSITIVE_Z"),
            (Self(5), "NEGATIVE_Z"),
            (Self(6), "POSITIVE_W"),
            (Self(7), "NEGATIVE_W"),
        ]
    }
    const fn aliases() -> &'static [(Self, &'static str)] {
        &[]
    }
    pub fn name(self) -> Option<&'static str> {
        name_of(Self::variants(), self)
    }
    pub fn from_name(name: &str) -> Option<Self> {
        value_of(Self::variants(), Self::aliases(), name)
    }
    pub fn is_known(self) -> bool {
        self.name().is_some()
    }
}

impl core::fmt::Debug for ViewportCoordinateSwizzleNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_enum(f, "ViewportCoordinateSwizzleNV", self.name(), self.0)
    }
}
