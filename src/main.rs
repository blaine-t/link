use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Responder};
use redb::{Database, Error, TableDefinition};

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("my_data");

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

fn db() -> Result<(), Error> {
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
    println!("DB written to and accessed");

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = db();
    HttpServer::new(|| App::new().service(hello).service(fs::Files::new("/", "./public")))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
