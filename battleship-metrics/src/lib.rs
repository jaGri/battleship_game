use async_trait::async_trait;
use tracing::info;

#[async_trait]
pub trait Metrics {
    async fn event(&self, name: &str);
}

pub struct TracingMetrics;

#[async_trait]
impl Metrics for TracingMetrics {
    async fn event(&self, name: &str) {
        info!("metric: {}", name);
    }
}

