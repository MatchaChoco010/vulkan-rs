/*
 * The C reference of the rendering comparison: the scenarios drawn with the Vulkan C API
 * (the headers of third_party/vulkan and the Vulkan loader). src/ draws the same
 * scenarios with the vulkan crate, call for call; the comparison runs both and checks
 * that they make the same Vulkan calls and read back the same pixels.
 *
 * The helpers here and in render.c match those of src/render.rs one to one. A change
 * to one side needs the same change to the other, or the traces differ.
 */
#ifndef RENDER_H
#define RENDER_H

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <vulkan/vulkan.h>

#define CHECK(call)                                                                        \
    do {                                                                                   \
        VkResult check_result_ = (call);                                                   \
        if (check_result_ != VK_SUCCESS) {                                                 \
            render_fail("%s:%d: %s returned %d", __FILE__, __LINE__, #call, check_result_); \
        }                                                                                  \
    } while (0)

#define COUNT(array) ((uint32_t)(sizeof(array) / sizeof((array)[0])))

#define MAX_OUTPUT_IMAGES 8

typedef struct {
    const char *name;
    const char *format;
    uint32_t width;
    uint32_t height;
    uint8_t *data;
    size_t size;
} OutputImage;

typedef struct {
    VkInstance instance;
    VkDebugUtilsMessengerEXT messenger;
    VkPhysicalDevice physical_device;
    VkPhysicalDeviceMemoryProperties memory_properties;
    VkDevice device;
    uint32_t queue_family;
    VkQueue queue;
    VkCommandPool command_pool;
    const char *shader_dir;
    OutputImage images[MAX_OUTPUT_IMAGES];
    uint32_t image_count;
} Ctx;

typedef struct {
    VkBuffer buffer;
    VkDeviceMemory memory;
    VkDeviceSize size;
} Buffer;

typedef struct {
    VkImage image;
    VkDeviceMemory memory;
    VkImageView view;
} Image;

/* An image and the view that image_create makes of all its mip levels and array layers. */
typedef struct {
    VkImageCreateInfo info;
    VkImageAspectFlags aspect;
    VkImageViewType view_type;
} ImageDesc;

/* The state of a graphics pipeline that differs between the scenarios. A NULL pointer
 * leaves that part of the create info NULL, except that a NULL vertex_input stands for no
 * vertex input, a NULL rasterization for filled polygons without culling and a NULL
 * multisample for `samples` samples without any other multisample state. rendering, when
 * not NULL, is chained to the create info for dynamic rendering. */
typedef struct {
    VkShaderModule vertex;
    VkShaderModule fragment;
    const char *vertex_entry;
    const char *fragment_entry;
    const VkPipelineVertexInputStateCreateInfo *vertex_input;
    const VkSpecializationInfo *vertex_specialization;
    const VkSpecializationInfo *fragment_specialization;
    VkPrimitiveTopology topology;
    VkBool32 primitive_restart;
    const VkPipelineRasterizationStateCreateInfo *rasterization;
    VkSampleCountFlagBits samples;
    const VkPipelineMultisampleStateCreateInfo *multisample;
    const VkPipelineDepthStencilStateCreateInfo *depth_stencil;
    const VkPipelineColorBlendAttachmentState *blend_attachments;
    uint32_t blend_attachment_count;
    const VkDynamicState *dynamic_states;
    uint32_t dynamic_state_count;
    VkPipelineLayout layout;
    VkRenderPass render_pass;
    uint32_t subpass;
    const VkPipelineRenderingCreateInfo *rendering;
    uint32_t width;
    uint32_t height;
} GraphicsDesc;

/* The feature structures of extensions that a scenario needs in the pNext chain of the
 * device, in addition to the features that every scenario enables. */
typedef enum {
    EXTENSION_FEATURES_NONE,
    EXTENSION_FEATURES_RAY_QUERY,
    EXTENSION_FEATURES_SHADER_OBJECT,
} ExtensionFeatures;

typedef struct {
    const char *name;
    void (*run)(Ctx *ctx);
    const char *const *device_extensions;
    uint32_t device_extension_count;
    ExtensionFeatures extension_features;
} Scenario;

void render_fail(const char *format, ...) __attribute__((noreturn, format(printf, 1, 2)));

/* Memory from malloc with every byte set to 0x55, which is what malloc means in the C
 * program. The Rust program fills what it allocates with 0xaa, so bytes that a program
 * hands to Vulkan without writing them differ between the two programs instead of being
 * equal by chance. */
void *filled_alloc(size_t size);
#define malloc(size) filled_alloc(size)

void ctx_create(Ctx *ctx, const char *shader_dir, const Scenario *scenario);
void ctx_destroy(Ctx *ctx);
uint32_t ctx_validation_messages(void);
int ctx_validation_enabled(void);

uint32_t memory_type(const Ctx *ctx, uint32_t type_bits, VkMemoryPropertyFlags flags);

Buffer buffer_create(Ctx *ctx, VkDeviceSize size, VkBufferUsageFlags usage,
                     VkMemoryPropertyFlags flags);
void buffer_write(Ctx *ctx, const Buffer *buffer, const void *data, size_t size);
void buffer_destroy(Ctx *ctx, Buffer *buffer);
Buffer buffer_with_data(Ctx *ctx, VkBufferUsageFlags usage, const void *data, size_t size);

ImageDesc image_desc(VkFormat format, uint32_t width, uint32_t height, VkImageUsageFlags usage,
                     VkImageAspectFlags aspect);
Image image_create(Ctx *ctx, const ImageDesc *desc);
void image_destroy(Ctx *ctx, Image *image);

VkCommandBuffer commands_begin(Ctx *ctx);
void commands_submit(Ctx *ctx, VkCommandBuffer command_buffer);

void image_barrier(VkCommandBuffer command_buffer, VkImage image, VkImageAspectFlags aspect,
                   uint32_t base_mip, uint32_t mip_count, VkImageLayout old_layout,
                   VkImageLayout new_layout, VkPipelineStageFlags2 src_stage,
                   VkAccessFlags2 src_access, VkPipelineStageFlags2 dst_stage,
                   VkAccessFlags2 dst_access);

void host_read_barrier(VkCommandBuffer command_buffer, VkPipelineStageFlags2 src_stage,
                       VkAccessFlags2 src_access);

/* Moves a color image from UNDEFINED to COLOR_ATTACHMENT_OPTIMAL, and from there to
 * TRANSFER_SRC_OPTIMAL after the rendering. */
void color_attachment_begin(VkCommandBuffer command_buffer, VkImage image);
void color_attachment_end(VkCommandBuffer command_buffer, VkImage image);

/* A clear value for a depth/stencil attachment. The bytes of the union that depthStencil
 * does not cover are zero, as with vk::ClearValue::depth_stencil. */
VkClearValue depth_stencil_clear(float depth, uint32_t stencil);

/* Begins dynamic rendering to one color attachment that is cleared to `clear`. */
void rendering_begin(VkCommandBuffer command_buffer, VkImageView view, const float clear[4],
                     uint32_t width, uint32_t height);

void image_upload(Ctx *ctx, VkImage image, uint32_t width, uint32_t height, const void *data,
                  size_t size, VkImageLayout final_layout);

/* Copies one aspect of mip level 0 of an image in TRANSFER_SRC_OPTIMAL to the host and
 * keeps it as an output image. */
void readback(Ctx *ctx, VkImage image, VkImageAspectFlags aspect, uint32_t width,
              uint32_t height, uint32_t texel_size, const char *name, const char *format);
void output_buffer(Ctx *ctx, const Buffer *buffer, uint32_t width, uint32_t height,
                   const char *name, const char *format);
void output_words(Ctx *ctx, const uint32_t *words, uint32_t count, const char *name);
void output_write(Ctx *ctx, const char *path);

/* Reads a SPIR-V file of the shader directory; the caller frees the words. */
uint32_t *shader_code(Ctx *ctx, const char *file_name, size_t *size);
VkShaderModule shader_module(Ctx *ctx, const char *file_name);
VkPipeline graphics_pipeline(Ctx *ctx, const GraphicsDesc *desc);
VkPipelineLayout pipeline_layout(Ctx *ctx, const VkDescriptorSetLayout *set_layouts,
                                 uint32_t set_layout_count, const VkPushConstantRange *ranges,
                                 uint32_t range_count);

extern const VkPipelineColorBlendAttachmentState BLEND_DISABLED;

#endif
