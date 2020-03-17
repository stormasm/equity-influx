use std::error::Error;
use csv::Reader;
use std::process;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("/tmp14/equity-influx/csv/examples/data/ui.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
