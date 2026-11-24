#include "render.h"

#include <stdio.h>
#include <string.h>

#include "scenarios.h"

int main(int argc, char **argv) {
    if (argc == 2 && strcmp(argv[1], "--list") == 0) {
        for (uint32_t i = 0; i < SCENARIO_COUNT; i++) {
            printf("%s\n", SCENARIOS[i].name);
        }
        return 0;
    }
    if (argc != 4) {
        fprintf(stderr, "usage: %s <scenario> <shader directory> <output file>\n", argv[0]);
        return 2;
    }
    const Scenario *scenario = NULL;
    for (uint32_t i = 0; i < SCENARIO_COUNT; i++) {
        if (strcmp(SCENARIOS[i].name, argv[1]) == 0) {
            scenario = &SCENARIOS[i];
        }
    }
    if (scenario == NULL) {
        fprintf(stderr, "no scenario named %s\n", argv[1]);
        return 2;
    }

    Ctx ctx;
    ctx_create(&ctx, argv[2], scenario->device_extensions, scenario->device_extension_count);
    printf("validation layer: %s\n", ctx_validation_enabled() ? "enabled" : "not found");
    scenario->run(&ctx);
    ctx_destroy(&ctx);
    output_write(&ctx, argv[3]);

    uint32_t messages = ctx_validation_messages();
    if (messages != 0) {
        fprintf(stderr, "the validation layer reported %u messages\n", messages);
        return 3;
    }
    return 0;
}
