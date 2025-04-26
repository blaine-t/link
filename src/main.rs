use actix_files as fs;
use actix_web::{get, post, web::{self, Redirect}, App, HttpResponse, HttpServer, Responder};
use redb::{Database, Error, TableDefinition};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormBody {
    url: String
}

const TABLE: TableDefinition<String, String> = TableDefinition::new("my_data");

#[get("/{name}")]
async fn serve_link(key: web::Path<String>, db: web::Data<Database>) -> impl Responder {
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(TABLE).unwrap();
    let result = table.get(key.into_inner()).unwrap().unwrap().value();
    
    Redirect::to(result).permanent()
}

#[post("/{name}")]
async fn create_link(key: web::Path<String>, web::Form(form): web::Form<FormBody>, db: web::Data<Database>) -> impl Responder {
    let value = &form.url;
    
    let write_txn = db.begin_write().unwrap();
    {
        let mut table = write_txn.open_table(TABLE).unwrap();
        table.insert(key.into_inner(), value).unwrap();
    }
    write_txn.commit().unwrap();

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Database::create("db.redb").unwrap());
    HttpServer::new(move || App::new().app_data(db.clone()).service(serve_link).service(create_link).service(fs::Files::new("/", "./public")))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
