// Internal Simulation, Hardware, and Physics Subsystems
mod config;
mod memory;
mod metrics;
mod montecarlo;
mod network;
mod protocols;
mod routing;
mod scheduler;
mod simulation;
mod swapping;

// Topology Generators
pub mod topology;

// Public Boundary Gateway and Orchestration Layers
mod api;
mod engine;

// Python FFI Bridge
#[cfg(feature = "python")]
pub mod python_bridge;

// .qnet File Format Support
pub mod diff;
pub mod io;
pub mod validation;

// Strict Boundary Contract Exports
pub use api::request::{
    EntanglementRequest, LinkDefinition, LinkType, NetworkTopologyPayload, NodeDefinition,
    QNetConfig, QNetConstraints, QNetExtensions, QNetFile, QNetLink, QNetLinkType, QNetMetadata,
    QNetNode, QNetNodeType, QNetVersion, SatelliteConditions, TopologyConfig, TopologyDiff,
    TopologyMetadata, TopologySnapshot,
};
pub use api::response::{MonteCarloStats, SimulationResult};
pub use config::SimulationConfig;
pub use diff::{diff_qnet_files, ConfigDiff, ConstraintsDiff, MetadataDiff, QNetDiff};
pub use engine::QNetEngine;
pub use io::{load_qnet_file, save_qnet_file, QNetError};
pub use routing::strategy::StrategyType;
pub use topology::generator::{generate_topology, TopologyType};
pub use validation::{QNetValidator, ValidationError, ValidationErrorKind, ValidationResult};
