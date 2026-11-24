struct Push
{
    float lod;
};

[[vk::push_constant]]
ConstantBuffer<Push> push;

[[vk::binding(0, 0)]]
Sampler2D checker;

struct VertexInput
{
    [[vk::location(0)]] float4 position : POSITION;
    [[vk::location(1)]] float2 uv : TEXCOORD;
};

struct VertexOutput
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD;
};

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    VertexOutput output;
    output.position = input.position;
    output.uv = input.uv;
    return output;
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    if (push.lod < 0.0)
    {
        return checker.Sample(input.uv);
    }
    return checker.SampleLevel(input.uv, push.lod);
}
