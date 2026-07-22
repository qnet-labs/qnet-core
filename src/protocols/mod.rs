// Higher-level quantum protocol implementations
// Each module provides a concrete protocol struct with static methods that build on top of
// the link-layer entanglement distribution provided by QNetEngine.

pub mod distributed;
pub mod qkd;
pub mod teleportation;

// Re-export result types from api::response (they are defined in api::response, not locally)
pub use crate::api::response::{
    DistributedComputingResult, DistributedComputingStats, PartyOutcome,
};
pub use crate::api::response::{QKDResult, QKDStats, TeleportationOutcome, TeleportationStats};

// Re-export request types and protocol engines
pub use distributed::DistributedComputingParameters;
pub use distributed::DistributedComputingProtocol;
pub use distributed::{BasisType, CoordinationTopology, MeasurementBasis};
pub use qkd::QKDParameters;
pub use qkd::QKDProtocol;
pub use teleportation::TeleportationParameters;
pub use teleportation::TeleportationProtocol;

// PurificationEngine is kept here for backward compatibility with scheduler.rs
mod purification;
pub use purification::PurificationEngine;
