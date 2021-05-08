#![feature(backtrace)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde_derive;

use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    middleware::Logger,
    web::Data,
    web,
    App, HttpRequest,
    HttpServer,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
};
use actix_cors::Cors;
use std::env;
use crate::config::{AppState, SECRET, Settings, ExplorerLog, CONFIG_FILE};

mod config;
mod route;
mod db;
mod log;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let config_file = env::current_dir()?.join(CONFIG_FILE);

    let settings = Settings::build(config_file).unwrap();


    ExplorerLog::init(&settings);

    let bind_address = env::var("BIND_ADDRESS").unwrap_or(settings.server.bind_address);


    HttpServer::new(move || {
        let state = AppState {
            secret: Vec::from(SECRET),
        };
        let cors = match env::var("FRONTEND_ORIGIN").ok() {
            Some(ref origin) => Cors::default()
                .allowed_origin(origin)
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
            None => Cors::default()
                .allowed_origin("*")
                .send_wildcard()
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
        };
        App::new()
            .data(Data::new(state))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(route::values)
    })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run()
        .await
}