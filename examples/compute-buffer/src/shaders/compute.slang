[vk::binding(0, 0)]
RWStructuredBuffer<uint> values;

[shader("compute")]
[numthreads(64, 1, 1)]
void computeMain(uint3 tid : SV_DispatchThreadID)
{
    values[tid.x] = values[tid.x] * 2 + 1;
}
