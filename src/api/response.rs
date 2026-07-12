use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationResult {
    pub success: bool,
    pub latency_ms: f64,
    pub final_fidelity: f64,
    pub execution_path: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonteCarloStats {
    pub total_runs: usize,
    pub empirical_success_rate: f64,
    pub mean_latency_ms: f64,
    pub mean_fidelity: f64,
    pub aggregate_congestion_drops: usize,
    pub link_utilization_heatmap: HashMap<String, usize>,
}
