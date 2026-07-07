use pyo3::prelude::*;

use super::types::*;
use super::qnet_io::PyQNetFile;
use crate::api::request::{LinkDefinition, LinkType, NetworkTopologyPayload, NodeDefinition, SatelliteConditions};
use crate::engine::QNetEngine;

// ============================================================================
// Main Engine
// ============================================================================

#[pyclass]
pub struct PyQNetEngine {
    inner: QNetEngine,
    /// Snapshot of the last network + config for post-hoc analysis.
    topology_snapshot: Option<(Vec<PyNodeDefinition>, Vec<PyLinkDefinition>, PySimulationConfig)>,
}

#[pymethods]
impl PyQNetEngine {
    #[new]
    fn new(config: Option<PySimulationConfig>) -> Self {
        let config = config.map(|c| crate::config::SimulationConfig {
            total_time_cutoff_ms: c.total_time_cutoff_ms,
            step_resolution_ms: c.step_resolution_ms,
            physical: crate::config::PhysicalConfig {
                baseline_purify_factor: c.physical.baseline_purify_factor,
                speed_of_light_in_fiber_km_ms: c.physical.speed_of_light_in_fiber_km_ms,
            },
        });
        Self {
            inner: QNetEngine::new(config),
            topology_snapshot: None,
        }
    }

    fn define_network(&mut self, nodes: Vec<PyNodeDefinition>, links: Vec<PyLinkDefinition>) {
        // Capture snapshot for post-hoc analysis (sensitivity, etc.)
        let config = PySimulationConfig {
            total_time_cutoff_ms: 5000.0,
            step_resolution_ms: 0.1,
            physical: PyPhysicalConfig {
                baseline_purify_factor: 0.22 / 10.0,
                speed_of_light_in_fiber_km_ms: 200.0,
            },
        };
        self.topology_snapshot = Some((nodes.clone(), links.clone(), config));

        let nodes: Vec<NodeDefinition> = nodes
            .into_iter()
            .map(|n| NodeDefinition {
                id: n.id,
                memory_lifetime_t2: n.memory_lifetime_t2,
            })
            .collect();
        let links: Vec<LinkDefinition> = links
            .into_iter()
            .map(|l| LinkDefinition {
                from_node: l.from_node,
                to: l.to,
                distance_km: l.distance_km,
                base_fidelity: l.base_fidelity,
                generation_rate_hz: l.generation_rate_hz,
                link_type: l.link_type.0,
                satellite_conditions: l.satellite_conditions.map(|sc| SatelliteConditions {
                    visibility: sc.visibility,
                    weather_factor: sc.weather_factor,
                }),
            })
            .collect();
        let payload = NetworkTopologyPayload { nodes, links };
        self.inner.define_network(payload);
    }

    fn request_entanglement(
        &self,
        from_node: String,
        to: String,
        fidelity_target: f64,
        max_latency_ms: f64,
        strategy: Option<PyStrategyType>,
    ) -> PyResult<PySimulationResult> {
        let request = crate::api::request::EntanglementRequest {
            from_node,
            to,
            fidelity_target,
            max_latency_ms,
            strategy: strategy.map(|s| s.0),
        };
        let result = self.inner.request_entanglement(request);
        Ok(PySimulationResult {
            success: result.success,
            latency_ms: result.latency_ms,
            final_fidelity: result.final_fidelity,
            execution_path: result.execution_path,
        })
    }

    fn simulate(
    &self,
    from_node: String,
    to: String,
    fidelity_target: f64,
    max_latency_ms: f64,
    runs: usize,
    strategy: Option<PyStrategyType>,
    seed: Option<u64>,
) -> PyResult<PyMonteCarloStats> {
    let network = self.inner.network.as_ref()
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "No network defined. Call define_network() first."
        ))?;

    let req = crate::api::request::EntanglementRequest {
        from_node: from_node.clone(),
        to: to.clone(),
        fidelity_target,
        max_latency_ms,
        strategy: strategy.map(|s| s.into_strategy_type()),
    };

    let stats = self.inner.simulate(req, runs, seed);

    // ── Build context for sensitivity_analysis() ──────────────────────────
    let context = PyMonteCarloContext {
        nodes: network.nodes.iter()
            .map(|(id, node)| (id.clone(), node.t2_lifetime))
            .collect(),
        links: network.links.iter()
            .map(|l| (
                l.from.clone(),
                l.to.clone(),
                l.distance,
                l.base_fidelity,
                l.rate_hz,
            ))
            .collect(),
        from_node,
        to_node: to,
        fidelity_target,
        max_latency_ms,
        strategy: strategy.map(|s| s.to_u8()),
        seed,
        total_time_cutoff_ms: self.inner.config.total_time_cutoff_ms,
        step_resolution_ms: self.inner.config.step_resolution_ms,
        baseline_purify_factor: self.inner.config.physical.baseline_purify_factor,
    };
    // ─────────────────────────────────────────────────────────────────────

    Ok(PyMonteCarloStats {
        total_runs: stats.total_runs,
        empirical_success_rate: stats.empirical_success_rate,
        mean_latency_ms: stats.mean_latency_ms,
        mean_fidelity: stats.mean_fidelity,
        aggregate_congestion_drops: stats.aggregate_congestion_drops,
        link_utilization_heatmap: stats.link_utilization_heatmap,
        _context: Some(context),
    })
}

    /// Load a topology from a loaded .qnet file directly into the engine.
    /// Bridges both type systems — no manual NodeDefinition / LinkDefinition needed.
    #[pyo3(name = "define_network_from_qnet")]
    fn define_network_from_qnet(&mut self, qf: PyQNetFile) {
        let payload = crate::api::request::NetworkTopologyPayload {
            nodes: qf.nodes.into_iter().map(|n| {
                crate::api::request::NodeDefinition {
                    id: n.id,
                    // .qnet uses milliseconds; engine expects seconds (t2 lifetime)
                    memory_lifetime_t2: n.memory_lifetime_ms.map(|ms| ms / 1000.0).unwrap_or(1.0),
                }
            }).collect(),
            links: qf.links.into_iter().map(|l| {
                let link_type = l.link_type.map(|lt| match lt.0 {
                    crate::api::request::QNetLinkType::Fiber => crate::api::request::LinkType::Fiber,
                    crate::api::request::QNetLinkType::Satellite => crate::api::request::LinkType::Satellite,
                }).unwrap_or(crate::api::request::LinkType::Fiber);
                let satellite_conditions = l.satellite.map(|sc| {
                    crate::api::request::SatelliteConditions {
                        visibility: sc.visibility,
                        weather_factor: sc.weather_factor,
                    }
                });
                crate::api::request::LinkDefinition {
                    from_node: l.src,
                    to: l.to,
                    distance_km: l.distance_km,
                    base_fidelity: l.base_fidelity,
                    generation_rate_hz: l.generation_rate_hz,
                    link_type,
                    satellite_conditions,
                }
            }).collect(),
        };
        self.inner.define_network(payload);
    }
}

/// Classmethod alternative to `QNetEngine(config?)`: load a .qnet file and return an initialized engine.
/// This is the "from_file" pattern — one call to start simulating.
#[pyfunction]
#[pyo3(name = "from_qnet_file")]
pub(crate) fn from_qnet_file_py(filepath: &str) -> PyResult<PyQNetEngine> {
    let qf = crate::io::load_qnet_file(filepath)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to load {}: {}", filepath, e)))?;

    let payload = NetworkTopologyPayload {
        nodes: qf.nodes.into_iter().map(|n| NodeDefinition {
            id: n.id,
            memory_lifetime_t2: n.memory_lifetime_ms.map(|ms| ms / 1000.0).unwrap_or(1.0),
        }).collect(),
        links: qf.links.into_iter().map(|l| {
            // QNetLinkType is a plain enum (not wrapped), so match directly
            let link_type = l.link_type.map(|lt| match lt {
                crate::api::request::QNetLinkType::Fiber => LinkType::Fiber,
                crate::api::request::QNetLinkType::Satellite => LinkType::Satellite,
            }).unwrap_or(LinkType::Fiber);
            let satellite_conditions = l.satellite.map(|sc| {
                SatelliteConditions {
                    visibility: sc.visibility,
                    weather_factor: sc.weather_factor,
                }
            });
            LinkDefinition {
                from_node: l.from,  // QNetLink uses "from" (not "src")
                to: l.to,
                distance_km: l.distance_km,
                base_fidelity: l.base_fidelity,
                generation_rate_hz: l.generation_rate_hz,
                link_type,
                satellite_conditions,
            }
        }).collect(),
    };

    let config = qf.config.as_ref().map(|c| crate::config::SimulationConfig {
        total_time_cutoff_ms: (c.max_attempts.unwrap_or(100) as f64) * 50.0,
        step_resolution_ms: 0.1,
        physical: crate::config::PhysicalConfig {
            baseline_purify_factor: c.alpha_loss.map(|a| a / 10.0).unwrap_or(0.22),
            speed_of_light_in_fiber_km_ms: 200.0,
        },
    });

    let mut engine = QNetEngine::new(config);
    engine.define_network(payload);
    Ok(PyQNetEngine { inner: engine, topology_snapshot: None })
}

// ============================================================================
// Topology Generator Wrapper Functions
// ============================================================================

#[pyfunction]
pub(crate) fn generate_topology(name: &str) -> PyResult<PyNetworkTopologyPayload> {
    use crate::topology::generator::{generate_topology as gen, TopologyType};

    let payload = match name {
        "telecom_backbone" => gen(TopologyType::TelecomBackbone),
        "repeater_chain" => gen(TopologyType::RepeaterChain { length: 4 }),
        "hybrid_satellite_fiber" => gen(TopologyType::HybridSatelliteFiber),
        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Invalid topology name. Use: 'telecom_backbone', 'repeater_chain', or 'hybrid_satellite_fiber'"
        )),
    };

    Ok(PyNetworkTopologyPayload {
        nodes: payload.nodes.into_iter().map(|n| PyNodeDefinition {
            id: n.id,
            memory_lifetime_t2: n.memory_lifetime_t2,
        }).collect(),
        links: payload.links.into_iter().map(|l| PyLinkDefinition {
            from_node: l.from_node,
            to: l.to,
            distance_km: l.distance_km,
            base_fidelity: l.base_fidelity,
            generation_rate_hz: l.generation_rate_hz,
            link_type: PyLinkType(l.link_type),
            satellite_conditions: l.satellite_conditions.map(|sc| PySatelliteConditions {
                visibility: sc.visibility,
                weather_factor: sc.weather_factor,
            }),
        }).collect(),
    })
}

/// Per-topology endpoint mapping for comparison
#[pyclass]
#[derive(Clone)]
pub struct TopologyEndpoints {
    #[pyo3(get, set)]
    pub topology_name: String,
    #[pyo3(get, set)]
    pub from_node: String,
    #[pyo3(get, set)]
    pub to_node: String,
}

#[pymethods]
impl TopologyEndpoints {
    #[new]
    fn new(topology_name: String, from_node: String, to_node: String) -> Self {
        Self { topology_name, from_node, to_node }
    }
}

/// Compare multiple network topologies for the same entanglement request
///
/// This function runs simulations across different topologies and returns
/// a comprehensive comparison report.
///
/// # Arguments
/// * `endpoints` - List of (topology_name, from_node, to_node) tuples, one per topology.
///   Each entry maps a generated topology name to the source and target nodes within that
///   specific topology (e.g., "telecom_backbone" → ("A", "C"), "hybrid_satellite_fiber" → ("Toronto", "London")).
/// * `fidelity_target` - Required fidelity threshold for all simulations
/// * `max_latency_ms` - Maximum allowed latency for all simulations
/// * `runs` - Number of Monte Carlo runs per topology
/// * `strategy` - Optional routing strategy (None for default)
///
/// # Example
/// ```python
/// from qnet_core import QNetEngine, StrategyType
///
/// report = compare_topologies(
///     endpoints=[
///         TopologyEndpoints("telecom_backbone", "A", "C"),
///         TopologyEndpoints("hybrid_satellite_fiber", "Toronto", "London"),
///     ],
///     fidelity_target=0.75,
///     max_latency_ms=5000.0,
///     runs=1000,
///     strategy=StrategyType.HighestFidelity
/// )
///
/// print(f"Recommended: {report.recommended_topology}")
/// ```
#[pyfunction]
pub(crate) fn compare_topologies(
    endpoints: Vec<TopologyEndpoints>,
    fidelity_target: f64,
    max_latency_ms: f64,
    runs: usize,
    strategy: Option<PyStrategyType>,
) -> PyResult<TopologyComparisonReport> {
    use crate::topology::generator::{generate_topology as gen, TopologyType};

    let mut results: Vec<TopologyComparisonResult> = Vec::new();

    // Map topology names to generators
    let topology_map: std::collections::HashMap<&str, TopologyType> = [
        ("fiber_only", TopologyType::TelecomBackbone),
        ("telecom_backbone", TopologyType::TelecomBackbone),
        ("repeater_chain", TopologyType::RepeaterChain { length: 4 }),
        ("hybrid_satellite_fiber", TopologyType::HybridSatelliteFiber),
        ("hybrid", TopologyType::HybridSatelliteFiber),
    ].iter().cloned().collect();

    // Validate that all endpoints reference known topologies
    for ep in &endpoints {
        if !topology_map.contains_key(ep.topology_name.as_str()) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Unknown topology in endpoints: '{}'. Supported: fiber_only, telecom_backbone, repeater_chain, hybrid_satellite_fiber, hybrid", ep.topology_name)
            ));
        }
    }

    // Run simulation for each endpoint with correct source/target for that topology
    for (name, from_node, to_node) in endpoints.iter().map(|ep| (
        ep.topology_name.clone(),
        ep.from_node.clone(),
        ep.to_node.clone(),
    )) {
        let topology_type = topology_map.get(name.as_str());

        if let Some(t) = topology_type {
            let payload = gen(*t);

            // Validate that source/target nodes exist in the generated topology (before moving payload)
            let node_ids: Vec<&str> = payload.nodes.iter().map(|n| n.id.as_str()).collect();
            if !node_ids.contains(&from_node.as_str()) {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!(
                        "Source node '{}' not found in topology '{}'. Available nodes: {:?}",
                        from_node, name, node_ids
                    )
                ));
            }
            if !node_ids.contains(&to_node.as_str()) {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!(
                        "Target node '{}' not found in topology '{}'. Available nodes: {:?}",
                        to_node, name, node_ids
                    )
                ));
            }

            // Create engine with this topology (payload moved here)
            let mut engine = QNetEngine::new(None);
            engine.define_network(payload);

            // Run Monte Carlo simulation
            let request = crate::api::request::EntanglementRequest {
                from_node: from_node.clone(),
                to: to_node.clone(),
                fidelity_target,
                max_latency_ms,
                strategy: strategy.map(|s| s.0),
            };

            let stats = engine.simulate(request, runs, None);

            results.push(TopologyComparisonResult {
                topology_name: name.clone(),
                success_rate: stats.empirical_success_rate,
                mean_latency_ms: stats.mean_latency_ms,
                mean_fidelity: stats.mean_fidelity,
                link_utilization: stats.link_utilization_heatmap,
            });
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Unknown topology: {}. Supported: fiber_only, telecom_backbone, repeater_chain, hybrid_satellite_fiber, hybrid", name)
            ));
        }
    }

    // Determine recommended topology based on success rate
    let recommended = results
        .iter()
        .max_by(|a, b| a.success_rate.partial_cmp(&b.success_rate).unwrap())
        .map(|r| r.topology_name.clone())
        .unwrap_or_else(|| "unknown".to_string());

    // Generate summary
    let summary = generate_comparison_summary(&results, &recommended);

    // Use the first endpoint's source/target for report metadata
    let (source_node, target_node) = endpoints.first()
        .map(|ep| (ep.from_node.clone(), ep.to_node.clone()))
        .unwrap_or_else(|| (String::new(), String::new()));
    Ok(TopologyComparisonReport {
        source_node,
        target_node,
        fidelity_target,
        max_latency_ms,
        runs,
        results,
        recommended_topology: recommended,
        summary,
    })
}

/// Generate a human-readable summary of the topology comparison
fn generate_comparison_summary(results: &[TopologyComparisonResult], recommended: &str) -> String {
    if results.is_empty() {
        return "No topologies evaluated".to_string();
    }

    let mut summary = format!(
        "Evaluated {} topologies. Recommended: {}. ",
        results.len(),
        recommended
    );

    // Find best success rate and best latency
    let best_success = results.iter().max_by(|a, b| a.success_rate.partial_cmp(&b.success_rate).unwrap()).unwrap();
    let best_latency = results.iter().min_by(|a, b| a.mean_latency_ms.partial_cmp(&b.mean_latency_ms).unwrap()).unwrap();

    summary.push_str(&format!(
        "Best success rate: {:.0$} ({}), Best latency: {:.2}ms ({}).",
        (best_success.success_rate * 100.0f64) as usize,
        best_success.topology_name,
        best_latency.mean_latency_ms,
        best_latency.topology_name
    ));

    // Add fidelity info
    let avg_fidelity: f64 = results.iter().map(|r| r.mean_fidelity).sum::<f64>() / results.len() as f64;
    summary.push_str(&format!(" Average fidelity: {:.4}.", avg_fidelity));

    summary
}

// ============================================================================
// Python Helper Functions
// ============================================================================

/// Save a PyQNetFile to disk (module-level save alias for PyQNetFile.save())
#[pyfunction]
pub(crate) fn save_qnet_file_wrapper(qf: &PyQNetFile, filepath: &str) -> PyResult<()> {
    let rust_file: crate::api::request::QNetFile = qf.into();
    crate::io::save_qnet_file(filepath, &rust_file)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))
}

#[pyfunction]
pub(crate) fn save_topology(engine: &PyQNetEngine, filepath: &str) -> PyResult<()> {
    let result = engine.inner.save_topology(filepath);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)),
    }
}

#[pyfunction]
pub(crate) fn load_topology(engine: &mut PyQNetEngine, filepath: &str) -> PyResult<()> {
    let result = engine.inner.load_topology(filepath);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)),
    }
}
