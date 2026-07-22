# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **New protocols**
  - QKD (Quantum Key Distribution) protocol — `run_qkd()`, with `QKDResult`, `QKDStats`, and `QKDParameters` Python bindings
  - Entanglement Teleportation protocol — `execute_teleportation()`, with `TeleportationOutcome`, `TeleportationStats`, and `TeleportationParameters` Python bindings
  - Distributed Computing protocol — `run_distributed_computation()`, with party-based star/mesh topologies, `PartyOutcome`, `DistributedComputingResult`, `DistributedComputingStats`, and `CoordinationTopology`/`MeasurementBasis` types
- **Protocol module refactoring** — extracted `PurificationEngine` from monolithic `protocols.rs` into a dedicated `src/protocols/` subdirectory (`purification.rs`, `qkd.rs`, `teleportation.rs`, `distributed.rs`)
- **16 new examples and tutorials** in `examples/`: basic entanglement, Monte Carlo ensembles, routing strategy comparison, builtin topology generators, satellite-fiber links, QKD key exchange, teleportation relay, distributed computing (star + mesh), .qnet file I/O, topology validation and diffing, and physical constant tuning
- **`from_qnet_file()` API** — load network topologies from `.qnet` files programmatically
- Pre-built topology generators (telecom backbone, repeater chain, hybrid satellite-fiber)
- Topology comparison and diffing tools
- `.qnet` file format for saving/loading network topologies
- PyO3 Python bindings with comprehensive API coverage

### Changed
- `src/lib.rs` — `mod protocols` promoted to `pub mod protocols` with re-exports for all new protocol types
- Python bridge (`src/python_bridge/`) — added 11 new PyO3 pyclasses and 3 module-level convenience functions (`qkd()`, `teleportation()`, `distributed_computation()`)

### Removed
- Root-level example network files (`invalid_network.qnet`, `network_v1.qnet`, `network_v2.qnet`) — moved into structured examples directory with documentation

### Fixed
- CI pipeline: removed Docker images and `--interpreter` flags for native maturin builds; fixed clippy, ruff, and mypy lints
- macOS pre-built binary: updated Intel Mac requirement from Catalina to macOS 14

### Security

