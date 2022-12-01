use actix_cors::Cors;
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
        name: "SPY Black".to_string(),
        image: "http://192.168.100.22:8080/images/two-cola.jpg".to_string(),
    });
    product_list.push(Product {
        name: "SPY Classic".to_string(),
        image: "http://192.168.100.22:8080/images/two-cola.jpg".to_string(),
    });
    product_list.push(Product {
        name: "SPY Red".to_string(),
        image: "http://192.168.100.22:8080/images/two-cola.jpg".to_string(),
    });
    product_list.push(Product {
        name: "SPY Sweet Kiss".to_string(),
        image: "http://192.168.100.22:8080/images/two-cola.jpg".to_string(),
    });
    product_list.push(Product {
        name: "SPY Kamikaze".to_string(),
        image: "http://192.168.100.22:8080/images/two-cola.jpg".to_string(),
    });
    product_list.push(Product {
        name: "SPY Mai Tai".to_string(),
        image: "http://192.168.100.22:8080/images/two-cola.jpg".to_string(),
    });
    product_list.push(Product {
        name: "SPY Butterfly Kiss".to_string(),
        image: "http://192.168.100.22:8080/images/two-cola.jpg".to_string(),
    });

    return web::Json(product_list);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://0.0.0.0:8088");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .service(Files::new("/images", "public/images").show_files_listing())
            .service(web::resource("/products").route(web::get().to(get_product)))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}
