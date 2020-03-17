use chrono::prelude::*;
use csv::Reader;
use std::collections::HashMap;
use std::error::Error;
use std::process;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("./examples/data/test.csv")?;

    let mut vec: Vec<HashMap<String,String>> = Vec::new();

    for result in rdr.records() {
        let mut entry: HashMap<String, String> = HashMap::new();
        let record = result?;
        let timestamp = &record[0];
        let close = &record[4];
        let volume = &record[5];
        let t1 = Utc.datetime_from_str(timestamp, "%Y-%m-%d %H:%M").unwrap();
        let t2 = t1.timestamp().to_string();
        // println!("{:?} {:?} {:?}", t2,close,volume);
        entry.insert("timestamp".to_string(), t2);
        entry.insert("close".to_string(), close.to_string());
        entry.insert("volume".to_string(), volume.to_string());
        println!("{:?}", entry);        
        vec.push(entry);
    }
    println!("{:?}",vec);
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
