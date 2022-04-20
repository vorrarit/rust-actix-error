use tracing_actix_web::TracingLogger;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_log::LogTracer;
use actix_web::{HttpServer, App};

mod routes;
mod services;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("rust_actix_error".into(), std::io::stdout);
    let subscriber = Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");
    
    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(routes::hello::hello)
            .service(routes::greet::greet)
            .service(routes::todo::todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}