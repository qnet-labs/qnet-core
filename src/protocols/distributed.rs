// Distributed Quantum Computing Protocol Implementation
//
// Simulates multi-party quantum computation over a quantum network:
// 1. Determine required entanglement links from the coordination topology
// 2. Establish each link via QNetEngine
// 3. Simulate coordinated measurements on shared entangled states
// 4. Compute overall protocol fidelity and per-party outcomes

use crate::api::request::EntanglementRequest;
use crate::api::response::{DistributedComputingResult, DistributedComputingStats, PartyOutcome};
use crate::routing::strategy::StrategyType;
use crate::QNetEngine;

// ============================================================================
// Request Types
// ============================================================================

/// Types of coordination topologies for distributed quantum computing.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CoordinationTopology {
    /// Star topology with a central coordinator node.
    Star { center_node: String },
    /// Ring topology where each party connects to its neighbors.
    Ring,
    /// All-to-all mesh (requires N(N-1)/2 entanglement links).
    Mesh,
    /// Explicit edge list for arbitrary topologies.
    Arbitrary { edges: Vec<(String, String)> },
}

impl CoordinationTopology {
    /// Resolve all required entanglement links for this topology given the participant nodes.
    pub fn required_edges(&self, participants: &[String]) -> Vec<(String, String)> {
        match self {
            CoordinationTopology::Star { center_node } => {
                // All parties connect to the center
                participants
                    .iter()
                    .filter(|n| *n != center_node)
                    .map(|n| (n.clone(), center_node.clone()))
                    .collect()
            }
            CoordinationTopology::Ring => {
                // Each party connects to next; last connects back to first
                let mut edges = Vec::new();
                for i in 0..participants.len() {
                    let next = (i + 1) % participants.len();
                    edges.push((participants[i].clone(), participants[next].clone()));
                }
                edges
            }
            CoordinationTopology::Mesh => {
                // All-to-all connections
                let mut edges = Vec::new();
                for i in 0..participants.len() {
                    for j in (i + 1)..participants.len() {
                        edges.push((participants[i].clone(), participants[j].clone()));
                    }
                }
                edges
            }
            CoordinationTopology::Arbitrary { edges } => edges.clone(),
        }
    }
}

/// Type of quantum state used for distributed measurement.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BasisType {
    /// Greenberger-Horne-Zeilinger (GHZ) state measurements.
    GHZ,
    /// Cluster graph state (measurement-based QC).
    Cluster,
    /// General graph state protocol.
    GraphGraph,
}

/// Measurement basis for the distributed protocol.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MeasurementBasis {
    /// Type of quantum state used as the measurement resource.
    pub basis_type: BasisType,
    /// Correlation strength between party measurements (0.0 = independent, 1.0 = fully correlated).
    #[serde(default = "default_correlation_strength")]
    pub correlation_strength: f64,
}

fn default_correlation_strength() -> f64 {
    0.85
}

/// Parameters for a distributed computing protocol run.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DistributedComputingParameters {
    /// Node IDs of all participating quantum computing parties.
    pub participants: Vec<String>,
    /// How the participants are connected for coordination.
    pub coordination_topology: CoordinationTopology,
    /// Shared measurement strategy and resource state type.
    pub measurement_basis: MeasurementBasis,
    /// Per-hop classical communication overhead (ms).
    #[serde(default = "default_classical_relay")]
    pub classical_relay_latency_ms: f64,
}

fn default_classical_relay() -> f64 {
    5.0
}

// ============================================================================
// Protocol Implementation
// ============================================================================

/// Core protocol struct with static methods — matches the PurificationEngine pattern.
pub struct DistributedComputingProtocol;

impl DistributedComputingProtocol {
    /// Run a distributed quantum computing protocol across participating nodes.
    ///
    /// Determines required entanglement links from `coordination_topology`, establishes each link
    /// via `engine.request_entanglement()`, then simulates coordinated measurements with fidelity
    /// computed as the weighted product of per-link fidelities and party local fidelities.
    pub fn execute(
        engine: &QNetEngine,
        params: DistributedComputingParameters,
    ) -> DistributedComputingResult {
        let participants = &params.participants;
        let success = engine.network.is_some();

        // Phase 1: Determine required links from topology
        let edges = if success {
            params.coordination_topology.required_edges(participants)
        } else {
            Vec::new()
        };

        // Phase 2: Establish all entanglement links
        let mut link_fidelities = Vec::new();
        let mut total_latency_ms = 0.0f64;
        let mut resource_links_used = Vec::new();

        if success {
            for (from, to) in &edges {
                let request = EntanglementRequest {
                    from_node: from.clone(),
                    to: to.clone(),
                    fidelity_target: 0.9,
                    max_latency_ms: params.classical_relay_latency_ms * 10.0,
                    strategy: Some(StrategyType::HighestFidelity),
                };

                let result = engine.request_entanglement(request);
                link_fidelities.push(if result.success {
                    result.final_fidelity
                } else {
                    0.0
                });
                total_latency_ms += result.latency_ms;
                resource_links_used.push(format!("{}->{}", from, to));
            }
        }

        // Phase 3: Compute per-party outcomes
        let mut party_results = Vec::new();
        let mut overall_fidelity = if success { 1.0 } else { 0.0 };

        if success {
            for node in participants {
                // Find all links involving this node and average their fidelities
                let node_link_fids: Vec<f64> = edges
                    .iter()
                    .zip(link_fidelities.iter())
                    .filter(|((a, b), _)| a == node || b == node)
                    .map(|(_, f)| *f)
                    .collect();

                let link_fidelity_avg = if !node_link_fids.is_empty() {
                    node_link_fids.iter().sum::<f64>() / node_link_fids.len() as f64
                } else {
                    1.0 // Node with no links in this topology - assume perfect local fidelity
                };

                let local_fid = link_fidelity_avg * params.measurement_basis.correlation_strength;
                let successful = node_link_fids.iter().all(|&f| f > 0.5);

                overall_fidelity *= local_fid;
                party_results.push(PartyOutcome {
                    node_id: node.clone(),
                    successful_measurement: successful,
                    local_fidelity: local_fid.min(0.999), // Cap at physical ceiling
                });
            }
        }

        let success = if success {
            !party_results.is_empty() && party_results.iter().all(|p| p.successful_measurement)
        } else {
            false
        };

        let coordination_overhead = if edges.is_empty() {
            0.0
        } else {
            (edges.len() as f64) * params.classical_relay_latency_ms
        };

        DistributedComputingResult {
            success,
            computation_fidelity: overall_fidelity.min(0.999),
            party_results,
            resource_links_used,
            total_latency_ms,
            coordination_overhead_ms: coordination_overhead,
        }
    }

    /// Run multiple independent distributed computing sessions and aggregate statistics.
    pub fn execute_ensemble(
        engine: &QNetEngine,
        params: DistributedComputingParameters,
        runs: usize,
    ) -> DistributedComputingStats {
        let mut success_count = 0usize;
        let mut total_fidelity = 0.0f64;
        let mut total_party_success_rate = 0.0f64;

        for _ in 0..runs {
            let result = DistributedComputingProtocol::execute(engine, params.clone());
            if result.success {
                success_count += 1;
                total_fidelity += result.computation_fidelity;
            }
            let party_success = result
                .party_results
                .iter()
                .filter(|p| p.successful_measurement)
                .count();
            let party_rate = if !result.party_results.is_empty() {
                party_success as f64 / result.party_results.len() as f64
            } else {
                0.0
            };
            total_party_success_rate += party_rate;
        }

        DistributedComputingStats {
            total_runs: runs,
            success_rate: if runs > 0 {
                success_count as f64 / runs as f64
            } else {
                0.0
            },
            mean_computation_fidelity: if success_count > 0 {
                total_fidelity / success_count as f64
            } else {
                0.0
            },
            mean_party_success_rate: if runs > 0 {
                total_party_success_rate / runs as f64
            } else {
                0.0
            },
        }
    }
}
