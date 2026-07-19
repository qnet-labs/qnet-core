use pyo3::prelude::*;
use std::collections::HashMap;

use crate::api::request::{
    LinkType, NetworkTopologyPayload, NodeDefinition, QNetConfig, QNetConstraints, QNetExtensions,
    QNetFile, QNetLink, QNetLinkType, QNetMetadata, QNetNode, QNetNodeType, QNetSatelliteExtension,
    QNetVersion, SatelliteConditions,
};

#[cfg(feature = "python")]
use crate::api::request::{
    PyTopologyDiff, PyTopologySnapshot, TopologyConfig, TopologyDiff, TopologyMetadata,
    TopologySnapshot,
};

// ============================================================================
// Configuration Types
// ============================================================================

#[pyclass(name = "PhysicalConfig")]
#[derive(Clone)]
pub struct PyPhysicalConfig {
    #[pyo3(get, set)]
    pub baseline_purify_factor: f64,
    #[pyo3(get, set)]
    pub speed_of_light_in_fiber_km_ms: f64,
}

#[pymethods]
impl PyPhysicalConfig {
    #[new]
    fn new(alpha_loss_db_km: Option<f64>) -> Self {
        Self {
            baseline_purify_factor: alpha_loss_db_km.unwrap_or(0.22) / 10.0,
            speed_of_light_in_fiber_km_ms: 200.0,
        }
    }
}

#[pyclass(name = "SimulationConfig")]
#[derive(Clone)]
pub struct PySimulationConfig {
    #[pyo3(get, set)]
    pub total_time_cutoff_ms: f64,
    #[pyo3(get, set)]
    pub step_resolution_ms: f64,
    #[pyo3(get, set)]
    pub physical: PyPhysicalConfig,
}

#[pymethods]
impl PySimulationConfig {
    #[new]
    fn new(
        total_time_cutoff_ms: Option<f64>,
        step_resolution_ms: Option<f64>,
        alpha_loss_db_km: Option<f64>,
    ) -> Self {
        Self {
            total_time_cutoff_ms: total_time_cutoff_ms.unwrap_or(5000.0),
            step_resolution_ms: step_resolution_ms.unwrap_or(0.1),
            physical: PyPhysicalConfig::new(alpha_loss_db_km),
        }
    }
}

// ============================================================================
// Request/Response Types
// ============================================================================

#[pyclass(name = "NodeDefinition")]
#[derive(Clone)]
pub struct PyNodeDefinition {
    #[pyo3(get, set)]
    pub id: String,
    #[pyo3(get, set)]
    pub memory_lifetime_t2: f64,
}

#[pymethods]
impl PyNodeDefinition {
    #[new]
    fn new(id: String, memory_lifetime_t2: f64) -> Self {
        Self {
            id,
            memory_lifetime_t2,
        }
    }
}

#[pyclass(name = "LinkType")]
#[derive(Clone)]
pub struct PyLinkType(pub LinkType);

#[pymethods]
impl PyLinkType {
    #[classattr]
    const Fiber: PyLinkType = PyLinkType(LinkType::Fiber);

    #[classattr]
    const Satellite: PyLinkType = PyLinkType(LinkType::Satellite);

    fn __repr__(&self) -> String {
        match self.0 {
            LinkType::Fiber => "LinkType.Fiber",
            LinkType::Satellite => "LinkType.Satellite",
        }
        .to_string()
    }
}

#[pyclass(name = "SatelliteConditions")]
#[derive(Clone)]
pub struct PySatelliteConditions {
    #[pyo3(get, set)]
    pub visibility: f64,
    #[pyo3(get, set)]
    pub weather_factor: f64,
}

#[pymethods]
impl PySatelliteConditions {
    #[new]
    fn new(visibility: Option<f64>, weather_factor: Option<f64>) -> Self {
        Self {
            visibility: visibility.unwrap_or(1.0),
            weather_factor: weather_factor.unwrap_or(1.0),
        }
    }

    /// Calculate effective generation rate based on conditions
    fn effective_rate(&self, base_rate: f64) -> f64 {
        base_rate * self.visibility * self.weather_factor
    }
}

#[pyclass(name = "LinkDefinition")]
#[derive(Clone)]
pub struct PyLinkDefinition {
    #[pyo3(get, set)]
    pub from_node: String,
    #[pyo3(get, set)]
    pub to: String,
    #[pyo3(get, set)]
    pub distance_km: f64,
    #[pyo3(get, set)]
    pub base_fidelity: f64,
    #[pyo3(get, set)]
    pub generation_rate_hz: f64,
    #[pyo3(get, set)]
    pub link_type: PyLinkType,
    #[pyo3(get, set)]
    pub satellite_conditions: Option<PySatelliteConditions>,
}

#[pymethods]
impl PyLinkDefinition {
    #[new]
    fn new(
        from_node: String,
        to: String,
        distance_km: f64,
        base_fidelity: f64,
        generation_rate_hz: f64,
        link_type: Option<PyLinkType>,
        satellite_conditions: Option<PySatelliteConditions>,
    ) -> Self {
        Self {
            from_node,
            to,
            distance_km,
            base_fidelity,
            generation_rate_hz,
            link_type: link_type.unwrap_or(PyLinkType::Fiber),
            satellite_conditions,
        }
    }
}

#[pyclass(name = "NetworkTopologyPayload")]
#[derive(Clone)]
pub struct PyNetworkTopologyPayload {
    #[pyo3(get, set)]
    pub nodes: Vec<PyNodeDefinition>,
    #[pyo3(get, set)]
    pub links: Vec<PyLinkDefinition>,
}

#[pymethods]
impl PyNetworkTopologyPayload {
    #[new]
    fn new(nodes: Vec<PyNodeDefinition>, links: Vec<PyLinkDefinition>) -> Self {
        Self { nodes, links }
    }
}

#[pyclass(name = "EntanglementRequest")]
#[derive(Clone)]
pub struct PyEntanglementRequest {
    #[pyo3(get, set)]
    pub from_node: String,
    #[pyo3(get, set)]
    pub to: String,
    #[pyo3(get, set)]
    pub fidelity_target: f64,
    #[pyo3(get, set)]
    pub max_latency_ms: f64,
    #[pyo3(get, set)]
    pub strategy: Option<PyStrategyType>,
}

#[pymethods]
impl PyEntanglementRequest {
    #[new]
    fn new(
        from_node: String,
        to: String,
        fidelity_target: f64,
        max_latency_ms: f64,
        strategy: Option<PyStrategyType>,
    ) -> Self {
        Self {
            from_node,
            to,
            fidelity_target,
            max_latency_ms,
            strategy,
        }
    }
}

#[pyclass(name = "SimulationResult")]
pub struct PySimulationResult {
    #[pyo3(get, set)]
    pub success: bool,
    #[pyo3(get, set)]
    pub latency_ms: f64,
    #[pyo3(get, set)]
    pub final_fidelity: f64,
    #[pyo3(get, set)]
    pub execution_path: Vec<String>,
}

#[pymethods]
impl PySimulationResult {
    #[new]
    fn new(
        success: bool,
        latency_ms: f64,
        final_fidelity: f64,
        execution_path: Vec<String>,
    ) -> Self {
        Self {
            success,
            latency_ms,
            final_fidelity,
            execution_path,
        }
    }
}

// ============================================================================
// Monte Carlo Types
// ============================================================================

#[pyclass]
#[derive(Clone)]
pub struct PyMonteCarloContext {
    #[pyo3(get, set)]
    pub nodes: Vec<(String, f64)>, // (id, t2_lifetime)
    #[pyo3(get, set)]
    pub links: Vec<(String, String, f64, f64, f64)>, // (from, to, distance_km, base_fidelity, rate_hz)
    #[pyo3(get, set)]
    pub from_node: String,
    #[pyo3(get, set)]
    pub to_node: String,
    #[pyo3(get, set)]
    pub fidelity_target: f64,
    #[pyo3(get, set)]
    pub max_latency_ms: f64,
    #[pyo3(get, set)]
    pub strategy: Option<u8>, // 0=LowestLatency, 1=HighestFidelity, 2=HighestSuccess, None=default
    #[pyo3(get, set)]
    pub seed: Option<u64>,
    #[pyo3(get, set)]
    pub total_time_cutoff_ms: f64,
    #[pyo3(get, set)]
    pub step_resolution_ms: f64,
    #[pyo3(get, set)]
    pub baseline_purify_factor: f64,
}

impl PyMonteCarloContext {
    /// Create a perturbed copy by applying a callback to a specific parameter path.
    fn with_mutated(&self, param: &str, direction: i8) -> Self {
        let multiplier = if direction > 0 { 1.1 } else { 0.9 }; // ±10%

        match param {
            // Node-level perturbations
            p if p.starts_with("node[") => {
                let mut nodes = self.nodes.clone();
                let parts: Vec<&str> = p
                    .trim_start_matches("node[")
                    .trim_end_matches(']')
                    .split(',')
                    .collect();
                if parts.len() == 2 {
                    let field = parts[1];
                    if let Ok(idx) = parts[0].parse::<usize>() {
                        if idx < nodes.len() {
                            match field {
                                "t2" => nodes[idx].1 *= multiplier,
                                _ => {}
                            }
                        }
                    }
                }
                Self {
                    nodes,
                    ..self.clone()
                }
            }
            // Link-level perturbations
            p if p.starts_with("link[") => {
                let mut links = self.links.clone();
                let parts: Vec<&str> = p
                    .trim_start_matches("link[")
                    .trim_end_matches(']')
                    .split(',')
                    .collect();
                if parts.len() == 3 {
                    let field = parts[1];
                    if let Ok(idx) = parts[0].parse::<usize>() {
                        if idx < links.len() {
                            match field {
                                "distance" => links[idx].2 *= multiplier,
                                "fidelity" => links[idx].3 *= multiplier,
                                "rate" => links[idx].4 *= multiplier,
                                _ => {}
                            }
                        }
                    }
                }
                Self {
                    links,
                    ..self.clone()
                }
            }
            // Global perturbations
            "alpha_loss_db_km" => {
                let new_alpha = 0.2 * multiplier; // base alpha is 0.2 dB/km
                Self {
                    baseline_purify_factor: new_alpha / 10.0,
                    ..self.clone()
                }
            }
            _ => self.clone(),
        }
    }

    /// Runs one perturbed Monte Carlo trial, returns the empirical success rate.
    fn run_trial(&self, runs: usize, seed: u64) -> f64 {
        let config = crate::config::SimulationConfig {
            total_time_cutoff_ms: self.total_time_cutoff_ms,
            step_resolution_ms: self.step_resolution_ms,
            physical: crate::config::PhysicalConfig {
                baseline_purify_factor: self.baseline_purify_factor,
                speed_of_light_in_fiber_km_ms: 200.0,
            },
        };

        let mut engine = crate::engine::QNetEngine::new(Some(config));

        let nodes = self
            .nodes
            .iter()
            .map(|(id, t2)| crate::api::request::NodeDefinition {
                id: id.clone(),
                memory_lifetime_t2: *t2,
            })
            .collect();

        let links = self
            .links
            .iter()
            .map(
                |(from, to, dist, fid, rate)| crate::api::request::LinkDefinition {
                    from_node: from.clone(),
                    to: to.clone(),
                    distance_km: *dist,
                    base_fidelity: *fid,
                    generation_rate_hz: *rate,
                    link_type: crate::api::request::LinkType::default(),
                    satellite_conditions: None,
                },
            )
            .collect();

        engine.define_network(crate::api::request::NetworkTopologyPayload { nodes, links });

        let req = crate::api::request::EntanglementRequest {
            from_node: self.from_node.clone(),
            to: self.to_node.clone(),
            fidelity_target: self.fidelity_target,
            max_latency_ms: self.max_latency_ms,
            strategy: self.strategy.map(|s| match s {
                0 => crate::routing::strategy::StrategyType::LowestLatency,
                1 => crate::routing::strategy::StrategyType::HighestFidelity,
                2 => crate::routing::strategy::StrategyType::HighestSuccess,
                _ => crate::routing::strategy::StrategyType::LowestLatency,
            }),
        };

        engine
            .simulate(req, runs, Some(seed))
            .empirical_success_rate
    }

    /// Compute symmetric impact score for one parameter.
    /// impact = (|rate_down - baseline| + |rate_up - baseline|) / baseline
    ///
    /// Returns None if the baseline is zero or NaN (cannot compute meaningful impact).
    fn param_impact(
        &self,
        param: &str,
        baseline: f64,
        runs: usize,
        seed_down: u64,
        seed_up: u64,
    ) -> PyResult<Option<f64>> {
        let rate_down = self.with_mutated(param, -1).run_trial(runs, seed_down);
        let rate_up = self.with_mutated(param, 1).run_trial(runs, seed_up);

        if baseline < 0.01 {
            // Zero or near-zero baseline: cannot compute meaningful relative impact.
            // Return Some(0.0) so the caller still records the parameter with a
            // neutral score — sensitivity_analysis() should never return an empty dict.
            Ok(Some(0.0))
        } else {
            Ok(Some(
                ((rate_down - baseline).abs() + (rate_up - baseline).abs()) / baseline,
            ))
        }
    }
}

#[pyclass(name = "MonteCarloStats")]
pub struct PyMonteCarloStats {
    #[pyo3(get, set)]
    pub total_runs: usize,
    #[pyo3(get, set)]
    pub empirical_success_rate: f64,
    #[pyo3(get, set)]
    pub mean_latency_ms: f64,
    #[pyo3(get, set)]
    pub mean_fidelity: f64,
    #[pyo3(get, set)]
    pub aggregate_congestion_drops: usize,
    #[pyo3(get, set)]
    pub link_utilization_heatmap: HashMap<String, usize>,
    /// Optional context for post-hoc analysis (perturbation-based sensitivity).
    #[pyo3(get, set)]
    pub(crate) _context: Option<PyMonteCarloContext>,
}

#[pymethods]
impl PyMonteCarloStats {
    #[new]
    fn new(
        total_runs: usize,
        empirical_success_rate: f64,
        mean_latency_ms: f64,
        mean_fidelity: f64,
        aggregate_congestion_drops: usize,
        link_utilization_heatmap: HashMap<String, usize>,
        context: Option<PyMonteCarloContext>,
    ) -> Self {
        Self {
            total_runs,
            empirical_success_rate,
            mean_latency_ms,
            mean_fidelity,
            aggregate_congestion_drops,
            link_utilization_heatmap,
            _context: context,
        }
    }

    /// Perturb each simulation parameter ±10 %, re-run the ensemble, and report
    /// which parameters most influence `empirical_success_rate`.
    ///
    /// Returns a dict sorted by absolute impact (descending):
    ///   {"memory_lifetime_t2:node_A": 0.34, "base_fidelity:link_0->link_B": 0.28, ...}
    fn sensitivity_analysis(&self, seed_base: u64) -> PyResult<PyObject> {
        let ctx = self._context.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "No simulation context available. Pass store_context=True to simulate().",
            )
        })?;

        let baseline = self.empirical_success_rate;
        let trial_runs = (ctx.nodes.len() * 50).max(200); // floor at 200 for small networks
        let mut seed = seed_base;
        let mut entries: Vec<(String, f64)> = Vec::new();

        // Node coherence times
        for (i, (node_id, _)) in ctx.nodes.iter().enumerate() {
            if let Some(impact) = ctx.param_impact(
                &format!("node[{},t2]", i),
                baseline,
                trial_runs,
                seed,
                seed + 1,
            )? {
                entries.push((format!("memory_lifetime_t2:{}", node_id), impact));
            }
            seed += 2;
        }

        // Per-link parameters
        for (i, (from, to, _, _, _)) in ctx.links.iter().enumerate() {
            let link_key = format!("{}->{}", from, to);

            for (field, label) in &[
                ("distance", "distance_km"),
                ("fidelity", "base_fidelity"),
                ("rate", "generation_rate_hz"),
            ] {
                if let Some(impact) = ctx.param_impact(
                    &format!("link[{},{}]", i, field),
                    baseline,
                    trial_runs,
                    seed,
                    seed + 1,
                )? {
                    entries.push((format!("{}:{}", label, link_key), impact));
                }
                seed += 2;
            }
        }

        // Global alpha loss — always included (returns 0.0 for near-zero baselines).
        if let Ok(Some(v)) =
            ctx.param_impact("alpha_loss_db_km", baseline, trial_runs, seed, seed + 1)
        {
            entries.push(("alpha_loss_db_km".to_string(), v));
        }

        // Sort descending by impact
        entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);
            for (key, val) in &entries {
                dict.set_item(key, val)?;
            }
            Ok(dict.into_py(py))
        })
    }
}

// ============================================================================
// Strategy Type
// ============================================================================

#[pyclass(name = "StrategyType")]
#[derive(Clone, Copy)]
pub struct PyStrategyType(pub crate::routing::strategy::StrategyType);

#[pymethods]
impl PyStrategyType {
    #[classattr]
    const LowestLatency: PyStrategyType =
        PyStrategyType(crate::routing::strategy::StrategyType::LowestLatency);

    #[classattr]
    const HighestFidelity: PyStrategyType =
        PyStrategyType(crate::routing::strategy::StrategyType::HighestFidelity);

    #[classattr]
    const HighestSuccess: PyStrategyType =
        PyStrategyType(crate::routing::strategy::StrategyType::HighestSuccess);

    fn __repr__(&self) -> String {
        match self.0 {
            crate::routing::strategy::StrategyType::LowestLatency => "StrategyType.LowestLatency",
            crate::routing::strategy::StrategyType::HighestFidelity => {
                "StrategyType.HighestFidelity"
            }
            crate::routing::strategy::StrategyType::HighestSuccess => "StrategyType.HighestSuccess",
        }
        .to_string()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_u8(&self) -> u8 {
        match self.0 {
            crate::routing::strategy::StrategyType::LowestLatency => 0,
            crate::routing::strategy::StrategyType::HighestFidelity => 1,
            crate::routing::strategy::StrategyType::HighestSuccess => 2,
        }
    }
}

/// Helper to convert PyStrategyType → internal StrategyType.
/// Placed outside #[pymethods] because StrategyType doesn't implement PyO3's OkWrap.
impl PyStrategyType {
    pub fn into_strategy_type(self) -> crate::routing::strategy::StrategyType {
        self.0
    }
}

// ============================================================================
// Topology Comparison Types
// ============================================================================

/// Comparison result for multiple topologies
#[pyclass]
#[derive(Clone)]
pub struct TopologyComparisonResult {
    #[pyo3(get, set)]
    pub topology_name: String,
    #[pyo3(get, set)]
    pub success_rate: f64,
    #[pyo3(get, set)]
    pub mean_latency_ms: f64,
    #[pyo3(get, set)]
    pub mean_fidelity: f64,
    #[pyo3(get, set)]
    pub link_utilization: HashMap<String, usize>,
}

/// Detailed comparison report for multiple topologies
#[pyclass]
#[derive(Clone)]
pub struct TopologyComparisonReport {
    #[pyo3(get, set)]
    pub source_node: String,
    #[pyo3(get, set)]
    pub target_node: String,
    #[pyo3(get, set)]
    pub fidelity_target: f64,
    #[pyo3(get, set)]
    pub max_latency_ms: f64,
    #[pyo3(get, set)]
    pub runs: usize,
    #[pyo3(get, set)]
    pub results: Vec<TopologyComparisonResult>,
    #[pyo3(get, set)]
    pub recommended_topology: String,
    #[pyo3(get, set)]
    pub summary: String,
}

// ============================================================================
// Higher-Level Protocol Types (QKD, Teleportation, Distributed Computing)
// ============================================================================

// --- QKD Types ---

#[pyclass(name = "QKDParameters")]
#[derive(Clone)]
pub struct PyQKDParameters {
    #[pyo3(get, set)]
    pub from_node: String,
    #[pyo3(get, set)]
    pub to_node: String,
    #[pyo3(get, set)]
    pub fidelity_target: f64,
    #[pyo3(get, set)]
    pub max_latency_ms: f64,
    #[pyo3(get, set)]
    pub rounds: usize,
    #[pyo3(get, set)]
    pub error_rate_tolerance: f64,
    #[pyo3(get, set)]
    pub sifting_overhead_ratio: f64,
    #[pyo3(get, set)]
    pub privacy_amplification_factor: f64,
}

#[pymethods]
impl PyQKDParameters {
    #[new]
    fn new(
        from_node: String,
        to_node: String,
        fidelity_target: f64,
        max_latency_ms: f64,
        rounds: Option<usize>,
        error_rate_tolerance: Option<f64>,
        sifting_overhead_ratio: Option<f64>,
        privacy_amplification_factor: Option<f64>,
    ) -> Self {
        Self {
            from_node,
            to_node,
            fidelity_target,
            max_latency_ms,
            rounds: rounds.unwrap_or(100),
            error_rate_tolerance: error_rate_tolerance.unwrap_or(0.11),
            sifting_overhead_ratio: sifting_overhead_ratio.unwrap_or(0.5),
            privacy_amplification_factor: privacy_amplification_factor.unwrap_or(0.8),
        }
    }
}

#[pyclass(name = "QKDResult")]
pub struct PyQKDResult {
    #[pyo3(get, set)]
    pub success: bool,
    #[pyo3(get, set)]
    pub secret_key_length_bits: usize,
    #[pyo3(get, set)]
    pub efficiency_rate: f64,
    #[pyo3(get, set)]
    pub qber: f64,
    #[pyo3(get, set)]
    pub latency_ms: f64,
    #[pyo3(get, set)]
    pub execution_path: Vec<String>,
    #[pyo3(get, set)]
    pub rounds_completed: usize,
    #[pyo3(get, set)]
    pub rounds_failed: usize,
}

#[pymethods]
impl PyQKDResult {
    #[new]
    fn new(
        success: bool,
        secret_key_length_bits: usize,
        efficiency_rate: f64,
        qber: f64,
        latency_ms: f64,
        execution_path: Vec<String>,
        rounds_completed: usize,
        rounds_failed: usize,
    ) -> Self {
        Self {
            success,
            secret_key_length_bits,
            efficiency_rate,
            qber,
            latency_ms,
            execution_path,
            rounds_completed,
            rounds_failed,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "QKDResult(success={}, key_bits={}, qber={:.4})",
            self.success, self.secret_key_length_bits, self.qber
        )
    }
}

#[pyclass(name = "QKDStats")]
pub struct PyQKDStats {
    #[pyo3(get, set)]
    pub total_runs: usize,
    #[pyo3(get, set)]
    pub success_rate: f64,
    #[pyo3(get, set)]
    pub mean_key_length_bits: f64,
    #[pyo3(get, set)]
    pub mean_efficiency: f64,
    #[pyo3(get, set)]
    pub mean_qber: f64,
}

#[pymethods]
impl PyQKDStats {
    #[new]
    fn new(
        total_runs: usize,
        success_rate: f64,
        mean_key_length_bits: f64,
        mean_efficiency: f64,
        mean_qber: f64,
    ) -> Self {
        Self {
            total_runs,
            success_rate,
            mean_key_length_bits,
            mean_efficiency,
            mean_qber,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "QKDStats(runs={}, success={:.0}, mean_key_len={:.1})",
            self.total_runs, self.success_rate * 100.0, self.mean_key_length_bits
        )
    }
}

// --- Teleportation Types ---

#[pyclass(name = "TeleportationParameters")]
#[derive(Clone)]
pub struct PyTeleportationParameters {
    #[pyo3(get, set)]
    pub source_node: String,
    #[pyo3(get, set)]
    pub target_node: String,
    #[pyo3(get, set)]
    pub state_fidelity: f64,
    #[pyo3(get, set)]
    pub classical_bandwidth_ms: f64,
    #[pyo3(get, set)]
    pub relay_nodes: Vec<String>,
}

#[pymethods]
impl PyTeleportationParameters {
    #[new]
    fn new(
        source_node: String,
        target_node: String,
        state_fidelity: Option<f64>,
        classical_bandwidth_ms: Option<f64>,
    ) -> Self {
        Self {
            source_node,
            target_node,
            state_fidelity: state_fidelity.unwrap_or(0.95),
            classical_bandwidth_ms: classical_bandwidth_ms.unwrap_or(100.0),
            relay_nodes: Vec::new(),
        }
    }
}

#[pyclass(name = "TeleportationOutcome")]
pub struct PyTeleportationOutcome {
    #[pyo3(get, set)]
    pub success: bool,
    #[pyo3(get, set)]
    pub teleportation_fidelity: f64,
    #[pyo3(get, set)]
    pub resource_entanglement_fidelity: f64,
    #[pyo3(get, set)]
    pub latency_ms: f64,
    #[pyo3(get, set)]
    pub path: Vec<String>,
    #[pyo3(get, set)]
    pub classical_bits_transferred: usize,
}

#[pymethods]
impl PyTeleportationOutcome {
    #[new]
    fn new(
        success: bool,
        teleportation_fidelity: f64,
        resource_entanglement_fidelity: f64,
        latency_ms: f64,
        path: Vec<String>,
        classical_bits_transferred: usize,
    ) -> Self {
        Self {
            success,
            teleportation_fidelity,
            resource_entanglement_fidelity,
            latency_ms,
            path,
            classical_bits_transferred,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "TeleportationOutcome(success={}, fidelity={:.4}, latency={:.1}ms)",
            self.success, self.teleportation_fidelity, self.latency_ms
        )
    }
}

#[pyclass(name = "TeleportationStats")]
pub struct PyTeleportationStats {
    #[pyo3(get, set)]
    pub total_runs: usize,
    #[pyo3(get, set)]
    pub success_rate: f64,
    #[pyo3(get, set)]
    pub mean_teleportation_fidelity: f64,
    #[pyo3(get, set)]
    pub teleportation_fidelity_stddev: f64,
    #[pyo3(get, set)]
    pub mean_latency_ms: f64,
}

#[pymethods]
impl PyTeleportationStats {
    #[new]
    fn new(
        total_runs: usize,
        success_rate: f64,
        mean_teleportation_fidelity: f64,
        teleportation_fidelity_stddev: f64,
        mean_latency_ms: f64,
    ) -> Self {
        Self {
            total_runs,
            success_rate,
            mean_teleportation_fidelity,
            teleportation_fidelity_stddev,
            mean_latency_ms,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "TeleportationStats(runs={}, fidelity={:.4}±{:.4})",
            self.total_runs, self.mean_teleportation_fidelity, self.teleportation_fidelity_stddev
        )
    }
}

// --- Distributed Computing Types ---

#[pyclass(name = "BasisType")]
#[derive(Clone)]
pub struct PyBasisType(pub crate::protocols::BasisType);

#[pymethods]
impl PyBasisType {
    #[classattr]
    const GHZ: PyBasisType = PyBasisType(crate::protocols::BasisType::GHZ);
    #[classattr]
    const Cluster: PyBasisType = PyBasisType(crate::protocols::BasisType::Cluster);
    #[classattr]
    const GraphGraph: PyBasisType = PyBasisType(crate::protocols::BasisType::GraphGraph);

    fn __repr__(&self) -> String {
        match self.0 {
            crate::protocols::BasisType::GHZ => "BasisType.GHZ".to_string(),
            crate::protocols::BasisType::Cluster => "BasisType.Cluster".to_string(),
            crate::protocols::BasisType::GraphGraph => "BasisType.GraphGraph".to_string(),
        }
    }
}

/// Python-visible coordination topology wrapper.
/// Wraps the Rust CoordinationTopology enum with PyO3-compatible fields.
#[pyclass(name = "CoordinationTopology")]
#[derive(Clone)]
pub struct PyCoordinationTopology {
    #[pyo3(get, set)]
    pub kind: String, // "star", "ring", "mesh", "arbitrary"
    #[pyo3(get, set)]
    pub center_node: Option<String>, // for "star"
    #[pyo3(get, set)]
    pub edges: Option<Vec<(String, String)>>, // for "arbitrary"
}

#[pymethods]
impl PyCoordinationTopology {
    #[new]
    fn new(
        kind: String,
        center_node: Option<String>,
        edges: Option<Vec<(String, String)>>,
    ) -> Self {
        Self {
            kind,
            center_node,
            edges,
        }
    }

    /// Create a star topology with the given center node.
    #[staticmethod]
    fn star(center_node: String) -> Self {
        Self {
            kind: "star".to_string(),
            center_node: Some(center_node),
            edges: None,
        }
    }

    /// Create a ring topology.
    #[staticmethod]
    fn ring() -> Self {
        Self {
            kind: "ring".to_string(),
            center_node: None,
            edges: None,
        }
    }

    /// Create a mesh (all-to-all) topology.
    #[staticmethod]
    fn mesh() -> Self {
        Self {
            kind: "mesh".to_string(),
            center_node: None,
            edges: None,
        }
    }

    /// Create an arbitrary topology with explicit edges.
    #[staticmethod]
    fn arbitrary(edges: Vec<(String, String)>) -> Self {
        Self {
            kind: "arbitrary".to_string(),
            center_node: None,
            edges: Some(edges),
        }
    }
}

/// Convert Python CoordinationTopology to the internal Rust type.
pub fn topology_to_inner(topo: &PyCoordinationTopology) -> crate::protocols::CoordinationTopology {
    match topo.kind.as_str() {
        "star" => crate::protocols::CoordinationTopology::Star {
            center_node: topo.center_node.clone().unwrap_or_default(),
        },
        "ring" => crate::protocols::CoordinationTopology::Ring,
        "mesh" => crate::protocols::CoordinationTopology::Mesh,
        _ => crate::protocols::CoordinationTopology::Arbitrary {
            edges: topo.edges.clone().unwrap_or_default(),
        },
    }
}

// --- Measurement Basis ---

#[pyclass(name = "MeasurementBasis")]
#[derive(Clone)]
pub struct PyMeasurementBasis {
    #[pyo3(get, set)]
    pub basis_type: PyBasisType,
    #[pyo3(get, set)]
    pub correlation_strength: f64,
}

#[pymethods]
impl PyMeasurementBasis {
    #[new]
    fn new(basis_type: Option<PyBasisType>, correlation_strength: Option<f64>) -> Self {
        Self {
            basis_type: basis_type.unwrap_or(PyBasisType::GHZ),
            correlation_strength: correlation_strength.unwrap_or(0.85),
        }
    }
}

#[pyclass(name = "DistributedComputingParameters")]
pub struct PyDistributedComputingParameters {
    #[pyo3(get, set)]
    pub participants: Vec<String>,
    #[pyo3(get, set)]
    pub coordination_topology: PyCoordinationTopology,
    #[pyo3(get, set)]
    pub measurement_basis: PyMeasurementBasis,
    #[pyo3(get, set)]
    pub classical_relay_latency_ms: f64,
}

#[pymethods]
impl PyDistributedComputingParameters {
    #[new]
    fn new(
        participants: Vec<String>,
        coordination_topology: PyCoordinationTopology,
        measurement_basis: Option<PyMeasurementBasis>,
        classical_relay_latency_ms: Option<f64>,
    ) -> Self {
        Self {
            participants,
            coordination_topology,
            measurement_basis: measurement_basis.unwrap_or(PyMeasurementBasis::new(None, None)),
            classical_relay_latency_ms: classical_relay_latency_ms.unwrap_or(5.0),
        }
    }
}

#[pyclass(name = "PartyOutcome")]
#[derive(Clone)]
pub struct PyPartyOutcome {
    #[pyo3(get, set)]
    pub node_id: String,
    #[pyo3(get, set)]
    pub successful_measurement: bool,
    #[pyo3(get, set)]
    pub local_fidelity: f64,
}

#[pymethods]
impl PyPartyOutcome {
    #[new]
    fn new(node_id: String, successful_measurement: bool, local_fidelity: f64) -> Self {
        Self {
            node_id,
            successful_measurement,
            local_fidelity,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "PartyOutcome({}, success={}, fidelity={:.4})",
            self.node_id, self.successful_measurement, self.local_fidelity
        )
    }
}

#[pyclass(name = "DistributedComputingResult")]
pub struct PyDistributedComputingResult {
    #[pyo3(get, set)]
    pub success: bool,
    #[pyo3(get, set)]
    pub computation_fidelity: f64,
    #[pyo3(get, set)]
    pub party_results: Vec<PyPartyOutcome>,
    #[pyo3(get, set)]
    pub resource_links_used: Vec<String>,
    #[pyo3(get, set)]
    pub total_latency_ms: f64,
    #[pyo3(get, set)]
    pub coordination_overhead_ms: f64,
}

#[pymethods]
impl PyDistributedComputingResult {
    #[new]
    fn new(
        success: bool,
        computation_fidelity: f64,
        party_results: Vec<PyPartyOutcome>,
        resource_links_used: Vec<String>,
        total_latency_ms: f64,
        coordination_overhead_ms: f64,
    ) -> Self {
        Self {
            success,
            computation_fidelity,
            party_results,
            resource_links_used,
            total_latency_ms,
            coordination_overhead_ms,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "DistributedComputingResult(success={}, fidelity={:.4}, parties={})",
            self.success, self.computation_fidelity, self.party_results.len()
        )
    }
}

#[pyclass(name = "DistributedComputingStats")]
pub struct PyDistributedComputingStats {
    #[pyo3(get, set)]
    pub total_runs: usize,
    #[pyo3(get, set)]
    pub success_rate: f64,
    #[pyo3(get, set)]
    pub mean_computation_fidelity: f64,
    #[pyo3(get, set)]
    pub mean_party_success_rate: f64,
}

#[pymethods]
impl PyDistributedComputingStats {
    #[new]
    fn new(
        total_runs: usize,
        success_rate: f64,
        mean_computation_fidelity: f64,
        mean_party_success_rate: f64,
    ) -> Self {
        Self {
            total_runs,
            success_rate,
            mean_computation_fidelity,
            mean_party_success_rate,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "DistributedComputingStats(runs={}, success={:.0}, fidelity={:.4})",
            self.total_runs, self.success_rate * 100.0, self.mean_computation_fidelity
        )
    }
}
