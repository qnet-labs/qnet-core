// QKD (Quantum Key Distribution) Protocol Implementation
//
// Simulates BB84-style key generation over a quantum network:
// 1. Establish entanglement links between sender and receiver via the network topology
// 2. Simulate Z-basis measurements on shared Bell states
// 3. Apply basis reconciliation (sifting), privacy amplification, and error correction
// 4. Derive a classical secret key with efficiency metrics

use crate::api::request::EntanglementRequest;
use crate::api::response::{QKDResult, QKDStats};
use crate::QNetEngine;
use crate::routing::strategy::StrategyType;

// ============================================================================
// Request Type
// ============================================================================

/// Parameters for a QKD protocol run.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QKDParameters {
    /// Sender node (Alice).
    pub from_node: String,
    /// Receiver node (Bob).
    pub to_node: String,
    /// Target entanglement fidelity for link establishment.
    pub fidelity_target: f64,
    /// Maximum latency budget for the entire QKD session (ms).
    pub max_latency_ms: f64,
    /// Number of entanglement generation rounds to attempt.
    #[serde(default = "default_rounds")]
    pub rounds: usize,
    /// Quantum bit error rate (QBER) threshold for security. Keys exceeding this are rejected.
    #[serde(default = "default_error_rate_tolerance")]
    pub error_rate_tolerance: f64,
    /// Fraction of rounds lost during basis reconciliation/sifting (0.0-1.0).
    #[serde(default = "default_sifting_overhead")]
    pub sifting_overhead_ratio: f64,
    /// Fraction of sifted key retained after privacy amplification (0.0-1.0).
    #[serde(default = "default_privacy_amplification")]
    pub privacy_amplification_factor: f64,
}

fn default_rounds() -> usize { 100 }
fn default_error_rate_tolerance() -> f64 { 0.11 } // Shor-Preskill bound
fn default_sifting_overhead() -> f64 { 0.5 } // BB84: half discarded for basis mismatch
fn default_privacy_amplification() -> f64 { 0.8 } // ~80% retained after error correction

// ============================================================================
// Protocol Implementation
// ============================================================================

/// Core protocol struct with static methods — matches the PurificationEngine pattern.
pub struct QKDProtocol;

impl QKDProtocol {
    /// Run a single QKD protocol simulation.
    ///
    /// Establishes entanglement links via `engine.request_entanglement()`, simulates
    /// Z-basis measurements on each successful Bell pair, and derives the secret key
    /// through sifting and privacy amplification.
    pub fn run(engine: &QNetEngine, params: QKDParameters) -> QKDResult {
        let success = engine.network.is_some();

        // Phase 1: Establish entanglement links round by round
        let mut rounds_completed = 0usize;
        let mut rounds_failed = 0usize;
        let mut total_latency_ms = 0.0f64;
        let mut fidelity_sum = 0.0f64;
        let mut execution_path = Vec::new();

        if success {
            for _ in 0..params.rounds {
                if total_latency_ms >= params.max_latency_ms {
                    rounds_failed += 1;
                    break;
                }

                let request = EntanglementRequest {
                    from_node: params.from_node.clone(),
                    to: params.to_node.clone(),
                    fidelity_target: params.fidelity_target,
                    max_latency_ms: params.max_latency_ms - total_latency_ms,
                    strategy: Some(StrategyType::HighestFidelity),
                };

                let result = engine.request_entanglement(request);

                if result.success {
                    rounds_completed += 1;
                    fidelity_sum += result.final_fidelity;
                    total_latency_ms += result.latency_ms;
                    execution_path = result.execution_path.clone();
                } else {
                    rounds_failed += 1;
                    // Apply backoff for failed attempts (2.5ms, matching scheduler logic)
                    total_latency_ms += 2.5;
                }
            }
        }

        // Phase 2: Derive key from successful entanglement links
        let success = rounds_completed > 0;

        // Simulate QBER based on observed fidelity: QBER ≈ (1 - fidelity) / 2 for BB84
        let qber = if rounds_completed > 0 {
            let avg_fidelity = fidelity_sum / rounds_completed as f64;
            (1.0 - avg_fidelity) / 2.0
        } else {
            1.0
        };

        // Apply sifting: BB84 basis reconciliation discards ~50% of rounds on average
        let sifted_bits = if success {
            (rounds_completed as f64 * (1.0 - params.sifting_overhead_ratio)) as usize
        } else {
            0
        };

        // Apply privacy amplification: reduce key length for error correction + security proof
        let secret_key_length = if success && qber < params.error_rate_tolerance {
            (sifted_bits as f64 * params.privacy_amplification_factor) as usize
        } else {
            0
        };

        let efficiency_rate = if params.rounds > 0 {
            secret_key_length as f64 / params.rounds as f64
        } else {
            0.0
        };

        QKDResult {
            success,
            secret_key_length_bits: secret_key_length,
            efficiency_rate,
            qber,
            latency_ms: total_latency_ms,
            execution_path,
            rounds_completed,
            rounds_failed,
        }
    }

    /// Run multiple independent QKD sessions and aggregate statistics.
    pub fn run_ensemble(engine: &QNetEngine, params: QKDParameters, runs: usize) -> QKDStats {
        let mut success_count = 0usize;
        let mut total_key_length = 0f64;
        let mut total_efficiency = 0.0f64;
        let mut total_qber = 0.0f64;

        for _ in 0..runs {
            let result = QKDProtocol::run(engine, params.clone());
            if result.success {
                success_count += 1;
                total_key_length += result.secret_key_length_bits as f64;
                total_efficiency += result.efficiency_rate;
                total_qber += result.qber;
            }
        }

        QKDStats {
            total_runs: runs,
            success_rate: if runs > 0 { success_count as f64 / runs as f64 } else { 0.0 },
            mean_key_length_bits: if success_count > 0 { total_key_length / success_count as f64 } else { 0.0 },
            mean_efficiency: if success_count > 0 { total_efficiency / success_count as f64 } else { 0.0 },
            mean_qber: if success_count > 0 { total_qber / success_count as f64 } else { 0.0 },
        }
    }
}
