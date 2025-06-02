mod booking;
mod routes;

use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listening 127.0.0.1:8080");

    HttpServer::new(|| {
            App::new()
                .service(routes::free)
                .service(routes::book)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


