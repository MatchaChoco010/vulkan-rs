[[vk::constant_id(0)]] const uint palette_base = 0;
[[vk::constant_id(1)]] const float brightness = 1.0;
[[vk::constant_id(2)]] const bool swap_red_and_blue = false;

static const float3 palette[8] = {
    float3(255.0, 255.0, 255.0) / 255.0,
    float3(220.0, 40.0, 40.0) / 255.0,
    float3(40.0, 220.0, 40.0) / 255.0,
    float3(30.0, 60.0, 240.0) / 255.0,
    float3(220.0, 220.0, 40.0) / 255.0,
    float3(40.0, 220.0, 220.0) / 255.0,
    float3(200.0, 100.0, 240.0) / 255.0,
    float3(250.0, 250.0, 250.0) / 255.0,
};

struct VertexOutput
{
    float4 position : SV_Position;
    float size : SV_PointSize;
    nointerpolation float3 color : COLOR;
};

[shader("vertex")]
VertexOutput vertexMain(
    [[vk::location(0)]] float2 position : POSITION,
    uint instance : SV_InstanceID,
    uint firstInstance : SV_StartInstanceLocation)
{
    VertexOutput output;
    output.position = float4(position, 0.0, 1.0);
    output.size = 1.0;
    output.color = palette[(palette_base + firstInstance + instance) % 8];
    return output;
}

[shader("fragment")]
float4 fragmentMain(nointerpolation float3 color : COLOR) : SV_Target
{
    float3 shaded = color * brightness;
    return float4(swap_red_and_blue ? shaded.bgr : shaded, 1.0);
}
