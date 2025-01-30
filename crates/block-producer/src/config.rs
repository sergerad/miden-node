use std::fmt::{Display, Formatter};

use miden_node_utils::config::{DEFAULT_BLOCK_PRODUCER_PORT, DEFAULT_STORE_PORT};
use serde::{Deserialize, Serialize};
use tonic::transport::Uri;

// Main config
// ================================================================================================

/// Block producer specific configuration
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlockProducerConfig {
    #[serde(with = "http_serde::uri")]
    pub endpoint: Uri,

    /// Store gRPC endpoint in the format `http://<host>[:<port>]`.
    #[serde(with = "http_serde::uri")]
    pub store_url: Uri,

    /// Enable or disable the verification of transaction proofs before they are accepted into the
    /// transaction queue.
    ///
    /// Disabling transaction proof verification will speed up transaction processing as proof
    /// verification may take ~15ms/proof. This is OK when all transactions are forwarded to the
    /// block producer from the RPC component as transaction proofs are also verified there.
    pub verify_tx_proofs: bool,
}

impl Display for BlockProducerConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{ endpoint: \"{}\", store_url: \"{}\" }}",
            self.endpoint, self.store_url
        ))
    }
}

impl Default for BlockProducerConfig {
    fn default() -> Self {
        Self {
            endpoint: Uri::try_from(format!("http://127.0.0.1:{DEFAULT_BLOCK_PRODUCER_PORT}"))
                .unwrap(),
            store_url: Uri::try_from(format!("http://127.0.0.1:{DEFAULT_STORE_PORT}")).unwrap(),
            verify_tx_proofs: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;

    use super::BlockProducerConfig;

    #[tokio::test]
    async fn default_block_producer_config() {
        // Default does not panic
        let config = BlockProducerConfig::default();
        // Default can bind
        let _listener = TcpListener::bind((
            config.endpoint.host().unwrap(),
            config.endpoint.port_u16().unwrap(),
        ))
        .await
        .unwrap();
    }
}
