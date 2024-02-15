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

/// Initializes the Liquidity Pool database. If the database file does not exist,
/// it creates a new Persy database file, initializes required segments, and commits the transaction.
/// If the database file already exists, it prints a message and does nothing.
///
/// # Returns
/// Returns a `Result` indicating success or an error.
/// - If successful, returns `Ok(())`.
/// - If an error occurs, returns `Err` with the corresponding error.
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

/// Creates a new Liquidity Pool in the database by inserting the serialized pool
/// into the "pools_found" segment and updating the index with the pool's UID.
///
/// # Arguments
/// - `pool`: A reference to the LiquidityPool to be inserted.
///
/// # Returns
/// Returns a `Result` indicating success or an error.
/// - If successful, returns `Ok(())`.
/// - If an error occurs, returns `Err` with the corresponding error
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

/// Reads a Liquidity Pool from the database using the provided UID.
///
/// # Arguments
/// - `uid`: The UID of the Liquidity Pool to be read.
///
/// # Returns
/// Returns a `Result` containing the deserialized LiquidityPool if found.
/// - If successful, returns `Ok(LiquidityPool)`.
/// - If the pool with the given UID is not found, returns `Err` with an error message.
/// - If any other error occurs, returns `Err` with the corresponding error.
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

    Ok(deserialized_lp)
}

/// Updates the Liquidity Pool with the specified UID in the database.
///
/// # Arguments
///
/// * `uid` - An integer representing the UID of the Liquidity Pool to be updated.
/// * `pool` - A reference to the LiquidityPool struct containing the updated data.
///
/// # Errors
///
/// Returns a Result indicating success or an error if the UID is not found or if any
/// underlying operation fails. The error type is a dynamic trait object (`Box<dyn std::error::Error>`).
///
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


/// Deletes the Liquidity Pool with the specified UID from the database.
///
/// # Arguments
///
/// * `uid` - An integer representing the UID of the Liquidity Pool to be deleted.
///
/// # Errors
///
/// Returns a Result indicating success or an error if the UID is not found or if any
/// underlying operation fails. The error type is a dynamic trait object (`Box<dyn std::error::Error>`).
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

/// Retrieves the count of Liquidity Pools stored in the database.
///
/// # Errors
///
/// Returns a Result containing the count of Liquidity Pools or an error if any
/// underlying operation fails. The error type is a dynamic trait object (`Box<dyn std::error::Error>`).
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

    Ok(count)
}