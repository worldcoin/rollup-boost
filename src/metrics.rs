use metrics::{Counter, Histogram, histogram};
use metrics_derive::Metrics;

#[derive(Metrics)]
#[metrics(scope = "rpc")]
pub struct ServerMetrics {
    #[metric(describe = "Count of forkchoice_updated_v3 calls proxied to the builder")]
    pub fcu_count: Counter,

    #[metric(describe = "Count of new_payload_v3 calls proxied to the builder")]
    pub new_payload_count: Counter,

    #[metric(describe = "Count of get_payload_v3 calls proxied to the builder")]
    pub get_payload_count: Counter,

    #[metric(describe = "World Chain builder response time in seconds")]
    pub builder_response_time: Histogram,
}

impl ServerMetrics {
    pub fn new() -> Self {
        Self {
            builder_response_time: histogram!("builder_response_time"),
            ..Default::default()
        }
    }
}
