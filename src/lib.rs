use std::fs::create_dir;
use std::path::Path;

use heed3::{types::*, Env, RoTxn, RwTxn, WithTls};
use heed3::{Database, EnvOpenOptions};

pub fn create_env() -> Env {
    let path = Path::new("db");
    let _ = create_dir(path);
    
    let env = unsafe { 
        EnvOpenOptions::new()
            .map_size(10 * 1024 * 1024 * 102) // Set map size to 10 MB
            .open(path)
            .expect("Failed to open DB") 
    };
    env
}

fn create_wtxn<'a>(env: &'a Env) -> RwTxn<'a> {
    let wtxn = env.write_txn().expect("Failed to create wtxn");
    wtxn
}

pub fn create_db(env: &Env) -> Database<Str, Str> {
    let mut wtxn = create_wtxn(env);
    let db: Database<Str, Str> = env.create_database(&mut wtxn, None).expect("Failed to create DB");
    wtxn.commit().expect("Failed to commit wtxn");
    db
}

pub fn create_rtxn<'a>(env: &'a Env) -> RoTxn<'a, WithTls> {
    let rtxn = env.read_txn().expect("Failed to create rtxn");
    rtxn
}

pub fn add_kvp(db: Database<Str, Str>, env: &Env, key: &str, value: &str) -> Result<(), heed3::Error> {
    let mut wtxn = create_wtxn(env);
    db.put(&mut wtxn, key, value)?;
    wtxn.commit()?;
    Ok(())
}

pub fn get_kvp<'a>(db: Database<Str, Str>, rtxn: &'a RoTxn, key: &str) -> Result<Option<&'a str>, heed3::Error> {
    let value = db.get(rtxn, key)?;
    Ok(value)
}
