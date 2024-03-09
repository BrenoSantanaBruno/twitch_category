mod api;

use api::fetch_and_print_streams;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::twitch_streams)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    fetch_and_print_streams().await.unwrap(); // Removendo o operador '?'

    Ok(())
}
