struct Push
{
    float level;
    uint dropped;
};

[[vk::push_constant]]
ConstantBuffer<Push> push;

struct ControlPoint
{
    float4 position : SV_Position;
};

struct PatchConstants
{
    float edges[4] : SV_TessFactor;
    float inside[2] : SV_InsideTessFactor;
};

struct DomainOutput
{
    float4 position : SV_Position;
    float2 uv : DOMAIN;
};

struct GeometryOutput
{
    float4 position : SV_Position;
    nointerpolation float3 color : COLOR;
};

[shader("vertex")]
ControlPoint vertexMain([[vk::location(0)]] float2 position : POSITION)
{
    ControlPoint output;
    output.position = float4(position, 0.0, 1.0);
    return output;
}

PatchConstants patchConstants()
{
    PatchConstants output;
    for (int edge = 0; edge < 4; edge++)
    {
        output.edges[edge] = push.level;
    }
    output.inside[0] = push.level;
    output.inside[1] = push.level;
    return output;
}

[shader("hull")]
[domain("quad")]
[partitioning("integer")]
[outputtopology("triangle_cw")]
[outputcontrolpoints(4)]
[patchconstantfunc("patchConstants")]
ControlPoint hullMain(InputPatch<ControlPoint, 4> patch, uint id : SV_OutputControlPointID)
{
    return patch[id];
}

[shader("domain")]
[domain("quad")]
DomainOutput domainMain(PatchConstants constants, float2 uv : SV_DomainLocation, OutputPatch<ControlPoint, 4> patch)
{
    DomainOutput output;
    float4 top = lerp(patch[0].position, patch[1].position, uv.x);
    float4 bottom = lerp(patch[2].position, patch[3].position, uv.x);
    output.position = lerp(top, bottom, uv.y);
    output.uv = uv;
    return output;
}

static const float3 palette[3] = {
    float3(230.0, 60.0, 60.0) / 255.0,
    float3(60.0, 230.0, 60.0) / 255.0,
    float3(60.0, 60.0, 230.0) / 255.0,
};

[shader("geometry")]
[maxvertexcount(3)]
void geometryMain(triangle DomainOutput input[3], inout TriangleStream<GeometryOutput> stream)
{
    float2 center = (input[0].uv + input[1].uv + input[2].uv) / 3.0;
    uint2 cell = uint2(floor(center * push.level));
    uint index = cell.x + uint(push.level) * cell.y;
    if (index == push.dropped)
    {
        return;
    }
    for (int corner = 0; corner < 3; corner++)
    {
        GeometryOutput output;
        output.position = input[corner].position;
        output.color = palette[index % 3];
        stream.Append(output);
    }
    stream.RestartStrip();
}

[shader("fragment")]
float4 fragmentMain(nointerpolation float3 color : COLOR) : SV_Target
{
    return float4(color, 1.0);
}
