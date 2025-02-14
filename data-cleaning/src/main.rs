use clap::Parser;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Parser)]
struct Args {
    input: String,
    output: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
struct Record {
    expected: String,
    actual: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { input, output } = Args::parse();
    let file = File::open(&input)?;
    let mut csv = csv::Reader::from_reader(file);

    let mut records = Vec::new();
    for record in csv.deserialize::<Record>() {
        let record = record?;
        if !records.iter().any(|r| *r == record) {
            records.push(Record {
                expected: record.expected.clone(),
                actual: record.expected.clone(),
            });
        }
        if record.expected != record.actual && !records.iter().any(|r| r.actual == record.actual) {
            records.push(record);
        }
    }

    records.sort_by_key(|r| -(r.actual.len() as isize));

    let output_file = File::create(&output)?;
    let mut output_csv = csv::Writer::from_writer(output_file);

    for record in records {
        output_csv.serialize(record)?;
    }

    Ok(())
}
