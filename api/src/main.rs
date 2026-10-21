use std::env;

#[tokio::main]
async fn main() {
    let app = api::app::router(api::app::pool());

    // LISTEN wins so a second instance can run beside the first; PORT alone
    // keeps the usual behaviour.
    let listen = env::var("LISTEN").unwrap_or_else(|_| {
        let port = env::var("PORT").unwrap_or_else(|_| "7070".to_string());

        format!("0.0.0.0:{port}")
    });

    let listener = tokio::net::TcpListener::bind(&listen)
        .await
        .unwrap_or_else(|_| panic!("failed to bind {listen}"));

    println!("listening on {}", listener.local_addr().expect("no addr"));

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("failed to serve");
}

async fn shutdown_signal() {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install SIGTERM handler");

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = terminate.recv() => {}
    }
}
