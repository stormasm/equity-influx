use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::prelude::*;
use csv::Reader;
use std::collections::HashMap;
//use std::error::Error;
//use std::process;

//#[allow(dead_code)]
fn read_csv(filename: &str) -> Result<(), Box<dyn Error>> {
//  let mut rdr = Reader::from_path("./examples/data/ui.csv")?;
    let mut rdr = Reader::from_path(filename)?;

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

fn dir_reader(mydir: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mypath = Path::new(&mydir);
    println!("Entries in {:?}:", mypath);

    let mut vec: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(mypath)? {
        let entry = entry?;
        let path = entry.path();
        //println!("{:?}", path.file_name().ok_or("No filename")?);
        vec.push(path);
    }

    Ok(vec)
}

#[allow(dead_code)]
fn file_stem(filename: &str) -> Option<&str> {
    let path = Path::new(filename);
    let name = path.file_stem().unwrap().to_str();
    println!("{:?}", name);
    name
}

fn processor(mydir: String) -> Result<(), Box<dyn Error>> {
    let vec = dir_reader(mydir).unwrap();
    // println!("vec len = {:?}", vec.len());

    //let _x = read_csv("./examples/data/ui.csv");

    for name in vec {
        // let filename = name.file_name().ok_or("No filename")?;
        let filename = name.to_str().unwrap();
        let _x = read_csv(filename);
        let filestem = name.file_stem();
        println!("{:?} {:?}", filename, filestem);
        // println!("{:?}", file_stem(filename));
    }

    Ok(())
}

fn main() {
    let mydir = String::from("/tmp14/equity-influx/csv/examples/data");
    let _ = processor(mydir);
}
