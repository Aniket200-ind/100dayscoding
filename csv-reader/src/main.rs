use csv;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Insufficient arguments provided. Usage: cargo run <input_file>",
        )));
    }

    let input_path = &args[1];
    let output_path = "output.txt"; // Or use args or config for path

    process_csv_file(input_path, output_path)?;

    Ok(())
}

fn process_csv_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    let mut csv_reader = csv::ReaderBuilder::new().from_reader(reader);

    let mut output_file = File::create(output_path)?;

    for result in csv_reader.records() {
        let record = result?;
        let formatted_line = format_record(&record)?;
        output_file.write_all(formatted_line.as_bytes())?;
    }

    Ok(())
}

fn format_record(record: &csv::StringRecord) -> Result<String, Box<dyn Error>> {
    if record.len() < 2 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Insufficient fields in record",
        )));
    }

    let name = &record[0];
    let email = &record[1];
    Ok(format!("Name: {}, Email: {}\n", name, email))
}