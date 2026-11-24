[[vk::binding(0, 0)]]
SamplerCube cube;

[[vk::binding(1, 0)]]
Sampler2DArray layers;

[[vk::binding(2, 0)]]
Sampler3D volume;

[[vk::binding(3, 0)]]
[[vk::image_format("rgba8ui")]]
Sampler2D<uint4> integers;

[[vk::binding(4, 0)]]
Texture2D pair;

[[vk::binding(5, 0)]]
SamplerState border;

[[vk::binding(6, 0)]]
SamplerState mirror;

[[vk::binding(7, 0)]]
Sampler2DShadow shadow;

static const float3 directions[6] = {
    float3(1.0, 0.0, 0.0), float3(-1.0, 0.0, 0.0), float3(0.0, 1.0, 0.0),
    float3(0.0, -1.0, 0.0), float3(0.0, 0.0, 1.0), float3(0.0, 0.0, -1.0),
};

[shader("vertex")]
float4 vertexMain(uint vertex : SV_VertexID) : SV_Position
{
    return float4(float(vertex & 1) * 4.0 - 1.0, float(vertex >> 1) * 4.0 - 1.0, 0.0, 1.0);
}

[shader("fragment")]
float4 fragmentMain(float4 position : SV_Position) : SV_Target
{
    uint2 pixel = uint2(position.xy);
    uint cell = pixel.x / 64 + 4 * (pixel.y / 64);
    float2 uv = float2(pixel % 64) / 64.0 + 1.0 / 128.0;
    if (cell < 6)
    {
        return cube.SampleLevel(directions[cell], 0.0);
    }
    if (cell < 9)
    {
        return layers.SampleLevel(float3(uv, float(cell - 6)), 0.0);
    }
    if (cell < 12)
    {
        float depths[3] = { 0.25, 0.75, 0.5 };
        return volume.SampleLevel(float3(uv, depths[cell - 9]), 0.0);
    }
    if (cell == 12)
    {
        return float4(integers.Load(int3(0, 0, 0))) / 255.0;
    }
    if (cell == 13)
    {
        return pair.SampleLevel(border, float2(1.5, 0.5), 0.0);
    }
    if (cell == 14)
    {
        return pair.SampleLevel(mirror, float2(1.75, 0.5), 0.0);
    }
    float reference = pixel.x % 64 < 32 ? 0.25 : 0.75;
    float lit = shadow.SampleCmpLevelZero(uv, reference);
    return float4(lit, lit, lit, 1.0);
}
