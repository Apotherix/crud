use axum::{routing::get, Router};
use crud::{build_router, db::init_db};
// use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::registry()
    // .with(fmt::layer)
    // .with(EnvFilter::from_default_env())
    // .init();

    let pool = init_db().await?;

    let app = build_router().await;

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server listening on http://{addr}");
    axum_server::bind(addr)
    .serve(app.into_make_service())
    .await?;

    Ok(())
}