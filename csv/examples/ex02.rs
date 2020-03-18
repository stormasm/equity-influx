use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

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

fn processor(mydir: String) -> Result<(), Box<dyn Error>> {
    let vec = dir_reader(mydir).unwrap();
    // println!("vec len = {:?}", vec.len());

    for name in vec {
        let filename = name.file_name().ok_or("No filename")?;
        println!("{:?}", filename);
    }

    Ok(())
}

fn main() {
    let mydir = String::from("/tmp14/equity-influx/csv/examples/data");
    let _ = processor(mydir);
}
