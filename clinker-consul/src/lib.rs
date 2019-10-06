use std::time::Duration;

use serde::Deserialize;

pub mod agent;
pub mod catalog;
pub mod errors;

use errors::*;

#[derive(Clone, Debug, Default)]
pub struct QueryOptions {
    pub datacenter: Option<String>,
    pub wait_index: Option<u64>,
    pub wait_time: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct QueryMeta {
    pub last_index: Option<u64>,
    pub request_time: Duration,
}

#[derive(Clone, Debug, Default)]
pub struct WriteOptions {
    pub datacenter: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WriteMeta {
    pub request_time: Duration,
}

use hyper::{client::HttpConnector, Body};
pub struct ConsulClient {
    http_client: hyper::Client<HttpConnector, Body>,
}

impl ConsulClient {
    pub fn new() -> ConsulClient {
        let http_client = hyper::Client::new();
        ConsulClient { http_client }
    }

    pub async fn datacenters(&self) -> Result<Vec<String>> {
        use futures_util::try_stream::TryStreamExt;

        let uri = "http://consul.service.int-us-central1.consul:8500/v1/catalog/datacenters"
            .parse()
            .unwrap();
        let http_response = self.http_client.get(uri).await.unwrap();
        let body = http_response.into_body().try_concat().await.unwrap();
        let datacenters = serde_json::from_slice(&body).unwrap();

        Ok(datacenters)
    }

    pub async fn services(&self) -> Result<Vec<catalog::CatalogService>> {
        use futures_util::try_stream::TryStreamExt;

        let uri = "http://consul.service.int-us-central1.consul:8500/v1/catalog/services"
            .parse()
            .unwrap();
        let http_response = self.http_client.get(uri).await.unwrap();
        let body = http_response.into_body().try_concat().await.unwrap();
        let services = serde_json::from_slice(&body).unwrap();

        Ok(services)
    }
}

struct Config {
    pub address: String,
    pub datacenter: Option<String>,
}
