use csv::Reader;
use std::error::Error;
use std::process;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("./examples/data/test.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
        println!("{:?}", &record[0]);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
