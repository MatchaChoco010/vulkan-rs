// Shader of the descriptor heap test in ../device.rs.
//
// Nothing here has a set or a binding. Every resource is reached through a handle, which
// is an index into the resource heap or the sampler heap, passed as push data.

struct Params
{
    float4 tint;
};

struct PushData
{
    DescriptorHandle<Texture2D<float4>> texture;
    DescriptorHandle<SamplerState> sampler;
    DescriptorHandle<StructuredBuffer<Params>> params;
};

[[vk::push_constant]]
PushData push;

[shader("vertex")]
float4 vertexMain(uint vertex_id : SV_VertexID) : SV_Position
{
    // One triangle that covers the whole target.
    float2 uv = float2((vertex_id << 1) & 2, vertex_id & 2);
    return float4(uv * 2.0 - 1.0, 0.0, 1.0);
}

[shader("fragment")]
float4 fragmentMain() : SV_Target0
{
    Texture2D<float4> texture = *push.texture;
    SamplerState texture_sampler = *push.sampler;
    StructuredBuffer<Params> params = *push.params;
    return texture.SampleLevel(texture_sampler, float2(0.5, 0.5), 0.0) * params[0].tint;
}
