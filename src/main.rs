use std::net::SocketAddr;

mod controllers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // tracing log
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // run
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes::routes().into_make_service())
        .await
        .unwrap();
}
