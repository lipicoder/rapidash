//! Rpc between client scheduler or executor
use std::time::Duration;
use tonic::codegen::StdError;
use tonic::transport::{Channel, Error};

pub async fn create_client_conn<D>(dst: D) -> Result<Channel, Error>
where
    D: TryInto<tonic::transport::Endpoint>,
    D::Error: Into<StdError>,
{
    let endpoint = tonic::transport::Endpoint::new(dst)?
        .connect_timeout(Duration::from_secs(20))
        .timeout(Duration::from_secs(20))
        // Disable Nagle's Algorithm since we don't want packets to wait
        .tcp_nodelay(true)
        .tcp_keepalive(Some(Duration::from_secs(3600)))
        .http2_keep_alive_interval(Duration::from_secs(300))
        .keep_alive_timeout(Duration::from_secs(20))
        .keep_alive_while_idle(true);
    endpoint.connect().await
}
