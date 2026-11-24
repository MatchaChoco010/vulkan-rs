struct Inline
{
    float4 first;
    float4 second;
};

[[vk::binding(0, 0)]]
ConstantBuffer<Inline> inlineBlock;

[[vk::binding(1, 0)]]
[[vk::image_format("rgba8")]]
RWBuffer<float4> texels;

[[vk::binding(2, 0)]]
SamplerState nearest;

[[vk::binding(3, 0)]]
Texture2D textures[];

[shader("vertex")]
float4 vertexMain(uint vertex : SV_VertexID) : SV_Position
{
    return float4(float(vertex & 1) * 4.0 - 1.0, float(vertex >> 1) * 4.0 - 1.0, 0.0, 1.0);
}

[shader("fragment")]
float4 fragmentMain(float4 position : SV_Position) : SV_Target
{
    uint2 pixel = uint2(position.xy);
    uint column = pixel.x / 64;
    uint row = pixel.y / 64;
    if (column == 0)
    {
        return row < 2 ? inlineBlock.first : inlineBlock.second;
    }
    if (column == 1)
    {
        return texels[row];
    }
    uint indices[4] = { 0, 1, 3, 3 };
    uint index = column == 2 ? indices[row] : 1;
    return textures[NonUniformResourceIndex(index)].SampleLevel(nearest, float2(0.5, 0.5), 0.0);
}
