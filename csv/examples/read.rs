use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::prelude::*;
use csv::Reader;
use infcsv::point::Point;

fn read_csv(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(filename)?;

    let mut vecp: Vec<Point> = Vec::new();
    let mut vec: Vec<HashMap<String, String>> = Vec::new();

    for result in rdr.records() {
        let mut entry: HashMap<String, String> = HashMap::new();
        let record = result?;
        let timestamp = &record[0];
        let close = &record[4];
        let volume = &record[5];
        let t1 = Utc.datetime_from_str(timestamp, "%Y-%m-%d %H:%M").unwrap();
        let t2 = t1.timestamp().to_string();
        entry.insert(
            "measurement".to_string(),
            file_stem(filename).unwrap().to_string(),
        );
        entry.insert("timestamp".to_string(), t2);
        entry.insert("close".to_string(), close.to_string());
        entry.insert("volume".to_string(), volume.to_string());

        let point: Point = Point {
            measurement: file_stem(filename).unwrap().to_string(),
            tagset: Point::set_tagset(),
            fieldset: Point::set_fieldset(volume.to_string(), close.to_string()),
            timestamp: t1.timestamp().to_string(),
        };
        vecp.push(point);
        vec.push(entry);
    }
    println!("{:?}\n", vec);
    println!("\n");
    println!("{:?}\n", vecp);
    println!("\n\n\n");
    Ok(())
}

fn dir_reader(mydir: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mypath = Path::new(&mydir);
    let mut vec: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(mypath)? {
        let entry = entry?;
        let path = entry.path();
        vec.push(path);
    }

    Ok(vec)
}

fn file_stem(filename: &str) -> Option<&str> {
    let path = Path::new(filename);
    let name = path.file_stem().unwrap().to_str();
    name
}

fn read_processor(mydir: String) -> Result<(), Box<dyn Error>> {
    let vec = dir_reader(mydir).unwrap();
    for name in vec {
        let filename = name.to_str().unwrap();
        let _x = read_csv(filename);
    }

    Ok(())
}

fn main() {
    let dirin = String::from("./examples/data");
    let _ = read_processor(dirin);
}
