use std::collections::{HashMap, HashSet};

use crate::api::request::EntanglementRequest;
use crate::api::response::SimulationResult;
use crate::config::SimulationConfig;
use crate::metrics::NetworkTelemetry;
use crate::protocols::PurificationEngine;
use crate::routing::{find_route, strategy::StrategyType};
use crate::simulation::{EventType, EventWrapper, SimulationEvent, SimulationRuntime};
use rand::SeedableRng;

pub struct NetworkOrchestratorPolicy {
    pub config: SimulationConfig,
    pub telemetry: NetworkTelemetry,
}

impl NetworkOrchestratorPolicy {
    pub fn new(config: SimulationConfig) -> Self {
        Self {
            config,
            telemetry: NetworkTelemetry::new(),
        }
    }

    /// Pure Decision logic controller that coordinates events without handling raw physics math directly.
    /// When `rng` is None, uses a deterministic default (same seed every time).
    /// Pass Some(StdRng) for reproducible simulations (Monte Carlo).
    pub fn coordinate_timeline(
        &mut self,
        network: &crate::network::QuantumNetwork,
        runtime: &mut SimulationRuntime,
        request: &EntanglementRequest,
        rng: Option<rand::rngs::StdRng>,
    ) -> SimulationResult {
        use rand::Rng;

        self.telemetry.total_requests += 1;

        let strategy = request.strategy.unwrap_or(StrategyType::HighestFidelity);
        let mut route_opt = find_route(network, &request.from_node, &request.to, strategy);

        // Failover Policy: Fall back to latency routing if target bounds are violated
        if let Some(ref r) = route_opt {
            if r.composite_fidelity < request.fidelity_target
                && strategy == StrategyType::HighestFidelity
            {
                route_opt = find_route(
                    network,
                    &request.from_node,
                    &request.to,
                    StrategyType::LowestLatency,
                );
            }
        }

        let route = match route_opt {
            Some(r) => r,
            None => {
                return SimulationResult {
                    success: false,
                    latency_ms: 0.0,
                    final_fidelity: 0.0,
                    execution_path: vec![],
                }
            }
        };

        // HARD CONSTRAINT: Check if route can possibly meet fidelity target even with purification
        if route.composite_fidelity < request.fidelity_target {
            let max_purified = PurificationEngine::bbpssw_distill(
                route.composite_fidelity,
                self.config.physical.baseline_purify_factor,
            );
            if max_purified < request.fidelity_target {
                return SimulationResult {
                    success: false,
                    latency_ms: 0.0,
                    final_fidelity: 0.0,
                    execution_path: route.path,
                };
            }
        }

        let mut timestamp_ms = 0.0;
        let mut running_fidelity = route.composite_fidelity;
        let mut is_satisfied = false;
        let mut rng = rng.unwrap_or_else(|| rand::rngs::StdRng::from_seed([0u8; 32]));

        // HARD CONSTRAINT: Calculate minimum latency for this route based on speed of light
        let min_route_latency_ms: f64 = route
            .path
            .windows(2)
            .map(|hop| {
                let hop_distance = network
                    .links
                    .iter()
                    .find(|l| {
                        (l.from == hop[0] && l.to == hop[1]) || (l.to == hop[0] && l.from == hop[1])
                    })
                    .map(|l| l.distance)
                    .unwrap_or(0.0);
                // Speed of light in fiber: 200 km/ms
                hop_distance / self.config.physical.speed_of_light_in_fiber_km_ms
            })
            .sum();

        // HARD CONSTRAINT: Fail immediately if minimum latency exceeds request limit
        if min_route_latency_ms > request.max_latency_ms {
            return SimulationResult {
                success: false,
                latency_ms: min_route_latency_ms,
                final_fidelity: 0.0,
                execution_path: route.path.clone(),
            };
        }

        while timestamp_ms < request.max_latency_ms
            && timestamp_ms < self.config.total_time_cutoff_ms
        {
            for hop in route.path.windows(2) {
                let link_id = format!("{}->{}", hop[0], hop[1]);
                self.telemetry.record_link_firing(&link_id);

                // HARD CONSTRAINT: Enforce stochastic link generation failure based on distance and fidelity
                let link_success_prob = self.calculate_link_success_probability(
                    network,
                    &hop[0],
                    &hop[1],
                    request.fidelity_target,
                    self.config.physical.baseline_purify_factor,
                );

                let link_generated = rng.gen_range(0.0..=1.0) < link_success_prob;

                if link_generated {
                    let base_fidelity = network
                        .links
                        .iter()
                        .find(|l| {
                            (l.from == hop[0] && l.to == hop[1])
                                || (l.to == hop[0] && l.from == hop[1])
                        })
                        .map(|l| l.base_fidelity)
                        .unwrap_or(0.0);
                    runtime.timeline.push(EventWrapper(SimulationEvent {
                        time: timestamp_ms,
                        event_type: EventType::AttemptLinkGeneration {
                            from: hop[0].clone(),
                            to: hop[1].clone(),
                            base_fidelity,
                        },
                    }));
                } else {
                    // Link generation failed - record congestion pressure
                    runtime.metrics_dropped_congestion += 1;
                }
            }

            runtime.process_events_up_to(timestamp_ms);

            // HARD CONSTRAINT: Check if any links failed due to congestion
            if runtime.metrics_dropped_congestion > 0 {
                // Congestion pressure applied - link generation failed
                timestamp_ms += 2.5; // Backoff delay
                self.telemetry.cumulative_queue_delay_ms += 2.5;
                runtime.link_establishments.clear();
                continue;
            }

            // Multi-hop: check if ALL hops have successful establishments in this time step
            let established: HashSet<(&str, &str)> = runtime
                .link_establishments
                .iter()
                .map(|(a, b, _)| {
                    let key = if *a < *b {
                        (a.as_str(), b.as_str())
                    } else {
                        (b.as_str(), a.as_str())
                    };
                    key
                })
                .collect();

            // Normalize hop keys using string comparison for consistency with established set
            let all_hops_done = route.path.windows(2).all(|hop| {
                let key = if hop[0] < hop[1] {
                    (hop[0].as_str(), hop[1].as_str())
                } else {
                    (hop[1].as_str(), hop[0].as_str())
                };
                established.contains(&key)
            });

            if all_hops_done && route.path.len() > 2 {
                // All links established this time step — compute end-to-end fidelity via BSM
                // Normalize hop keys using string comparison for consistency
                let hop_fidelities: HashMap<(&str, &str), f64> = runtime
                    .link_establishments
                    .iter()
                    .map(|(a, b, f)| {
                        let key = if *a < *b {
                            (a.as_str(), b.as_str())
                        } else {
                            (b.as_str(), a.as_str())
                        };
                        (key, *f)
                    })
                    .collect();

                // Use string-based comparison for hop keys, consistent with link_fidelities
                let e2e_fidelity = route
                    .path
                    .windows(2)
                    .map(|hop| {
                        let key = if hop[0] < hop[1] {
                            (hop[0].as_str(), hop[1].as_str())
                        } else {
                            (hop[1].as_str(), hop[0].as_str())
                        };
                        hop_fidelities.get(&key).copied().unwrap_or(1.0)
                    })
                    .fold(1.0, |acc, f| acc * f);

                // Apply BSM transformation at each intermediate repeater
                running_fidelity = e2e_fidelity;
                for _ in 1..route.path.len() - 1 {
                    running_fidelity = PurificationEngine::bbpssw_distill(
                        running_fidelity,
                        self.config.physical.baseline_purify_factor,
                    );
                    running_fidelity = running_fidelity.min(0.999);
                }
            }

            runtime.link_establishments.clear(); // reset for next iteration

            // Optimization Policy: Execute purification steps when required
            if running_fidelity < request.fidelity_target && running_fidelity > 0.5 {
                timestamp_ms += 0.85; // Fixed operational propagation latency penalty
                running_fidelity = PurificationEngine::bbpssw_distill(
                    running_fidelity,
                    self.config.physical.baseline_purify_factor,
                );

                // HARD CONSTRAINT: Cap fidelity at 0.999
                running_fidelity = running_fidelity.min(0.999);
            }

            // HARD CONSTRAINT: Check if fidelity target is met
            if running_fidelity >= request.fidelity_target {
                is_satisfied = true;
                break;
            }

            // HARD CONSTRAINT: Check if we've exceeded latency budget
            if timestamp_ms >= request.max_latency_ms {
                break;
            }

            // Recovery Policy: Apply backoff delays if link generation fails
            let backoff_penalty = 2.5;
            timestamp_ms += backoff_penalty;
            self.telemetry.cumulative_queue_delay_ms += backoff_penalty;

            runtime.metrics_dropped_congestion = 0;
            runtime.metrics_failed_swaps = 0;
        }

        // HARD CONSTRAINT: Final fidelity check - must meet target if not satisfied
        if !is_satisfied && running_fidelity < request.fidelity_target {
            return SimulationResult {
                success: false,
                latency_ms: timestamp_ms,
                final_fidelity: 0.0,
                execution_path: route.path,
            };
        }

        if is_satisfied {
            self.telemetry.successful_requests += 1;
        }
        self.telemetry.finalize();

        SimulationResult {
            success: is_satisfied,
            latency_ms: timestamp_ms,
            final_fidelity: if is_satisfied { running_fidelity } else { 0.0 },
            execution_path: route.path,
        }
    }

    /// Calculate link success probability based on distance, fidelity target, and network conditions
    /// This implements HARD constraints with realistic stochastic failure modeling
    fn calculate_link_success_probability(
        &self,
        network: &crate::network::QuantumNetwork,
        from: &str,
        to: &str,
        fidelity_target: f64,
        _purify_factor: f64,
    ) -> f64 {
        let link = network
            .links
            .iter()
            .find(|l| (l.from == *from && l.to == *to) || (l.to == *from && l.from == *to));

        match link {
            Some(l) => {
                // Base success probability inversely proportional to distance
                // Longer links have higher loss due to fiber attenuation
                // Using scaled power-law: penalty grows quadratically with normalized distance
                let distance_factor = l.distance / 100.0;
                let distance_penalty = (distance_factor / (distance_factor + 1.0)).powi(2);

                // Fidelity target penalty - harder targets reduce success rate
                // But only moderately to allow for realistic simulation
                let fidelity_penalty = if l.base_fidelity < fidelity_target {
                    (fidelity_target - l.base_fidelity) * 0.3
                } else {
                    0.0
                };

                // Base probability reduced by distance and fidelity penalties
                // Minimum 0.5 (50%), maximum 0.98 (98%) for realistic stochastic behavior
                let base_prob = 1.0 - distance_penalty * 0.45 - fidelity_penalty;
                base_prob.clamp(0.50, 0.98)
            }
            None => 0.0, // Link doesn't exist
        }
    }
}
