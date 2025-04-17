pub mod config;
pub mod item;

use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
    http::Method
};

use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a single route
    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(
                    [
                        Method::GET,
                        Method::POST,
                        Method::PATCH,
                        Method::PUT,
                        Method::DELETE
                    ]
                )
        
        )
        .route("/", get(|| async { "OK" }));

    // run it with hyper on localhost:3000
    let addr= SocketAddr::from(([0,0,0,0], 3000));
    println!("Server is starting on: {:?}", addr);


    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
