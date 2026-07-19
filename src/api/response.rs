use std::collections::HashMap;

// ============================================================================
// Core Simulation Types
// ============================================================================

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

// ============================================================================
// Higher-Level Protocol Result Types
// ============================================================================

/// Aggregated statistics for QKD protocol runs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QKDStats {
    pub total_runs: usize,
    /// Fraction of runs that produced a valid secret key.
    pub success_rate: f64,
    /// Average secret key length (bits) across successful runs.
    pub mean_key_length_bits: f64,
    /// Average key/bit efficiency ratio across successful runs.
    pub mean_efficiency: f64,
    /// Average QBER observed in successful runs.
    pub mean_qber: f64,
}

/// Result of a single QKD protocol execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QKDResult {
    pub success: bool,
    /// Final secret key length in bits (after sifting and privacy amplification).
    pub secret_key_length_bits: usize,
    /// Ratio of useful key bits to total generation attempts.
    pub efficiency_rate: f64,
    /// Quantum bit error rate observed during the protocol.
    pub qber: f64,
    /// Total time for the QKD session in ms.
    pub latency_ms: f64,
    /// Nodes visited during entanglement link establishment.
    pub execution_path: Vec<String>,
    /// Number of entangled pairs successfully established.
    pub rounds_completed: usize,
    /// Number of failed entanglement generation attempts.
    pub rounds_failed: usize,
}

/// Outcome of a single teleportation protocol execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeleportationOutcome {
    pub success: bool,
    /// Fidelity of the teleported quantum state at Bob's end.
    pub teleportation_fidelity: f64,
    /// Fidelity of the resource Bell pair used for teleportation.
    pub resource_entanglement_fidelity: f64,
    /// Total latency (entanglement + classical relay) in ms.
    pub latency_ms: f64,
    /// Nodes involved in the teleportation chain.
    pub path: Vec<String>,
    /// Classical bits transferred (2 per teleported qubit).
    pub classical_bits_transferred: usize,
}

/// Aggregated statistics for teleportation protocol runs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeleportationStats {
    pub total_runs: usize,
    /// Fraction of runs where the resource entanglement was established.
    pub success_rate: f64,
    /// Average teleportation fidelity across successful runs.
    pub mean_teleportation_fidelity: f64,
    /// Standard deviation of teleportation fidelity (0 if no successful runs).
    pub teleportation_fidelity_stddev: f64,
    /// Average total latency across successful runs in ms.
    pub mean_latency_ms: f64,
}

/// Outcome for an individual party in a distributed protocol.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PartyOutcome {
    pub node_id: String,
    /// Whether this party's local measurements succeeded.
    pub successful_measurement: bool,
    /// Local measurement fidelity (affected by link quality and correlation strength).
    pub local_fidelity: f64,
}

/// Result of a single distributed computing protocol execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DistributedComputingResult {
    pub success: bool,
    /// Overall computation fidelity (weighted product of per-link and per-party fidelities).
    pub computation_fidelity: f64,
    /// Per-party measurement outcomes.
    pub party_results: Vec<PartyOutcome>,
    /// Entanglement links consumed during the protocol.
    pub resource_links_used: Vec<String>,
    /// Total time for the distributed computation in ms.
    pub total_latency_ms: f64,
    /// Classical coordination overhead across all relay hops in ms.
    pub coordination_overhead_ms: f64,
}

/// Aggregated statistics for distributed computing protocol runs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DistributedComputingStats {
    pub total_runs: usize,
    /// Fraction of runs where the overall protocol succeeded.
    pub success_rate: f64,
    /// Average computation fidelity across successful runs.
    pub mean_computation_fidelity: f64,
    /// Average fraction of parties with successful measurements (0.0-1.0).
    pub mean_party_success_rate: f64,
}
