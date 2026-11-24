struct VertexInput
{
    [[vk::location(0)]] float2 position : POSITION;
    [[vk::location(1)]] float3 color : COLOR;
};

struct VertexOutput
{
    float4 position : SV_Position;
    float3 color : COLOR;
    float2 uv : TEXCOORD;
};

struct Targets
{
    float4 albedo : SV_Target0;
    float4 normal : SV_Target1;
    float4 material : SV_Target2;
};

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    VertexOutput output;
    output.position = float4(input.position, 0.0, 1.0);
    output.color = input.color;
    output.uv = input.position * 0.5 + 0.5;
    return output;
}

[shader("fragment")]
Targets fragmentMain(VertexOutput input)
{
    Targets targets;
    targets.albedo = float4(input.color, 1.0);
    targets.normal = float4(input.uv, 0.5, 1.0);
    targets.material = float4(1.0 - input.color, 1.0);
    return targets;
}

[[vk::input_attachment_index(0)]]
[[vk::binding(0, 0)]]
SubpassInput<float4> albedo;

[[vk::input_attachment_index(1)]]
[[vk::binding(1, 0)]]
SubpassInput<float4> normal;

[[vk::input_attachment_index(2)]]
[[vk::binding(2, 0)]]
SubpassInput<float4> material;

[shader("vertex")]
float4 compositeVertexMain(uint vertex : SV_VertexID) : SV_Position
{
    float2 corners[3] = { float2(-1.0, -1.0), float2(3.0, -1.0), float2(-1.0, 3.0) };
    return float4(corners[vertex], 0.0, 1.0);
}

[shader("fragment")]
float4 compositeFragmentMain() : SV_Target
{
    float4 a = albedo.SubpassLoad();
    float4 n = normal.SubpassLoad();
    float4 m = material.SubpassLoad();
    return float4(a.r * n.x, a.g * n.y + m.g * 0.25, max(a.b, m.b * 0.5), 1.0);
}
