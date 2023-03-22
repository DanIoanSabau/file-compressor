extern crate flate2;

use flate2::{
    write::GzEncoder,
    Compression,
};

use std::{
    env,
    fs::File,
    io,
    time::Instant
};

fn main() {
    let program_arguments = env::args().collect::<Vec<String>>();
    if 3 != program_arguments.len() {
        eprintln!("Usage: cargo run <source> <destination>");
        return;
    }

    // input file & reader initialization
    let source_file = match File::open(&program_arguments[1]) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Cannot open the source file because of an error: {}.", error);
            return;
        }
    };
    let mut input_reader = io::BufReader::new(source_file);

    // output file initialization
    let output_file = match File::create(&program_arguments[2]) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Cannot open the output file because of an error: {}.", error);
            return;
        }
    };

    // encoder initialization
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    // start encoding
    let start_time = Instant::now();

    if let Err(error) = io::copy(&mut input_reader, &mut encoder) {
        eprintln!("Cannot copy the content of the source file to the encoder because of an error: {}.", error);
        return;
    }

    let output_file = match encoder.finish() {
        Ok(output) => output,
        Err(error) => {
            eprintln!("Cannot finish the encoding because of an error: {}.", error);
            return;
        }
    };

    // finished the encoding & printing the results

    println!(
        "Elapsed time: {:?}",
        start_time.elapsed()
    );

    println!(
        "Source file size: {:?}",
        input_reader.get_ref().metadata().unwrap().len()
    );

    println!(
        "Output file size: {:?}",
        output_file.metadata().unwrap().len()
    );
}
