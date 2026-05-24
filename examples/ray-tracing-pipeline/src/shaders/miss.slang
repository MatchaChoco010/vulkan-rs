struct Payload
{
    uint hit;
    uint instanceId;
    uint primitiveId;
    float2 barycentrics;
};

[shader("miss")]
void missMain(out Payload payload)
{
    payload.hit = 0;
    payload.instanceId = 0;
    payload.primitiveId = 0;
    payload.barycentrics = float2(0.0);
}
