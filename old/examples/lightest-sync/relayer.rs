use celo_light_client::Header;

use hyper::client::{Client, HttpConnector};
use hyper::http::Request;
use hyper::Body;

use serde::de::DeserializeOwned;
use serde_json::json;

pub struct Relayer {
    client: Client<HttpConnector, Body>,
    uri: String,
}

impl Relayer {
    pub fn new(uri: String) -> Self {
        Self {
            client: Client::new(),
            uri,
        }
    }

    pub async fn get_block_header_by_number(&self, hex_num: &str) -> Result<Header, Box<dyn std::error::Error>> {
        let req = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [hex_num, true],
            "id": 1,
        });

        return self.fetch(req).await;
    }

    async fn fetch<'de, T: DeserializeOwned>(&self, body: serde_json::Value) -> Result<T, Box<dyn std::error::Error>> {
        let req = Request::builder()
            .method("POST")
            .uri(&self.uri)
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .expect("request builder");

        #[derive(Deserialize)]
        struct Container<T> {
            result: T,
        }

        let response = (&self.client).request(req).await?;
        let buf = hyper::body::to_bytes(response).await?;
        let container: Container<T> = serde_json::from_slice(&buf)?;

        Ok(container.result)
    }
}
