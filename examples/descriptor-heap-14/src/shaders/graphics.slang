struct InstanceData
{
    float4 translation_scale;
    uint material_index;
    uint3 _pad;
};

struct MaterialData
{
    DescriptorHandle<Texture2D<float4>> base_color_texture;
    uint2 _pad;
    float4 tint;
};

struct PushConstants
{
    float4x4 clip_from_world;
    DescriptorHandle<StructuredBuffer<MaterialData>> materials_handle;
    DescriptorHandle<SamplerState> sampler_handle;
};

[[vk::push_constant]]
PushConstants pc;

struct VertexIn
{
    [[vk::location(0)]] float3 position : POSITION;
    [[vk::location(1)]] float3 normal : NORMAL;
    [[vk::location(2)]] float2 uv : TEXCOORD0;
    [[vk::location(3)]] float4 instance_translation_scale : TEXCOORD1;
    [[vk::location(4)]] uint material_index : TEXCOORD2;
};

struct VertexOut
{
    float4 position : SV_Position;
    [[vk::location(0)]] float3 normal : TEXCOORD0;
    [[vk::location(1)]] float2 uv : TEXCOORD1;
    [[vk::location(2)]] nointerpolation uint material_index : TEXCOORD2;
};

[shader("vertex")]
VertexOut vertexMain(VertexIn input, uint instance_id : SV_InstanceID)
{
    float4 instance = input.instance_translation_scale;
    float3 world_position = input.position * instance.w + instance.xyz;

    VertexOut output;
    output.position = mul(pc.clip_from_world, float4(world_position, 1.0));
    output.normal = normalize(input.normal);
    output.uv = input.uv;
    output.material_index = input.material_index;
    return output;
}

[shader("fragment")]
float4 fragmentMain(VertexOut input) : SV_Target0
{
    StructuredBuffer<MaterialData> materials = *pc.materials_handle;
    MaterialData material = materials[input.material_index];

    Texture2D<float4> base_color_texture = *material.base_color_texture;
    SamplerState material_sampler = *pc.sampler_handle;

    float3 n = normalize(input.normal);
    float light = max(dot(n, normalize(float3(-0.4, 0.65, -0.55))), 0.0);
    float3 base_color = base_color_texture.SampleLevel(material_sampler, input.uv, 0.0).rgb;
    float3 color = base_color * material.tint.rgb * (0.45 + 0.55 * light);
    return float4(color, 1.0);
}
