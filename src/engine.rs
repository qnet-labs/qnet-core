use crate::api::request::{LinkDefinition, NetworkTopologyPayload, EntanglementRequest, TopologySnapshot, TopologyDiff, TopologyConfig};
use crate::api::response::{SimulationResult, MonteCarloStats};
use crate::config::SimulationConfig;
use crate::network::QuantumNetwork;
use crate::simulation::SimulationRuntime;
use crate::scheduler::NetworkOrchestratorPolicy;
use crate::montecarlo::MonteCarloSimulationEngine;
use std::collections::HashMap;

pub struct QNetEngine {
    pub(crate) config: SimulationConfig,
    pub(crate) network: Option<QuantumNetwork>,
}

impl QNetEngine {
    pub fn new(config: Option<SimulationConfig>) -> Self {
        Self {
            config: config.unwrap_or_default(),
            network: None,
        }
    }

    pub fn define_network(&mut self, payload: NetworkTopologyPayload) {
        let mut net = QuantumNetwork::new();
        for node in payload.nodes {
            net.add_node(&node.id, node.memory_lifetime_t2);
        }
        for link in payload.links {
            net.add_link(
                &link.from_node,
                &link.to,
                link.distance_km,
                link.base_fidelity,
                link.generation_rate_hz,
                link.link_type,
            );
        }
        self.network = Some(net);
    }

    pub fn request_entanglement(&self, request: EntanglementRequest) -> SimulationResult {
        let network = self.network.as_ref().expect("Network topology must be explicitly defined prior to run processing.");
        let mut runtime = SimulationRuntime::new();
        let mut orchestrator = NetworkOrchestratorPolicy::new(self.config.clone());

        orchestrator.coordinate_timeline(network, &mut runtime, &request, None)
    }

    pub fn simulate(&self, request: EntanglementRequest, runs: usize, seed: Option<u64>) -> MonteCarloStats {
        let network = self.network.as_ref().expect("Network topology must be explicitly defined prior to run processing.");

        // Use time-based default seed if none provided — avoids all iterations using the same RNG.
        let effective_seed = seed.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0)
        });

        MonteCarloSimulationEngine::execute_ensemble(network, self.config.clone(), request, runs, Some(effective_seed))
    }

    /// Export current network topology to JSON string
    pub fn export_topology(&self) -> Option<String> {
        let network = self.network.as_ref()?;

        let mut snapshot = TopologySnapshot::new("exported_topology", "1.0");
        snapshot.config = TopologyConfig {
            alpha_loss: self.config.physical.baseline_purify_factor * 10.0,
            gamma_swapping: self.config.physical.baseline_purify_factor,
        };

        for node in &network.nodes {
            snapshot.add_node(&node.0, node.1.t2_lifetime);
        }
        for link in &network.links {
            snapshot.add_link(
                &link.from,
                &link.to,
                link.distance,
                link.base_fidelity,
                link.rate_hz,
            );
        }

        Some(serde_json::to_string_pretty(&snapshot).unwrap_or_default())
    }

    /// Import network topology from JSON string
    pub fn import_topology(&mut self, json_data: &str) -> Result<(), String> {
        let snapshot: TopologySnapshot = serde_json::from_str(json_data)
            .map_err(|e| format!("Failed to parse topology JSON: {}", e))?;

        self.network = Some(self.create_network_from_snapshot(&snapshot));
        Ok(())
    }

    /// Save topology to a file
    pub fn save_topology(&self, filepath: &str) -> Result<(), String> {
        let json = self.export_topology()
            .ok_or("No network topology to save")?;
        std::fs::write(filepath, json)
            .map_err(|e| format!("Failed to write topology file: {}", e))
    }

    /// Load topology from a file
    pub fn load_topology(&mut self, filepath: &str) -> Result<(), String> {
        let json = std::fs::read_to_string(filepath)
            .map_err(|e| format!("Failed to read topology file: {}", e))?;
        self.import_topology(&json)
    }

    /// Create a network from a topology snapshot
    fn create_network_from_snapshot(&self, snapshot: &TopologySnapshot) -> QuantumNetwork {
        let mut net = QuantumNetwork::new();
        for node in &snapshot.nodes {
            net.add_node(&node.id, node.memory_lifetime_t2);
        }
        for link in &snapshot.links {
            net.add_link(
                &link.from_node,
                &link.to,
                link.distance_km,
                link.base_fidelity,
                link.generation_rate_hz,
                link.link_type,
            );
        }
        net
    }

    /// Compare two topology snapshots and return a diff
    pub fn diff_topologies(snapshot1: &TopologySnapshot, snapshot2: &TopologySnapshot) -> TopologyDiff {
        let name = format!("{} vs {}", snapshot1.metadata.name, snapshot2.metadata.name);
        let mut diff = TopologyDiff::new(&name);

        // Build maps for nodes
        let nodes1: HashMap<&str, f64> = snapshot1.nodes.iter().map(|n| (n.id.as_str(), n.memory_lifetime_t2)).collect();
        let nodes2: HashMap<&str, f64> = snapshot2.nodes.iter().map(|n| (n.id.as_str(), n.memory_lifetime_t2)).collect();

        // Find node differences
        for (id, lifetime) in &nodes1 {
            match nodes2.get(id) {
                None => diff.nodes_removed.push(id.to_string()),
                Some(&other_lifetime) => {
                    if (lifetime - other_lifetime).abs() > 0.001 {
                        diff.nodes_modified.push(id.to_string());
                    }
                }
            }
        }
        for (id, _) in &nodes2 {
            if !nodes1.contains_key(id) {
                diff.nodes_added.push(id.to_string());
            }
        }

        // Build maps for links
        let links1: HashMap<String, &LinkDefinition> = snapshot1.links.iter().map(|l| (l.link_key(), l)).collect();
        let links2: HashMap<String, &LinkDefinition> = snapshot2.links.iter().map(|l| (l.link_key(), l)).collect();

        // Find link differences
        for (key, link) in &links1 {
            match links2.get(key) {
                None => diff.links_removed.push(key.clone()),
                Some(other) => {
                    if link != other {
                        diff.links_modified.push(key.clone());
                    }
                }
            }
        }
        for (key, _) in &links2 {
            if !links1.contains_key(key) {
                diff.links_added.push(key.clone());
            }
        }

        diff.summary = diff.generate_summary();
        diff
    }
}

// Helper method for link key generation
impl LinkDefinition {
    fn link_key(&self) -> String {
        format!("{}->{}", self.from_node, self.to)
    }
}
