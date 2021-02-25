use actix_web::{middleware, App, HttpServer};
use wy_music_api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut bind_address = BIND_ADDRESS.to_string();

    if let Ok(config) = WrapperConfig::default() {
        bind_address = config.active().get_bind_address();
    } else {
        if let Ok(config) = WrapperConfig::active_default() {
            bind_address = config.get_bind_address();
        }
    }

    #[cfg(debug_assertions)]
    println!("Debugging enabled, bind_address is : {}", bind_address);

    #[cfg(not(debug_assertions))]
    println!("bind_address is : {}", bind_address);

    //crypto_test();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(config)
    })
    .bind(&bind_address)?
    .run()
    .await
}
