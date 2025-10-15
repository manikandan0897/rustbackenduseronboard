use axum::{
    Router,
};
use tokio::net::TcpListener;

pub mod app;
pub mod routes;
pub mod controllers;

#[tokio::main]
async fn main() {

   let app:Router = app::create_app();
   
    // run our app with hyper, listening globally on port 4000
    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    println!("🚀 Server running on http://0.0.0.0:4000");
    axum::serve(listener, app).await.unwrap();
}