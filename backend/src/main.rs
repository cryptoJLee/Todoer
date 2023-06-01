#[macro_use] extern crate diesel;

use actix_cors::Cors;
use actix_web::{App, HttpServer, HttpResponse, middleware::Logger};
use actix_service::Service;
use futures::future::{Either, ok};

mod jwt;
mod views;
mod to_do;
mod schema;
mod database;
mod models;
mod config;
mod counter;
mod json_serialization;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");

    let site_counter = counter::Counter {count: 0};
    site_counter.save().unwrap();

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        let app = App::new()
            .wrap_fn(|req, srv| {
                let passed: bool;

                let mut site_counter = counter::Counter::load().unwrap();
                site_counter.count += 1;
                println!("{:?}", &site_counter);
                site_counter.save().unwrap();
                if req.path().contains(&format!("/{}/", ALLOWED_VERSION)) {
                    passed = true;
                } else {
                    passed = false;
                }
                let end_result;
                if passed == true {
                    end_result = Either::Left(srv.call(req))
                } else {
                    let resp = HttpResponse::NotImplemented().body(
                        format!("only {} API is supported", ALLOWED_VERSION)
                    );
                    end_result = Either::Right(
                        ok(req.into_response(resp)
                            .map_into_boxed_body()
                        )
                    );
                }
                async {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"));
        app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}