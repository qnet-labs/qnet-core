use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct NetworkTelemetry {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub cumulative_queue_delay_ms: f64,
    pub link_firings: HashMap<String, usize>,
}

impl NetworkTelemetry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_link_firing(&mut self, link_id: &str) {
        *self.link_firings.entry(link_id.to_string()).or_insert(0) += 1;
    }

    pub fn finalize(&mut self) {
        // Analytical hooks for tracking trace state mutations can be added here
    }
}