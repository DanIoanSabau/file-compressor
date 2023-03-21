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
    let mut program_arguments = env::args();
    if 3 != program_arguments.len() {
        eprintln!("Usage: <source> <destination>");
        return;
    }

    // input file & reader initialization
    let source_file_path = program_arguments.nth(1)
        .expect("Cannot get the source file path from the arguments!");
    let source_file = File::open(&source_file_path)
        .expect("Cannot open the source file!");
    let mut input_reader = io::BufReader::new(source_file);

    // output file initialization
    let output_file_path = program_arguments.nth(0)
        .expect("Cannot get the output file path from the program arguments!");
    let output_file = File::create(&output_file_path)
       .expect("Cannot create the output file!");

    // encoder initialization
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    // start encoding
    let start_time = Instant::now();

    io::copy(&mut input_reader, &mut encoder)
        .expect("Cannot copy the content of the source file to the encoder!");

    let output = encoder.finish()
        .expect("Cannot finish the encoding!");

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
        output.metadata().unwrap().len()
    );
}
