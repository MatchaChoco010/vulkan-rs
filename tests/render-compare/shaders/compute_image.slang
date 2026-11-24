struct Push
{
    uint tile;
    uint seed;
};

struct Palette
{
    float4 colors[4];
};

[[vk::push_constant]]
ConstantBuffer<Push> push;

[[vk::binding(0, 0)]]
[[vk::image_format("rgba8")]]
RWTexture2D<float4> image;

[[vk::binding(1, 0)]]
ConstantBuffer<Palette> palette;

[[vk::binding(2, 0)]]
RWStructuredBuffer<uint> counters;

[shader("compute")]
[numthreads(8, 8, 1)]
void computeMain(uint3 id : SV_DispatchThreadID, uint3 group : SV_GroupID, uint index : SV_GroupIndex)
{
    uint cell = (id.x / push.tile + id.y / push.tile + push.seed) % 4;
    float4 color = palette.colors[cell];
    color.g = float((id.x ^ id.y) & 255) / 255.0;
    image[id.xy] = color;
    InterlockedAdd(counters[cell], 1);
    if (index == 0)
    {
        counters[4 + group.y * 32 + group.x] = group.x * 1000 + group.y;
    }
}
