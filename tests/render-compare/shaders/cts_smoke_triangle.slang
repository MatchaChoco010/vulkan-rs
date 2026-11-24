/*
 * The shaders of dEQP-VK.api.smoke.triangle of the Khronos Vulkan Conformance Test Suite
 * (https://github.com/KhronosGroup/VK-GL-CTS,
 * external/vulkancts/modules/vulkan/api/vktApiSmokeTests.cpp, createTriangleProgs),
 * ported from GLSL ES 3.10 to Slang.
 *
 * Copyright (c) 2015 Google Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

[shader("vertex")]
float4 vertexMain(float4 position : POSITION) : SV_Position
{
    return position;
}

[shader("fragment")]
float4 fragmentMain() : SV_Target
{
    return float4(1.0, 0.0, 1.0, 1.0);
}
