struct VertexInput
{
    [[vk::location(0)]] float2 corner : CORNER;
    [[vk::location(1)]] float2 offset : OFFSET;
    [[vk::location(2)]] float4 color : COLOR;
};

struct VertexOutput
{
    float4 position : SV_Position;
    float4 color : COLOR;
};

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    VertexOutput output;
    output.position = float4(input.corner * 0.2 + input.offset, 0.0, 1.0);
    output.color = input.color;
    return output;
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    return input.color;
}
