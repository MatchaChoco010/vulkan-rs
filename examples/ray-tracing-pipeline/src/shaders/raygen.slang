[vk::binding(0, 0)]
RaytracingAccelerationStructure scene;

[vk::binding(1, 0)]
RWStructuredBuffer<uint> output_values;

static const uint WIDTH = 128;
static const uint HEIGHT = 128;

struct Vertex
{
    float4 position;
    float4 color;
};

struct PushConstants
{
    uint64_t vertexAddress;
    uint64_t indexAddress;
    float3 cameraOrigin;
    float pad0;
    float4x4 worldFromClip;
    float3 backgroundBottom;
    float pad1;
    float3 backgroundTop;
    float pad2;
};

[vk::push_constant]
PushConstants params;

struct Payload
{
    uint hit;
    uint instanceId;
    uint primitiveId;
    float2 barycentrics;
};

uint pack_color(float3 color)
{
    uint r = (uint)round(saturate(color.r) * 255.0);
    uint g = (uint)round(saturate(color.g) * 255.0);
    uint b = (uint)round(saturate(color.b) * 255.0);
    return r | (g << 8) | (b << 16) | (255u << 24);
}

[shader("raygeneration")]
void raygenMain()
{
    uint2 pixel = DispatchRaysIndex().xy;
    uint2 size = DispatchRaysDimensions().xy;
    float2 uv = (float2(pixel) + 0.5) / float2(size);
    float2 ndc = float2(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0);
    float4 farPoint = mul(params.worldFromClip, float4(ndc, 1.0, 1.0));
    float3 worldPoint = farPoint.xyz / farPoint.w;

    RayDesc ray;
    ray.Origin = params.cameraOrigin;
    ray.Direction = normalize(worldPoint - params.cameraOrigin);
    ray.TMin = 0.001;
    ray.TMax = 1000.0;

    Payload payload;
    payload.hit = 0;
    payload.instanceId = 0;
    payload.primitiveId = 0;
    payload.barycentrics = float2(0.0);
    TraceRay(scene, 0u, 0xffu, 0u, 0u, 0u, ray, payload);

    float3 color = lerp(params.backgroundBottom, params.backgroundTop, uv.y);
    if (payload.hit != 0)
    {
        Vertex* vertices = reinterpret<Vertex*>(params.vertexAddress);
        uint* indices = reinterpret<uint*>(params.indexAddress);

        uint firstIndex = payload.primitiveId * 3;
        Vertex v0 = vertices[indices[firstIndex + 0]];
        Vertex v1 = vertices[indices[firstIndex + 1]];
        Vertex v2 = vertices[indices[firstIndex + 2]];
        float3 bary = float3(
            1.0 - payload.barycentrics.x - payload.barycentrics.y,
            payload.barycentrics.x,
            payload.barycentrics.y);
        color = bary.x * v0.color.xyz + bary.y * v1.color.xyz + bary.z * v2.color.xyz;
    }

    output_values[pixel.y * WIDTH + pixel.x] = pack_color(color);
}
