use std::{env, process};

use coreboros::warrior::Warrior;

/// This binary takes a path to a Redcode source file as command line input, performs syntax and semantic analyses,
/// constructs a valid `Warrior`, and prints it in Load File format to stdout.
/// Reference: <https://corewar.co.uk/standards/icws94.htm#3.0>
fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Usage: asm <input.red>");
        process::exit(1);
    }

    let filepath = &args[0];

    match Warrior::from_file(filepath) {
        Err(err) => {
            eprintln!("Error: {err}");
        }

        Ok(warrior) => {
            let load_file = warrior.to_load_file();
            println!("Load File:\n\n{load_file}");
        }
    }
}
