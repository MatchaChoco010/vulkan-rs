struct MeshVertex
{
    float4 position;
    float4 color;
};

struct MeshDraw
{
    uint vertexOffset;
    uint indexOffset;
    uint triangleCount;
    uint _pad;
};

[vk::binding(0, 0)]
StructuredBuffer<MeshVertex> gVertices;

[vk::binding(1, 0)]
StructuredBuffer<uint> gIndices;

[vk::binding(2, 0)]
StructuredBuffer<MeshDraw> gDraws;

struct TaskPayload
{
    uint vertexOffset;
    uint indexOffset;
    uint triangleCount;
    uint _pad;
};

struct PushConstants
{
    float4x4 clipFromWorld;
};

[vk::push_constant]
PushConstants pc;

struct VertexOut
{
    float4 position : SV_Position;
    float3 color : COLOR0;
};

groupshared TaskPayload gTaskPayload;

[shader("amplification")]
[numthreads(1, 1, 1)]
void taskMain(uint3 dispatchThreadId : SV_DispatchThreadID)
{
    MeshDraw draw = gDraws[dispatchThreadId.x];
    gTaskPayload.vertexOffset = draw.vertexOffset;
    gTaskPayload.indexOffset = draw.indexOffset;
    gTaskPayload.triangleCount = draw.triangleCount;
    gTaskPayload._pad = 0;
    DispatchMesh(1, 1, 1, gTaskPayload);
}

[shader("mesh")]
[numthreads(1, 1, 1)]
[outputtopology("triangle")]
void meshMain(
    in payload TaskPayload payload,
    out vertices VertexOut verts[3],
    out indices uint3 tris[1])
{
    SetMeshOutputCounts(3, payload.triangleCount);

    uint i0 = gIndices[payload.indexOffset + 0];
    uint i1 = gIndices[payload.indexOffset + 1];
    uint i2 = gIndices[payload.indexOffset + 2];

    MeshVertex v0 = gVertices[payload.vertexOffset + i0];
    MeshVertex v1 = gVertices[payload.vertexOffset + i1];
    MeshVertex v2 = gVertices[payload.vertexOffset + i2];

    verts[0].position = mul(pc.clipFromWorld, v0.position);
    verts[0].color = v0.color.rgb;
    verts[1].position = mul(pc.clipFromWorld, v1.position);
    verts[1].color = v1.color.rgb;
    verts[2].position = mul(pc.clipFromWorld, v2.position);
    verts[2].color = v2.color.rgb;

    tris[0] = uint3(0, 1, 2);
}

struct FragmentOut
{
    float4 color : SV_Target0;
};

[shader("fragment")]
FragmentOut fragmentMain(VertexOut input)
{
    FragmentOut output;
    output.color = float4(input.color, 1.0);
    return output;
}
