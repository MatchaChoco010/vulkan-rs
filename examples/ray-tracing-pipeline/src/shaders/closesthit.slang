struct Payload
{
    uint hit;
    uint instanceId;
    uint primitiveId;
    float2 barycentrics;
};

[shader("closesthit")]
void closestHitMain(out Payload payload, BuiltInTriangleIntersectionAttributes attributes)
{
    payload.hit = 1;
    payload.instanceId = InstanceID();
    payload.primitiveId = PrimitiveIndex();
    payload.barycentrics = attributes.barycentrics;
}
