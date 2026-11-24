struct VertexOutput
{
    float4 position : SV_Position;
    float4 color : COLOR;
};

[shader("vertex")]
VertexOutput vertexMain(
    [[vk::location(0)]] float2 position : POSITION,
    [[vk::location(1)]] float4 color : COLOR)
{
    VertexOutput output;
    output.position = float4(position, 0.0, 1.0);
    output.color = color;
    return output;
}

[shader("fragment")]
float4 fragmentMain(float4 color : COLOR) : SV_Target
{
    return color;
}
