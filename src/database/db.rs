use sled;

pub fn main() -> sled::Result<()> {
    // this directory will be created if it does not exist
    let path = "data/db";

    // works like std::fs::open
    let db = sled::open(path)?;

    // key and value types can be `Vec<u8>`, `[u8]`, or `str`.
    let key = "my key";

    // `generate_id`
    let value = db.generate_id()?.to_be_bytes();

    dbg!(
        db.insert(key, &value)?, // as in BTreeMap::insert
        db.get(key)?,            // as in BTreeMap::get
        db.remove(key)?,         // as in BTreeMap::remove
    );

    Ok(())
}
