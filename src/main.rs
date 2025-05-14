mod api;
mod repository;
use aws_sdk_dynamodb::config;
use repository::ddb::{self, DDBRepository};
use api::task::{
    get_task
};

use actix_web::{web::Data, App, HttpServer, middleware::Logger};

#[ actix_web::main]
async fn main() -> std::io::Result<()>{
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();
    let config = aws_config::load_from_env().await.unwrap();
    HttpServer::new(|| {
        let ddb_repo : DDBRepository = DDBRepository::init("task".to_string(), config.clone());
        let ddb_data = Data::new(ddb_repo);
        let logger: Logger = Logger::default();
        let ddb_repo : DDBRepository = DDBRepository::init("task".to_string(), config.clone());
        let ddb_data = Data::new(ddb_repo);
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await


}
