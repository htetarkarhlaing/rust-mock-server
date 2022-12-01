use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Product {
    name: String,
    image: String,
}

async fn get_product() -> impl Responder {
    let mut product_list: Vec<Product> = Vec::new();
    product_list.push(Product {
        name: "Spy Red".to_string(),
        image: "http://localhost:8080/images/two-cola.jpg".to_string(),
    });
    product_list.push(Product {
        name: "Spy Black".to_string(),
        image: "http://localhost:8080/images/two-cola.jpg".to_string(),
    });

    return web::Json(product_list);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/images", "public/images").show_files_listing())
            .service(web::resource("/products").route(web::get().to(get_product)))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
