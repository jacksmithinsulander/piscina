use persy::{Persy, PersyId, ValueMode, Config};
use std::fs;

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
    let rec = "aaaa".as_bytes();
    let id = tx.insert("data", rec)?;

    tx.put::<String, PersyId>("index", "key".to_string(), id)?;
    let prepared = tx.prepare()?;
    prepared.commit()?;

    let mut read_id = persy.get::<String, PersyId>("index", &"key".to_string())?;
    if let Some(id) = read_id.next() {
        let value = persy.read("data", &id)?;
        assert_eq!(Some(rec.to_vec()), value);
        ////////println!(value);
    }
    Ok(())
}