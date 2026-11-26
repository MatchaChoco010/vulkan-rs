mod fake_device;
mod model;

use model::{
    AIMS, Coverage, GuardUse, Harness, Mix, Op, Pick, Request, Scenario, World, aimed_scenarios,
    config, scenarios,
};
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};

proptest! {
    #![proptest_config(config(128))]

    #[test]
    fn every_sequence_of_operations_behaves_as_the_documentation_says(
        scenario in scenarios_the_property_test_checks()
    ) {
        Harness::run(&scenario).map_err(TestCaseError::fail)?;
    }
}

fn random_scenarios() -> impl Strategy<Value = Scenario> {
    scenarios(80)
}

fn scenarios_with_every_aim() -> impl Strategy<Value = Scenario> {
    aimed_scenarios(&AIMS)
}

fn scenarios_the_property_test_checks() -> impl Strategy<Value = Scenario> {
    prop_oneof![3 => random_scenarios(), 1 => scenarios_with_every_aim()]
}

const RANDOM_SCENARIOS: usize = 400;
const AIMED_SCENARIOS: usize = 360;
const LEAST_REACHED: u64 = 10;

fn coverage_of(strategy: impl Strategy<Value = Scenario>, scenario_count: usize) -> Coverage {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let mut coverage = Coverage::default();
    for _ in 0..scenario_count {
        let scenario = strategy.new_tree(&mut runner).unwrap().current();
        let reached =
            Harness::run(&scenario).unwrap_or_else(|problem| panic!("{scenario:?}: {problem}"));
        coverage.merge(&reached);
    }
    coverage
}

const MAPPING_STATES: [&str; 5] = [
    "unmapped",
    "raw",
    "raw twice or more",
    "persistent",
    "raw and persistent",
];
const MAPPING_OPERATIONS: [&str; 7] = [
    "map_memory ",
    "unmap_memory ",
    "map_persistent ",
    "map ",
    "Flush",
    "Invalidate",
    "free ",
];

fn mapping_operations_in_mapping_states(coverage: &Coverage) -> Vec<(String, u64)> {
    let mut reached = Vec::new();
    for state in MAPPING_STATES {
        for operation in MAPPING_OPERATIONS {
            let count = coverage
                .events
                .iter()
                .filter(|(event, _)| {
                    event.starts_with(operation) && event.ends_with(&format!(": {state}"))
                })
                .map(|(_, count)| count)
                .sum();
            reached.push((
                format!("{operation}on an allocation that is {state}"),
                count,
            ));
        }
    }
    reached
}

fn assert_every_mapping_operation_met_every_mapping_state(coverage: &Coverage) {
    for (operation_in_state, count) in mapping_operations_in_mapping_states(coverage) {
        assert!(count > 0, "no {operation_in_state}");
    }
}

#[test]
fn the_generated_sequences_reach_every_outcome_the_model_distinguishes() {
    let random = coverage_of(random_scenarios(), RANDOM_SCENARIOS);
    let mut coverage = random.clone();
    coverage.merge(&coverage_of(scenarios_with_every_aim(), AIMED_SCENARIOS));
    for (event, count) in &coverage.events {
        println!("{count:>8}  {event}");
    }
    println!("{} pairs of consecutive outcomes", random.pairs.len());

    let mut required = mapping_operations_in_mapping_states(&coverage);
    let mut require = |event: &str| required.push((event.to_string(), coverage.count(event)));
    for handle_use in model::HANDLE_USES {
        for handle in ["stale", "foreign"] {
            require(&format!("{handle} handle: {handle_use:?}"));
        }
    }
    for event in [
        "allocate: invalid description: 0 bytes",
        "allocate: invalid description: alignment that is not a power of two",
        "allocate: invalid description: DEVICE_MASK",
        "invalid description without a compatible memory type",
        "invalid description larger than the memory heap",
        "allocate: no compatible memory type",
        "allocate: larger than the memory heap",
        "allocate: the device is out of memory",
        "allocate: another Vulkan error",
        "allocate: every memory type is out of memory",
        "allocate: falls back to another memory type",
        "allocate: falls back past a heap that is too small",
        "allocate: falls back past a memory type that is out of memory",
        "allocate: frees the empty blocks of the heap and retries",
        "allocate: the retry after freeing the empty blocks succeeds",
        "allocate: the retry after freeing the empty blocks fails",
        "allocate: into a shared block",
        "allocate: a new shared block",
        "allocate: a new block of the size of the allocation",
        "allocate: memory of its own",
        "allocate: mapped at once",
        "allocate: mapped at once in a block that is mapped",
        "allocate: mapped is ignored for memory that is not host-visible",
        "allocate: mapping fails and the allocation is undone",
        "allocate: vkMapMemory fails and the allocation is undone",
        "allocate: vkMapMemory returns null and the allocation is undone",
        "allocate: mapping fails in an existing block, which stays",
        "allocate: mapping fails, and the new block is kept empty",
        "allocate: mapping fails, and the new block is freed",
        "allocation after alignment padding",
        "allocation aligned to nonCoherentAtomSize",
        "allocation takes a small remainder",
        "allocation of zero-initialized memory",
        "shared block limited by its memory heap",
        "free releases the last mapping of a block",
        "free: the empty block is kept",
        "free: the empty block is freed",
        "free: a second empty block is freed",
        "free: the empty block is kept beside one with other flags",
        "allocate: tries the empty block after the blocks that hold allocations",
        "allocate: reuses the empty block",
        "allocate: reuses the empty block for the other kind of resource",
        "free_empty_blocks: frees a block",
        "drop: frees the blocks",
        "free_all: nothing is live",
        "free_all: nothing is mapped",
        "free_all: with mapped blocks",
        "stale handle at the place of a live allocation",
        "map_memory NotHostVisible: unmapped",
        "map_memory Fails(ERROR_MEMORY_MAP_FAILED): unmapped",
        "map_memory Fails(ERROR_OUT_OF_DEVICE_MEMORY): unmapped",
        "map_memory ReturnsNull: unmapped",
        "map_memory MapsTheBlock: unmapped",
        "map_memory SharesTheMapping: unmapped",
        "map_persistent NotHostVisible: unmapped",
        "map_persistent Fails(ERROR_MEMORY_MAP_FAILED): unmapped",
        "map_persistent Fails(ERROR_OUT_OF_DEVICE_MEMORY): unmapped",
        "map_persistent ReturnsNull: unmapped",
        "map_persistent SharesTheMapping: unmapped",
        "map_persistent again: persistent",
        "map NotHostVisible: unmapped",
        "map Fails(ERROR_MEMORY_MAP_FAILED): unmapped",
        "map Fails(ERROR_OUT_OF_DEVICE_MEMORY): unmapped",
        "map ReturnsNull: unmapped",
        "map SharesTheMapping: unmapped",
        "unmap_memory keeps the block mapped: raw",
        "unmap_memory unmaps the block: raw",
        "Flush fails on the device",
        "Invalidate fails on the device",
        "Flush fails on the device with ERROR_OUT_OF_DEVICE_MEMORY",
        "Invalidate fails on the device with ERROR_OUT_OF_DEVICE_MEMORY",
        "Flush fails on the device with ERROR_OUT_OF_HOST_MEMORY",
        "Invalidate fails on the device with ERROR_OUT_OF_HOST_MEMORY",
        "Flush through the guard",
        "Invalidate through the guard",
        "Flush makes no Vulkan call for coherent memory",
        "Invalidate makes no Vulkan call for coherent memory",
        "Flush through the guard makes no Vulkan call for coherent memory",
        "Invalidate through the guard makes no Vulkan call for coherent memory",
    ] {
        require(event);
    }
    for name in ["flush_allocation_range", "invalidate_allocation_range"] {
        for outcome in [
            "refused: unmapped",
            "outside the allocation",
            "of 0 bytes",
            "of coherent memory",
            "fails on the device",
            "fails on the device with ERROR_OUT_OF_DEVICE_MEMORY",
            "fails on the device with ERROR_OUT_OF_HOST_MEMORY",
            "of a part of the allocation",
        ] {
            require(&format!("{name} {outcome}"));
        }
    }
    for name in ["flush_range", "invalidate_range"] {
        for outcome in [
            "",
            " outside the allocation",
            " of 0 bytes",
            " of coherent memory",
        ] {
            require(&format!("{name} through the guard{outcome}"));
        }
    }
    let seldom = required
        .iter()
        .filter(|&&(_, count)| count < LEAST_REACHED)
        .collect::<Vec<_>>();
    assert!(
        seldom.is_empty(),
        "outcomes reached fewer than {LEAST_REACHED} times: {seldom:?}"
    );

    let count = |prefix: &str| {
        random
            .events
            .iter()
            .filter(|(event, _)| event.starts_with(prefix))
            .map(|(_, count)| count)
            .sum::<u64>()
    };
    let placed = count("allocate: into a shared block")
        + count("allocate: a new")
        + count("allocate: memory of its own");
    assert!(
        count("allocate: into a shared block") * 5 > placed,
        "few allocations share a block"
    );
    assert!(
        count("allocate: no compatible memory type") * 4 < count("allocate: "),
        "many requests have no memory type"
    );
    assert!(
        count("free one of several") * 4 > count("free the last of a block"),
        "few frees leave other allocations in the block"
    );
    assert!(
        count("unmap_memory unmaps") + count("unmap_memory keeps") > count("unmap_memory") / 8,
        "few unmap_memory calls release a mapping"
    );
    assert!(
        random.pairs.len() > 1500,
        "few pairs of consecutive outcomes"
    );
}

const TARGET: Pick = Pick::any(0);
const NEIGHBOUR: Pick = Pick::any(1);
const COHERENT_TYPE: u32 = 1;
const NON_COHERENT_TYPE: u32 = 2;

fn mapping_operations() -> [Op; 7] {
    [
        Op::MapMemory(TARGET),
        Op::UnmapMemory(TARGET),
        Op::MapPersistent(TARGET),
        Op::Guard(
            TARGET,
            GuardUse {
                flush: true,
                invalidate: true,
                flush_range: true,
                invalidate_range: true,
                range_offset: 1,
                range_size: 7,
            },
        ),
        Op::Flush(TARGET),
        Op::Invalidate(TARGET),
        Op::Free(TARGET),
    ]
}

fn mapping_states() -> [Vec<Op>; 6] {
    [
        vec![],
        vec![Op::MapMemory(TARGET)],
        vec![Op::MapMemory(TARGET), Op::MapMemory(TARGET)],
        vec![Op::MapPersistent(TARGET)],
        vec![Op::MapMemory(TARGET), Op::MapPersistent(TARGET)],
        vec![
            Op::MapMemory(TARGET),
            Op::MapMemory(TARGET),
            Op::MapPersistent(TARGET),
        ],
    ]
}

fn neighbours(memory_type: u32) -> [Vec<Op>; 4] {
    let neighbour = Op::Allocate(Request::plain(300, 1, memory_type));
    [
        vec![],
        vec![neighbour],
        vec![neighbour, Op::MapMemory(NEIGHBOUR)],
        vec![neighbour, Op::MapPersistent(NEIGHBOUR)],
    ]
}

fn every_run_of_mapping_operations(length: usize) -> Coverage {
    let operations = mapping_operations();
    let mut coverage = Coverage::default();
    let mut scenarios = 0;
    for memory_type in [COHERENT_TYPE, NON_COHERENT_TYPE] {
        for neighbour in neighbours(memory_type) {
            for state in mapping_states() {
                for run in 0..operations.len().pow(length as u32) {
                    let mut ops = vec![Op::Allocate(Request::plain(100, 16, memory_type))];
                    ops.extend(neighbour.iter().copied());
                    ops.extend(state.iter().copied());
                    ops.extend((0..length).map(|position| {
                        operations[run / operations.len().pow(position as u32) % operations.len()]
                    }));
                    let scenario = Scenario {
                        world: World::plain(),
                        mix: Mix::EVERYTHING,
                        ops,
                    };
                    let reached = Harness::run(&scenario)
                        .unwrap_or_else(|problem| panic!("{scenario:?}: {problem}"));
                    coverage.merge(&reached);
                    scenarios += 1;
                }
            }
        }
    }
    assert_eq!(scenarios, 2 * 4 * 6 * operations.len().pow(length as u32));
    coverage
}

#[test]
fn every_pair_of_mapping_operations_from_every_mapping_state_follows_the_model() {
    let coverage = every_run_of_mapping_operations(2);
    assert_every_mapping_operation_met_every_mapping_state(&coverage);
    for event in [
        "free releases the last mapping of a block",
        "unmap_memory keeps the block mapped: raw",
        "unmap_memory unmaps the block: raw",
        "map_memory SharesTheMapping: unmapped",
        "map_persistent SharesTheMapping: unmapped",
        "map SharesTheMapping: unmapped",
        "free the last of a block: persistent",
        "free one of several: persistent",
        "Flush makes no Vulkan call for coherent memory",
        "Invalidate makes no Vulkan call for coherent memory",
        "Flush through the guard makes no Vulkan call for coherent memory",
        "Invalidate through the guard makes no Vulkan call for coherent memory",
        "Flush through the guard",
        "Invalidate through the guard",
        "flush_range through the guard",
        "invalidate_range through the guard",
        "flush_range through the guard of coherent memory",
        "invalidate_range through the guard of coherent memory",
    ] {
        assert!(coverage.count(event) > 0, "never reached: {event}");
    }
}

#[test]
fn slow_every_triple_of_mapping_operations_from_every_mapping_state_follows_the_model() {
    every_run_of_mapping_operations(3);
}

fn every_sequence_of_placements(length: usize) -> usize {
    let requests = [
        Request::plain(1, 1, NON_COHERENT_TYPE),
        Request::plain(1000, 1, NON_COHERENT_TYPE),
        Request::plain(2000, 256, NON_COHERENT_TYPE),
        Request::plain(3841, 1, NON_COHERENT_TYPE),
        Request::plain(4097, 1, NON_COHERENT_TYPE),
    ];
    let alphabet = requests
        .into_iter()
        .map(Op::Allocate)
        .chain((0..3).map(|index| Op::Free(Pick::any(index))))
        .collect::<Vec<_>>();
    let mut scenarios = 0;
    for run in 0..alphabet.len().pow(length as u32) {
        let ops = (0..length)
            .map(|position| alphabet[run / alphabet.len().pow(position as u32) % alphabet.len()])
            .collect();
        let scenario = Scenario {
            world: World::plain(),
            mix: Mix::EVERYTHING,
            ops,
        };
        Harness::run(&scenario).unwrap_or_else(|problem| panic!("{scenario:?}: {problem}"));
        scenarios += 1;
    }
    scenarios
}

#[test]
fn every_sequence_of_four_placements_and_frees_follows_the_model() {
    assert_eq!(every_sequence_of_placements(4), 4096);
}

#[test]
fn slow_every_sequence_of_five_placements_and_frees_follows_the_model() {
    assert_eq!(every_sequence_of_placements(5), 32768);
}
