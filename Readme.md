Features:

Async TCP/HTTP proxy
Simple load balancing (round-robin or least-connections)
Metrics via Prometheus
Graceful shutdown & connection handling
Built with tokio, hyper, tower, tracing

# Rust Async Reverse Proxy

Simple async reverse proxy built with:
- Tokio
- Hyper
- Tower
- Tracing

## Run

Start two upstream servers on ports 8081 and 8082 (can use `python3 -m http.server 8081` etc.)

Then run the proxy:

```bash
cargo run
