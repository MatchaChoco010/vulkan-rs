mod render;
mod scenarios;

use std::alloc::{GlobalAlloc, Layout, System};
use std::path::Path;

use render::Ctx;

const FILL: u8 = 0xaa;

struct FilledAllocations;

unsafe impl GlobalAlloc for FilledAllocations {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let memory = System.alloc(layout);
            if !memory.is_null() {
                memory.write_bytes(FILL, layout.size());
            }
            memory
        }
    }

    unsafe fn dealloc(&self, memory: *mut u8, layout: Layout) {
        unsafe { System.dealloc(memory, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, memory: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            let memory = System.realloc(memory, layout, new_size);
            if !memory.is_null() && new_size > layout.size() {
                memory
                    .add(layout.size())
                    .write_bytes(FILL, new_size - layout.size());
            }
            memory
        }
    }
}

#[global_allocator]
static ALLOCATOR: FilledAllocations = FilledAllocations;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "--list" {
        for scenario in scenarios::SCENARIOS {
            println!("{}", scenario.name);
        }
        return;
    }
    if args.len() != 4 {
        eprintln!(
            "usage: {} <scenario> <shader directory> <output file>",
            args[0]
        );
        std::process::exit(2);
    }
    let Some(scenario) = scenarios::SCENARIOS
        .iter()
        .chain([&scenarios::INVALID_CALL])
        .find(|scenario| scenario.name == args[1])
    else {
        eprintln!("no scenario named {}", args[1]);
        std::process::exit(2);
    };

    let mut ctx = Ctx::new(Path::new(&args[2]), scenario);
    println!(
        "validation layer: {}",
        if render::validation_enabled() {
            "enabled"
        } else {
            "not found"
        }
    );
    (scenario.run)(&mut ctx);
    ctx.destroy();
    ctx.output_write(Path::new(&args[3]));

    let messages = render::validation_messages();
    if messages != 0 {
        eprintln!("the validation layer reported {messages} messages");
        std::process::exit(3);
    }
}
