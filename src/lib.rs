// Internal Simulation, Hardware, and Physics Subsystems
mod config;
mod metrics;
mod scheduler;
mod montecarlo;
mod simulation;
mod network;
mod routing;
mod memory;
mod swapping;
mod protocols;

// Topology Generators
pub mod topology;

// Public Boundary Gateway and Orchestration Layers
mod api;
mod engine;

// Python FFI Bridge
#[cfg(feature = "python")]
pub mod python_bridge;

// .qnet File Format Support
pub mod io;
pub mod validation;
pub mod diff;

// Strict Boundary Contract Exports
pub use engine::QNetEngine;
pub use api::request::{EntanglementRequest, NetworkTopologyPayload, NodeDefinition, LinkDefinition, LinkType, SatelliteConditions, TopologySnapshot, TopologyMetadata, TopologyConfig, TopologyDiff, QNetFile, QNetNode, QNetLink, QNetConfig, QNetConstraints, QNetMetadata, QNetExtensions, QNetNodeType, QNetLinkType, QNetVersion};
pub use api::response::{SimulationResult, MonteCarloStats};
pub use routing::strategy::StrategyType;
pub use config::SimulationConfig;
pub use topology::generator::{generate_topology, TopologyType};
pub use io::{load_qnet_file, save_qnet_file, QNetError};
pub use validation::{QNetValidator, ValidationResult, ValidationError, ValidationErrorKind};
pub use diff::{diff_qnet_files, QNetDiff, MetadataDiff, ConfigDiff, ConstraintsDiff};
