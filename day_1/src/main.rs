use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    vec,
};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    path: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    let mut last_depth: i32 = 0;
    let mut increases: i32 = -1; // start at -1 because first will alsways be an increase
    let mut measurements: Vec<i32> = vec![];
    for line in reader.lines() {
        let c_depth: i32 = line?.parse::<i32>().unwrap();
        if last_depth < c_depth {
            increases += 1;
        }
        last_depth = c_depth;
        measurements.push(c_depth);
    }

    println!("Increases: \t\t\t{}", increases);

    let mut sum_of_last = 0;
    let mut increases_of_three = -1;
    for c_measurement_key in 0..measurements.len() {
        if let None = measurements.get(c_measurement_key + 2) {
            break;
        }
        let c_sum: i32 = measurements[c_measurement_key..=c_measurement_key + 2]
            .iter()
            .sum();

        if sum_of_last < c_sum {
            increases_of_three += 1;
        }
        sum_of_last = c_sum;
    }

    println!("Group of three increases: \t{:?}", increases_of_three); //typo :)

    Ok(())
}
