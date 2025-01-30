use std::fmt::{Display, Formatter};

use miden_node_utils::config::{
    DEFAULT_BLOCK_PRODUCER_PORT, DEFAULT_NODE_RPC_PORT, DEFAULT_STORE_PORT,
};
use serde::{Deserialize, Serialize};
use tonic::transport::Uri;

// Main config
// ================================================================================================

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RpcConfig {
    #[serde(with = "http_serde::uri")]
    pub endpoint: Uri,
    /// Store gRPC endpoint in the format `http://<host>[:<port>]`.
    #[serde(with = "http_serde::uri")]
    pub store_url: Uri,
    /// Block producer gRPC endpoint in the format `http://<host>[:<port>]`.
    #[serde(with = "http_serde::uri")]
    pub block_producer_url: Uri,
}

impl RpcConfig {
    pub fn endpoint_url(&self) -> String {
        self.endpoint.to_string()
    }
}

impl Display for RpcConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{ endpoint: \"{}\", store_url: \"{}\", block_producer_url: \"{}\" }}",
            self.endpoint, self.store_url, self.block_producer_url
        ))
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            endpoint: Uri::try_from(format!("http://0.0.0.0:{DEFAULT_NODE_RPC_PORT}")).unwrap(),
            store_url: Uri::try_from(format!("http://127.0.0.1:{DEFAULT_STORE_PORT}")).unwrap(),
            block_producer_url: Uri::try_from(format!(
                "http://127.0.0.1:{DEFAULT_BLOCK_PRODUCER_PORT}"
            ))
            .unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;

    use super::RpcConfig;

    #[tokio::test]
    async fn default_rpc_config() {
        // Default does not panic
        let config = RpcConfig::default();
        // Default can bind
        let _listener = TcpListener::bind((
            config.endpoint.host().unwrap(),
            config.endpoint.port_u16().unwrap(),
        ))
        .await
        .unwrap();
    }
}
