# fastrace-reqwest

[![Crates.io](https://img.shields.io/crates/v/fastrace-reqwest.svg?style=flat-square&logo=rust)](https://crates.io/crates/fastrace-reqwest)
[![Documentation](https://img.shields.io/docsrs/fastrace-reqwest?style=flat-square&logo=rust)](https://docs.rs/fastrace-reqwest/)
[![MSRV 1.83.0](https://img.shields.io/badge/MSRV-1.83.0-green?style=flat-square&logo=rust)](https://www.whatrustisit.com)
[![CI Status](https://img.shields.io/github/actions/workflow/status/fast/fastrace-reqwest/ci.yml?style=flat-square&logo=github)](https://github.com/fast/fastrace-reqwest/actions)
[![License](https://img.shields.io/crates/l/fastrace-reqwest?style=flat-square)](https://github.com/fast/fastrace-reqwest/blob/main/LICENSE)

Distributed tracing integration for [reqwest](https://crates.io/crates/reqwest) HTTP client requests with [fastrace](https://crates.io/crates/fastrace).

## Overview

`fastrace-reqwest` provides automatic trace context propagation for HTTP requests made with the `reqwest` client. It works seamlessly with the `fastrace` library to extract and inject trace context into outgoing requests.

## What is Context Propagation?

Context propagation is a fundamental concept in distributed tracing that enables the correlation of operations spanning multiple services. When a request moves from one service to another, trace context information needs to be passed along, ensuring that all operations are recorded as part of the same trace.

`fastrace-reqwest` implements the [W3C Trace Context](https://www.w3.org/TR/trace-context/) standard for propagating trace information between services. This ensures compatibility with other tracing systems that follow the same standard.

## Features

- 🔄 **Automatic Context Propagation**: Automatically inject trace context into outgoing HTTP requests.
- 🌉 **Seamless Integration**: Works seamlessly with the `fastrace` library for complete distributed tracing.
- 📊 **Full Compatibility**: Works with fastrace's collection and reporting capabilities.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
fastrace = "0.7"
fastrace-reqwest = "0.1"
```

## Usage

### HTTP Client

```rust
use fastrace::prelude::*;
use fastrace_reqwest::traceparent_headers;
use reqwest::Client;

#[fastrace::trace]
async fn send_request() {
    let client = Client::new();
    
    // Add trace context headers to your request.
    let response = client
        .get("https://api.example.com/data")
        .headers(traceparent_headers())
        .send()
        .await
        .unwrap();
        
    // Process response...
}
```

## How It Works

`fastrace-reqwest` enables automatic propagation of trace context between services by:

1. **Extracting** the current trace context from the calling function.
2. **Formatting** it according to the W3C Trace Context specification.
3. **Injecting** it into outgoing HTTP requests as headers
4. Working seamlessly with the companion `fastrace-poem` library (or other compatible frameworks) to extract the context on the receiving end.

### Complete Example

Check out the [examples directory](https://github.com/fast/fastrace-reqwest/tree/main/examples) for complete working examples showing:

- `client.rs` - How to send requests with trace context
- `server.rs` - How to receive and process trace context using `fastrace-poem`

To run the examples:

```bash
# First start the server
cargo run --example server

# Then in another terminal, run the client
cargo run --example client
```

## How It Works

1. When making an HTTP request, `traceparent_headers()` checks for a current span context.
2. If found, it encodes the context as a `traceparent` HTTP header following the W3C standard.
3. The receiving service extracts this header and continues the trace.

## License

This project is licensed under the [Apache-2.0](./LICENSE) license.
