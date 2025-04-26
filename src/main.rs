use actix_files as fs;
use actix_web::{get, post, web, App, HttpServer, Responder};
use redb::{Database, Error, TableDefinition};

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("my_data");

#[get("/{name}")]
async fn serve_link(key: web::Path<String>, db: web::Data<Database>) -> impl Responder {
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(TABLE).unwrap();
    format!("Hello {}!", table.get(key.as_str()).unwrap().unwrap().value())
}

#[post("/{name}")]
async fn create_link(name: web::Path<String>, db: web::Data<Database>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Database::create("db.redb").unwrap());
    HttpServer::new(move || App::new().app_data(db.clone()).service(serve_link).service(create_link).service(fs::Files::new("/", "./public")))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
