use hyper::{Client, Request, Response, Body, Server, service::{make_service_fn, service_fn}, Uri};
use tower::ServiceBuilder;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use tracing::{info, error};
use crate::config::Config;

pub struct Proxy {
    config: Arc<Config>,
    counter: AtomicUsize,
}

impl Proxy {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            counter: AtomicUsize::new(0),
        }
    }

    pub async fn run(&self, addr: std::net::SocketAddr) -> anyhow::Result<()> {
        let client = Client::new();
        let config = self.config.clone();
        let counter = Arc::new(self.counter.clone());

        let make_svc = make_service_fn(move |_| {
            let client = client.clone();
            let config = config.clone();
            let counter = counter.clone();

            async move {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    let client = client.clone();
                    let config = config.clone();
                    let backend = pick_backend(&config, &counter);
                    proxy_request(client, req, backend)
                }))
            }
        });

        info!("Proxy listening on http://{}", addr);
        Server::bind(&addr).serve(make_svc).await?;
        Ok(())
    }
}

fn pick_backend(config: &Config, counter: &AtomicUsize) -> String {
    let idx = counter.fetch_add(1, Ordering::Relaxed) % config.backends.len();
    config.backends[idx].clone()
}

async fn proxy_request(client: Client<hyper::client::HttpConnector>, mut req: Request<Body>, backend: String) -> Result<Response<Body>, hyper::Error> {
    let uri_string = format!("{}{}", backend, req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("/"));
    let uri = uri_string.parse::<Uri>().unwrap();

    *req.uri_mut() = uri;

    info!("Forwarding request to: {}", req.uri());

    match client.request(req).await {
        Ok(resp) => Ok(resp),
        Err(e) => {
            error!("Request failed: {:?}", e);
            let mut response = Response::new(Body::from("Upstream error"));
            *response.status_mut() = hyper::StatusCode::BAD_GATEWAY;
            Ok(response)
        }
    }
}
