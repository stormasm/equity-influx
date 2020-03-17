use chrono::prelude::*;
use csv::Reader;
use std::error::Error;
use std::process;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("./examples/data/test.csv")?;
    for result in rdr.records() {
        let record = result?;
        let timestamp = &record[0];
        let close = &record[4];
        let volume = &record[5];
        let t1 = Utc.datetime_from_str(timestamp, "%Y-%m-%d %H:%M").unwrap();
        let t2 = t1.timestamp();
        println!("{:?} {:?} {:?}", t2,close,volume);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
