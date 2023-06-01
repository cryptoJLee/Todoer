#[macro_use] extern crate diesel;

use actix_cors::Cors;
use actix_web::{App, HttpServer, HttpResponse};
use actix_service::Service;
use futures::future::{Either, ok};

mod jwt;
mod views;
mod to_do;
mod schema;
mod database;
mod models;
mod config;
mod json_serialization;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{}-{}", req.method(), req.uri());
                let passed: bool;
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
            .wrap(cors);
        app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}