// Internal Simulation, Hardware, and Physics Subsystems
mod config;
mod memory;
mod metrics;
mod montecarlo;
mod network;
mod routing;
mod scheduler;
mod simulation;
mod swapping;

// Higher-level protocol implementations (QKD, teleportation, distributed computing)
pub mod protocols;

// Topology Generators
pub mod topology;

// Strict Boundary Gateway and Orchestration Layers
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
pub use protocols::{DistributedComputingResult, DistributedComputingStats, PartyOutcome, QKDResult, QKDStats, TeleportationOutcome, TeleportationStats};
pub use protocols::qkd::QKDParameters;
pub use protocols::teleportation::TeleportationParameters;
pub use protocols::distributed::{CoordinationTopology, DistributedComputingParameters, MeasurementBasis, BasisType};
pub use config::SimulationConfig;
pub use diff::{diff_qnet_files, ConfigDiff, ConstraintsDiff, MetadataDiff, QNetDiff};
pub use engine::QNetEngine;
pub use io::{load_qnet_file, save_qnet_file, QNetError};
pub use routing::strategy::StrategyType;
pub use topology::generator::{generate_topology, TopologyType};
pub use validation::{QNetValidator, ValidationError, ValidationErrorKind, ValidationResult};
