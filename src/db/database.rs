use persy::{Config, Persy, PersyId, Transaction, ValueMode};
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json;

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
        //tx.create_index::<String, PersyId>("index", ValueMode::Replace)?;
        let prepared = tx.prepare()?;
        prepared.commit()?;
    }
    let mut tx = persy.begin()?;
    let rec = "KUK".as_bytes();
    println!("{:?}", std::str::from_utf8(rec));
    let id = tx.insert("data", rec)?;

    println!("The ID is: {:?}", id);

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

#[derive(Serialize, Deserialize)]
pub struct LiquidityPool {
    uid: i32,
    chain: String,
    time_of_creation: i32,
    token_a_name: String,
    token_a_symbol: String,
    token_a_amount: i32,
    token_a_price: i32,
    token_b_name: String,
    token_b_symbol: String,
    token_b_amount: i32,
    token_b_price: i32,
}

pub fn init_lp_database() -> Result<(), Box<dyn std::error::Error>> {
    let file_path: &str = "./data/db.pers";
    let create_segment: bool;
    if !fs::metadata(&file_path).is_ok() {
        let _ = Persy::create(&file_path)?;
        create_segment = true;
    } else {
        println!("DB already exists!");
        create_segment = false;
    }
    let persy: Persy = Persy::open(&file_path, Config::new())?;
    if create_segment {
        let mut tx: Transaction= persy.begin()?;
        tx.create_segment("pools_found")?;
        tx.create_index::<String, PersyId>("index", ValueMode::Replace)?;
        let prepared = tx.prepare()?;
        prepared.commit()?;
    }

    Ok(())
}

pub fn create_lp(pool: &LiquidityPool) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: &str = "./data/db.pers";
    if !fs::metadata(&file_path).is_ok() {
        println!("No database found");
    }
    
    let persy: Persy = Persy::open(&file_path, Config::new())?;   

    let mut tx = persy.begin()?;
    let serialized = serde_json::to_vec(pool)?;
    let serialized_bytes = serialized.as_slice();
    let id = tx.insert("pools_found", serialized_bytes)?;

    println!("The ID is: {:?}", id);

    tx.put::<String, PersyId>("index", pool.uid.to_string(), id)?;

    let prepared = tx.prepare()?;
    prepared.commit()?;

    Ok(())
}

pub fn read_lp(uid: i32) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: &str = "./data/db.pers";
    if !fs::metadata(&file_path).is_ok() {
        println!("No database found");
    }
    
    let persy: Persy = Persy::open(&file_path, Config::new())?;

    let mut read_id = persy.get::<String, PersyId>("index", &uid.to_string())?;

    //let id = read_id.next();

    let id = match read_id.next() {
        Some(id) => id,
        None => {
            // Handle the case where no ID is found, e.g., return an error or a default value
            return Err("Pool not found".into());
        }
    };

    let value = persy.read("pools_found", &id)?;

    println!("{:?}", value);

    Ok(())
}

//fn update_lp() {}
//fn delete_lp() {}