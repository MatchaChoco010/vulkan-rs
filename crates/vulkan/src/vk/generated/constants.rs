#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use crate::vk::*;

/// C constant `STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS: usize = 6;
/// C constant `STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS: usize = 2;
/// C constant `STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS: usize = 8;
/// C constant `STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS: usize = 4;
/// C constant `STD_VIDEO_AV1_MAX_NUM_CB_POINTS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_NUM_CB_POINTS: usize = 10;
/// C constant `STD_VIDEO_AV1_MAX_NUM_CR_POINTS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_NUM_CR_POINTS: usize = 10;
/// C constant `STD_VIDEO_AV1_MAX_NUM_PLANES`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_NUM_PLANES: usize = 3;
/// C constant `STD_VIDEO_AV1_MAX_NUM_POS_CHROMA`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_NUM_POS_CHROMA: usize = 25;
/// C constant `STD_VIDEO_AV1_MAX_NUM_POS_LUMA`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_NUM_POS_LUMA: usize = 24;
/// C constant `STD_VIDEO_AV1_MAX_NUM_Y_POINTS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_NUM_Y_POINTS: usize = 14;
/// C constant `STD_VIDEO_AV1_MAX_SEGMENTS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_SEGMENTS: usize = 8;
/// C constant `STD_VIDEO_AV1_MAX_TILE_COLS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_TILE_COLS: u32 = 64;
/// C constant `STD_VIDEO_AV1_MAX_TILE_ROWS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_MAX_TILE_ROWS: u32 = 64;
/// C constant `STD_VIDEO_AV1_NUM_REF_FRAMES`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_NUM_REF_FRAMES: usize = 8;
/// C constant `STD_VIDEO_AV1_PRIMARY_REF_NONE`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_PRIMARY_REF_NONE: u8 = 7;
/// C constant `STD_VIDEO_AV1_REFS_PER_FRAME`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_REFS_PER_FRAME: usize = 7;
/// C constant `STD_VIDEO_AV1_SEG_LVL_MAX`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_SEG_LVL_MAX: usize = 8;
/// C constant `STD_VIDEO_AV1_SELECT_INTEGER_MV`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_SELECT_INTEGER_MV: u8 = 2;
/// C constant `STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS: u32 = 2;
/// C constant `STD_VIDEO_AV1_SKIP_MODE_FRAMES`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_SKIP_MODE_FRAMES: usize = 2;
/// C constant `STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_av1std`.
pub const STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME: usize = 8;
/// C constant `STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE`, provided by the Vulkan Video
/// Std header `vulkan_video_codec_h264std_decode`.
pub const STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE: usize = 2;
/// C constant `STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h265std_decode`.
pub const STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE: usize = 8;
/// C constant `STD_VIDEO_H264_CPB_CNT_LIST_SIZE`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h264std`.
pub const STD_VIDEO_H264_CPB_CNT_LIST_SIZE: usize = 32;
/// C constant `STD_VIDEO_H264_MAX_CHROMA_PLANES`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h264std`.
pub const STD_VIDEO_H264_MAX_CHROMA_PLANES: usize = 2;
/// C constant `STD_VIDEO_H264_MAX_NUM_LIST_REF`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h264std`.
pub const STD_VIDEO_H264_MAX_NUM_LIST_REF: usize = 32;
/// C constant `STD_VIDEO_H264_NO_REFERENCE_PICTURE`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h264std`.
pub const STD_VIDEO_H264_NO_REFERENCE_PICTURE: u8 = 255;
/// C constant `STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h264std`.
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS: usize = 16;
/// C constant `STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h264std`.
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS: usize = 6;
/// C constant `STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h264std`.
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS: usize = 64;
/// C constant `STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h264std`.
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS: usize = 6;
/// C constant `STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE: usize = 6;
/// C constant `STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE`, provided by the Vulkan
/// Video Std header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE: usize = 19;
/// C constant `STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE`, provided by the Vulkan
/// Video Std header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE: usize = 21;
/// C constant `STD_VIDEO_H265_CPB_CNT_LIST_SIZE`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_CPB_CNT_LIST_SIZE: usize = 32;
/// C constant `STD_VIDEO_H265_MAX_CHROMA_PLANES`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_MAX_CHROMA_PLANES: usize = 2;
/// C constant `STD_VIDEO_H265_MAX_DELTA_POC`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_MAX_DELTA_POC: usize = 48;
/// C constant `STD_VIDEO_H265_MAX_DPB_SIZE`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_MAX_DPB_SIZE: usize = 16;
/// C constant `STD_VIDEO_H265_MAX_LONG_TERM_PICS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_MAX_LONG_TERM_PICS: usize = 16;
/// C constant `STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS: usize = 32;
/// C constant `STD_VIDEO_H265_MAX_NUM_LIST_REF`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_MAX_NUM_LIST_REF: usize = 15;
/// C constant `STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS: u32 = 64;
/// C constant `STD_VIDEO_H265_NO_REFERENCE_PICTURE`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_NO_REFERENCE_PICTURE: u8 = 255;
/// C constant `STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE`, provided by the Vulkan
/// Video Std header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE: usize = 3;
/// C constant `STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE`, provided by the Vulkan
/// Video Std header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE: usize = 128;
/// C constant `STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS`, provided by the Vulkan Video
/// Std header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS: usize = 64;
/// C constant `STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS: usize = 6;
/// C constant `STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS`, provided by the Vulkan Video
/// Std header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS: usize = 64;
/// C constant `STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS: usize = 2;
/// C constant `STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS: usize = 16;
/// C constant `STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS: usize = 6;
/// C constant `STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS: usize = 64;
/// C constant `STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS: usize = 6;
/// C constant `STD_VIDEO_H265_SUBLAYERS_LIST_SIZE`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_h265std`.
pub const STD_VIDEO_H265_SUBLAYERS_LIST_SIZE: usize = 7;
/// C constant `STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_vp9std`.
pub const STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS: usize = 2;
/// C constant `STD_VIDEO_VP9_MAX_REF_FRAMES`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_vp9std`.
pub const STD_VIDEO_VP9_MAX_REF_FRAMES: usize = 4;
/// C constant `STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_vp9std`.
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB: usize = 3;
/// C constant `STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS`, provided by the Vulkan Video Std
/// header `vulkan_video_codec_vp9std`.
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS: usize = 7;
/// C constant `STD_VIDEO_VP9_MAX_SEGMENTS`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_vp9std`.
pub const STD_VIDEO_VP9_MAX_SEGMENTS: usize = 8;
/// C constant `STD_VIDEO_VP9_NUM_REF_FRAMES`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_vp9std`.
pub const STD_VIDEO_VP9_NUM_REF_FRAMES: u32 = 8;
/// C constant `STD_VIDEO_VP9_REFS_PER_FRAME`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_vp9std`.
pub const STD_VIDEO_VP9_REFS_PER_FRAME: u32 = 3;
/// C constant `STD_VIDEO_VP9_SEG_LVL_MAX`, provided by the Vulkan Video Std header
/// `vulkan_video_codec_vp9std`.
pub const STD_VIDEO_VP9_SEG_LVL_MAX: usize = 4;
/// Unused attachment sentinel.
///
/// C constant
/// [`VK_ATTACHMENT_UNUSED`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ATTACHMENT_UNUSED.html).
#[doc(alias = "VK_ATTACHMENT_UNUSED")]
pub const ATTACHMENT_UNUSED: u32 = 4294967295;
/// Alignment requirement for DGF1 compressed data.
///
/// C constant
/// [`VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX.html).
#[doc(alias = "VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX")]
pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;
/// Alignment requirement for DGF1 compressed data.
///
/// C constant
/// [`VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX.html).
#[doc(alias = "VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX")]
pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;
/// High occupancy priority constant.
///
/// C constant
/// [`VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV.html).
#[doc(alias = "VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV")]
pub const COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75;
/// Low occupancy priority constant.
///
/// C constant
/// [`VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV.html).
#[doc(alias = "VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV")]
pub const COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25;
/// Normal occupancy priority constant.
///
/// C constant
/// [`VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV.html).
#[doc(alias = "VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV")]
pub const COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.5;
/// Length of a data graph toolchain version string.
///
/// C constant
/// [`VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM.html).
#[doc(alias = "VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM")]
pub const DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: usize = 3;
/// Boolean false value.
///
/// C constant
/// [`VK_FALSE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FALSE.html).
#[doc(alias = "VK_FALSE")]
pub const FALSE: u32 = 0;
/// Maximum LOD unclamped access sentinel.
///
/// C constant
/// [`VK_LOD_CLAMP_NONE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LOD_CLAMP_NONE.html).
#[doc(alias = "VK_LOD_CLAMP_NONE")]
pub const LOD_CLAMP_NONE: f32 = 1000.0;
/// Length of a locally unique device identifier.
///
/// C constant
/// [`VK_LUID_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LUID_SIZE.html).
#[doc(alias = "VK_LUID_SIZE")]
pub const LUID_SIZE: usize = 8;
/// Length of a data graph TOSA profile or extension name string.
///
/// C constant
/// [`VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM.html).
#[doc(alias = "VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM")]
pub const MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM: usize = 128;
/// Length of a driver name string.
///
/// C constant
/// [`VK_MAX_DESCRIPTION_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DESCRIPTION_SIZE.html).
#[doc(alias = "VK_MAX_DESCRIPTION_SIZE")]
pub const MAX_DESCRIPTION_SIZE: usize = 256;
/// Length of a physical device handle array.
///
/// C constant
/// [`VK_MAX_DEVICE_GROUP_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DEVICE_GROUP_SIZE.html).
#[doc(alias = "VK_MAX_DEVICE_GROUP_SIZE")]
pub const MAX_DEVICE_GROUP_SIZE: usize = 32;
/// Length of a physical device driver information string.
///
/// C constant
/// [`VK_MAX_DRIVER_INFO_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DRIVER_INFO_SIZE.html).
#[doc(alias = "VK_MAX_DRIVER_INFO_SIZE")]
pub const MAX_DRIVER_INFO_SIZE: usize = 256;
/// Maximum length of a physical device driver name string.
///
/// C constant
/// [`VK_MAX_DRIVER_NAME_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DRIVER_NAME_SIZE.html).
#[doc(alias = "VK_MAX_DRIVER_NAME_SIZE")]
pub const MAX_DRIVER_NAME_SIZE: usize = 256;
/// Maximum length of a layer of extension name string.
///
/// C constant
/// [`VK_MAX_EXTENSION_NAME_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_EXTENSION_NAME_SIZE.html).
#[doc(alias = "VK_MAX_EXTENSION_NAME_SIZE")]
pub const MAX_EXTENSION_NAME_SIZE: usize = 256;
/// Length of an array of global queue priorities.
///
/// C constant
/// [`VK_MAX_GLOBAL_PRIORITY_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_GLOBAL_PRIORITY_SIZE.html).
#[doc(alias = "VK_MAX_GLOBAL_PRIORITY_SIZE")]
pub const MAX_GLOBAL_PRIORITY_SIZE: usize = 16;
/// Length of an array of memory heaps.
///
/// C constant
/// [`VK_MAX_MEMORY_HEAPS`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_MEMORY_HEAPS.html).
#[doc(alias = "VK_MAX_MEMORY_HEAPS")]
pub const MAX_MEMORY_HEAPS: usize = 16;
/// Length of an array of memory types.
///
/// C constant
/// [`VK_MAX_MEMORY_TYPES`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_MEMORY_TYPES.html).
#[doc(alias = "VK_MAX_MEMORY_TYPES")]
pub const MAX_MEMORY_TYPES: usize = 32;
/// Length of a data graph operation name string.
///
/// C constant
/// [`VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM.html).
#[doc(alias = "VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM")]
pub const MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: usize = 128;
/// Length of a physical device name string.
///
/// C constant
/// [`VK_MAX_PHYSICAL_DEVICE_NAME_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PHYSICAL_DEVICE_NAME_SIZE.html).
#[doc(alias = "VK_MAX_PHYSICAL_DEVICE_NAME_SIZE")]
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
/// Maximum length of binary key.
///
/// C constant
/// [`VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR.html).
#[doc(alias = "VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR")]
pub const MAX_PIPELINE_BINARY_KEY_SIZE_KHR: usize = 32;
/// Maximum length of a shader module identifier.
///
/// C constant
/// [`VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT.html).
#[doc(alias = "VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT")]
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: usize = 32;
/// Maximum number of tensor rolling backing wraps.
///
/// C constant
/// [`VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM.html).
#[doc(alias = "VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM")]
pub const MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM: usize = 4;
/// Length of an array of supported queue priorities.
///
/// C constant
/// [`VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR.html).
#[doc(alias = "VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR")]
pub const MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: usize = 7;
/// Length of an array of supported queue priorities.
///
/// C constant
/// [`VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR.html).
#[doc(alias = "VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR")]
pub const MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: usize = 3;
/// Sentinel for global acceleration structure partitions.
///
/// C constant
/// [`VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV.html).
#[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV")]
pub const PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = 4294967295;
/// External queue family index sentinel.
///
/// C constant
/// [`VK_QUEUE_FAMILY_EXTERNAL`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_EXTERNAL.html).
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL")]
pub const QUEUE_FAMILY_EXTERNAL: u32 = 4294967294;
/// Foreign queue family index sentinel.
///
/// C constant
/// [`VK_QUEUE_FAMILY_FOREIGN_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_FOREIGN_EXT.html).
#[doc(alias = "VK_QUEUE_FAMILY_FOREIGN_EXT")]
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = 4294967293;
/// Ignored queue family index sentinel.
///
/// C constant
/// [`VK_QUEUE_FAMILY_IGNORED`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_IGNORED.html).
#[doc(alias = "VK_QUEUE_FAMILY_IGNORED")]
pub const QUEUE_FAMILY_IGNORED: u32 = 4294967295;
/// Sentinel for all remaining 3D slices.
///
/// C constant
/// [`VK_REMAINING_3D_SLICES_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_3D_SLICES_EXT.html).
#[doc(alias = "VK_REMAINING_3D_SLICES_EXT")]
pub const REMAINING_3D_SLICES_EXT: u32 = 4294967295;
/// Sentinel for all remaining array layers.
///
/// C constant
/// [`VK_REMAINING_ARRAY_LAYERS`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_ARRAY_LAYERS.html).
#[doc(alias = "VK_REMAINING_ARRAY_LAYERS")]
pub const REMAINING_ARRAY_LAYERS: u32 = 4294967295;
/// Sentinel for all remaining mipmap levels.
///
/// C constant
/// [`VK_REMAINING_MIP_LEVELS`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_MIP_LEVELS.html).
#[doc(alias = "VK_REMAINING_MIP_LEVELS")]
pub const REMAINING_MIP_LEVELS: u32 = 4294967295;
/// Sentinel for an unused shader index.
///
/// C constant
/// [`VK_SHADER_INDEX_UNUSED_AMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_INDEX_UNUSED_AMDX.html).
#[doc(alias = "VK_SHADER_INDEX_UNUSED_AMDX")]
pub const SHADER_INDEX_UNUSED_AMDX: u32 = 4294967295;
/// Sentinel for an unused shader index.
///
/// C constant
/// [`VK_SHADER_UNUSED_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_UNUSED_KHR.html).
#[doc(alias = "VK_SHADER_UNUSED_KHR")]
pub const SHADER_UNUSED_KHR: u32 = 4294967295;
/// Subpass index sentinel expanding synchronization scope outside a subpass.
///
/// C constant
/// [`VK_SUBPASS_EXTERNAL`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBPASS_EXTERNAL.html).
#[doc(alias = "VK_SUBPASS_EXTERNAL")]
pub const SUBPASS_EXTERNAL: u32 = 4294967295;
/// Boolean true value.
///
/// C constant
/// [`VK_TRUE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_TRUE.html).
#[doc(alias = "VK_TRUE")]
pub const TRUE: u32 = 1;
/// Length of a universally unique device or driver build identifier.
///
/// C constant
/// [`VK_UUID_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_UUID_SIZE.html).
#[doc(alias = "VK_UUID_SIZE")]
pub const UUID_SIZE: usize = 16;
/// Sentinel value to use entire remaining array length.
///
/// C constant
/// [`VK_WHOLE_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_WHOLE_SIZE.html).
#[doc(alias = "VK_WHOLE_SIZE")]
pub const WHOLE_SIZE: u64 = 18446744073709551615;
