use std::env;
use std::fs;
use std::io::{Read, Write};

fn copy_file(input_file_name: &str, output_file_name: &str) -> Result<(), String> {
    let mut input_file = fs::File::open(input_file_name)
        .map_err(|err| format!("error opening input file {}: {}", input_file_name, err))?;

    let mut contents = Vec::new();

    input_file
        .read_to_end(&mut contents)
        .map_err(|err| format!("error reading input file:{}", err))?;

    let mut output_file = fs::File::create(output_file_name)
        .map_err(|err| format!("error opening output file {}: {}", output_file_name, err))?;

    output_file
        .write_all(&contents)
        .map_err(|err| format!("error writing output file: {}", err))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();

    if args.len() < 3 {
        eprintln!("usage: {} <from> <to>", program);
        return;
    }

    if let Err(err) = copy_file(&args[1], &args[2]) {
        eprintln!("{}", err);
        return;
    }

    println!("Success! File {} copied to {}", &args[1], &args[2])
}
