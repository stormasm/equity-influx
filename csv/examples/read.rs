use chrono::prelude::*;
use csv::Reader;
use std::error::Error;
use std::process;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("./examples/data/test.csv")?;
    for result in rdr.records() {
        let record = result?;
        // println!("{:?}", record);
        // println!("{:?}", &record[0]);
        let timestamp = &record[0];
        // println!("{:?}",timestamp);
        //let t1 = DateTime::parse_from_str(timestamp,"%Y-%m-%d %H:%M");
        let t2 = Utc.datetime_from_str(timestamp, "%Y-%m-%d %H:%M").unwrap();
        println!("{:?}", t2);
        let t3 = t2.timestamp();
        println!("{:?}", t3);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
