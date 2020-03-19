use std::collections::HashMap;
use std::error::Error;
//use std::fs;
//use std::path::{Path, PathBuf};

fn write_lp(dirname: &str, vec: Vec<HashMap<String, String>>) -> Result<(), Box<dyn Error>> {
    println!("{:?}\n", dirname);
    println!("{:?}\n", vec);
    for line in vec {
        println!("{:?}\n", line);
    }

    Ok(())
}

fn processor(mydir: String, myvec: Vec<HashMap<String,String>>) -> Result<(), Box<dyn Error>> {
    let _x = write_lp(&mydir,myvec);
    Ok(())
}

fn get_vector() -> Vec<HashMap<String, String>> {
    let mut foo = HashMap::new();
    foo.insert("volume".to_string(), "344.00".to_string());
    vec![foo]
}

//Ref
//https://stackoverflow.com/questions/49452040/how-can-a-vector-of-hashmaps-be-created-and-returned-from-main

fn main() {
    let dirout = String::from("./tmp/out");
    let vec = get_vector();
    let _ = processor(dirout,vec);
}

// let mut vec = vec![{"volume": "344000.00", "close": "127.85", "timestamp": "1583712000", "measurement": "ui"}, {"close": "135.63", "measurement": "ui", "volume": "240900.00", "timestamp": "1583798400"}, {"measurement": "ui", "close": "129.50", "timestamp": "1583884800", "volume": "336700.00"}, {"measurement": "ui", "volume": "411600.00", "timestamp": "1583971200", "close": "113.45"}, {"close": "122.92", "volume": "440800.00", "measurement": "ui", "timestamp": "1584057600"}];
