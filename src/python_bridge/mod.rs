//! Python FFI bridge for qnet-core.
//!
//! Split into submodules by concern:
//! - `types` — all #[pyclass] struct definitions (pure data)
//! - `engine` — PyQNetEngine, topology generators/comparisons (simulation workflow)
//! - `qnet_io` — .qnet file types, From impls, load/validate/diff functions (serialization)

pub mod engine;
pub mod qnet_io;
pub mod types;

// Re-export all public types for external use
pub use engine::*;
pub use qnet_io::*;
pub use types::*;

// Bring #[pyfunction] items into scope for wrap_pyfunction! macro.
// These are the absolute paths from the crate root (super:: resolves to crate:: not crate::python_bridge).
use crate::python_bridge::engine::{
    compare_topologies, from_qnet_file_py, generate_topology, load_topology,
    save_qnet_file_wrapper, save_topology,
};
use crate::python_bridge::qnet_io::{diff, load_qnet_file, validate};

#[cfg(feature = "python")]
use crate::python_bridge::qnet_io::diff_topologies;

use pyo3::prelude::*;
use pyo3::types::PyModule;

/// The top-level Python module registration.
/// Registered as `qnet_core` in Cargo.toml via [lib] name = "qnet_core".
#[pymodule]
fn qnet_core(_py: Python, m: &PyModule) -> PyResult<()> {
    // Classes (each has #[pyclass(name = "CleanName")] — Python-visible name is clean, Rust struct keeps Py prefix)
    m.add_class::<PyQNetEngine>()?;           // → QNetEngine
    m.add_class::<PyPhysicalConfig>()?;       // → PhysicalConfig
    m.add_class::<PyLinkDefinition>()?;       // → LinkDefinition
    m.add_class::<PySatelliteConditions>()?;  // → SatelliteConditions
    m.add_class::<PyEntanglementRequest>()?;  // → EntanglementRequest
    m.add_class::<PyMonteCarloStats>()?;      // → MonteCarloStats
    m.add_class::<TopologyComparisonResult>()?; // → TopologyComparisonResult (already clean)
    m.add_class::<TopologyComparisonReport>()?; // → TopologyComparisonReport (already clean)
    m.add_class::<PyLinkType>()?;             // → LinkType
    m.add_class::<PyStrategyType>()?;         // → StrategyType
    m.add_class::<PySimulationConfig>()?;     // → SimulationConfig
    m.add_class::<PyNodeDefinition>()?;       // → NodeDefinition
    m.add_class::<PyNetworkTopologyPayload>()?; // → NetworkTopologyPayload
    m.add_class::<PyQNetFile>()?;             // → QNetFile
    m.add_class::<PyQNetNode>()?;             // → QNetNode
    m.add_class::<PyQNetLink>()?;             // → QNetLink
    m.add_class::<PyQNetConfig>()?;           // → QNetConfig
    m.add_class::<PyQNetConstraints>()?;      // → QNetConstraints
    m.add_class::<PyQNetMetadata>()?;         // → QNetMetadata
    m.add_class::<PyQNetNodeType>()?;         // → QNetNodeType
    m.add_class::<PyQNetLinkType>()?;         // → QNetLinkType
    m.add_class::<PyQNetSatelliteExtension>()?; // → QNetSatelliteExtension
    m.add_class::<TopologyEndpoints>()?;      // → TopologyEndpoints (already clean)

    // Functions
    m.add_function(wrap_pyfunction!(generate_topology, m)?)?;
    m.add_function(wrap_pyfunction!(compare_topologies, m)?)?;
    m.add_function(wrap_pyfunction!(save_topology, m)?)?;
    m.add_function(wrap_pyfunction!(load_topology, m)?)?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_function(wrap_pyfunction!(load_qnet_file, m)?)?;
    m.add_function(wrap_pyfunction!(diff, m)?)?;
    #[cfg(feature = "python")]
    m.add_function(wrap_pyfunction!(diff_topologies, m)?)?;

    // Export from_qnet_file function
    m.add_function(wrap_pyfunction!(from_qnet_file_py, m)?)?;

    // All classes above expose clean Python names via #[pyclass(name = "...")], so no aliases needed.
    // These Rust struct names are for backward compat — users can still access them if they ever need to:
    // m.add("PyQNetFile", m.getattr("PyQNetFile")?)?;  // → QNetFile (redundant)

    // Convenience aliases
    m.add("load", m.getattr("load_qnet_file")?)?;
    m.add_function(wrap_pyfunction!(save_qnet_file_wrapper, m)?)?;
    m.add("save", m.getattr("save_qnet_file_wrapper")?)?;

    Ok(())
}
