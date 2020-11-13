use actix_web::{
    middleware::Logger,
    web::{delete, get, put, resource, scope},
    App, HttpServer,
};
mod kv;
mod kv_handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db: kv::SledDb = kv::KVStore::init("/tmp/sled");

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new().data(db.clone()).wrap(Logger::default()).service(
            scope("/api").service(
                resource("/{key}")
                    .route(get().to(kv_handler::get))
                    .route(put().to(kv_handler::put))
                    .route(delete().to(kv_handler::delete)),
            ),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
