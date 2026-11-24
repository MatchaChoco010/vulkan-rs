struct Push
{
    float4 color;
    float4 rect;
};

[[vk::push_constant]]
ConstantBuffer<Push> push;

[shader("vertex")]
float4 vertexMain(uint vertex : SV_VertexID) : SV_Position
{
    float2 corners[6] = {
        float2(0.0, 0.0), float2(1.0, 0.0), float2(1.0, 1.0),
        float2(1.0, 1.0), float2(0.0, 1.0), float2(0.0, 0.0),
    };
    float2 corner = corners[vertex];
    return float4(lerp(push.rect.xy, push.rect.zw, corner), 0.0, 1.0);
}

[shader("fragment")]
float4 fragmentMain() : SV_Target
{
    return push.color;
}
