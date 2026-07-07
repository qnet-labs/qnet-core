//! Python FFI bridge for qnet-core.
//!
//! Split into submodules by concern:
//! - `types` — all #[pyclass] struct definitions (pure data)
//! - `engine` — PyQNetEngine, topology generators/comparisons (simulation workflow)
//! - `qnet_io` — .qnet file types, From impls, load/validate/diff functions (serialization)

pub mod types;
pub mod engine;
pub mod qnet_io;

// Re-export all public types for external use
pub use types::*;
pub use engine::*;
pub use qnet_io::*;

// Bring #[pyfunction] items into scope for wrap_pyfunction! macro.
// These are the absolute paths from the crate root (super:: resolves to crate:: not crate::python_bridge).
use crate::python_bridge::engine::{compare_topologies, from_qnet_file_py, generate_topology, load_topology, save_qnet_file_wrapper, save_topology};
use crate::python_bridge::qnet_io::{diff, load_qnet_file, validate};

#[cfg(feature = "python")]
use crate::python_bridge::qnet_io::diff_topologies;

use pyo3::prelude::*;
use pyo3::types::PyModule;

/// The top-level Python module registration.
/// Registered as `qnet_core` in Cargo.toml via [lib] name = "qnet_core".
#[pymodule]
fn qnet_core(_py: Python, m: &PyModule) -> PyResult<()> {
    // Classes
    m.add_class::<PyQNetEngine>()?;
    m.add_class::<PyPhysicalConfig>()?;
    m.add_class::<PyLinkDefinition>()?;
    m.add_class::<PySatelliteConditions>()?;
    m.add_class::<PyEntanglementRequest>()?;
    m.add_class::<PyMonteCarloStats>()?;
    m.add_class::<TopologyComparisonResult>()?;
    m.add_class::<TopologyComparisonReport>()?;
    m.add_class::<PyLinkType>()?;
    m.add_class::<PyStrategyType>()?;
    m.add_class::<PySimulationConfig>()?;
    m.add_class::<PyNodeDefinition>()?;
    m.add_class::<PyNetworkTopologyPayload>()?;
    m.add_class::<PyQNetFile>()?;
    m.add_class::<PyQNetNode>()?;
    m.add_class::<PyQNetLink>()?;
    m.add_class::<PyQNetConfig>()?;
    m.add_class::<PyQNetConstraints>()?;
    m.add_class::<PyQNetMetadata>()?;
    m.add_class::<PyQNetNodeType>()?;
    m.add_class::<PyQNetLinkType>()?;
    m.add_class::<PyQNetSatelliteExtension>()?;
    m.add_class::<TopologyEndpoints>()?;

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

    // Export clean aliases (without Py prefix) for backward compatibility
    m.add("QNetEngine", m.getattr("PyQNetEngine")?)?;
    m.add("StrategyType", m.getattr("PyStrategyType")?)?;
    m.add("NodeDefinition", m.getattr("PyNodeDefinition")?)?;
    m.add("LinkDefinition", m.getattr("PyLinkDefinition")?)?;
    m.add("EntanglementRequest", m.getattr("PyEntanglementRequest")?)?;
    m.add("MonteCarloStats", m.getattr("PyMonteCarloStats")?)?;
    m.add("SimulationConfig", m.getattr("PySimulationConfig")?)?;
    m.add("PhysicalConfig", m.getattr("PyPhysicalConfig")?)?;
    m.add("SatelliteConditions", m.getattr("PySatelliteConditions")?)?;
    m.add("LinkType", m.getattr("PyLinkType")?)?;

    // Export PyQNetFile as qnet_file for backward compatibility
    m.add("PyQNetFile", m.getattr("PyQNetFile")?)?;

    // Convenience aliases
    m.add("load", m.getattr("load_qnet_file")?)?;
    m.add_function(wrap_pyfunction!(save_qnet_file_wrapper, m)?)?;
    m.add("save", m.getattr("save_qnet_file_wrapper")?)?;

    Ok(())
}
