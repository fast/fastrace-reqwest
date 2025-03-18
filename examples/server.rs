//! Example demonstrating how to use fastrace-poem to receive trace context in HTTP server requests.
//!
//! This example shows:
//! 1. Setting up a fastrace reporter.
//! 2. Creating a Poem HTTP server with FastraceMiddleware for trace context extraction.
//! 3. Handling requests while maintaining trace context.
//!
//! To run this example:
//! `cargo run --example server`
//! Then use the client example to send requests to it.

use fastrace::collector::Config;
use fastrace::collector::ConsoleReporter;
use fastrace_poem::FastraceMiddleware;
use poem::EndpointExt;
use poem::Request;
use poem::Response;
use poem::Route;
use poem::Server;
use poem::get;
use poem::handler;
use poem::listener::TcpListener;

/// A simple ping handler that responds with "pong".
/// The handler is annotated with the fastrace::trace attribute to create a span.
#[handler]
#[fastrace::trace]
fn ping(_req: &Request) -> Response {
    Response::builder().body("pong")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize fastrace with the console reporter.
    fastrace::set_reporter(ConsoleReporter, Config::default());

    // Create a route with the ping handler and add the FastraceMiddleware.
    // The middleware extracts trace context from incoming requests.
    let app = Route::new().at("/ping", get(ping)).with(FastraceMiddleware);

    // Start the server.
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await?;

    // Flush any remaining traces before the program exits.
    fastrace::flush();

    Ok(())
}
