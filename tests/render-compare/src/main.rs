mod render;
mod scenarios;

use std::path::Path;

use render::Ctx;

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
        .find(|scenario| scenario.name == args[1])
    else {
        eprintln!("no scenario named {}", args[1]);
        std::process::exit(2);
    };

    let mut ctx = Ctx::new(Path::new(&args[2]), scenario.device_extensions);
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
