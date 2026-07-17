use crate::routing::strategy::StrategyType;

/// Node definition for quantum network
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDefinition {
    pub id: String,
    pub memory_lifetime_t2: f64,
}

/// Simulation configuration for topology serialization
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyConfig {
    /// Alpha loss coefficient (dB/km) for fiber attenuation
    pub alpha_loss: f64,
    /// Gamma swapping fidelity factor for repeater operations
    pub gamma_swapping: f64,
}

impl Default for TopologyConfig {
    fn default() -> Self {
        Self {
            alpha_loss: 0.2,
            gamma_swapping: 0.85,
        }
    }
}

/// Complete topology snapshot with metadata
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologySnapshot {
    /// Metadata about the topology
    pub metadata: TopologyMetadata,
    /// Network nodes
    pub nodes: Vec<NodeDefinition>,
    /// Network links
    pub links: Vec<LinkDefinition>,
    /// Simulation configuration
    pub config: TopologyConfig,
}

// PyO3 integration for TopologySnapshot
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "TopologySnapshot")]
#[derive(Clone)]
pub struct PyTopologySnapshot {
    #[pyo3(get, set)]
    pub metadata: PyTopologyMetadata,
    #[pyo3(get, set)]
    pub nodes: Vec<crate::python_bridge::PyNodeDefinition>,
    #[pyo3(get, set)]
    pub links: Vec<crate::python_bridge::PyLinkDefinition>,
    #[pyo3(get, set)]
    pub config: PyTopologyConfig,
}

#[cfg(feature = "python")]
#[pyclass(name = "TopologyMetadata")]
#[derive(Clone)]
pub struct PyTopologyMetadata {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub version: String,
}

#[cfg(feature = "python")]
#[pyclass(name = "TopologyConfig")]
#[derive(Clone)]
pub struct PyTopologyConfig {
    #[pyo3(get, set)]
    pub alpha_loss: f64,
    #[pyo3(get, set)]
    pub gamma_swapping: f64,
}

/// Metadata for topology snapshots
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyMetadata {
    /// Name of the topology
    pub name: String,
    /// Version identifier
    pub version: String,
}

impl Default for TopologyMetadata {
    fn default() -> Self {
        Self {
            name: String::from("unnamed"),
            version: String::from("1.0"),
        }
    }
}

impl TopologySnapshot {
    /// Create a new topology snapshot
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            metadata: TopologyMetadata {
                name: name.to_string(),
                version: version.to_string(),
            },
            nodes: Vec::new(),
            links: Vec::new(),
            config: TopologyConfig::default(),
        }
    }

    /// Add a node to the topology
    pub fn add_node(&mut self, id: &str, memory_lifetime_t2: f64) {
        self.nodes.push(NodeDefinition {
            id: id.to_string(),
            memory_lifetime_t2,
        });
    }

    /// Add a link to the topology
    pub fn add_link(
        &mut self,
        from: &str,
        to: &str,
        distance_km: f64,
        base_fidelity: f64,
        generation_rate_hz: f64,
    ) {
        self.links.push(LinkDefinition {
            from_node: from.to_string(),
            to: to.to_string(),
            distance_km,
            base_fidelity,
            generation_rate_hz,
            link_type: LinkType::Fiber,
            satellite_conditions: None,
        });
    }
}

/// Result of comparing two topology snapshots
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyDiff {
    /// Name of the topology
    pub name: String,
    /// Nodes added in the diff
    pub nodes_added: Vec<String>,
    /// Nodes removed in the diff
    pub nodes_removed: Vec<String>,
    /// Nodes modified in the diff
    pub nodes_modified: Vec<String>,
    /// Links added in the diff
    pub links_added: Vec<String>,
    /// Links removed in the diff
    pub links_removed: Vec<String>,
    /// Links modified in the diff
    pub links_modified: Vec<String>,
    /// Summary of the diff
    pub summary: String,
}

// PyO3 integration for TopologyDiff
#[cfg(feature = "python")]
#[pyclass(name = "TopologyDiff")]
#[derive(Clone)]
pub struct PyTopologyDiff {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub nodes_added: Vec<String>,
    #[pyo3(get, set)]
    pub nodes_removed: Vec<String>,
    #[pyo3(get, set)]
    pub nodes_modified: Vec<String>,
    #[pyo3(get, set)]
    pub links_added: Vec<String>,
    #[pyo3(get, set)]
    pub links_removed: Vec<String>,
    #[pyo3(get, set)]
    pub links_modified: Vec<String>,
    #[pyo3(get, set)]
    pub summary: String,
}

impl TopologyDiff {
    /// Create a new empty topology diff
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            nodes_added: Vec::new(),
            nodes_removed: Vec::new(),
            nodes_modified: Vec::new(),
            links_added: Vec::new(),
            links_removed: Vec::new(),
            links_modified: Vec::new(),
            summary: String::new(),
        }
    }

    /// Generate a human-readable summary
    pub fn generate_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str(&format!(
            "Diff for '{}': {} nodes added, {} nodes removed, {} nodes modified. ",
            self.name,
            self.nodes_added.len(),
            self.nodes_removed.len(),
            self.nodes_modified.len()
        ));
        summary.push_str(&format!(
            "{} links added, {} links removed, {} links modified.",
            self.links_added.len(),
            self.links_removed.len(),
            self.links_modified.len()
        ));
        summary
    }
}

/// Link type for quantum network connections
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum LinkType {
    #[default]
    /// Fiber-optic link: high bandwidth, moderate loss over distance
    Fiber,
    /// Satellite link: lower loss over long distances, but limited bandwidth
    Satellite,
}

/// Physical conditions for satellite links
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SatelliteConditions {
    /// Visibility factor (0.0-1.0) - affected by weather, atmospheric conditions
    pub visibility: f64,
    /// Weather penalty factor (0.0-1.0) - rain, fog, cloud cover
    pub weather_factor: f64,
}

impl Default for SatelliteConditions {
    fn default() -> Self {
        Self {
            visibility: 1.0,
            weather_factor: 1.0,
        }
    }
}

impl SatelliteConditions {
    /// Calculate effective generation rate based on conditions
    pub fn effective_rate(&self, base_rate: f64) -> f64 {
        base_rate * self.visibility * self.weather_factor
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LinkDefinition {
    pub from_node: String,
    pub to: String,
    pub distance_km: f64,
    pub base_fidelity: f64,
    pub generation_rate_hz: f64,
    #[serde(default)]
    pub link_type: LinkType,
    #[serde(default)]
    pub satellite_conditions: Option<SatelliteConditions>,
}

impl Default for LinkDefinition {
    fn default() -> Self {
        Self {
            from_node: String::new(),
            to: String::new(),
            distance_km: 0.0,
            base_fidelity: 0.0,
            generation_rate_hz: 0.0,
            link_type: LinkType::Fiber,
            satellite_conditions: None,
        }
    }
}

impl LinkDefinition {
    /// Get the effective generation rate considering link type and satellite conditions
    pub fn effective_rate(&self) -> f64 {
        match self.link_type {
            LinkType::Fiber => self.generation_rate_hz,
            LinkType::Satellite => {
                if let Some(conditions) = &self.satellite_conditions {
                    conditions.effective_rate(self.generation_rate_hz)
                } else {
                    self.generation_rate_hz
                }
            }
        }
    }

    /// Get the effective fidelity considering link type physics
    pub fn effective_fidelity(&self, distance_km: f64) -> f64 {
        match self.link_type {
            LinkType::Fiber => {
                // Exponential loss for fiber: fidelity decreases with distance
                // Typical fiber attenuation: 0.2 dB/km at 1550nm
                let alpha_db_km = 0.2;
                let loss_db = alpha_db_km * distance_km;
                let transmission_efficiency = 10f64.powf(-loss_db / 10.0);
                // Base fidelity decays with transmission efficiency
                self.base_fidelity * transmission_efficiency + (1.0 - transmission_efficiency) * 0.5
            }
            LinkType::Satellite => {
                // Lower loss for satellite, but still distance-dependent
                // Free-space optical: much lower attenuation (~0.02 dB/km equivalent)
                let alpha_db_km = 0.02;
                let loss_db = alpha_db_km * distance_km;
                let transmission_efficiency = 10f64.powf(-loss_db / 10.0);
                // Weather conditions already factored into effective_rate
                self.base_fidelity * transmission_efficiency + (1.0 - transmission_efficiency) * 0.5
            }
        }
    }
}

#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkTopologyPayload {
    pub nodes: Vec<NodeDefinition>,
    pub links: Vec<LinkDefinition>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntanglementRequest {
    pub from_node: String,
    pub to: String,
    pub fidelity_target: f64,
    pub max_latency_ms: f64,
    pub strategy: Option<StrategyType>,
}

// PyO3 integration for NetworkTopologyPayload
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymethods]
impl NetworkTopologyPayload {
    fn __repr__(&self) -> String {
        format!(
            "NetworkTopologyPayload(nodes={}, links={})",
            self.nodes.len(),
            self.links.len()
        )
    }
}

// ============================================================================
// .qnet File Format Types
// ============================================================================

/// QNet file format version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QNetVersion {
    V1_0,
}

impl std::fmt::Display for QNetVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QNetVersion::V1_0 => write!(f, "1.0"),
        }
    }
}

impl std::str::FromStr for QNetVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.0" => Ok(QNetVersion::V1_0),
            v => Err(format!("Unsupported version: {}", v)),
        }
    }
}

/// Node type for quantum network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QNetNodeType {
    #[default]
    Ground,
    Satellite,
    Repeater,
}

/// Link type for qnet format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QNetLinkType {
    #[default]
    Fiber,
    Satellite,
}

/// Satellite extension for links
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// QNet file format uses snake_case to match the .qnet file convention
// #[serde(rename_all = "camelCase")]
pub struct QNetSatelliteExtension {
    pub visibility: f64,
    pub weather_factor: f64,
}

impl Default for QNetSatelliteExtension {
    fn default() -> Self {
        Self {
            visibility: 1.0,
            weather_factor: 1.0,
        }
    }
}

/// Enhanced node definition for qnet format
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct QNetNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_lifetime_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_capacity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<QNetNodeType>,
}

/// Enhanced link definition for qnet format
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct QNetLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub from: String,
    pub to: String,
    pub distance_km: f64,
    pub base_fidelity: f64,
    pub generation_rate_hz: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<QNetLinkType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite: Option<QNetSatelliteExtension>,
}

/// Simulation configuration for qnet format
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct QNetConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_loss: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta_fidelity_decay: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma_swapping: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

impl Default for QNetConfig {
    fn default() -> Self {
        Self {
            alpha_loss: Some(0.04),
            beta_fidelity_decay: Some(0.02),
            gamma_swapping: Some(0.85),
            max_attempts: Some(100),
        }
    }
}

/// Constraints for network simulation
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct QNetConstraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fidelity_target: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency_ms: Option<f64>,
}

/// Metadata for qnet files
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct QNetMetadata {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Extensions for qnet format (future-proofing)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct QNetExtensions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<serde_json::Value>,
}

/// Top-level qnet file format container
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct QNetFile {
    pub version: String,
    pub metadata: QNetMetadata,
    pub nodes: Vec<QNetNode>,
    pub links: Vec<QNetLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<QNetConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<QNetConstraints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<QNetExtensions>,
}

impl QNetFile {
    pub fn new(name: &str) -> Self {
        Self {
            version: "1.0".to_string(),
            metadata: QNetMetadata {
                name: name.to_string(),
                description: None,
                author: None,
                created_at: None,
            },
            nodes: Vec::new(),
            links: Vec::new(),
            config: None,
            constraints: None,
            extensions: None,
        }
    }

    pub fn add_node(&mut self, id: &str) {
        self.nodes.push(QNetNode {
            id: id.to_string(),
            memory_lifetime_ms: None,
            memory_capacity: None,
            node_type: None,
        });
    }

    pub fn add_link(
        &mut self,
        from: &str,
        to: &str,
        distance_km: f64,
        base_fidelity: f64,
        generation_rate_hz: f64,
    ) {
        self.links.push(QNetLink {
            id: None,
            from: from.to_string(),
            to: to.to_string(),
            distance_km,
            base_fidelity,
            generation_rate_hz,
            link_type: None,
            satellite: None,
        });
    }
}
