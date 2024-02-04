use persy::{Config, Persy, PersyId, Transaction, ValueMode};
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidityPool {
    pub uid: i32,
    pub chain: String,
    pub time_of_creation: i32,
    pub token_a_name: String,
    pub token_a_symbol: String,
    pub token_a_amount: i32,
    pub token_a_price: i32,
    pub token_b_name: String,
    pub token_b_symbol: String,
    pub token_b_amount: i32,
    pub token_b_price: i32,
}

pub fn init_lp_database() -> Result<(), Box<dyn std::error::Error>> {
    let file_path: &str = "./data/db.persy";
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
    let file_path: &str = "./data/db.persy";
    if !fs::metadata(&file_path).is_ok() {
        println!("No database found");
    }
    
    let persy: Persy = Persy::open(&file_path, Config::new())?;   

    let mut tx = persy.begin()?;
    let serialized = serde_json::to_vec(pool)?;
    let serialized_bytes = serialized.as_slice();
    let id = tx.insert("pools_found", serialized_bytes)?;

    tx.put::<String, PersyId>("index", pool.uid.to_string(), id)?;

    let prepared = tx.prepare()?;
    prepared.commit()?;

    Ok(())
}

pub fn read_lp(uid: i32) -> Result<LiquidityPool, Box<dyn std::error::Error>> {
    let file_path: &str = "./data/db.persy";
    if !fs::metadata(&file_path).is_ok() {
        println!("No database found");
    }
    
    let persy: Persy = Persy::open(&file_path, Config::new())?;

    let mut read_id = persy.get::<String, PersyId>("index", &uid.to_string())?;

    let id = match read_id.next() {
        Some(id) => id,
        None => {
            return Err("Pool not found".into());
        }
    };

    let value = persy.read("pools_found", &id)?;

    let bytes = value.unwrap();

    let deserialized_lp: LiquidityPool = serde_json::from_slice(&bytes)?;

    // println!("Deserialized value: {:?}", deserialized_lp);

    Ok(deserialized_lp)
}

pub fn update_lp(uid: i32, pool: &LiquidityPool) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: &str = "./data/db.persy";
    if !fs::metadata(&file_path).is_ok() {
        println!("No database found");
    }
    
    let persy: Persy = Persy::open(&file_path, Config::new())?;

    let mut tx = persy.begin()?;
    
    let mut read_id = persy.get::<String, PersyId>("index", &uid.to_string())?;

    let id = match read_id.next() {
        Some(id) => id,
        None => {
            return Err("Pool not found".into());
        }
    };
    
    let serialized = serde_json::to_vec(pool)?;
    let serialized_bytes = serialized.as_slice();

    tx.update("pools_found", &id, &serialized_bytes)?;

    let prepared = tx.prepare()?;
    prepared.commit()?;

    Ok(())
}

pub fn delete_lp(uid: i32) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: &str = "./data/db.persy";
    if !fs::metadata(&file_path).is_ok() {
        println!("No database found");
    }
    
    let persy: Persy = Persy::open(&file_path, Config::new())?;

    let mut tx: Transaction = persy.begin()?;

    let mut read_id = persy.get::<String, PersyId>("index", &uid.to_string())?;

    let id = match read_id.next() {
        Some(id) => id,
        None => {
            return Err("Pool not found".into());
        }
    };

    tx.delete("pools_found", &id)?;

    let prepared = tx.prepare()?;
    prepared.commit()?;

    Ok(())
}

pub fn get_lp_count() -> Result<i32, Box<dyn std::error::Error>> {
    let file_path: &str = "./data/db.persy";
    if !fs::metadata(&file_path).is_ok() {
        println!("No database found");
    }
    
    let persy: Persy = Persy::open(&file_path, Config::new())?;

    let mut count = 0;
    for (_,_) in persy.scan("pools_found")? {
        //println!("record size:{}",content.len());
        count+=1;
    }

    //println!("Pools found: {:?}", count);

    Ok(count)
}