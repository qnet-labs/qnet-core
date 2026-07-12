use crate::api::request::EntanglementRequest;
use crate::api::response::MonteCarloStats;
use crate::config::SimulationConfig;
use crate::network::QuantumNetwork;
use crate::scheduler::NetworkOrchestratorPolicy;
use crate::simulation::SimulationRuntime;
use rand::SeedableRng;
use std::collections::HashMap;

pub struct MonteCarloSimulationEngine;

impl MonteCarloSimulationEngine {
    pub fn execute_ensemble(
        network: &QuantumNetwork,
        config: SimulationConfig,
        request: EntanglementRequest,
        runs: usize,
        seed: Option<u64>,
    ) -> MonteCarloStats {
        let mut total_successes = 0;
        let mut total_latency = 0.0;
        let mut total_fidelity = 0.0;
        let mut aggregate_congestion_drops = 0;
        let mut unified_heatmap: HashMap<String, usize> = HashMap::new();

        for i in 0..runs {
            let mut runtime = SimulationRuntime::new();
            let mut orchestrator = NetworkOrchestratorPolicy::new(config.clone());

            // Deterministic RNG: derive per-iteration seed to avoid temporal correlation
            let rng = seed.map(|s| rand::rngs::StdRng::seed_from_u64(s.wrapping_add(i as u64)));

            let res = orchestrator.coordinate_timeline(network, &mut runtime, &request, rng);

            if res.success {
                total_successes += 1;
                total_fidelity += res.final_fidelity;
            }
            total_latency += res.latency_ms;
            aggregate_congestion_drops += runtime.metrics_dropped_congestion;

            for (link, counts) in orchestrator.telemetry.link_firings {
                *unified_heatmap.entry(link).or_insert(0) += counts;
            }
        }

        let runs_f = runs as f64;
        MonteCarloStats {
            total_runs: runs,
            empirical_success_rate: (total_successes as f64) / runs_f,
            mean_latency_ms: total_latency / runs_f,
            mean_fidelity: if total_successes > 0 {
                total_fidelity / (total_successes as f64)
            } else {
                0.0
            },
            aggregate_congestion_drops,
            link_utilization_heatmap: unified_heatmap,
        }
    }
}
