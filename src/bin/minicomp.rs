mod minimal_elf;
mod formula_parser;
mod business_logic;

use std::env;
use std::fs::File;
use std::io::Write;

fn assemble(equations: &Vec<business_logic::Equation>) -> Vec<u8> {
    let mut machine_code = Vec::new();
    let mut bytes = business_logic::assemble_binary(equations);
    machine_code.append(&mut bytes);

    machine_code
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <output_file> <function_definitions>", args[0]);
        std::process::exit(1);
    }

    let equations = business_logic::parse_input_formula(&args[2]);

    let machine_code = assemble(&equations);

    let mut file = File::create(&args[1]).expect("Failed to create output file");
    file.write_all(&machine_code)
        .expect("Failed to write machine code to file");
}

