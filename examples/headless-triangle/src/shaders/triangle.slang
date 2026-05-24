struct VSIn {
    float2 position : POSITION;
    float3 color : COLOR0;
};

struct VSOut {
    float4 position : SV_Position;
    float3 color : COLOR0;
};

[shader("vertex")]
VSOut vertexMain(VSIn input) {
    VSOut output;
    output.position = float4(input.position, 0.0, 1.0);
    output.color = input.color;
    return output;
}

[shader("fragment")]
float4 fragmentMain(VSOut input) : SV_Target {
    return float4(input.color, 1.0);
}
