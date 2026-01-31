use clap::Parser;
use csv::Reader;
use promptput::input;
use std::{error::Error, fs::File, path::Path};

/// CSV - delete columns entirely based on first row (column headings) and print progress
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to edit
    #[arg(short, long)]
    filename: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    let filename: String = args.filename;

    if !Path::new(&filename).exists() {
        panic!("File does not exist.");
    }

    let mut reader: Reader<File> = Reader::from_path(&filename)?;
    let headers: Vec<String> = reader
        .headers()?
        .iter()
        .map(|header: &str| header.to_string())
        .collect();
    println!("{:?}", headers);

    let header_choice: String = input("Which header would you like to remove?");
    match headers
        .iter()
        .any(|header: &String| header.eq(&header_choice))
    {
        true => println!("Exists!"),
        false => panic!("Header does not exist!"),
    };
    // work out index of header_choice in headers
    let index: usize = headers
        .iter()
        .position(|header: &String| header == &header_choice)
        .expect("name was already validated to exist");
    println!("Index: {index}");
    // calculate lines of file
    let line_count: usize = reader.records().count();
    println!("Number of lines (iterator): {line_count}");
    // for each line of file (ignoring first line (headers)):
    let mut reader: Reader<File> = Reader::from_path(&filename)?;
    reader
        .records()
        .for_each(|record: Result<csv::StringRecord, csv::Error>| {
            let record: csv::StringRecord = record.unwrap();

            let line: String = record.iter().collect::<Vec<&str>>().join(", ");
            println!("{line}");
        });
    // GOTO index based on ',' characters
    // remove all items in that specific column

    Ok(())
}
