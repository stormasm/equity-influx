use std::collections::HashMap;
use std::error::Error;

fn write_lp(_dirname: &str, vec: Vec<HashMap<String, String>>) -> Result<(), Box<dyn Error>> {
    for line in vec {
        println!("{:?}\n", line);
    }

    Ok(())
}

fn processor(mydir: String, myvec: Vec<HashMap<String, String>>) -> Result<(), Box<dyn Error>> {
    let _x = write_lp(&mydir, myvec);
    Ok(())
}

fn get_vector() -> Vec<HashMap<String, String>> {
    let mut foo = HashMap::new();
    let mut bar = HashMap::new();
    let mut baz = HashMap::new();

    foo.insert("volume".to_string(), "344000.00".to_string());
    foo.insert("close".to_string(), "127.85".to_string());
    foo.insert("timestamp".to_string(), "1583712000".to_string());
    foo.insert("measurement".to_string(), "ui".to_string());

    bar.insert("volume".to_string(), "144000.00".to_string());
    bar.insert("close".to_string(), "125.85".to_string());
    bar.insert("timestamp".to_string(), "1583798400".to_string());
    bar.insert("measurement".to_string(), "ui".to_string());

    baz.insert("volume".to_string(), "244000.00".to_string());
    baz.insert("close".to_string(), "126.85".to_string());
    baz.insert("timestamp".to_string(), "1583884800".to_string());
    baz.insert("measurement".to_string(), "ui".to_string());

    vec![foo, bar, baz]
}

fn main() {
    let dirout = String::from("./tmp/out");
    let vec = get_vector();
    let _ = processor(dirout, vec);
}

// let mut vec = vec![{"volume": "344000.00", "close": "127.85", "timestamp": "1583712000", "measurement": "ui"}, {"close": "135.63", "measurement": "ui", "volume": "240900.00", "timestamp": "1583798400"}, {"measurement": "ui", "close": "129.50", "timestamp": "1583884800", "volume": "336700.00"}, {"measurement": "ui", "volume": "411600.00", "timestamp": "1583971200", "close": "113.45"}, {"close": "122.92", "volume": "440800.00", "measurement": "ui", "timestamp": "1584057600"}];
// https://stackoverflow.com/questions/49452040/how-can-a-vector-of-hashmaps-be-created-and-returned-from-main
