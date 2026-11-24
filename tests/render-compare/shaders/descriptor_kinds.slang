struct ColorBlock
{
    float4 color;
};

struct Push
{
    uint index;
};

[[vk::push_constant]]
ConstantBuffer<Push> push;

[[vk::binding(0, 0)]]
ConstantBuffer<ColorBlock> dynamicBlock;

[[vk::binding(1, 0)]]
StructuredBuffer<float4> storageColors;

[[vk::binding(2, 0)]]
Buffer<float4> texelColors;

[[vk::binding(3, 0)]]
Texture2D<float4> sampledImage;

[[vk::binding(4, 0)]]
SamplerState immutableSampler;

[[vk::binding(5, 0)]]
Sampler2D combined[3];

[[vk::binding(0, 1)]]
ConstantBuffer<ColorBlock> pushedBlock;

[shader("vertex")]
float4 vertexMain(uint vertex : SV_VertexID) : SV_Position
{
    float2 corners[3] = { float2(-1.0, -1.0), float2(3.0, -1.0), float2(-1.0, 3.0) };
    return float4(corners[vertex], 0.0, 1.0);
}

[shader("fragment")]
float4 fragmentMain(float4 position : SV_Position) : SV_Target
{
    uint band = uint(position.x) / 32;
    switch (band)
    {
    case 0:
        return dynamicBlock.color;
    case 1:
        return storageColors[1];
    case 2:
        return texelColors.Load(2);
    case 3:
        return sampledImage.Sample(immutableSampler, float2(0.75, 0.25));
    case 4:
        return combined[push.index].Sample(float2(0.5, 0.5));
    case 5:
        return pushedBlock.color;
    case 6:
        return combined[0].Sample(float2(0.5, 0.5));
    default:
        return storageColors[0];
    }
}
