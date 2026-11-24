use super::{definition, unmarked_arrays};
use crate::emit::{constants, squash};
use crate::model::{CType, Model};
use crate::naming::{const_name, field_name};
use crate::test_support::catch_quietly;
use std::collections::BTreeSet;
use vulkan_registry::{Field, Registry, Struct};

#[derive(Debug)]
enum Length {
    Fixed(&'static [(&'static str, usize)]),
    InMember(&'static str),
    InAnotherStructure(&'static str),
    NotInTheStructure,
    One,
}

#[derive(Debug)]
struct StdVideoArray {
    structure: &'static str,
    member: &'static str,
    length: Length,
    source: &'static str,
}

const fn fixed(
    structure: &'static str,
    member: &'static str,
    dimensions: &'static [(&'static str, usize)],
    source: &'static str,
) -> StdVideoArray {
    StdVideoArray {
        structure,
        member,
        length: Length::Fixed(dimensions),
        source,
    }
}

const fn in_member(
    structure: &'static str,
    member: &'static str,
    count: &'static str,
    source: &'static str,
) -> StdVideoArray {
    StdVideoArray {
        structure,
        member,
        length: Length::InMember(count),
        source,
    }
}

const fn in_another_structure(
    structure: &'static str,
    member: &'static str,
    len: &'static str,
    source: &'static str,
) -> StdVideoArray {
    StdVideoArray {
        structure,
        member,
        length: Length::InAnotherStructure(len),
        source,
    }
}

const fn not_in_the_structure(
    structure: &'static str,
    member: &'static str,
    source: &'static str,
) -> StdVideoArray {
    StdVideoArray {
        structure,
        member,
        length: Length::NotInTheStructure,
        source,
    }
}

const fn one(structure: &'static str, member: &'static str, source: &'static str) -> StdVideoArray {
    StdVideoArray {
        structure,
        member,
        length: Length::One,
        source,
    }
}

struct StdVideoConstant {
    name: &'static str,
    value: &'static str,
    source: &'static str,
}

const fn constant(
    name: &'static str,
    value: &'static str,
    source: &'static str,
) -> StdVideoConstant {
    StdVideoConstant {
        name,
        value,
        source,
    }
}

const ARRAYS: &[StdVideoArray] = &[
    fixed(
        "StdVideoAV1CDEF",
        "cdef_y_pri_strength",
        &[("STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS", 8)],
        "AV1 5.9.19 reads cdef_y_pri_strength[i] for i < (1 << cdef_bits), and cdef_bits is f(2)",
    ),
    fixed(
        "StdVideoAV1CDEF",
        "cdef_y_sec_strength",
        &[("STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS", 8)],
        "AV1 5.9.19 reads cdef_y_sec_strength[i] for i < (1 << cdef_bits), and cdef_bits is f(2)",
    ),
    fixed(
        "StdVideoAV1CDEF",
        "cdef_uv_pri_strength",
        &[("STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS", 8)],
        "AV1 5.9.19 reads cdef_uv_pri_strength[i] for i < (1 << cdef_bits), and cdef_bits is f(2)",
    ),
    fixed(
        "StdVideoAV1CDEF",
        "cdef_uv_sec_strength",
        &[("STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS", 8)],
        "AV1 5.9.19 reads cdef_uv_sec_strength[i] for i < (1 << cdef_bits), and cdef_bits is f(2)",
    ),
    fixed(
        "StdVideoAV1FilmGrain",
        "point_y_value",
        &[("STD_VIDEO_AV1_MAX_NUM_Y_POINTS", 14)],
        "AV1 5.9.30 reads point_y_value[i] for i < num_y_points, at most 14 by 6.8.20",
    ),
    fixed(
        "StdVideoAV1FilmGrain",
        "point_y_scaling",
        &[("STD_VIDEO_AV1_MAX_NUM_Y_POINTS", 14)],
        "AV1 5.9.30 reads point_y_scaling[i] for i < num_y_points, at most 14 by 6.8.20",
    ),
    fixed(
        "StdVideoAV1FilmGrain",
        "point_cb_value",
        &[("STD_VIDEO_AV1_MAX_NUM_CB_POINTS", 10)],
        "AV1 5.9.30 reads point_cb_value[i] for i < num_cb_points, at most 10 by 6.8.20",
    ),
    fixed(
        "StdVideoAV1FilmGrain",
        "point_cb_scaling",
        &[("STD_VIDEO_AV1_MAX_NUM_CB_POINTS", 10)],
        "AV1 5.9.30 reads point_cb_scaling[i] for i < num_cb_points, at most 10 by 6.8.20",
    ),
    fixed(
        "StdVideoAV1FilmGrain",
        "point_cr_value",
        &[("STD_VIDEO_AV1_MAX_NUM_CR_POINTS", 10)],
        "AV1 5.9.30 reads point_cr_value[i] for i < num_cr_points, at most 10 by 6.8.20",
    ),
    fixed(
        "StdVideoAV1FilmGrain",
        "point_cr_scaling",
        &[("STD_VIDEO_AV1_MAX_NUM_CR_POINTS", 10)],
        "AV1 5.9.30 reads point_cr_scaling[i] for i < num_cr_points, at most 10 by 6.8.20",
    ),
    fixed(
        "StdVideoAV1FilmGrain",
        "ar_coeffs_y_plus_128",
        &[("STD_VIDEO_AV1_MAX_NUM_POS_LUMA", 24)],
        "AV1 5.9.30 reads ar_coeffs_y_plus_128[i] for i < numPosLuma = 2 * ar_coeff_lag * (ar_coeff_lag + 1), and ar_coeff_lag is f(2)",
    ),
    fixed(
        "StdVideoAV1FilmGrain",
        "ar_coeffs_cb_plus_128",
        &[("STD_VIDEO_AV1_MAX_NUM_POS_CHROMA", 25)],
        "AV1 5.9.30 reads ar_coeffs_cb_plus_128[i] for i < numPosChroma, which is numPosLuma + 1 when num_y_points is not 0",
    ),
    fixed(
        "StdVideoAV1FilmGrain",
        "ar_coeffs_cr_plus_128",
        &[("STD_VIDEO_AV1_MAX_NUM_POS_CHROMA", 25)],
        "AV1 5.9.30 reads ar_coeffs_cr_plus_128[i] for i < numPosChroma, which is numPosLuma + 1 when num_y_points is not 0",
    ),
    fixed(
        "StdVideoAV1GlobalMotion",
        "GmType",
        &[("STD_VIDEO_AV1_NUM_REF_FRAMES", 8)],
        "AV1 5.9.24 sets GmType[ref] for ref from LAST_FRAME (1) to ALTREF_FRAME (7), so the array has an element for each of the 8 values of a reference frame",
    ),
    fixed(
        "StdVideoAV1GlobalMotion",
        "gm_params",
        &[
            ("STD_VIDEO_AV1_NUM_REF_FRAMES", 8),
            ("STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS", 6),
        ],
        "AV1 5.9.24 sets gm_params[ref][i] for ref from LAST_FRAME (1) to ALTREF_FRAME (7) and i < 6",
    ),
    fixed(
        "StdVideoAV1LoopFilter",
        "loop_filter_level",
        &[("STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS", 4)],
        "AV1 5.9.11 reads loop_filter_level[0] to loop_filter_level[3]",
    ),
    fixed(
        "StdVideoAV1LoopFilter",
        "loop_filter_ref_deltas",
        &[("STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME", 8)],
        "AV1 5.9.11 reads loop_filter_ref_deltas[i] for i < TOTAL_REFS_PER_FRAME (8)",
    ),
    fixed(
        "StdVideoAV1LoopFilter",
        "loop_filter_mode_deltas",
        &[("STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS", 2)],
        "AV1 5.9.11 reads loop_filter_mode_deltas[i] for i < 2",
    ),
    fixed(
        "StdVideoAV1LoopRestoration",
        "FrameRestorationType",
        &[("STD_VIDEO_AV1_MAX_NUM_PLANES", 3)],
        "AV1 5.9.20 sets FrameRestorationType[i] for i < NumPlanes, which 5.5.2 sets to 1 or 3",
    ),
    fixed(
        "StdVideoAV1LoopRestoration",
        "LoopRestorationSize",
        &[("STD_VIDEO_AV1_MAX_NUM_PLANES", 3)],
        "AV1 5.9.20 sets LoopRestorationSize[i] for i < NumPlanes, which 5.5.2 sets to 1 or 3",
    ),
    fixed(
        "StdVideoAV1Segmentation",
        "FeatureEnabled",
        &[("STD_VIDEO_AV1_MAX_SEGMENTS", 8)],
        "AV1 5.9.14 sets FeatureEnabled[i][j] for i < MAX_SEGMENTS (8), and the header keeps the SEG_LVL_MAX (8) flags of a segment as the bits of one element",
    ),
    fixed(
        "StdVideoAV1Segmentation",
        "FeatureData",
        &[
            ("STD_VIDEO_AV1_MAX_SEGMENTS", 8),
            ("STD_VIDEO_AV1_SEG_LVL_MAX", 8),
        ],
        "AV1 5.9.14 sets FeatureData[i][j] for i < MAX_SEGMENTS (8) and j < SEG_LVL_MAX (8)",
    ),
    fixed(
        "StdVideoAV1SequenceHeader",
        "reserved1",
        &[("5", 5)],
        "padding: the Vulkan specification describes the member as used only for padding purposes and otherwise ignored, and video.xml says it is reserved for future use and must be initialized with 0",
    ),
    one(
        "StdVideoAV1SequenceHeader",
        "pColorConfig",
        "AV1 5.5.1: a sequence header has one color_config()",
    ),
    one(
        "StdVideoAV1SequenceHeader",
        "pTimingInfo",
        "AV1 5.5.1: a sequence header has one timing_info()",
    ),
    fixed(
        "StdVideoAV1TileInfo",
        "reserved1",
        &[("7", 7)],
        "padding: the Vulkan specification describes the member as used only for padding purposes and otherwise ignored, and video.xml says it is reserved for future use and must be initialized with 0",
    ),
    in_member(
        "StdVideoAV1TileInfo",
        "pMiColStarts",
        "TileCols",
        "the Vulkan specification (AV1 decode picture information): an array of TileCols elements that corresponds to MiColStarts of AV1 6.8.14; AV1 5.9.15 also sets MiColStarts[TileCols] to MiCols, which the array leaves out",
    ),
    in_member(
        "StdVideoAV1TileInfo",
        "pMiRowStarts",
        "TileRows",
        "the Vulkan specification (AV1 decode picture information): an array of TileRows elements that corresponds to MiRowStarts of AV1 6.8.14; AV1 5.9.15 also sets MiRowStarts[TileRows] to MiRows, which the array leaves out",
    ),
    in_member(
        "StdVideoAV1TileInfo",
        "pWidthInSbsMinus1",
        "TileCols",
        "AV1 5.9.15 reads width_in_sbs_minus_1[i] for i < TileCols, and the Vulkan specification (AV1 decode and encode picture information) describes an array of TileCols elements",
    ),
    in_member(
        "StdVideoAV1TileInfo",
        "pHeightInSbsMinus1",
        "TileRows",
        "AV1 5.9.15 reads height_in_sbs_minus_1[i] for i < TileRows, and the Vulkan specification (AV1 decode and encode picture information) describes an array of TileRows elements",
    ),
    fixed(
        "StdVideoDecodeAV1PictureInfo",
        "SkipModeFrame",
        &[("STD_VIDEO_AV1_SKIP_MODE_FRAMES", 2)],
        "AV1 5.9.22 sets SkipModeFrame[0] and SkipModeFrame[1]",
    ),
    fixed(
        "StdVideoDecodeAV1PictureInfo",
        "reserved2",
        &[("3", 3)],
        "padding: the Vulkan specification describes the member as used only for padding purposes and otherwise ignored, and video.xml says it is reserved for future use and must be initialized with 0",
    ),
    fixed(
        "StdVideoDecodeAV1PictureInfo",
        "OrderHints",
        &[("STD_VIDEO_AV1_NUM_REF_FRAMES", 8)],
        "AV1 5.9.2 sets OrderHints[refFrame] for refFrame = LAST_FRAME + i with i < REFS_PER_FRAME, so the array has an element for each of the 8 values of a reference frame",
    ),
    fixed(
        "StdVideoDecodeAV1PictureInfo",
        "expectedFrameId",
        &[("STD_VIDEO_AV1_NUM_REF_FRAMES", 8)],
        "AV1 5.9.2 sets expectedFrameId[i] for i < REFS_PER_FRAME (7); the header gives NUM_REF_FRAMES (8) elements",
    ),
    one(
        "StdVideoDecodeAV1PictureInfo",
        "pTileInfo",
        "AV1 5.9.2: an uncompressed header has one tile_info()",
    ),
    one(
        "StdVideoDecodeAV1PictureInfo",
        "pQuantization",
        "AV1 5.9.2: an uncompressed header has one quantization_params()",
    ),
    one(
        "StdVideoDecodeAV1PictureInfo",
        "pSegmentation",
        "AV1 5.9.2: an uncompressed header has one segmentation_params()",
    ),
    one(
        "StdVideoDecodeAV1PictureInfo",
        "pLoopFilter",
        "AV1 5.9.2: an uncompressed header has one loop_filter_params()",
    ),
    one(
        "StdVideoDecodeAV1PictureInfo",
        "pCDEF",
        "AV1 5.9.2: an uncompressed header has one cdef_params()",
    ),
    one(
        "StdVideoDecodeAV1PictureInfo",
        "pLoopRestoration",
        "AV1 5.9.2: an uncompressed header has one lr_params()",
    ),
    one(
        "StdVideoDecodeAV1PictureInfo",
        "pGlobalMotion",
        "AV1 5.9.2: an uncompressed header has one global_motion_params()",
    ),
    one(
        "StdVideoDecodeAV1PictureInfo",
        "pFilmGrain",
        "AV1 5.9.2: an uncompressed header has one film_grain_params()",
    ),
    fixed(
        "StdVideoDecodeAV1ReferenceInfo",
        "SavedOrderHints",
        &[("STD_VIDEO_AV1_NUM_REF_FRAMES", 8)],
        "AV1 7.20 saves SavedOrderHints[i][j + LAST_FRAME] for j < REFS_PER_FRAME, so the array has an element for each of the 8 values of a reference frame",
    ),
    fixed(
        "StdVideoDecodeH264PictureInfo",
        "PicOrderCnt",
        &[("STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE", 2)],
        "H.264 8.2.1: a frame has two picture order counts, TopFieldOrderCnt and BottomFieldOrderCnt",
    ),
    fixed(
        "StdVideoDecodeH264ReferenceInfo",
        "PicOrderCnt",
        &[("STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE", 2)],
        "H.264 8.2.1: a frame has two picture order counts, TopFieldOrderCnt and BottomFieldOrderCnt",
    ),
    fixed(
        "StdVideoDecodeH265PictureInfo",
        "RefPicSetStCurrBefore",
        &[("STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE", 8)],
        "H.265 8.3.2: RefPicSetStCurrBefore is one of the lists whose entries NumPicTotalCurr (7.4.7.2) counts, and A.4.1 e) limits NumPicTotalCurr to 8",
    ),
    fixed(
        "StdVideoDecodeH265PictureInfo",
        "RefPicSetStCurrAfter",
        &[("STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE", 8)],
        "H.265 8.3.2: RefPicSetStCurrAfter is one of the lists whose entries NumPicTotalCurr (7.4.7.2) counts, and A.4.1 e) limits NumPicTotalCurr to 8",
    ),
    fixed(
        "StdVideoDecodeH265PictureInfo",
        "RefPicSetLtCurr",
        &[("STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE", 8)],
        "H.265 8.3.2: RefPicSetLtCurr is one of the lists whose entries NumPicTotalCurr (7.4.7.2) counts, and A.4.1 e) limits NumPicTotalCurr to 8",
    ),
    fixed(
        "StdVideoDecodeVP9PictureInfo",
        "reserved1",
        &[("3", 3)],
        "padding: the Vulkan specification describes the member as used only for padding purposes and otherwise ignored, and video.xml says it is reserved for future use and must be initialized with 0",
    ),
    one(
        "StdVideoDecodeVP9PictureInfo",
        "pColorConfig",
        "VP9 6.2: an uncompressed header has one color_config()",
    ),
    one(
        "StdVideoDecodeVP9PictureInfo",
        "pLoopFilter",
        "VP9 6.2: an uncompressed header has one loop_filter_params()",
    ),
    one(
        "StdVideoDecodeVP9PictureInfo",
        "pSegmentation",
        "VP9 6.2: an uncompressed header has one segmentation_params()",
    ),
    fixed(
        "StdVideoEncodeAV1PictureInfo",
        "ref_order_hint",
        &[("STD_VIDEO_AV1_NUM_REF_FRAMES", 8)],
        "AV1 5.9.2 reads ref_order_hint[i] for i < NUM_REF_FRAMES (8)",
    ),
    fixed(
        "StdVideoEncodeAV1PictureInfo",
        "ref_frame_idx",
        &[("STD_VIDEO_AV1_REFS_PER_FRAME", 7)],
        "AV1 5.9.2 reads ref_frame_idx[i] for i < REFS_PER_FRAME (7)",
    ),
    fixed(
        "StdVideoEncodeAV1PictureInfo",
        "reserved1",
        &[("3", 3)],
        "padding: the Vulkan specification describes the member as used only for padding purposes and otherwise ignored, and video.xml says it is reserved for future use and must be initialized with 0",
    ),
    fixed(
        "StdVideoEncodeAV1PictureInfo",
        "delta_frame_id_minus_1",
        &[("STD_VIDEO_AV1_REFS_PER_FRAME", 7)],
        "AV1 5.9.2 reads delta_frame_id_minus_1 for each i < REFS_PER_FRAME (7)",
    ),
    one(
        "StdVideoEncodeAV1PictureInfo",
        "pTileInfo",
        "AV1 5.9.2: an uncompressed header has one tile_info()",
    ),
    one(
        "StdVideoEncodeAV1PictureInfo",
        "pQuantization",
        "AV1 5.9.2: an uncompressed header has one quantization_params()",
    ),
    one(
        "StdVideoEncodeAV1PictureInfo",
        "pSegmentation",
        "AV1 5.9.2: an uncompressed header has one segmentation_params()",
    ),
    one(
        "StdVideoEncodeAV1PictureInfo",
        "pLoopFilter",
        "AV1 5.9.2: an uncompressed header has one loop_filter_params()",
    ),
    one(
        "StdVideoEncodeAV1PictureInfo",
        "pCDEF",
        "AV1 5.9.2: an uncompressed header has one cdef_params()",
    ),
    one(
        "StdVideoEncodeAV1PictureInfo",
        "pLoopRestoration",
        "AV1 5.9.2: an uncompressed header has one lr_params()",
    ),
    one(
        "StdVideoEncodeAV1PictureInfo",
        "pGlobalMotion",
        "AV1 5.9.2: an uncompressed header has one global_motion_params()",
    ),
    one(
        "StdVideoEncodeAV1PictureInfo",
        "pExtensionHeader",
        "AV1 5.3.3: an OBU has one obu_extension_header()",
    ),
    not_in_the_structure(
        "StdVideoEncodeAV1PictureInfo",
        "pBufferRemovalTimes",
        "AV1 5.9.2 reads buffer_removal_time[opNum] for opNum <= operating_points_cnt_minus_1, and the Vulkan specification (AV1 encode picture information) describes an array of VkVideoEncodeAV1SessionParametersCreateInfoKHR::stdOperatingPointCount elements",
    ),
    fixed(
        "StdVideoEncodeAV1ReferenceInfo",
        "reserved1",
        &[("3", 3)],
        "padding: the Vulkan specification describes the member as used only for padding purposes and otherwise ignored, and video.xml says it is reserved for future use and must be initialized with 0",
    ),
    one(
        "StdVideoEncodeAV1ReferenceInfo",
        "pExtensionHeader",
        "AV1 5.3.3: an OBU has one obu_extension_header()",
    ),
    fixed(
        "StdVideoEncodeH264PictureInfo",
        "reserved1",
        &[("3", 3)],
        "padding: the Vulkan specification describes the member as used only for padding purposes and otherwise ignored, and video.xml says it is reserved for future use and must be initialized with 0",
    ),
    one(
        "StdVideoEncodeH264PictureInfo",
        "pRefLists",
        "the Vulkan specification (H.264 encode picture information): one StdVideoEncodeH264ReferenceListsInfo structure for the reference lists of the picture",
    ),
    fixed(
        "StdVideoEncodeH264ReferenceListsInfo",
        "RefPicList0",
        &[("STD_VIDEO_H264_MAX_NUM_LIST_REF", 32)],
        "H.264 8.2.4: RefPicList0 has num_ref_idx_l0_active_minus1 + 1 entries, at most 32 by 7.4.3",
    ),
    fixed(
        "StdVideoEncodeH264ReferenceListsInfo",
        "RefPicList1",
        &[("STD_VIDEO_H264_MAX_NUM_LIST_REF", 32)],
        "H.264 8.2.4: RefPicList1 has num_ref_idx_l1_active_minus1 + 1 entries, at most 32 by 7.4.3",
    ),
    fixed(
        "StdVideoEncodeH264ReferenceListsInfo",
        "reserved1",
        &[("7", 7)],
        "padding: video.xml says that the member is reserved for future use and must be initialized with 0",
    ),
    in_member(
        "StdVideoEncodeH264ReferenceListsInfo",
        "pRefList0ModOperations",
        "refList0ModOpCount",
        "the Vulkan specification (H.264 encode picture information) and video.xml: an array of refList0ModOpCount entries of the ref_pic_list_modification() of H.264 7.3.3.1",
    ),
    in_member(
        "StdVideoEncodeH264ReferenceListsInfo",
        "pRefList1ModOperations",
        "refList1ModOpCount",
        "the Vulkan specification (H.264 encode picture information) and video.xml: an array of refList1ModOpCount entries of the ref_pic_list_modification() of H.264 7.3.3.1",
    ),
    in_member(
        "StdVideoEncodeH264ReferenceListsInfo",
        "pRefPicMarkingOperations",
        "refPicMarkingOpCount",
        "the Vulkan specification (H.264 encode picture information) and video.xml: an array of refPicMarkingOpCount entries of the dec_ref_pic_marking() of H.264 7.3.3.3",
    ),
    one(
        "StdVideoEncodeH264SliceHeader",
        "pWeightTable",
        "H.264 7.3.3: a slice header has one pred_weight_table()",
    ),
    fixed(
        "StdVideoEncodeH264WeightTable",
        "luma_weight_l0",
        &[("STD_VIDEO_H264_MAX_NUM_LIST_REF", 32)],
        "H.264 7.3.3.2 reads luma_weight_l0[i] for i <= num_ref_idx_l0_active_minus1, which 7.4.3 limits to 31",
    ),
    fixed(
        "StdVideoEncodeH264WeightTable",
        "luma_offset_l0",
        &[("STD_VIDEO_H264_MAX_NUM_LIST_REF", 32)],
        "H.264 7.3.3.2 reads luma_offset_l0[i] for i <= num_ref_idx_l0_active_minus1, which 7.4.3 limits to 31",
    ),
    fixed(
        "StdVideoEncodeH264WeightTable",
        "chroma_weight_l0",
        &[
            ("STD_VIDEO_H264_MAX_NUM_LIST_REF", 32),
            ("STD_VIDEO_H264_MAX_CHROMA_PLANES", 2),
        ],
        "H.264 7.3.3.2 reads chroma_weight_l0[i][j] for i <= num_ref_idx_l0_active_minus1, which 7.4.3 limits to 31, and j < 2 (Cb and Cr)",
    ),
    fixed(
        "StdVideoEncodeH264WeightTable",
        "chroma_offset_l0",
        &[
            ("STD_VIDEO_H264_MAX_NUM_LIST_REF", 32),
            ("STD_VIDEO_H264_MAX_CHROMA_PLANES", 2),
        ],
        "H.264 7.3.3.2 reads chroma_offset_l0[i][j] for i <= num_ref_idx_l0_active_minus1, which 7.4.3 limits to 31, and j < 2 (Cb and Cr)",
    ),
    fixed(
        "StdVideoEncodeH264WeightTable",
        "luma_weight_l1",
        &[("STD_VIDEO_H264_MAX_NUM_LIST_REF", 32)],
        "H.264 7.3.3.2 reads luma_weight_l1[i] for i <= num_ref_idx_l1_active_minus1, which 7.4.3 limits to 31",
    ),
    fixed(
        "StdVideoEncodeH264WeightTable",
        "luma_offset_l1",
        &[("STD_VIDEO_H264_MAX_NUM_LIST_REF", 32)],
        "H.264 7.3.3.2 reads luma_offset_l1[i] for i <= num_ref_idx_l1_active_minus1, which 7.4.3 limits to 31",
    ),
    fixed(
        "StdVideoEncodeH264WeightTable",
        "chroma_weight_l1",
        &[
            ("STD_VIDEO_H264_MAX_NUM_LIST_REF", 32),
            ("STD_VIDEO_H264_MAX_CHROMA_PLANES", 2),
        ],
        "H.264 7.3.3.2 reads chroma_weight_l1[i][j] for i <= num_ref_idx_l1_active_minus1, which 7.4.3 limits to 31, and j < 2 (Cb and Cr)",
    ),
    fixed(
        "StdVideoEncodeH264WeightTable",
        "chroma_offset_l1",
        &[
            ("STD_VIDEO_H264_MAX_NUM_LIST_REF", 32),
            ("STD_VIDEO_H264_MAX_CHROMA_PLANES", 2),
        ],
        "H.264 7.3.3.2 reads chroma_offset_l1[i][j] for i <= num_ref_idx_l1_active_minus1, which 7.4.3 limits to 31, and j < 2 (Cb and Cr)",
    ),
    fixed(
        "StdVideoEncodeH265LongTermRefPics",
        "lt_idx_sps",
        &[("STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS", 32)],
        "H.265 7.3.6.1 reads lt_idx_sps[i] for i < num_long_term_sps, which 7.4.7.1 limits to num_long_term_ref_pics_sps, at most 32 by 7.4.3.2.1",
    ),
    fixed(
        "StdVideoEncodeH265LongTermRefPics",
        "poc_lsb_lt",
        &[("STD_VIDEO_H265_MAX_LONG_TERM_PICS", 16)],
        "H.265 7.3.6.1 reads poc_lsb_lt[i] for num_long_term_sps <= i < num_long_term_sps + num_long_term_pics, and 7.4.7.1 limits the number of these entries, together with the short-term ones, to sps_max_dec_pic_buffering_minus1, at most MaxDpbSize - 1 = 15 by A.4.2",
    ),
    fixed(
        "StdVideoEncodeH265LongTermRefPics",
        "delta_poc_msb_present_flag",
        &[("STD_VIDEO_H265_MAX_DELTA_POC", 48)],
        "H.265 7.3.6.1 reads delta_poc_msb_present_flag[i] for i < num_long_term_sps + num_long_term_pics, a sum that 7.4.7.1 limits to sps_max_dec_pic_buffering_minus1, at most MaxDpbSize - 1 = 15 by A.4.2; the header gives the 32 + 16 elements of STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS and STD_VIDEO_H265_MAX_LONG_TERM_PICS together",
    ),
    fixed(
        "StdVideoEncodeH265LongTermRefPics",
        "delta_poc_msb_cycle_lt",
        &[("STD_VIDEO_H265_MAX_DELTA_POC", 48)],
        "H.265 7.3.6.1 reads delta_poc_msb_cycle_lt[i] for i < num_long_term_sps + num_long_term_pics, a sum that 7.4.7.1 limits to sps_max_dec_pic_buffering_minus1, at most MaxDpbSize - 1 = 15 by A.4.2; the header gives the 32 + 16 elements of STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS and STD_VIDEO_H265_MAX_LONG_TERM_PICS together",
    ),
    fixed(
        "StdVideoEncodeH265PictureInfo",
        "reserved1",
        &[("7", 7)],
        "padding: the Vulkan specification describes the member as used only for padding purposes and otherwise ignored, and video.xml says it is reserved for future use and must be initialized with 0",
    ),
    one(
        "StdVideoEncodeH265PictureInfo",
        "pRefLists",
        "the Vulkan specification (H.265 encode picture information): one StdVideoEncodeH265ReferenceListsInfo structure for the reference lists of the picture",
    ),
    one(
        "StdVideoEncodeH265PictureInfo",
        "pShortTermRefPicSet",
        "H.265 7.3.6.1: a slice segment header has one st_ref_pic_set(num_short_term_ref_pic_sets)",
    ),
    one(
        "StdVideoEncodeH265PictureInfo",
        "pLongTermRefPics",
        "H.265 7.3.6.1: a slice segment header has one list of long-term reference pictures, which StdVideoEncodeH265LongTermRefPics holds",
    ),
    fixed(
        "StdVideoEncodeH265ReferenceListsInfo",
        "RefPicList0",
        &[("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15)],
        "H.265 8.3.4: RefPicList0 has num_ref_idx_l0_active_minus1 + 1 entries, at most 15 by 7.4.7.1",
    ),
    fixed(
        "StdVideoEncodeH265ReferenceListsInfo",
        "RefPicList1",
        &[("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15)],
        "H.265 8.3.4: RefPicList1 has num_ref_idx_l1_active_minus1 + 1 entries, at most 15 by 7.4.7.1",
    ),
    fixed(
        "StdVideoEncodeH265ReferenceListsInfo",
        "list_entry_l0",
        &[("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15)],
        "H.265 7.3.6.2 reads list_entry_l0[i] for i <= num_ref_idx_l0_active_minus1, which 7.4.7.1 limits to 14",
    ),
    fixed(
        "StdVideoEncodeH265ReferenceListsInfo",
        "list_entry_l1",
        &[("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15)],
        "H.265 7.3.6.2 reads list_entry_l1[i] for i <= num_ref_idx_l1_active_minus1, which 7.4.7.1 limits to 14",
    ),
    one(
        "StdVideoEncodeH265SliceSegmentHeader",
        "pWeightTable",
        "H.265 7.3.6.1: a slice segment header has one pred_weight_table()",
    ),
    fixed(
        "StdVideoEncodeH265WeightTable",
        "delta_luma_weight_l0",
        &[("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15)],
        "H.265 7.3.6.3 reads delta_luma_weight_l0[i] for i <= num_ref_idx_l0_active_minus1, which 7.4.7.1 limits to 14",
    ),
    fixed(
        "StdVideoEncodeH265WeightTable",
        "luma_offset_l0",
        &[("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15)],
        "H.265 7.3.6.3 reads luma_offset_l0[i] for i <= num_ref_idx_l0_active_minus1, which 7.4.7.1 limits to 14",
    ),
    fixed(
        "StdVideoEncodeH265WeightTable",
        "delta_chroma_weight_l0",
        &[
            ("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15),
            ("STD_VIDEO_H265_MAX_CHROMA_PLANES", 2),
        ],
        "H.265 7.3.6.3 reads delta_chroma_weight_l0[i][j] for i <= num_ref_idx_l0_active_minus1, which 7.4.7.1 limits to 14, and j < 2 (Cb and Cr)",
    ),
    fixed(
        "StdVideoEncodeH265WeightTable",
        "delta_chroma_offset_l0",
        &[
            ("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15),
            ("STD_VIDEO_H265_MAX_CHROMA_PLANES", 2),
        ],
        "H.265 7.3.6.3 reads delta_chroma_offset_l0[i][j] for i <= num_ref_idx_l0_active_minus1, which 7.4.7.1 limits to 14, and j < 2 (Cb and Cr)",
    ),
    fixed(
        "StdVideoEncodeH265WeightTable",
        "delta_luma_weight_l1",
        &[("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15)],
        "H.265 7.3.6.3 reads delta_luma_weight_l1[i] for i <= num_ref_idx_l1_active_minus1, which 7.4.7.1 limits to 14",
    ),
    fixed(
        "StdVideoEncodeH265WeightTable",
        "luma_offset_l1",
        &[("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15)],
        "H.265 7.3.6.3 reads luma_offset_l1[i] for i <= num_ref_idx_l1_active_minus1, which 7.4.7.1 limits to 14",
    ),
    fixed(
        "StdVideoEncodeH265WeightTable",
        "delta_chroma_weight_l1",
        &[
            ("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15),
            ("STD_VIDEO_H265_MAX_CHROMA_PLANES", 2),
        ],
        "H.265 7.3.6.3 reads delta_chroma_weight_l1[i][j] for i <= num_ref_idx_l1_active_minus1, which 7.4.7.1 limits to 14, and j < 2 (Cb and Cr)",
    ),
    fixed(
        "StdVideoEncodeH265WeightTable",
        "delta_chroma_offset_l1",
        &[
            ("STD_VIDEO_H265_MAX_NUM_LIST_REF", 15),
            ("STD_VIDEO_H265_MAX_CHROMA_PLANES", 2),
        ],
        "H.265 7.3.6.3 reads delta_chroma_offset_l1[i][j] for i <= num_ref_idx_l1_active_minus1, which 7.4.7.1 limits to 14, and j < 2 (Cb and Cr)",
    ),
    fixed(
        "StdVideoH264HrdParameters",
        "bit_rate_value_minus1",
        &[("STD_VIDEO_H264_CPB_CNT_LIST_SIZE", 32)],
        "H.264 E.1.2 reads bit_rate_value_minus1[SchedSelIdx] for SchedSelIdx <= cpb_cnt_minus1, which E.2.2 limits to the range 0 to 31",
    ),
    fixed(
        "StdVideoH264HrdParameters",
        "cpb_size_value_minus1",
        &[("STD_VIDEO_H264_CPB_CNT_LIST_SIZE", 32)],
        "H.264 E.1.2 reads cpb_size_value_minus1[SchedSelIdx] for SchedSelIdx <= cpb_cnt_minus1, which E.2.2 limits to the range 0 to 31",
    ),
    fixed(
        "StdVideoH264HrdParameters",
        "cbr_flag",
        &[("STD_VIDEO_H264_CPB_CNT_LIST_SIZE", 32)],
        "H.264 E.1.2 reads cbr_flag[SchedSelIdx] for SchedSelIdx <= cpb_cnt_minus1, which E.2.2 limits to the range 0 to 31",
    ),
    one(
        "StdVideoH264PictureParameterSet",
        "pScalingLists",
        "H.264 7.3.2.2: a PPS has one set of scaling lists, which the Vulkan specification (H.264 PPS) describes as the structure that pScalingLists points to",
    ),
    fixed(
        "StdVideoH264ScalingLists",
        "ScalingList4x4",
        &[
            ("STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS", 6),
            ("STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS", 16),
        ],
        "H.264 7.3.2.1.1 and 7.3.2.2 read scaling_list(ScalingList4x4[i], 16, ...) for i < 6",
    ),
    fixed(
        "StdVideoH264ScalingLists",
        "ScalingList8x8",
        &[
            ("STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS", 6),
            ("STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS", 64),
        ],
        "H.264 7.3.2.1.1 and 7.3.2.2 read scaling_list(ScalingList8x8[i - 6], 64, ...) for 6 <= i < 12 when chroma_format_idc is 3, and for 6 <= i < 8 otherwise",
    ),
    in_member(
        "StdVideoH264SequenceParameterSet",
        "pOffsetForRefFrame",
        "num_ref_frames_in_pic_order_cnt_cycle",
        "H.264 7.4.2.1.1: offset_for_ref_frame[i] is a list of num_ref_frames_in_pic_order_cnt_cycle values, in the range 0 to 255, which the uint8_t count holds",
    ),
    one(
        "StdVideoH264SequenceParameterSet",
        "pScalingLists",
        "H.264 7.3.2.1.1: an SPS has one set of scaling lists, which the Vulkan specification (H.264 SPS) describes as the structure that pScalingLists points to",
    ),
    one(
        "StdVideoH264SequenceParameterSet",
        "pSequenceParameterSetVui",
        "H.264 7.3.2.1.1: an SPS has one vui_parameters()",
    ),
    one(
        "StdVideoH264SequenceParameterSetVui",
        "pHrdParameters",
        "H.264 E.1.1 has one hrd_parameters() for the NAL and one for the VCL HRD, and the Vulkan specification (H.264 SPS) describes one StdVideoH264HrdParameters structure for either flag",
    ),
    fixed(
        "StdVideoH265DecPicBufMgr",
        "max_latency_increase_plus1",
        &[("STD_VIDEO_H265_SUBLAYERS_LIST_SIZE", 7)],
        "H.265 7.3.2.1 and 7.3.2.2.1 read vps_max_latency_increase_plus1[i] and sps_max_latency_increase_plus1[i] for i <= vps_max_sub_layers_minus1 or sps_max_sub_layers_minus1, which 7.4.3.1 and 7.4.3.2.1 limit to 6",
    ),
    fixed(
        "StdVideoH265DecPicBufMgr",
        "max_dec_pic_buffering_minus1",
        &[("STD_VIDEO_H265_SUBLAYERS_LIST_SIZE", 7)],
        "H.265 7.3.2.1 and 7.3.2.2.1 read vps_max_dec_pic_buffering_minus1[i] and sps_max_dec_pic_buffering_minus1[i] for i <= vps_max_sub_layers_minus1 or sps_max_sub_layers_minus1, which 7.4.3.1 and 7.4.3.2.1 limit to 6",
    ),
    fixed(
        "StdVideoH265DecPicBufMgr",
        "max_num_reorder_pics",
        &[("STD_VIDEO_H265_SUBLAYERS_LIST_SIZE", 7)],
        "H.265 7.3.2.1 and 7.3.2.2.1 read vps_max_num_reorder_pics[i] and sps_max_num_reorder_pics[i] for i <= vps_max_sub_layers_minus1 or sps_max_sub_layers_minus1, which 7.4.3.1 and 7.4.3.2.1 limit to 6",
    ),
    fixed(
        "StdVideoH265HrdParameters",
        "cpb_cnt_minus1",
        &[("STD_VIDEO_H265_SUBLAYERS_LIST_SIZE", 7)],
        "H.265 E.2.2 reads cpb_cnt_minus1[i] for i <= maxNumSubLayersMinus1, which 7.4.3.1 and 7.4.3.2.1 limit to 6",
    ),
    fixed(
        "StdVideoH265HrdParameters",
        "elemental_duration_in_tc_minus1",
        &[("STD_VIDEO_H265_SUBLAYERS_LIST_SIZE", 7)],
        "H.265 E.2.2 reads elemental_duration_in_tc_minus1[i] for i <= maxNumSubLayersMinus1, which 7.4.3.1 and 7.4.3.2.1 limit to 6",
    ),
    fixed(
        "StdVideoH265HrdParameters",
        "reserved",
        &[("3", 3)],
        "padding: the Vulkan specification describes the member as used only for padding purposes and otherwise ignored, and video.xml says it is reserved for future use and must be initialized with 0",
    ),
    in_another_structure(
        "StdVideoH265HrdParameters",
        "pSubLayerHrdParametersNal",
        "*_max_sub_layers_minus1 + 1",
        "the Vulkan specification (H.265 VPS and SPS): an array of vps_max_sub_layers_minus1 + 1 or sps_max_sub_layers_minus1 + 1 elements, members of the structure that holds the HRD parameters; H.265 E.2.2 has one sub_layer_hrd_parameters() for each sub-layer",
    ),
    in_another_structure(
        "StdVideoH265HrdParameters",
        "pSubLayerHrdParametersVcl",
        "*_max_sub_layers_minus1 + 1",
        "the Vulkan specification (H.265 VPS and SPS): an array of vps_max_sub_layers_minus1 + 1 or sps_max_sub_layers_minus1 + 1 elements, members of the structure that holds the HRD parameters; H.265 E.2.2 has one sub_layer_hrd_parameters() for each sub-layer",
    ),
    fixed(
        "StdVideoH265LongTermRefPicsSps",
        "lt_ref_pic_poc_lsb_sps",
        &[("STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS", 32)],
        "H.265 7.3.2.2.1 reads lt_ref_pic_poc_lsb_sps[i] for i < num_long_term_ref_pics_sps, which 7.4.3.2.1 limits to 32",
    ),
    fixed(
        "StdVideoH265PictureParameterSet",
        "cb_qp_offset_list",
        &[("STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE", 6)],
        "H.265 7.3.2.3.2 reads cb_qp_offset_list[i] for i <= chroma_qp_offset_list_len_minus1, which 7.4.3.3.2 limits to 5",
    ),
    fixed(
        "StdVideoH265PictureParameterSet",
        "cr_qp_offset_list",
        &[("STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE", 6)],
        "H.265 7.3.2.3.2 reads cr_qp_offset_list[i] for i <= chroma_qp_offset_list_len_minus1, which 7.4.3.3.2 limits to 5",
    ),
    fixed(
        "StdVideoH265PictureParameterSet",
        "column_width_minus1",
        &[("STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE", 19)],
        "H.265 7.3.2.3.1 reads column_width_minus1[i] for i < num_tile_columns_minus1, which A.4.1 f) limits to less than MaxTileCols, at most 20 in Table A.8 at the levels that StdVideoH265LevelIdc names; levels 7 to 7.2, which it does not name, allow 40",
    ),
    fixed(
        "StdVideoH265PictureParameterSet",
        "row_height_minus1",
        &[("STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE", 21)],
        "H.265 7.3.2.3.1 reads row_height_minus1[i] for i < num_tile_rows_minus1, which A.4.1 f) limits to less than MaxTileRows, at most 22 in Table A.8 at the levels that StdVideoH265LevelIdc names; levels 7 to 7.2, which it does not name, allow 44",
    ),
    one(
        "StdVideoH265PictureParameterSet",
        "pScalingLists",
        "H.265 7.3.2.3.1: a PPS has one scaling_list_data()",
    ),
    one(
        "StdVideoH265PictureParameterSet",
        "pPredictorPaletteEntries",
        "H.265 7.3.2.3.3: a PPS has one palette predictor initializer, which StdVideoH265PredictorPaletteEntries holds",
    ),
    fixed(
        "StdVideoH265PredictorPaletteEntries",
        "PredictorPaletteEntries",
        &[
            ("STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE", 3),
            (
                "STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE",
                128,
            ),
        ],
        "H.265 7.3.2.2.3 reads sps_palette_predictor_initializer[comp][i] for i <= sps_num_palette_predictor_initializers_minus1, and 7.3.2.3.3 reads pps_palette_predictor_initializer[comp][i] for i < pps_num_palette_predictor_initializers, both for comp < numComps, which is 1 or 3; 7.4.3.2.3 and 7.4.3.3.3 bound the number of entries by PaletteMaxPredictorSize, at most 128 by A.3.7 and A.3.8",
    ),
    fixed(
        "StdVideoH265ScalingLists",
        "ScalingList4x4",
        &[
            ("STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS", 6),
            ("STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS", 16),
        ],
        "H.265 7.3.4: ScalingList[0][matrixId] for matrixId < 6, with coefNum = Min(64, 1 << (4 + (sizeId << 1))) coefficients",
    ),
    fixed(
        "StdVideoH265ScalingLists",
        "ScalingList8x8",
        &[
            ("STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS", 6),
            ("STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS", 64),
        ],
        "H.265 7.3.4: ScalingList[1][matrixId] for matrixId < 6, with coefNum = Min(64, 1 << (4 + (sizeId << 1))) coefficients",
    ),
    fixed(
        "StdVideoH265ScalingLists",
        "ScalingList16x16",
        &[
            ("STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS", 6),
            ("STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS", 64),
        ],
        "H.265 7.3.4: ScalingList[2][matrixId] for matrixId < 6, with coefNum = Min(64, 1 << (4 + (sizeId << 1))) coefficients",
    ),
    fixed(
        "StdVideoH265ScalingLists",
        "ScalingList32x32",
        &[
            ("STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS", 2),
            ("STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS", 64),
        ],
        "H.265 7.3.4: ScalingList[3][matrixId] for matrixId 0 and 3 (matrixId steps by 3 for sizeId 3), with coefNum = 64 coefficients",
    ),
    fixed(
        "StdVideoH265ScalingLists",
        "ScalingListDCCoef16x16",
        &[("STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS", 6)],
        "H.265 7.3.4: scaling_list_dc_coef_minus8[0][matrixId] for matrixId < 6",
    ),
    fixed(
        "StdVideoH265ScalingLists",
        "ScalingListDCCoef32x32",
        &[("STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS", 2)],
        "H.265 7.3.4: scaling_list_dc_coef_minus8[1][matrixId] for matrixId 0 and 3",
    ),
    one(
        "StdVideoH265SequenceParameterSet",
        "pProfileTierLevel",
        "H.265 7.3.2.2.1: an SPS has one profile_tier_level()",
    ),
    one(
        "StdVideoH265SequenceParameterSet",
        "pDecPicBufMgr",
        "H.265 7.3.2.2.1: an SPS has one set of the sps_max_dec_pic_buffering_minus1, sps_max_num_reorder_pics and sps_max_latency_increase_plus1 lists, which StdVideoH265DecPicBufMgr holds",
    ),
    one(
        "StdVideoH265SequenceParameterSet",
        "pScalingLists",
        "H.265 7.3.2.2.1: an SPS has one scaling_list_data()",
    ),
    in_member(
        "StdVideoH265SequenceParameterSet",
        "pShortTermRefPicSet",
        "num_short_term_ref_pic_sets",
        "H.265 7.3.2.2.1 reads st_ref_pic_set(i) for i < num_short_term_ref_pic_sets, and the Vulkan specification (H.265 SPS) describes an array of num_short_term_ref_pic_sets elements",
    ),
    one(
        "StdVideoH265SequenceParameterSet",
        "pLongTermRefPicsSps",
        "H.265 7.3.2.2.1: an SPS has one list of candidate long-term reference pictures, which StdVideoH265LongTermRefPicsSps holds",
    ),
    one(
        "StdVideoH265SequenceParameterSet",
        "pSequenceParameterSetVui",
        "H.265 7.3.2.2.1: an SPS has one vui_parameters()",
    ),
    one(
        "StdVideoH265SequenceParameterSet",
        "pPredictorPaletteEntries",
        "H.265 7.3.2.2.3: an SPS has one palette predictor initializer, which StdVideoH265PredictorPaletteEntries holds",
    ),
    one(
        "StdVideoH265SequenceParameterSetVui",
        "pHrdParameters",
        "H.265 E.2.1: the VUI has one hrd_parameters()",
    ),
    fixed(
        "StdVideoH265ShortTermRefPicSet",
        "delta_poc_s0_minus1",
        &[("STD_VIDEO_H265_MAX_DPB_SIZE", 16)],
        "H.265 7.3.7 reads delta_poc_s0_minus1[i] for i < num_negative_pics, which 7.4.8 limits to sps_max_dec_pic_buffering_minus1, at most MaxDpbSize - 1 = 15 by 7.4.3.2.1 and A.4.2",
    ),
    fixed(
        "StdVideoH265ShortTermRefPicSet",
        "delta_poc_s1_minus1",
        &[("STD_VIDEO_H265_MAX_DPB_SIZE", 16)],
        "H.265 7.3.7 reads delta_poc_s1_minus1[i] for i < num_positive_pics, which 7.4.8 limits to sps_max_dec_pic_buffering_minus1 - num_negative_pics, at most MaxDpbSize - 1 = 15 by 7.4.3.2.1 and A.4.2",
    ),
    fixed(
        "StdVideoH265SubLayerHrdParameters",
        "bit_rate_value_minus1",
        &[("STD_VIDEO_H265_CPB_CNT_LIST_SIZE", 32)],
        "H.265 E.2.3 reads bit_rate_value_minus1[i] for i < CpbCnt, which E.3.3 sets to cpb_cnt_minus1[subLayerId] + 1, at most 32 by E.3.2",
    ),
    fixed(
        "StdVideoH265SubLayerHrdParameters",
        "cpb_size_value_minus1",
        &[("STD_VIDEO_H265_CPB_CNT_LIST_SIZE", 32)],
        "H.265 E.2.3 reads cpb_size_value_minus1[i] for i < CpbCnt, which E.3.3 sets to cpb_cnt_minus1[subLayerId] + 1, at most 32 by E.3.2",
    ),
    fixed(
        "StdVideoH265SubLayerHrdParameters",
        "cpb_size_du_value_minus1",
        &[("STD_VIDEO_H265_CPB_CNT_LIST_SIZE", 32)],
        "H.265 E.2.3 reads cpb_size_du_value_minus1[i] for i < CpbCnt, which E.3.3 sets to cpb_cnt_minus1[subLayerId] + 1, at most 32 by E.3.2",
    ),
    fixed(
        "StdVideoH265SubLayerHrdParameters",
        "bit_rate_du_value_minus1",
        &[("STD_VIDEO_H265_CPB_CNT_LIST_SIZE", 32)],
        "H.265 E.2.3 reads bit_rate_du_value_minus1[i] for i < CpbCnt, which E.3.3 sets to cpb_cnt_minus1[subLayerId] + 1, at most 32 by E.3.2",
    ),
    one(
        "StdVideoH265VideoParameterSet",
        "pDecPicBufMgr",
        "H.265 7.3.2.1: a VPS has one set of the vps_max_dec_pic_buffering_minus1, vps_max_num_reorder_pics and vps_max_latency_increase_plus1 lists, which StdVideoH265DecPicBufMgr holds",
    ),
    one(
        "StdVideoH265VideoParameterSet",
        "pHrdParameters",
        "the Vulkan specification (H.265 VPS) describes one StdVideoH265HrdParameters structure that pHrdParameters points to; H.265 7.3.2.1 can hold vps_num_hrd_parameters hrd_parameters(), but StdVideoH265VideoParameterSet has no member for that number",
    ),
    one(
        "StdVideoH265VideoParameterSet",
        "pProfileTierLevel",
        "H.265 7.3.2.1: a VPS has one profile_tier_level()",
    ),
    fixed(
        "StdVideoVP9LoopFilter",
        "loop_filter_ref_deltas",
        &[("STD_VIDEO_VP9_MAX_REF_FRAMES", 4)],
        "VP9 6.2.8 reads loop_filter_ref_deltas[i] for i < 4 (MAX_REF_FRAMES in section 3)",
    ),
    fixed(
        "StdVideoVP9LoopFilter",
        "loop_filter_mode_deltas",
        &[("STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS", 2)],
        "VP9 6.2.8 reads loop_filter_mode_deltas[i] for i < 2 (MAX_MODE_LF_DELTAS in section 3)",
    ),
    fixed(
        "StdVideoVP9Segmentation",
        "segmentation_tree_probs",
        &[("STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS", 7)],
        "VP9 6.2.11 reads segmentation_tree_probs[i] for i < 7",
    ),
    fixed(
        "StdVideoVP9Segmentation",
        "segmentation_pred_prob",
        &[("STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB", 3)],
        "VP9 6.2.11 reads segmentation_pred_prob[i] for i < 3",
    ),
    fixed(
        "StdVideoVP9Segmentation",
        "FeatureEnabled",
        &[("STD_VIDEO_VP9_MAX_SEGMENTS", 8)],
        "VP9 6.2.11 sets FeatureEnabled[i][j] for i < MAX_SEGMENTS (8), and the header keeps the SEG_LVL_MAX (4) flags of a segment as the bits of one element",
    ),
    fixed(
        "StdVideoVP9Segmentation",
        "FeatureData",
        &[
            ("STD_VIDEO_VP9_MAX_SEGMENTS", 8),
            ("STD_VIDEO_VP9_SEG_LVL_MAX", 4),
        ],
        "VP9 6.2.11 sets FeatureData[i][j] for i < MAX_SEGMENTS (8) and j < SEG_LVL_MAX (4)",
    ),
];

const CONSTANTS: &[StdVideoConstant] = &[
    constant(
        "STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS",
        "6",
        "AV1 5.9.24: gm_params[ref][i] for i < 6",
    ),
    constant(
        "STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS",
        "2",
        "AV1 5.9.11: loop_filter_mode_deltas[i] for i < 2",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS",
        "8",
        "AV1 5.9.19: 1 << cdef_bits strengths, and cdef_bits is f(2)",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS",
        "4",
        "AV1 5.9.11: loop_filter_level[0] to loop_filter_level[3]",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_NUM_CB_POINTS",
        "10",
        "AV1 6.8.20: num_cb_points is less than or equal to 10",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_NUM_CR_POINTS",
        "10",
        "AV1 6.8.20: num_cr_points is less than or equal to 10",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_NUM_PLANES",
        "3",
        "AV1 5.5.2: NumPlanes is 1 or 3",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_NUM_POS_CHROMA",
        "25",
        "AV1 5.9.30: numPosChroma = numPosLuma + 1 when num_y_points is not 0, at most 25",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_NUM_POS_LUMA",
        "24",
        "AV1 5.9.30: numPosLuma = 2 * ar_coeff_lag * (ar_coeff_lag + 1), and ar_coeff_lag is f(2), at most 24",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_NUM_Y_POINTS",
        "14",
        "AV1 6.8.20: num_y_points is less than or equal to 14",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_SEGMENTS",
        "8",
        "AV1 section 3: MAX_SEGMENTS is 8",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_TILE_COLS",
        "64",
        "AV1 section 3: MAX_TILE_COLS is 64, and 6.8.14 limits TileCols to it",
    ),
    constant(
        "STD_VIDEO_AV1_MAX_TILE_ROWS",
        "64",
        "AV1 section 3: MAX_TILE_ROWS is 64, and 6.8.14 limits TileRows to it",
    ),
    constant(
        "STD_VIDEO_AV1_NUM_REF_FRAMES",
        "8",
        "AV1 section 3: NUM_REF_FRAMES is 8",
    ),
    constant(
        "STD_VIDEO_AV1_PRIMARY_REF_NONE",
        "7",
        "AV1 section 3: PRIMARY_REF_NONE is 7",
    ),
    constant(
        "STD_VIDEO_AV1_REFS_PER_FRAME",
        "7",
        "AV1 section 3: REFS_PER_FRAME is 7",
    ),
    constant(
        "STD_VIDEO_AV1_SEG_LVL_MAX",
        "8",
        "AV1 section 3: SEG_LVL_MAX is 8",
    ),
    constant(
        "STD_VIDEO_AV1_SELECT_INTEGER_MV",
        "2",
        "AV1 section 3: SELECT_INTEGER_MV is 2",
    ),
    constant(
        "STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS",
        "2",
        "AV1 section 3: SELECT_SCREEN_CONTENT_TOOLS is 2",
    ),
    constant(
        "STD_VIDEO_AV1_SKIP_MODE_FRAMES",
        "2",
        "AV1 5.9.22: SkipModeFrame[0] and SkipModeFrame[1]",
    ),
    constant(
        "STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME",
        "8",
        "AV1 section 3: TOTAL_REFS_PER_FRAME is 8",
    ),
    constant(
        "STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE",
        "2",
        "H.264 8.2.1: TopFieldOrderCnt and BottomFieldOrderCnt",
    ),
    constant(
        "STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE",
        "8",
        "H.265 A.4.1 e): NumPicTotalCurr, which counts the entries of RefPicSetStCurrBefore, RefPicSetStCurrAfter and RefPicSetLtCurr (7.4.7.2), is at most 8 at every level that StdVideoH265LevelIdc names",
    ),
    constant(
        "STD_VIDEO_H264_CPB_CNT_LIST_SIZE",
        "32",
        "H.264 E.2.2: cpb_cnt_minus1 is in the range 0 to 31",
    ),
    constant(
        "STD_VIDEO_H264_MAX_CHROMA_PLANES",
        "2",
        "H.264 7.3.3.2: chroma weights for j < 2 (Cb and Cr)",
    ),
    constant(
        "STD_VIDEO_H264_MAX_NUM_LIST_REF",
        "32",
        "H.264 7.4.3: num_ref_idx_l0_active_minus1 and num_ref_idx_l1_active_minus1 are at most 31",
    ),
    constant(
        "STD_VIDEO_H264_NO_REFERENCE_PICTURE",
        "255",
        "video.xml: the value of an entry of RefPicList0 or RefPicList1 that names no reference picture, which is not a slot index (VkVideoReferenceSlotInfoKHR::slotIndex)",
    ),
    constant(
        "STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS",
        "16",
        "H.264 7.3.2.1.1: scaling_list(ScalingList4x4[i], 16, ...)",
    ),
    constant(
        "STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS",
        "6",
        "H.264 7.3.2.1.1: ScalingList4x4[i] for i < 6",
    ),
    constant(
        "STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS",
        "64",
        "H.264 7.3.2.1.1: scaling_list(ScalingList8x8[i - 6], 64, ...)",
    ),
    constant(
        "STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS",
        "6",
        "H.264 7.3.2.1.1: ScalingList8x8[i - 6] for 6 <= i < 12 when chroma_format_idc is 3",
    ),
    constant(
        "STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE",
        "6",
        "H.265 7.4.3.3.2: chroma_qp_offset_list_len_minus1 is in the range 0 to 5",
    ),
    constant(
        "STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE",
        "19",
        "H.265 A.4.1 f) and Table A.8: num_tile_columns_minus1 is less than MaxTileCols, at most 20 at the levels that StdVideoH265LevelIdc names; levels 7 to 7.2, which it does not name, allow 40",
    ),
    constant(
        "STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE",
        "21",
        "H.265 A.4.1 f) and Table A.8: num_tile_rows_minus1 is less than MaxTileRows, at most 22 at the levels that StdVideoH265LevelIdc names; levels 7 to 7.2, which it does not name, allow 44",
    ),
    constant(
        "STD_VIDEO_H265_CPB_CNT_LIST_SIZE",
        "32",
        "H.265 E.3.2: cpb_cnt_minus1[i] is in the range 0 to 31",
    ),
    constant(
        "STD_VIDEO_H265_MAX_CHROMA_PLANES",
        "2",
        "H.265 7.3.6.3: chroma weights for j < 2 (Cb and Cr)",
    ),
    constant(
        "STD_VIDEO_H265_MAX_DELTA_POC",
        "48",
        "the sum of STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS (32) and STD_VIDEO_H265_MAX_LONG_TERM_PICS (16), more than the 15 entries to which H.265 7.4.7.1 limits num_long_term_sps + num_long_term_pics",
    ),
    constant(
        "STD_VIDEO_H265_MAX_DPB_SIZE",
        "16",
        "H.265 A.4.2 (A-2): MaxDpbSize is at most 16",
    ),
    constant(
        "STD_VIDEO_H265_MAX_LONG_TERM_PICS",
        "16",
        "H.265 7.4.7.1: num_long_term_pics is at most sps_max_dec_pic_buffering_minus1, which A.4.2 bounds by MaxDpbSize - 1",
    ),
    constant(
        "STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS",
        "32",
        "H.265 7.4.3.2.1: num_long_term_ref_pics_sps is in the range 0 to 32",
    ),
    constant(
        "STD_VIDEO_H265_MAX_NUM_LIST_REF",
        "15",
        "H.265 7.4.7.1: num_ref_idx_l0_active_minus1 and num_ref_idx_l1_active_minus1 are in the range 0 to 14",
    ),
    constant(
        "STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS",
        "64",
        "H.265 7.4.3.2.1: num_short_term_ref_pic_sets is in the range 0 to 64",
    ),
    constant(
        "STD_VIDEO_H265_NO_REFERENCE_PICTURE",
        "255",
        "video.xml: the value of an entry of a reference picture list that names no reference picture, which is not a slot index (VkVideoReferenceSlotInfoKHR::slotIndex)",
    ),
    constant(
        "STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE",
        "3",
        "H.265 7.3.2.2.3: palette predictor initializers for comp < 3",
    ),
    constant(
        "STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE",
        "128",
        "H.265 A.3.7 and A.3.8: PaletteMaxPredictorSize is at most 128",
    ),
    constant(
        "STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS",
        "64",
        "H.265 7.3.4: coefNum is 64 for sizeId 2",
    ),
    constant(
        "STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS",
        "6",
        "H.265 7.3.4: matrixId < 6 for sizeId 2",
    ),
    constant(
        "STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS",
        "64",
        "H.265 7.3.4: coefNum is 64 for sizeId 3",
    ),
    constant(
        "STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS",
        "2",
        "H.265 7.3.4: matrixId 0 and 3 for sizeId 3",
    ),
    constant(
        "STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS",
        "16",
        "H.265 7.3.4: coefNum is 16 for sizeId 0",
    ),
    constant(
        "STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS",
        "6",
        "H.265 7.3.4: matrixId < 6 for sizeId 0",
    ),
    constant(
        "STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS",
        "64",
        "H.265 7.3.4: coefNum is 64 for sizeId 1",
    ),
    constant(
        "STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS",
        "6",
        "H.265 7.3.4: matrixId < 6 for sizeId 1",
    ),
    constant(
        "STD_VIDEO_H265_SUBLAYERS_LIST_SIZE",
        "7",
        "H.265 7.4.3.1 and 7.4.3.2.1: vps_max_sub_layers_minus1 and sps_max_sub_layers_minus1 are in the range 0 to 6",
    ),
    constant(
        "STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS",
        "2",
        "VP9 section 3: MAX_MODE_LF_DELTAS is 2",
    ),
    constant(
        "STD_VIDEO_VP9_MAX_REF_FRAMES",
        "4",
        "VP9 section 3: MAX_REF_FRAMES is 4",
    ),
    constant(
        "STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB",
        "3",
        "VP9 6.2.11: segmentation_pred_prob[i] for i < 3",
    ),
    constant(
        "STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS",
        "7",
        "VP9 6.2.11: segmentation_tree_probs[i] for i < 7",
    ),
    constant(
        "STD_VIDEO_VP9_MAX_SEGMENTS",
        "8",
        "VP9 section 3: MAX_SEGMENTS is 8",
    ),
    constant(
        "STD_VIDEO_VP9_NUM_REF_FRAMES",
        "8",
        "VP9 section 3: NUM_REF_FRAMES is 8",
    ),
    constant(
        "STD_VIDEO_VP9_REFS_PER_FRAME",
        "3",
        "VP9 section 3: REFS_PER_FRAME is 3",
    ),
    constant(
        "STD_VIDEO_VP9_SEG_LVL_MAX",
        "4",
        "VP9 section 3: SEG_LVL_MAX is 4",
    ),
];

fn is_array(m: &Field) -> bool {
    let ty = CType::of(m);
    ty.is_array() || ty.depth() > 0
}

fn std_video_structures(ir: &Registry) -> impl Iterator<Item = &Struct> {
    ir.structs.iter().filter(|s| s.video_std_header.is_some())
}

fn value_of(ir: &Registry, dimension: &str) -> Option<usize> {
    dimension.parse().ok().or_else(|| {
        let c = ir.constants.iter().find(|c| c.name == dimension)?;
        c.value.parse().ok()
    })
}

fn rust_dimension(dimension: &str) -> String {
    if dimension.parse::<usize>().is_ok() {
        dimension.to_string()
    } else {
        const_name(dimension)
    }
}

fn declaration<'g>(generated: &'g str, field: &str) -> Option<&'g str> {
    let (_, rest) = generated.split_once(&format!("pub {field}: "))?;
    rest.split(',').next()
}

fn check_member(ir: &Registry, s: &Struct, m: &Field, generated: &str) {
    let name = format!("{}::{}", s.name, m.name);
    let array = ARRAYS
        .iter()
        .find(|array| array.structure == s.name && array.member == m.name)
        .unwrap_or_else(|| {
            panic!(
                "{name} is an array of a Vulkan Video Std structure that the table of expected lengths does not list: add it with the length that the codec specification gives"
            )
        });
    let source = array.source;
    let field = field_name(&m.name);
    match array.length {
        Length::Fixed(dimensions) => {
            let names = dimensions.iter().map(|(name, _)| *name).collect::<Vec<_>>();
            assert!(
                m.fixed_size_array == names,
                "{name}: the registry gives other dimensions than the table, which takes them from {source}: {:?} instead of {names:?}",
                m.fixed_size_array
            );
            for (dimension, expected) in dimensions {
                let value = value_of(ir, dimension);
                assert!(
                    value == Some(*expected),
                    "{name}: the dimension {dimension} has another value than the table, which takes it from {source}: {value:?} instead of {expected}"
                );
            }
            let declared = declaration(generated, &field)
                .unwrap_or_else(|| panic!("{name} has no field `{field}`: {generated}"));
            let suffix = dimensions
                .iter()
                .rev()
                .map(|(dimension, _)| format!("; {}]", rust_dimension(dimension)))
                .collect::<String>();
            let element = declared
                .strip_prefix(&"[".repeat(dimensions.len()))
                .and_then(|rest| rest.strip_suffix(&suffix));
            assert!(
                element.is_some_and(|element| !element.contains(['[', ']', ';'])),
                "{name} is declared as `{declared}`, not as an array of the dimensions {names:?} in this order"
            );
        }
        Length::InMember(count) => {
            assert!(
                m.len.as_deref() == Some(count),
                "{name}: the registry gives another length than the table, which takes it from {source}: {:?} instead of `{count}`",
                m.len
            );
            let sets_the_count = format!(
                "self.{} = len_as_count(value.len(), \"{name}\", \"{count}\");",
                field_name(count)
            );
            let sets_the_pointer = format!("self.{field} = value.as_ptr();");
            assert!(
                generated.contains(&sets_the_count) && generated.contains(&sets_the_pointer),
                "the setter of {name} does not take a slice whose length it stores in `{count}`: {generated}"
            );
        }
        Length::InAnotherStructure(len) => {
            assert!(
                m.len.as_deref() == Some(len),
                "{name}: the registry gives another length than the table, which takes it from {source}: {:?} instead of `{len}`",
                m.len
            );
            assert!(
                generated.contains(&format!("self.{field} = value.as_ptr();")),
                "the setter of {name} does not take a reference to an array: {generated}"
            );
        }
        Length::NotInTheStructure => {
            let unmarked = format!(
                "{name} is not an array without a length in the registry that the generator lists, which the table takes from {source}"
            );
            assert!(m.len.is_none(), "{unmarked}");
            assert!(unmarked_arrays::length(s, m).is_some(), "{unmarked}");
            let sets_the_pointer = format!(
                "self.{field} = if value.is_empty() {{ core::ptr::null() }} else {{ value.as_ptr() }};"
            );
            assert!(
                generated.contains(&sets_the_pointer),
                "the setter of {name} does not take a slice: {generated}"
            );
        }
        Length::One => {
            let not_one = format!(
                "{name} is not a pointer to one value, which the table takes from {source}"
            );
            assert!(m.len.is_none(), "{not_one}");
            assert!(unmarked_arrays::length(s, m).is_none(), "{not_one}");
            assert!(
                generated.contains(&format!("self.{field} = core::ptr::from_ref(value);")),
                "the setter of {name} does not take a reference to one value: {generated}"
            );
        }
    }
}

fn check_arrays(ir: &Registry) -> usize {
    let model = Model::new(ir);
    let mut checked = BTreeSet::new();
    for s in std_video_structures(ir) {
        assert!(
            !s.union,
            "{} is a union of a Vulkan Video Std header, whose members this check does not read",
            s.name
        );
        let generated = squash(&definition(&model, s));
        for m in s.members.iter().filter(|m| is_array(m)) {
            check_member(ir, s, m, &generated);
            checked.insert((s.name.as_str(), m.name.as_str()));
        }
    }
    for array in ARRAYS {
        assert!(
            checked.contains(&(array.structure, array.member)),
            "{}::{} is in the table of expected lengths, which takes it from {}, but the registry has no such array in a Vulkan Video Std structure",
            array.structure,
            array.member,
            array.source
        );
    }
    checked.len()
}

fn check_constants(ir: &Registry) -> usize {
    let emitted = squash(&constants::emit(&Model::new(ir)));
    let mut checked = BTreeSet::new();
    for c in ir.constants.iter().filter(|c| c.video_std_header.is_some()) {
        let expected = CONSTANTS
            .iter()
            .find(|expected| expected.name == c.name)
            .unwrap_or_else(|| {
                panic!(
                    "{} is a constant of a Vulkan Video Std header that the table of expected values does not list: add it with the value that the codec specification gives",
                    c.name
                )
            });
        assert!(
            c.value == expected.value,
            "{}: the registry gives another value than the table, which takes it from {}: {} instead of {}",
            c.name,
            expected.source,
            c.value,
            expected.value
        );
        let declared = emitted
            .split_once(&format!("pub const {}: ", const_name(&c.name)))
            .and_then(|(_, rest)| rest.split(';').next())
            .unwrap_or_else(|| panic!("{} is not emitted: {emitted}", c.name));
        assert!(
            declared.ends_with(&format!(" = {}", expected.value)),
            "{} is emitted as `{declared}`",
            c.name
        );
        checked.insert(c.name.as_str());
    }
    for expected in CONSTANTS {
        assert!(
            checked.contains(expected.name),
            "{} is in the table of expected values, which takes it from {}, but the registry has no such constant in a Vulkan Video Std header",
            expected.name,
            expected.source
        );
    }
    checked.len()
}

#[test]
fn every_std_video_array_has_the_length_that_the_codec_specifications_give() {
    let arrays = check_arrays(Registry::get());
    assert_eq!(arrays, ARRAYS.len());
    assert!(arrays >= 150, "only {arrays} arrays were checked");
}

#[test]
fn every_std_video_constant_has_the_value_that_the_codec_specifications_give() {
    let constants = check_constants(Registry::get());
    assert_eq!(constants, CONSTANTS.len());
    assert!(constants >= 60, "only {constants} constants were checked");
}

#[test]
fn the_tables_list_each_array_and_constant_once_with_its_source() {
    let mut arrays = ARRAYS
        .iter()
        .map(|array| (array.structure, array.member))
        .collect::<Vec<_>>();
    arrays.sort_unstable();
    arrays.dedup();
    assert_eq!(arrays.len(), ARRAYS.len(), "an array is listed twice");
    let mut constants = CONSTANTS.iter().map(|c| c.name).collect::<Vec<_>>();
    constants.sort_unstable();
    constants.dedup();
    assert_eq!(
        constants.len(),
        CONSTANTS.len(),
        "a constant is listed twice"
    );
    let sources = ARRAYS
        .iter()
        .map(|array| array.source)
        .chain(CONSTANTS.iter().map(|c| c.source));
    for source in sources {
        assert!(
            source.len() > 20,
            "`{source}` does not say where a length comes from"
        );
    }
}

fn changed(change: impl FnOnce(&mut Registry)) -> Registry {
    let mut ir = Registry::get().clone();
    change(&mut ir);
    ir
}

fn member<'r>(ir: &'r mut Registry, structure: &str, member: &str) -> &'r mut Field {
    ir.structs
        .iter_mut()
        .find(|s| s.name == structure)
        .and_then(|s| s.members.iter_mut().find(|m| m.name == member))
        .unwrap_or_else(|| panic!("the registry has no {structure}::{member}"))
}

fn arrays_check_fails(ir: &Registry) -> String {
    catch_quietly(|| check_arrays(ir)).expect_err("the check of the arrays passes")
}

#[test]
fn an_array_that_the_table_does_not_list_fails_the_check() {
    let ir = changed(|ir| {
        let s = ir
            .structs
            .iter_mut()
            .find(|s| s.name == "StdVideoH264SequenceParameterSet")
            .expect("the registry has StdVideoH264SequenceParameterSet");
        let mut list = s.members[0].clone();
        list.name = "offset_list".to_string();
        list.full_ty = "int32_t".to_string();
        list.ty = "int32_t".to_string();
        list.fixed_size_array = vec!["STD_VIDEO_H264_MAX_NUM_LIST_REF".to_string()];
        s.members.push(list);
    });
    let message = arrays_check_fails(&ir);
    assert!(
        message.starts_with(
            "StdVideoH264SequenceParameterSet::offset_list is an array of a Vulkan Video Std structure that the table of expected lengths does not list"
        ),
        "{message}"
    );
}

#[test]
fn a_listed_array_that_the_registry_does_not_have_fails_the_check() {
    let ir = changed(|ir| {
        for s in &mut ir.structs {
            s.members.retain(|m| m.name != "pMiRowStarts");
        }
    });
    let message = arrays_check_fails(&ir);
    assert!(
        message.starts_with(
            "StdVideoAV1TileInfo::pMiRowStarts is in the table of expected lengths, which takes it from the Vulkan specification"
        ),
        "{message}"
    );
}

#[test]
fn a_dimension_that_the_registry_changes_fails_the_check() {
    let ir = changed(|ir| {
        member(ir, "StdVideoH265ScalingLists", "ScalingList32x32").fixed_size_array = vec![
            "STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS".to_string(),
            "STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS".to_string(),
        ];
    });
    let message = arrays_check_fails(&ir);
    assert!(
        message.starts_with(
            "StdVideoH265ScalingLists::ScalingList32x32: the registry gives other dimensions than the table, which takes them from H.265 7.3.4"
        ),
        "{message}"
    );

    let ir = changed(|ir| {
        let c = ir
            .constants
            .iter_mut()
            .find(|c| c.name == "STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE")
            .expect("the registry has STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE");
        c.value = "7".to_string();
    });
    let message = arrays_check_fails(&ir);
    assert!(
        message.starts_with(
            "StdVideoDecodeH265PictureInfo::RefPicSetStCurrBefore: the dimension STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE has another value than the table, which takes it from H.265 8.3.2"
        ),
        "{message}"
    );
    let message =
        catch_quietly(|| check_constants(&ir)).expect_err("the check of the constants passes");
    assert!(
        message.starts_with(
            "STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE: the registry gives another value than the table, which takes it from H.265 A.4.1"
        ),
        "{message}"
    );

    let ir = changed(|ir| {
        member(ir, "StdVideoEncodeAV1PictureInfo", "reserved1").fixed_size_array =
            vec!["4".to_string()];
    });
    let message = arrays_check_fails(&ir);
    assert!(
        message.starts_with("StdVideoEncodeAV1PictureInfo::reserved1: the registry gives other dimensions than the table"),
        "{message}"
    );
}

#[test]
fn a_pointer_whose_length_the_registry_changes_fails_the_check() {
    for (structure, pointer, len, expected) in [
        (
            "StdVideoH265SequenceParameterSet",
            "pShortTermRefPicSet",
            Some("num_long_term_ref_pics_sps"),
            "StdVideoH265SequenceParameterSet::pShortTermRefPicSet: the registry gives another length than the table, which takes it from H.265 7.3.2.2.1",
        ),
        (
            "StdVideoH265HrdParameters",
            "pSubLayerHrdParametersNal",
            None,
            "StdVideoH265HrdParameters::pSubLayerHrdParametersNal: the registry gives the length None, not `*_max_sub_layers_minus1 + 1`: correct the list of arrays whose length is computed from values that their setter cannot see",
        ),
        (
            "StdVideoH265VideoParameterSet",
            "pHrdParameters",
            Some("vps_max_sub_layers_minus1"),
            "StdVideoH265VideoParameterSet::pHrdParameters is not a pointer to one value, which the table takes from the Vulkan specification",
        ),
        (
            "StdVideoEncodeAV1PictureInfo",
            "pBufferRemovalTimes",
            Some("reserved1"),
            "StdVideoEncodeAV1PictureInfo::pBufferRemovalTimes: the registry gives the length `reserved1` now: ",
        ),
    ] {
        let ir = changed(|ir| member(ir, structure, pointer).len = len.map(str::to_string));
        let message = arrays_check_fails(&ir);
        assert!(message.starts_with(expected), "{message}");
    }
}

#[test]
fn a_constant_that_the_table_does_not_list_fails_the_check() {
    let ir = changed(|ir| {
        let mut c = ir
            .constants
            .iter()
            .find(|c| c.video_std_header.is_some())
            .expect("the registry has a Vulkan Video Std constant")
            .clone();
        c.name = "STD_VIDEO_H264_MAX_SLICES".to_string();
        ir.constants.push(c);
    });
    let message =
        catch_quietly(|| check_constants(&ir)).expect_err("the check of the constants passes");
    assert!(
        message.starts_with(
            "STD_VIDEO_H264_MAX_SLICES is a constant of a Vulkan Video Std header that the table of expected values does not list"
        ),
        "{message}"
    );

    let ir = changed(|ir| {
        ir.constants
            .retain(|c| c.name != "STD_VIDEO_VP9_REFS_PER_FRAME")
    });
    let message =
        catch_quietly(|| check_constants(&ir)).expect_err("the check of the constants passes");
    assert!(
        message.starts_with(
            "STD_VIDEO_VP9_REFS_PER_FRAME is in the table of expected values, which takes it from VP9 section 3"
        ),
        "{message}"
    );
}
