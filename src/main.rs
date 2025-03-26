use redb::{Database, Error, TableDefinition};

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("my_data");

fn main() -> Result<(), Error> {
    let db = Database::create("db.redb")?;
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE)?;
        table.insert("my_key", "123")?;
    }
    write_txn.commit()?;

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;
    assert_eq!(table.get("my_key")?.unwrap().value(), "123");

    Ok(())
}
