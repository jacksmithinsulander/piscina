use persy::{Persy, PersyId, ValueMode, Config};
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyData {
    // Define your struct fields here
    // Example:
    value: String,
}

pub fn test_persy() -> Result<(), Box<dyn std::error::Error>>  {
    let file_path=  "./data/db.pers";
    let create_segment;
    if !fs::metadata(&file_path).is_ok() {
        let _ = Persy::create(&file_path)?;
        create_segment = true;
    } else {
        create_segment = false;
    }
    let persy = Persy::open(&file_path, Config::new())?;
    if create_segment {
        let mut tx = persy.begin()?;
        tx.create_segment("data")?;
        //let data = vec![1;20];
        //let id = tx.insert("seg", &data);
        tx.create_index::<String, PersyId>("index", ValueMode::Replace)?;
        let prepared = tx.prepare()?;
        prepared.commit()?;
    }
    let mut tx = persy.begin()?;
    let rec = "KUK".as_bytes();
    println!("{:?}", std::str::from_utf8(rec));
    let id = tx.insert("data", rec)?;

    tx.put::<String, PersyId>("index", "key".to_string(), id)?;
    let prepared = tx.prepare()?;
    prepared.commit()?;

    let mut read_id = persy.get::<String, PersyId>("index", &"key".to_string())?;
    if let Some(id) = read_id.next() {
        let value = persy.read("data", &id)?;
        assert_eq!(Some(rec.to_vec()), value);
        //let value_decoded: MyData = serde_json::from_slice(&value)?;
        if let Some(value) = value {
            println!("{:?}", std::str::from_utf8(&value));
        } else {
            println!("Value not found");
        }
    }
    Ok(())
}