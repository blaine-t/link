use std::fs::create_dir;
use std::path::Path;

use heed3::{byteorder, types::*};
use heed3::{Database, EnvOpenOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("db");
    let _ = create_dir(path);
    let env = unsafe { EnvOpenOptions::new().open(path)? };

    // we will open the default unnamed database
    let mut wtxn = env.write_txn()?;
    let db: Database<Str, Str> = env.create_database(&mut wtxn, None)?;

    db.put(&mut wtxn, "help", "help")?;
    wtxn.commit()?;

    // opening a read transaction
    // to check if those values are now available
    let mut rtxn = env.read_txn()?;

    println!("{:?}", db.get(&rtxn, "help")?);
    Ok(())
}
