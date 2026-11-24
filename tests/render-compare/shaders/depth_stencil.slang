struct Push
{
    float4 color;
};

[[vk::push_constant]]
ConstantBuffer<Push> push;

[shader("vertex")]
float4 vertexMain([[vk::location(0)]] float3 position : POSITION) : SV_Position
{
    return float4(position, 1.0);
}

[shader("fragment")]
float4 fragmentMain() : SV_Target
{
    return push.color;
}
