[[vk::binding(0, 0)]]
RaytracingAccelerationStructure scene;

static const float2 palette[4] = {
    float2(0.0, 0.0),
    float2(1.0, 0.25),
    float2(0.25, 1.0),
    float2(1.0, 1.0),
};

[shader("vertex")]
float4 vertexMain(uint vertex : SV_VertexID) : SV_Position
{
    return float4(float(vertex & 1) * 4.0 - 1.0, float(vertex >> 1) * 4.0 - 1.0, 0.0, 1.0);
}

[shader("fragment")]
float4 fragmentMain(float4 position : SV_Position) : SV_Target
{
    RayDesc ray;
    ray.Origin = float3(position.xy / 128.0 - 1.0, -1.0);
    ray.Direction = float3(0.0, 0.0, 1.0);
    ray.TMin = 0.0;
    ray.TMax = 10.0;
    RayQuery<RAY_FLAG_NONE> query;
    query.TraceRayInline(scene, RAY_FLAG_NONE, 0x01, ray);
    while (query.Proceed())
    {
    }
    if (query.CommittedStatus() != COMMITTED_TRIANGLE_HIT)
    {
        return float4(0.0, 0.0, 0.0, 1.0);
    }
    return float4(palette[query.CommittedInstanceID() % 4], query.CommittedRayT() / 2.0, 1.0);
}
