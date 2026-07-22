// Entanglement-based Quantum State Teleportation Protocol Implementation
//
// Simulates teleporting a quantum state across pre-established entanglement links:
// 1. Establish resource Bell pair between source (Alice) and target (Bob)
// 2. Simulate Bell-state measurement at the source
// 3. Compute resulting fidelity at the target based on resource entanglement quality
// 4. Account for classical communication latency through intermediate relays

use crate::api::request::EntanglementRequest;
use crate::api::response::{TeleportationOutcome, TeleportationStats};
use crate::config::PhysicalConfig;
use crate::routing::strategy::StrategyType;
use crate::QNetEngine;

// ============================================================================
// Request Type
// ============================================================================

/// Parameters for a teleportation protocol run.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeleportationParameters {
    /// Alice's node (source of the quantum state to teleport).
    pub source_node: String,
    /// Bob's node (target receiving the teleported state).
    pub target_node: String,
    /// Fidelity of the quantum state being teleported (0.0-1.0).
    #[serde(default = "default_state_fidelity")]
    pub state_fidelity: f64,
    /// Latency budget for classical communication channel (ms).
    #[serde(default = "default_classical_bandwidth")]
    pub classical_bandwidth_ms: f64,
    /// Optional intermediate repeater nodes for multi-hop teleportation.
    #[serde(default)]
    pub relay_nodes: Vec<String>,
}

fn default_state_fidelity() -> f64 {
    0.95
}
fn default_classical_bandwidth() -> f64 {
    100.0
}

// ============================================================================
// Protocol Implementation
// ============================================================================

/// Core protocol struct with static methods — matches the PurificationEngine pattern.
pub struct TeleportationProtocol;

impl TeleportationProtocol {
    /// Execute quantum state teleportation between source and target nodes.
    ///
    /// Uses `engine.request_entanglement()` to establish the resource Bell pair, then computes
    /// the resulting teleported state fidelity based on:
    ///   teleportation_fidelity = source_state_fidelity * resource_entanglement_fidelity
    ///
    /// Classical relay latency is added via path distance / speed_of_light_in_fiber.
    pub fn execute(engine: &QNetEngine, params: TeleportationParameters) -> TeleportationOutcome {
        // Phase 1: Establish resource entanglement between source and target
        let success = engine.network.is_some();

        let (resource_fidelity, latency_ms, _path) = if success {
            let request = EntanglementRequest {
                from_node: params.source_node.clone(),
                to: params.target_node.clone(),
                fidelity_target: params.state_fidelity,
                max_latency_ms: params.classical_bandwidth_ms,
                strategy: Some(StrategyType::HighestFidelity),
            };

            let link_result = engine.request_entanglement(request);
            (
                link_result.final_fidelity,
                link_result.latency_ms,
                link_result.execution_path.clone(),
            )
        } else {
            (0.0, 0.0, Vec::new())
        };

        // Phase 2: Compute teleportation fidelity
        // Teleportation fidelity = state preparation fidelity * resource Bell pair fidelity
        let teleported_fidelity = if resource_fidelity > 0.0 {
            params.state_fidelity * resource_fidelity
        } else {
            0.0
        };

        // Phase 3: Account for classical relay through intermediate nodes
        let speed_of_light_km_ms = PhysicalConfig::default().speed_of_light_in_fiber_km_ms;
        let mut total_latency = latency_ms;

        let mut full_path = vec![params.source_node.clone()];
        for relay in &params.relay_nodes {
            full_path.push(relay.clone());
        }
        if success {
            full_path.push(params.target_node.clone());
        }

        if !params.relay_nodes.is_empty() && success {
            // Estimate link distances from topology (use average 50km/hop as default)
            let relay_latency = params.relay_nodes.len() as f64 * 50.0 / speed_of_light_km_ms;
            total_latency += relay_latency;
        }

        TeleportationOutcome {
            success: resource_fidelity > 0.5,
            teleportation_fidelity: teleported_fidelity.min(0.999),
            resource_entanglement_fidelity: resource_fidelity,
            latency_ms: total_latency,
            path: full_path,
            classical_bits_transferred: 2, // Standard teleportation sends 2 classical bits per qubit
        }
    }

    /// Run multiple independent teleportation sessions and aggregate statistics.
    pub fn execute_ensemble(
        engine: &QNetEngine,
        params: TeleportationParameters,
        runs: usize,
    ) -> TeleportationStats {
        let mut success_count = 0usize;
        let mut total_fidelity = 0.0f64;
        let mut total_latency = 0.0f64;
        let mut sq_sum = 0.0f64;

        for _ in 0..runs {
            let result = TeleportationProtocol::execute(engine, params.clone());
            if result.success {
                success_count += 1;
                total_fidelity += result.teleportation_fidelity;
                total_latency += result.latency_ms;
                sq_sum += result.teleportation_fidelity * result.teleportation_fidelity;
            }
        }

        let n = if success_count > 0 {
            success_count as f64
        } else {
            1.0
        };
        let mean_fidelity = total_fidelity / n;
        let variance = sq_sum / n - mean_fidelity * mean_fidelity;
        // Clamp variance to 0 to avoid NaN from sqrt of negative due to float precision
        let stddev = if variance > 0.0 { variance.sqrt() } else { 0.0 };

        TeleportationStats {
            total_runs: runs,
            success_rate: if runs > 0 {
                success_count as f64 / runs as f64
            } else {
                0.0
            },
            mean_teleportation_fidelity: mean_fidelity,
            teleportation_fidelity_stddev: stddev,
            mean_latency_ms: if success_count > 0 {
                total_latency / n
            } else {
                0.0
            },
        }
    }
}
