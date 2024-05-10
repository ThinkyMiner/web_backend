#![allow(unused)]

use std::net::SocketAddr;

use axum::{response::Html, routing::get, serve, Router};

#[tokio::main]
async fn main(){

    let routes_hello = Router::new().route(
        "/hello",
        get(|| async {Html("Hello <strong>World!</strong>")}),
    );

    // region: Start the server

    let addr = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("->> Listening on {:?} \n", addr);
    serve(addr, routes_hello.into_make_service()).await.unwrap();

    // endregion: Start the server

}