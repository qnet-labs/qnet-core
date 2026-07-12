use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::types::*;
use crate::api::request::{
    QNetConfig, QNetConstraints, QNetFile, QNetLink, QNetMetadata, QNetNode,
};
use crate::diff::{diff_qnet_files, QNetDiff};
use crate::validation::QNetValidator;

// ============================================================================
// .qnet File Format Python Bindings
// ============================================================================

// ---------------------------------------------------------------------------
// Helper pyclasses mirroring Rust types in src/api/request.rs
// ---------------------------------------------------------------------------

#[pyclass]
#[derive(Clone)]
pub struct PyQNetMetadata {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub description: Option<String>,
    #[pyo3(get, set)]
    pub author: Option<String>,
    #[pyo3(get, set)]
    pub created_at: Option<String>,
}

#[pymethods]
impl PyQNetMetadata {
    #[new]
    fn new(
        name: String,
        description: Option<String>,
        author: Option<String>,
        created_at: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            author,
            created_at,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "PyQNetMetadata(name='{}', description={:?}, author={:?})",
            self.name, self.description, self.author
        )
    }
}

#[pyclass]
#[derive(Clone, Copy, Debug)]
pub struct PyQNetNodeType(pub crate::api::request::QNetNodeType);

#[pymethods]
impl PyQNetNodeType {
    #[classattr]
    const Ground: PyQNetNodeType = PyQNetNodeType(crate::api::request::QNetNodeType::Ground);
    #[classattr]
    const Satellite: PyQNetNodeType = PyQNetNodeType(crate::api::request::QNetNodeType::Satellite);
    #[classattr]
    const Repeater: PyQNetNodeType = PyQNetNodeType(crate::api::request::QNetNodeType::Repeater);

    fn __repr__(&self) -> String {
        match self.0 {
            crate::api::request::QNetNodeType::Ground => "QNetNodeType.Ground",
            crate::api::request::QNetNodeType::Satellite => "QNetNodeType.Satellite",
            crate::api::request::QNetNodeType::Repeater => "QNetNodeType.Repeater",
        }
        .to_string()
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct PyQNetLinkType(pub crate::api::request::QNetLinkType);

#[pymethods]
impl PyQNetLinkType {
    #[classattr]
    const Fiber: PyQNetLinkType = PyQNetLinkType(crate::api::request::QNetLinkType::Fiber);
    #[classattr]
    const Satellite: PyQNetLinkType = PyQNetLinkType(crate::api::request::QNetLinkType::Satellite);

    fn __repr__(&self) -> String {
        match self.0 {
            crate::api::request::QNetLinkType::Fiber => "QNetLinkType.Fiber",
            crate::api::request::QNetLinkType::Satellite => "QNetLinkType.Satellite",
        }
        .to_string()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyQNetSatelliteExtension {
    #[pyo3(get, set)]
    pub visibility: f64,
    #[pyo3(get, set)]
    pub weather_factor: f64,
}

#[pymethods]
impl PyQNetSatelliteExtension {
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

// ---------------------------------------------------------------------------
// Main .qnet container types (mirrors QNetFile / QNetNode / QNetLink etc.)
// ---------------------------------------------------------------------------

#[pyclass]
#[derive(Clone)]
pub struct PyQNetFile {
    #[pyo3(get, set)]
    pub version: String,
    #[pyo3(get, set)]
    pub metadata: PyQNetMetadata,
    #[pyo3(get, set)]
    pub nodes: Vec<PyQNetNode>,
    #[pyo3(get, set)]
    pub links: Vec<PyQNetLink>,
    #[pyo3(get, set)]
    pub config: Option<PyQNetConfig>,
    #[pyo3(get, set)]
    pub constraints: Option<PyQNetConstraints>,
    #[pyo3(get, set)]
    pub extensions: Option<PyObject>,
}

#[pyclass]
#[derive(Clone)]
pub struct PyQNetNode {
    #[pyo3(get, set)]
    pub id: String,
    #[pyo3(get, set)]
    pub memory_lifetime_ms: Option<f64>,
    #[pyo3(get, set)]
    pub memory_capacity: Option<u32>,
    #[pyo3(get, set)]
    pub node_type: Option<PyQNetNodeType>,
}

#[pymethods]
impl PyQNetNode {
    #[new]
    fn new(
        id: String,
        memory_lifetime_ms: Option<f64>,
        memory_capacity: Option<u32>,
        node_type: Option<PyQNetNodeType>,
    ) -> Self {
        Self {
            id,
            memory_lifetime_ms,
            memory_capacity,
            node_type,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "PyQNetNode(id='{}', memory={:?}, capacity={:?}, type={:?})",
            self.id, self.memory_lifetime_ms, self.memory_capacity, self.node_type
        )
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyQNetLink {
    #[pyo3(get, set)]
    pub id: String,
    /// Link source node (Python kwarg is "src" to avoid Python keyword conflict with "from")
    #[pyo3(get, set)]
    pub src: String,
    #[pyo3(get, set)]
    pub to: String,
    #[pyo3(get, set)]
    pub distance_km: f64,
    #[pyo3(get, set)]
    pub base_fidelity: f64,
    #[pyo3(get, set)]
    pub generation_rate_hz: f64,
    #[pyo3(get, set)]
    pub link_type: Option<PyQNetLinkType>,
    #[pyo3(get, set)]
    pub satellite: Option<PyQNetSatelliteExtension>,
}

#[pymethods]
impl PyQNetLink {
    #[new]
    fn new(
        id: String,
        src: String,
        to: String,
        distance_km: f64,
        base_fidelity: f64,
        generation_rate_hz: f64,
        link_type: Option<PyQNetLinkType>,
        satellite: Option<PyQNetSatelliteExtension>,
    ) -> Self {
        Self {
            id,
            src,
            to,
            distance_km,
            base_fidelity,
            generation_rate_hz,
            link_type,
            satellite,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "PyQNetLink(id={:?}, src='{}', to='{}', dist={}km, fid={:.2}, rate={}Hz)",
            self.id,
            self.src,
            self.to,
            self.distance_km,
            self.base_fidelity,
            self.generation_rate_hz
        )
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyQNetConfig {
    #[pyo3(get, set)]
    pub alpha_loss: Option<f64>,
    #[pyo3(get, set)]
    pub beta_fidelity_decay: Option<f64>,
    #[pyo3(get, set)]
    pub gamma_swapping: Option<f64>,
    #[pyo3(get, set)]
    pub max_attempts: Option<u32>,
}

#[pymethods]
impl PyQNetConfig {
    #[new]
    fn new(
        alpha_loss: Option<f64>,
        beta_fidelity_decay: Option<f64>,
        gamma_swapping: Option<f64>,
        max_attempts: Option<u32>,
    ) -> Self {
        Self {
            alpha_loss,
            beta_fidelity_decay,
            gamma_swapping,
            max_attempts,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "PyQNetConfig(alpha_loss={:?}, beta_fidelity_decay={:?}, gamma_swapping={:?}, max_attempts={:?})",
            self.alpha_loss, self.beta_fidelity_decay, self.gamma_swapping, self.max_attempts
        )
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyQNetConstraints {
    #[pyo3(get, set)]
    pub fidelity_target: Option<f64>,
    #[pyo3(get, set)]
    pub max_latency_ms: Option<f64>,
}

#[pymethods]
impl PyQNetConstraints {
    #[new]
    fn new(fidelity_target: Option<f64>, max_latency_ms: Option<f64>) -> Self {
        Self {
            fidelity_target,
            max_latency_ms,
        }
    }

    fn __eq__(&self, other: &PyQNetConstraints) -> bool {
        self.fidelity_target == other.fidelity_target && self.max_latency_ms == other.max_latency_ms
    }

    fn __repr__(&self) -> String {
        format!(
            "PyQNetConstraints(fidelity_target={:?}, max_latency_ms={:?})",
            self.fidelity_target, self.max_latency_ms
        )
    }
}

#[pymethods]
impl PyQNetFile {
    /// Create a new empty .qnet file with the given name.
    /// Version defaults to "1.0", all other fields start empty/null.
    #[new]
    fn new(name: String) -> Self {
        Self {
            version: "1.0".to_string(),
            metadata: PyQNetMetadata {
                name,
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

    /// Add a node with optional fields.
    /// If optional args are None, the .qnet file will write null for that field.
    #[pyo3(name = "add_node")]
    fn add_node(
        &mut self,
        id: String,
        memory_lifetime_ms: Option<f64>,
        memory_capacity: Option<u32>,
        node_type: Option<PyQNetNodeType>,
    ) {
        self.nodes.push(PyQNetNode {
            id: id,
            memory_lifetime_ms,
            memory_capacity,
            node_type,
        });
    }

    /// Add a link with optional fields.
    /// link_type and satellite default to None (writes null in .qnet JSON).
    #[pyo3(name = "add_link")]
    fn add_link(
        &mut self,
        id: String,
        src: String,
        to: String,
        distance_km: f64,
        base_fidelity: f64,
        generation_rate_hz: f64,
        link_type: Option<PyQNetLinkType>,
        satellite: Option<PyQNetSatelliteExtension>,
    ) {
        self.links.push(PyQNetLink {
            id,
            src,
            to,
            distance_km,
            base_fidelity,
            generation_rate_hz,
            link_type,
            satellite,
        });
    }

    /// Save this .qnet file to disk as pretty-printed JSON.
    fn save(&self, filepath: &str) -> PyResult<()> {
        let rust_file: QNetFile = (&*self).into();
        crate::io::save_qnet_file(filepath, &rust_file)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))
    }

    fn __repr__(&self) -> String {
        format!(
            "PyQNetFile(version='{}', metadata={{name:'{}'}}, nodes={}, links={})",
            self.version,
            self.metadata.name,
            self.nodes.len(),
            self.links.len()
        )
    }
}

// ============================================================================
// Conversion helpers: Python types <-> Rust API types
// ============================================================================

impl From<&PyQNetFile> for QNetFile {
    fn from(f: &PyQNetFile) -> Self {
        Self {
            version: f.version.clone(),
            metadata: QNetMetadata {
                name: f.metadata.name.clone(),
                description: f.metadata.description.clone(),
                author: f.metadata.author.clone(),
                created_at: f.metadata.created_at.clone(),
            },
            nodes: f
                .nodes
                .iter()
                .map(|n| QNetNode {
                    id: n.id.clone(),
                    memory_lifetime_ms: n.memory_lifetime_ms,
                    memory_capacity: n.memory_capacity.map(|v| v as u32),
                    node_type: n.node_type.map(|t| t.0),
                })
                .collect(),
            links: f
                .links
                .iter()
                .map(|l| QNetLink {
                    id: Some(l.id.clone()),
                    from: l.src.clone(),
                    to: l.to.clone(),
                    distance_km: l.distance_km,
                    base_fidelity: l.base_fidelity,
                    generation_rate_hz: l.generation_rate_hz,
                    link_type: l.link_type.map(|t| t.0),
                    satellite: l.satellite.as_ref().map(|s| {
                        crate::api::request::QNetSatelliteExtension {
                            visibility: s.visibility,
                            weather_factor: s.weather_factor,
                        }
                    }),
                })
                .collect(),
            config: f.config.as_ref().map(|c| QNetConfig {
                alpha_loss: c.alpha_loss,
                beta_fidelity_decay: c.beta_fidelity_decay,
                gamma_swapping: c.gamma_swapping,
                max_attempts: c.max_attempts.map(|v| v as u32),
            }),
            constraints: f.constraints.as_ref().map(|c| QNetConstraints {
                fidelity_target: c.fidelity_target,
                max_latency_ms: c.max_latency_ms,
            }),
            extensions: None, // Python bindings don't yet expose extensions
        }
    }
}

// ============================================================================
// Python functions for .qnet file I/O and validation
// ============================================================================

/// Load a .qnet file from disk and return it as a PyQNetFile object.
///
/// Validates version (must be "1.0") and graph connectivity before returning.
/// Raises RuntimeError on IO errors, parse errors, or validation failures.
#[pyfunction]
pub(crate) fn load_qnet_file(filepath: &str) -> PyResult<PyQNetFile> {
    let rust_file = crate::io::load_qnet_file(filepath).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to load {}: {}",
            filepath, e
        ))
    })?;

    Ok(PyQNetFile {
        version: rust_file.version,
        metadata: PyQNetMetadata {
            name: rust_file.metadata.name,
            description: rust_file.metadata.description,
            author: rust_file.metadata.author,
            created_at: rust_file.metadata.created_at,
        },
        nodes: rust_file
            .nodes
            .into_iter()
            .map(|n| PyQNetNode {
                id: n.id,
                memory_lifetime_ms: n.memory_lifetime_ms,
                memory_capacity: n.memory_capacity,
                node_type: n.node_type.map(|t| PyQNetNodeType(t)),
            })
            .collect(),
        links: rust_file
            .links
            .into_iter()
            .map(|l| PyQNetLink {
                id: l.id.clone().unwrap_or_default(),
                src: l.from,
                to: l.to,
                distance_km: l.distance_km,
                base_fidelity: l.base_fidelity,
                generation_rate_hz: l.generation_rate_hz,
                link_type: l.link_type.map(|t| PyQNetLinkType(t)),
                satellite: l.satellite.map(|s| PyQNetSatelliteExtension {
                    visibility: s.visibility,
                    weather_factor: s.weather_factor,
                }),
            })
            .collect(),
        config: rust_file.config.map(|c| PyQNetConfig {
            alpha_loss: c.alpha_loss,
            beta_fidelity_decay: c.beta_fidelity_decay,
            gamma_swapping: c.gamma_swapping,
            max_attempts: c.max_attempts.map(|v| v as u32),
        }),
        constraints: rust_file.constraints.map(|c| PyQNetConstraints {
            fidelity_target: c.fidelity_target,
            max_latency_ms: c.max_latency_ms,
        }),
        extensions: None, // Python bindings don't yet expose extensions
    })
}

/// Validate a .qnet file and return structured results as a Python dict.
///
/// Returns:
///     dict with keys:
///         - "valid" (bool): True if the file is valid
///         - "filepath" (str): The path that was validated
///         - "errors" (list[dict]): List of error dicts with "type" and "message" keys
///         - "warnings" (list[dict]): List of warning dicts with "message" and optional "path" keys
#[pyfunction]
pub(crate) fn validate(filepath: &str) -> PyResult<PyObject> {
    let qnet_file = crate::io::load_qnet_file(filepath).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to load {}: {}",
            filepath, e
        ))
    })?;

    let validation = QNetValidator::validate_all(&qnet_file);

    Python::with_gil(|py| {
        let dict = PyDict::new(py);
        dict.set_item("valid", validation.is_valid)?;
        dict.set_item("filepath", filepath)?;

        // Structured error list
        let py_errors = pyo3::types::PyList::new(py, Vec::<PyObject>::new());
        for error in &validation.errors {
            let err_dict = PyDict::new(py);
            let err_type_str = match &error.error_type {
                crate::validation::ValidationErrorKind::DuplicateNodeId { id } => {
                    format!("DuplicateNodeId: {}", id)
                }
                crate::validation::ValidationErrorKind::UnknownNodeReference {
                    node_id,
                    link_id,
                } => {
                    format!(
                        "UnknownNodeReference: {}{}",
                        node_id,
                        link_id
                            .as_ref()
                            .map(|l| format!("[{}]", l))
                            .unwrap_or_default()
                    )
                }
                crate::validation::ValidationErrorKind::SelfLoop { link_id } => {
                    format!("SelfLoop: {}", link_id)
                }
                crate::validation::ValidationErrorKind::InvalidDistance { link_id, value } => {
                    format!("InvalidDistance: {} km for link '{}'", value, link_id)
                }
                crate::validation::ValidationErrorKind::InvalidFidelity { link_id, value } => {
                    format!("InvalidFidelity: {} for link '{}'", value, link_id)
                }
                crate::validation::ValidationErrorKind::InvalidGenerationRate {
                    link_id,
                    value,
                } => {
                    format!("InvalidGenerationRate: {} Hz for link '{}'", value, link_id)
                }
                crate::validation::ValidationErrorKind::GraphDisconnected { component_count } => {
                    format!("GraphDisconnected: {} components", component_count)
                }
            };
            err_dict.set_item("type", err_type_str)?;
            err_dict.set_item("message", error.to_string())?;
            if let Some(ref path) = error.path {
                err_dict.set_item("path", path)?;
            }
            py_errors.append(err_dict)?;
        }
        dict.set_item("errors", py_errors)?;

        // Structured warning list
        let py_warnings = pyo3::types::PyList::new(py, Vec::<PyObject>::new());
        for warning in &validation.warnings {
            let warn_dict = PyDict::new(py);
            warn_dict.set_item("message", &warning.message)?;
            if let Some(ref path) = warning.path {
                warn_dict.set_item("path", path)?;
            }
            py_warnings.append(warn_dict)?;
        }
        dict.set_item("warnings", py_warnings)?;

        Ok::<_, PyErr>(dict.into_py(py))
    })
}

#[pyfunction]
pub(crate) fn diff(file1: &str, file2: &str) -> PyResult<PyObject> {
    let qnet1 = crate::io::load_qnet_file(file1);
    let qnet2 = crate::io::load_qnet_file(file2);

    Python::with_gil(|py| {
        let dict = PyDict::new(py);

        match (qnet1, qnet2) {
            (Ok(f1), Ok(f2)) => {
                let diff_result = diff_qnet_files(file1, file2, &f1, &f2);
                dict.set_item("summary", diff_result.summary)?;
                dict.set_item("nodes_added", diff_result.nodes_added)?;
                dict.set_item("nodes_removed", diff_result.nodes_removed)?;
                dict.set_item("nodes_modified", diff_result.nodes_modified)?;
                dict.set_item("links_added", diff_result.links_added)?;
                dict.set_item("links_removed", diff_result.links_removed)?;
                dict.set_item("links_modified", diff_result.links_modified)?;
            }
            (Err(e1), _) => {
                dict.set_item("error", format!("Failed to load {}: {}", file1, e1))?;
            }
            (_, Err(e2)) => {
                dict.set_item("error", format!("Failed to load {}: {}", file2, e2))?;
            }
        }

        Ok(dict.into_py(py))
    })
}

// ============================================================================
// Topology snapshot diff (feature-gated)
// ============================================================================

#[cfg(feature = "python")]
use crate::api::request::{
    PyTopologyDiff, PyTopologySnapshot, TopologyConfig, TopologyDiff, TopologyMetadata,
    TopologySnapshot,
};

#[cfg(feature = "python")]
#[pyfunction]
pub(crate) fn diff_topologies(
    snapshot1: PyTopologySnapshot,
    snapshot2: PyTopologySnapshot,
) -> PyResult<PyTopologyDiff> {
    let diff1 = TopologySnapshot {
        metadata: TopologyMetadata {
            name: snapshot1.metadata.name.clone(),
            version: snapshot1.metadata.version.clone(),
        },
        nodes: snapshot1
            .nodes
            .into_iter()
            .map(|n| crate::api::request::NodeDefinition {
                id: n.id,
                memory_lifetime_t2: n.memory_lifetime_t2,
            })
            .collect(),
        links: snapshot1
            .links
            .into_iter()
            .map(|l| crate::api::request::LinkDefinition {
                from_node: l.from_node,
                to: l.to,
                distance_km: l.distance_km,
                base_fidelity: l.base_fidelity,
                generation_rate_hz: l.generation_rate_hz,
                link_type: l.link_type.0,
                satellite_conditions: l.satellite_conditions.map(|sc| {
                    crate::api::request::SatelliteConditions {
                        visibility: sc.visibility,
                        weather_factor: sc.weather_factor,
                    }
                }),
            })
            .collect(),
        config: TopologyConfig {
            alpha_loss: snapshot1.config.alpha_loss,
            gamma_swapping: snapshot1.config.gamma_swapping,
        },
    };
    let diff2 = TopologySnapshot {
        metadata: TopologyMetadata {
            name: snapshot2.metadata.name.clone(),
            version: snapshot2.metadata.version.clone(),
        },
        nodes: snapshot2
            .nodes
            .into_iter()
            .map(|n| crate::api::request::NodeDefinition {
                id: n.id,
                memory_lifetime_t2: n.memory_lifetime_t2,
            })
            .collect(),
        links: snapshot2
            .links
            .into_iter()
            .map(|l| crate::api::request::LinkDefinition {
                from_node: l.from_node,
                to: l.to,
                distance_km: l.distance_km,
                base_fidelity: l.base_fidelity,
                generation_rate_hz: l.generation_rate_hz,
                link_type: l.link_type.0,
                satellite_conditions: l.satellite_conditions.map(|sc| {
                    crate::api::request::SatelliteConditions {
                        visibility: sc.visibility,
                        weather_factor: sc.weather_factor,
                    }
                }),
            })
            .collect(),
        config: TopologyConfig {
            alpha_loss: snapshot2.config.alpha_loss,
            gamma_swapping: snapshot2.config.gamma_swapping,
        },
    };
    let diff = crate::engine::QNetEngine::diff_topologies(&diff1, &diff2);
    Ok(PyTopologyDiff {
        name: diff.name,
        nodes_added: diff.nodes_added,
        nodes_removed: diff.nodes_removed,
        nodes_modified: diff.nodes_modified,
        links_added: diff.links_added,
        links_removed: diff.links_removed,
        links_modified: diff.links_modified,
        summary: diff.summary,
    })
}
