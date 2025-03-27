use fastrace::collector::Config;
use fastrace::collector::ConsoleReporter;

#[tokio::main]
async fn main() {
    // Initialize fastrace with the console reporter.
    fastrace::set_reporter(ConsoleReporter, Config::default());

    let app = axum::Router::new()
        .route("/ping", axum::routing::get(ping))
        // Add a the FastraceLayer to routes.
        // The layer extracts trace context from incoming requests.
        .layer(fastrace_axum::FastraceLayer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[fastrace::trace]
async fn ping() -> &'static str {
    "pong"
}
