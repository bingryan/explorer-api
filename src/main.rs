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

    let bind_address = &settings.server.bind_address;
    let state = AppState::new(&settings);

    let meili_client = state.meili_client;
    let meili_client_state = meili_client.is_healthy().await;
    if !meili_client_state {
        llog::error!("Could not ping meilisearch server to address {} with apikey: {}",
               &settings.meilisearch.host,
               &settings.meilisearch.apikey);
        std::process::exit(101);
    }

    HttpServer::new(move || {
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
            // .data(Data::new(state))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(route::values)
    })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run()
        .await
}