# qnet-core Python SDK

[📖 Full API docs](https://qnet-core.dev/docs/) — [Tutorial Index](https://qnet-core.dev/docs/tutorials/overview)

Python bindings for the **qnet-core** quantum network simulation engine. Simulates entanglement distribution across repeater networks, models link generation protocols, fidelity purification (BBPSSW), and routing strategies for quantum communication. Also supports higher-level quantum protocols: QKD key distribution, state teleportation, and distributed quantum computing.

## Installation

```bash
pip install qnet-core
```

## Quick Start

```python
from qnet_core import QNetEngine, StrategyType, NodeDefinition, LinkDefinition

# 1. Create engine (optional custom config)
engine = QNetEngine()

# 2. Define network topology
nodes = [
    NodeDefinition(id="A", memory_lifetime_t2=1.0),
    NodeDefinition(id="B", memory_lifetime_t2=1.0),
]

links = [
    LinkDefinition(
        from_node="A", to="B",
        distance_km=10.0,
        base_fidelity=0.95,
        generation_rate_hz=1000.0,
    ),
]

engine.define_network(nodes=nodes, links=links)

# 3a. Single simulation
result = engine.request_entanglement(
    from_node="A",
    to="B",
    fidelity_target=0.9,
    max_latency_ms=100.0,
    strategy=StrategyType.HighestFidelity,
)
print(f"Success: {result.success}  Fidelity: {result.final_fidelity:.4f}")
print(f"Path: {' -> '.join(result.execution_path)}")

# 3b. Monte Carlo ensemble (1000 runs)
stats = engine.simulate(
    from_node="A",
    to="B",
    fidelity_target=0.9,
    max_latency_ms=100.0,
    runs=1000,
)
print(f"Success rate: {stats.empirical_success_rate:.2%}")
print(f"Mean latency: {stats.mean_latency_ms:.1f} ms")
print(f"Link utilization: {stats.link_utilization_heatmap}")
```

## Pre-built Topologies

qnet-core ships with three ready-to-use topology generators:

```python
from qnet_core import generate_topology

# Telecom backbone (mesh-style fiber network)
backbone = generate_topology("telecom_backbone")

# Linear repeater chain (default length=4)
chain = generate_topology("repeater_chain")

# Hybrid satellite + fiber network
hybrid = generate_topology("hybrid_satellite_fiber")

# Access the generated nodes/links
for node in backbone.nodes:
    print(node.id, node.memory_lifetime_t2)

for link in backbone.links:
    print(f"{link.from_node} -> {link.to}: {link.distance_km} km, {link.base_fidelity:.2f} fidelity")
```

| Topology name            | Type                      | Description                    |
|--------------------------|---------------------------|--------------------------------|
| `telecom_backbone`       | TelecomBackbone           | Mesh fiber network             |
| `repeater_chain`         | RepeaterChain (length=4)  | Linear chain of 4 repeaters    |
| `hybrid_satellite_fiber` | HybridSatelliteFiber      | Satellite + fiber mix          |

## Higher-Level Quantum Protocols

qnet-core supports three higher-level quantum networking protocols on top of the entanglement simulation layer. Each protocol is available as both an engine method and a module-level convenience function. See the [example files](../../examples/) (TODO: update to `qnet-labs/qnet-examples` after repo split) for complete runnable demos.

### QKD (Quantum Key Distribution)

BB84-style protocol for secure key exchange between two nodes.

```python
from qnet_core import QKDParameters, qkd

# Engine method
params = QKDParameters(from_node="A", to_node="B", fidelity_target=0.9, max_latency_ms=5000.0, rounds=100)
result = engine.run_qkd(params=params)

# Convenience function
result = qkd(engine, "A", "B", 0.9, 5000.0, rounds=100)
```

#### `QKDParameters`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `from_node` | str | — | Source node ID |
| `to_node` | str | — | Destination node ID |
| `fidelity_target` | float | — | Required entanglement fidelity |
| `max_latency_ms` | float | — | Maximum allowed latency (ms) |
| `rounds` | int | 100 | Number of QKD rounds |
| `error_rate_tolerance` | float | 0.11 | BB84 error threshold |
| `sifting_overhead_ratio` | float | 0.5 | Public sifting overhead |
| `privacy_amplification_factor` | float | 0.8 | Privacy amplification factor |

#### `QKDResult`

| Field | Type | Description |
|-------|------|-------------|
| `success` | bool | Whether secure key was established |
| `secret_key_length_bits` | int | Length of generated secret key (bits) |
| `efficiency_rate` | float | Key generation efficiency [0, 1] |
| `qber` | float | Quantum bit error rate |
| `latency_ms` | float | Total protocol latency (ms) |
| `execution_path` | List[str] | Node IDs traversed |
| `rounds_completed` | int | Number of successful rounds |
| `rounds_failed` | int | Number of failed rounds |

#### `QKDStats` (Monte Carlo ensemble)

> **Note:** Monte Carlo ensemble for QKD is planned but not yet implemented. `run_qkd()` currently returns a single `QKDResult`.

| Field | Type | Description |
|-------|------|-------------|
| `total_runs` | int | Number of QKD runs executed |
| `success_rate` | float | Fraction of successful runs [0, 1] |
| `mean_key_length_bits` | float | Mean secret key length (bits) |
| `mean_efficiency` | float | Mean key generation efficiency |
| `mean_qber` | float | Mean quantum bit error rate |

### Teleportation

Entanglement-based quantum state teleportation across the network.

```python
from qnet_core import TeleportationParameters, teleportation

params = TeleportationParameters(source_node="A", target_node="B")
params.relay_nodes = ["B"]  # intermediate relays (optional)
outcome = engine.execute_teleportation(params=params)

# Convenience function (no relay setup needed)
from qnet_core import teleportation as tp_fn
outcome = tp_fn(engine, "A", "B", state_fidelity=0.95)
```

#### `TeleportationParameters`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source_node` | str | — | Source node (Alice) |
| `target_node` | str | — | Target node (Bob) |
| `state_fidelity` | float | 0.95 | Input state fidelity |
| `classical_bandwidth_ms` | float | 100.0 | Classical channel latency (ms) |
| `relay_nodes` | List[str] | [] | Intermediate relay node IDs |

#### `TeleportationOutcome`

| Field | Type | Description |
|-------|------|-------------|
| `success` | bool | Whether teleportation succeeded |
| `teleportation_fidelity` | float | Fidelity of teleported state |
| `resource_entanglement_fidelity` | float | Fidelity of resource entanglement links |
| `latency_ms` | float | End-to-end latency (ms) |
| `path` | List[str] | Node IDs traversed |
| `classical_bits_transferred` | int | Number of classical bits sent |

#### `TeleportationStats` (Monte Carlo ensemble)

> **Note:** Monte Carlo ensemble for teleportation is planned but not yet implemented. `execute_teleportation()` currently returns a single `TeleportationOutcome`.

| Field | Type | Description |
|-------|------|-------------|
| `total_runs` | int | Number of teleportation runs executed |
| `success_rate` | float | Fraction of successful runs [0, 1] |
| `mean_teleportation_fidelity` | float | Mean teleportation fidelity |
| `teleportation_fidelity_stddev` | float | Standard deviation of teleportation fidelity |
| `mean_latency_ms` | float | Mean end-to-end latency (ms) |

### Distributed Quantum Computing

Multi-party quantum computation with coordinated measurements. Supports star, ring, mesh, and arbitrary topologies with GHZ / cluster / graph measurement bases.

```python
from qnet_core import (
    DistributedComputingParameters, CoordinationTopology,
    MeasurementBasis, BasisType, distributed_computation,
)

topology = CoordinationTopology.star("A")
basis = MeasurementBasis(basis_type=BasisType.GHZ, correlation_strength=0.85)
result = engine.run_distributed_computation(
    participants=["A", "B", "C"],
    coordination_topology=topology,
    measurement_basis=basis,
)

# Convenience function
result = distributed_computation(engine, ["A","B","C"], CoordinationTopology.mesh(), basis)
```

#### Enumerations

##### `BasisType` — Measurement basis for distributed protocols

| Member | Description |
|--------|-------------|
| `BasisType.GHZ` | GHZ-state measurement basis |
| `BasisType.Cluster` | Cluster-state measurement basis |
| `BasisType.GraphGraph` | Graph-state measurement basis |

##### `CoordinationTopology` — Party coordination pattern

| Method/Field | Description |
|--------------|-------------|
| `.star(center_node)` | Star topology with center node |
| `.ring()` | Ring topology (circular) |
| `.mesh()` | All-to-all mesh topology |
| `.arbitrary(edges)` | Custom edges `[(src, dst), ...]` |
| `kind` | `"star"`, `"ring"`, `"mesh"`, `"arbitrary"` |
| `center_node` | Center node for star (optional) |
| `edges` | Edge list for arbitrary (optional) |

#### `DistributedComputingParameters`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `participants` | List[str] | — | Participant node IDs |
| `coordination_topology` | CoordinationTopology | — | Coordination pattern |
| `measurement_basis` | MeasurementBasis | GHZ, 0.85 | Measurement basis config |
| `classical_relay_latency_ms` | float | 5.0 | Classical relay latency (ms) |

#### `DistributedComputingResult`

| Field | Type | Description |
|-------|------|-------------|
| `success` | bool | Whether computation succeeded |
| `computation_fidelity` | float | Fidelity of the distributed computation |
| `party_results` | List[PartyOutcome] | Per-party measurement outcomes |
| `resource_links_used` | List[str] | Links used during protocol |
| `total_latency_ms` | float | End-to-end latency (ms) |
| `coordination_overhead_ms` | float | Coordination overhead (ms) |

#### `PartyOutcome`

| Field | Type | Description |
|-------|------|-------------|
| `node_id` | str | Participant node ID |
| `successful_measurement` | bool | Whether measurement succeeded |
| `local_fidelity` | float | Local measurement fidelity |

#### `DistributedComputingStats` (Monte Carlo ensemble)

> **Note:** Monte Carlo ensemble for distributed computation is planned but not yet implemented. `run_distributed_computation()` currently returns a single `DistributedComputingResult`.

| Field | Type | Description |
|-------|------|-------------|
| `total_runs` | int | Number of runs executed |
| `success_rate` | float | Fraction of successful runs [0, 1] |
| `mean_computation_fidelity` | float | Mean computation fidelity |
| `mean_coordination_overhead_ms` | float | Mean coordination overhead (ms) |

### Module-level convenience functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `qkd()` | `(engine, from_node, to_node, fidelity_target, max_latency_ms, rounds?, error_rate_tolerance?) -> QKDResult` | Run BB84 QKD (convenience) |
| `teleportation()` | `(engine, source_node, target_node, state_fidelity?, classical_bandwidth_ms?) -> TeleportationOutcome` | Execute teleportation (convenience) |
| `distributed_computation()` | `(engine, participants, coordination_topology, measurement_basis, classical_relay_latency_ms?) -> DistributedComputingResult` | Run distributed computation (convenience) |

### Topology Comparison

Compare multiple pre-built topologies for the same source→target pair.

```python
from qnet_core import compare_topologies, TopologyEndpoints, StrategyType

report = compare_topologies(
    endpoints=[
        TopologyEndpoints("telecom_backbone", "A", "C"),
        TopologyEndpoints("hybrid_satellite_fiber", "Toronto", "London"),
    ],
    fidelity_target=0.75,
    max_latency_ms=5000.0,
    runs=1000,
    strategy=StrategyType.HighestFidelity,
)

print(f"Recommended: {report.recommended_topology}")
print(report.summary)

for r in report.results:
    print(f"{r.topology_name}: success={r.success_rate:.2%}, latency={r.mean_latency_ms:.1f}ms, fidelity={r.mean_fidelity:.4f}")
```

| Function / Class | Signature | Description |
|------------------|-----------|-------------|
| `compare_topologies(endpoints, fidelity_target, max_latency_ms, runs, strategy)` | `(List[TopologyEndpoints], float, float, int, Optional[StrategyType]) -> TopologyComparisonReport` | Runs Monte Carlo on each topology and picks the best by success rate |
| `TopologyEndpoints(topology_name, from_node, to_node)` | `(str, str, str)` | Maps a generated topology name to its source and target nodes |
| **TopologyComparisonReport** fields | — | `source_node`, `target_node`, `fidelity_target`, `max_latency_ms`, `runs`, `results` (List), `recommended_topology`, `summary` |
| **TopologyComparisonResult** fields | — | `topology_name`, `success_rate`, `mean_latency_ms`, `mean_fidelity`, `link_utilization` |

### Topology Snapshots & Diffing

Programmatic comparison of two topology snapshots.

```python
from qnet_core import diff_topologies, TopologySnapshot, TopologyMetadata, TopologyConfig

snap1 = TopologySnapshot(
    metadata=TopologyMetadata(name="v1", version="1.0"),
    nodes=[...],
    links=[...],
    config=TopologyConfig(alpha_loss=0.22, gamma_swapping=0.9),
)

snap2 = TopologySnapshot(...)

diff = diff_topologies(snap1, snap2)
print(diff.summary)
# Access: diff.nodes_added, diff.nodes_removed, diff.nodes_modified
#         diff.links_added, diff.links_removed, diff.links_modified
```

| Class | Fields |
|-------|--------|
| `TopologyMetadata` | `name: str`, `version: str` |
| `TopologyConfig` | `alpha_loss: float`, `gamma_swapping: float` |
| `TopologySnapshot` | `metadata`, `nodes: List[NodeDefinition]`, `links: List[LinkDefinition]`, `config` |
| **TopologyDiff** | `name`, `nodes_added`, `nodes_removed`, `nodes_modified`, `links_added`, `links_removed`, `links_modified`, `summary` (all `List[str]`) |

### .qnet File Format — Load / Save / Validate

Save and load network topologies as JSON files.

#### Programmatic creation & save

```python
from qnet_core import QNetFile, QNetNodeType, QNetLinkType, QNetSatelliteExtension

qf = QNetFile(name="my-network")
qf.metadata.description = "A hybrid satellite-fiber network"
qf.metadata.author = "Alice"

qf.add_node("Toronto", memory_lifetime_ms=2000.0, memory_capacity=10, node_type=QNetNodeType.Ground)
qf.add_node("London", memory_lifetime_ms=2000.0, node_type=QNetNodeType.Ground)
qf.add_link(
    id="link-1", src="Toronto", to="London",
    distance_km=5600.0, base_fidelity=0.85, generation_rate_hz=500.0,
    link_type=QNetLinkType.Satellite,
    satellite=QNetSatelliteExtension(visibility=0.9, weather_factor=0.8),
)
qf.save("output.qnet")
```

#### From a .qnet file (factory)

```python
from qnet_core import from_qnet_file

# One call to load and start simulating
engine = from_qnet_file("network.qnet")

result = engine.request_entanglement(
    from_node="A", to="B",
    fidelity_target=0.9, max_latency_ms=100.0,
)
```

#### Load & validate

```python
from qnet_core import load_qnet_file, validate, diff

# Load as structured Python object
qf = load_qnet_file("network.qnet")
print(qf.metadata.name)
print(len(qf.nodes), len(qf.links))

# Validate (returns a dict, does NOT raise)
result = validate("network.qnet")
if not result["valid"]:
    for err in result["errors"]:
        print(f"  ERROR [{err['type']}]: {err['message']}")
    for warn in result["warnings"]:
        print(f"  WARN: {warn['message']}")

# Diff two files on disk
diff = diff("v1.qnet", "v2.qnet")
print(diff["summary"])
print(f"Nodes added: {diff['nodes_added']}, removed: {diff['nodes_removed']}, modified: {diff['nodes_modified']}")
print(f"Links added: {diff['links_added']}, removed: {diff['links_removed']}, modified: {diff['links_modified']}")
```

| Function / Class | Signature | Description |
|------------------|-----------|-------------|
| `from_qnet_file(filepath)` | `(str) -> QNetEngine` | Factory: load a .qnet file and return an initialized engine |
| `load_qnet_file(filepath)` | `(str) -> QNetFile` | Load and parse a .qnet JSON file |
| `save_topology(engine, filepath)` | `(QNetEngine, str) -> None` | Persist engine's current topology to .qnet |
| `load_topology(engine, filepath)` | `(QNetEngine, str) -> None` | Load a .qnet file into an existing engine |
| `validate(filepath)` | `(str) -> dict` | Returns `{"valid", "filepath", "errors", "warnings"}` — **does not raise** |
| `diff(file1, file2)` | `(str, str) -> dict` | Returns diff keys (`summary`, `_added`, `_removed`, `_modified`) or `{"error": ...}` |

#### `.qnet` container types

| Class / Method | Signature | Description |
|----------------|-----------|-------------|
| **QNetFile** | `(name: str)` | Container for .qnet JSON (load/save/diff) |
| `QNetFile.nodes` | — | List of nodes in this file |
| `QNetFile.links` | — | List of links in this file |
| `QNetFile.add_node(...)` | `(id, memory_lifetime_ms?, memory_capacity?, node_type?)` | Push a node onto the file |
| `QNetFile.add_link(...)` | `(id, src, to, distance_km, base_fidelity, generation_rate_hz, link_type?, satellite?)` | Push a link onto the file |
| `QNetFile.save(...)` | `(filepath: str)` | Write pretty-printed JSON to disk |
| **QNetNode** | `(id, memory_lifetime_ms?, memory_capacity?, node_type?)` | Node for .qnet format |
| **QNetLink** | `(id, src, to, distance_km, base_fidelity, generation_rate_hz, link_type?, satellite?)` | Link for .qnet format |
| **QNetConfig** | `(alpha_loss?, beta_fidelity_decay?, gamma_swapping?, max_attempts?)` | Physics config for .qnet |
| **QNetConstraints** | `(fidelity_target?, max_latency_ms?)` | Constraints — supports `__eq__` |
| **QNetMetadata** | `(name, description?, author?, created_at?)` | Metadata for .qnet files |

## API Reference

### Engine

#### `QNetEngine(config: Optional[SimulationConfig] = None)`

Main entry point for all simulations.

| Method | Signature | Returns | Description |
|--------|-----------|---------|-------------|
| `__init__` | `(config: Optional[SimulationConfig] = None)` | — | Create engine with optional config |
| `define_network` | `(nodes: List[NodeDefinition], links: List[LinkDefinition])` | `None` | Set or replace network topology |
| `request_entanglement` | `(from_node: str, to: str, fidelity_target: float, max_latency_ms: float, strategy: Optional[StrategyType] = None) -> SimulationResult` | `SimulationResult` | Run a single simulation |
| `simulate` | `(from_node: str, to: str, fidelity_target: float, max_latency_ms: float, runs: int, strategy: Optional[StrategyType] = None) -> MonteCarloStats` | `MonteCarloStats` | Run Monte Carlo ensemble |
| `run_qkd` | `(params: QKDParameters) -> QKDResult` | `QKDResult` | Run BB84-style quantum key distribution |
| `execute_teleportation` | `(params: TeleportationParameters) -> TeleportationOutcome` | `TeleportationOutcome` | Execute entanglement-based state teleportation |
| `run_distributed_computation` | `(participants: List[str], coordination_topology: CoordinationTopology, measurement_basis: MeasurementBasis, classical_relay_latency_ms: Optional[float]) -> DistributedComputingResult` | `DistributedComputingResult` | Run distributed quantum computing protocol |

### Enums

#### `StrategyType` — Routing strategies

| Member | Description |
|--------|-------------|
| `StrategyType.LowestLatency` | Minimize end-to-end latency |
| `StrategyType.HighestFidelity` | Maximize entanglement fidelity |
| `StrategyType.HighestSuccess` | Maximize link generation success rate |

#### `LinkType` — Link physical type

| Member | Description |
|--------|-------------|
| `LinkType.Fiber` | Optical fiber link |
| `LinkType.Satellite` | Satellite (free-space) link |

#### `QNetNodeType` — Node classification (for .qnet files)

| Member | Description |
|--------|-------------|
| `QNetNodeType.Ground` | Ground-based station |
| `QNetNodeType.Satellite` | Orbital satellite node |
| `QNetNodeType.Repeater` | Quantum repeater |

#### `QNetLinkType` — Link classification (for .qnet files)

| Member | Description |
|--------|-------------|
| `QNetLinkType.Fiber` | Fiber optic link |
| `QNetLinkType.Satellite` | Satellite free-space link |

### Configuration Types

#### `SimulationConfig(total_time_cutoff_ms: float, step_resolution_ms: float, physical: PhysicalConfig)`

Simulation timeline configuration.

| Parameter | Default | Description |
|-----------|---------|-------------|
| `total_time_cutoff_ms` | 5000.0 | Maximum simulation wall-clock time (ms) |
| `step_resolution_ms` | 0.1 | Event scheduler tick size (ms) |
| `physical` | — | See PhysicalConfig below |

#### `PhysicalConfig(alpha_loss_db_km: Optional[float] = None)`

Physical layer constants for link loss and propagation models.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha_loss_db_km` | float | `0.22` | Fiber attenuation in dB/km (standard telecom fiber ≈ 0.22 dB/km at 1550 nm) |

Derived constants (not configurable): `baseline_purify_factor ≈ alpha/10`, `speed_of_light_in_fiber_km_ms = 200.0`.

#### `SatelliteConditions(visibility: Optional[float]=1.0, weather_factor: Optional[float]=1.0)`

Atmospheric conditions for satellite links.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `visibility` | float | 1.0 | Atmospheric visibility (0-1) |
| `weather_factor` | float | 1.0 | Weather attenuation multiplier |
| **Method** | `effective_rate(base_rate: float) -> float` | | Returns `base_rate * visibility * weather_factor` |

### Topology Types

#### `NodeDefinition(id: str, memory_lifetime_t2: float)`

Quantum node (station or repeater).

| Field | Type | Description |
|-------|------|-------------|
| `id` | str | Unique node identifier |
| `memory_lifetime_t2` | float | Qubit T2 coherence time (s) |

#### `LinkDefinition(from_node: str, to: str, distance_km: float, base_fidelity: float, generation_rate_hz: float, link_type: Optional[LinkType]=Fiber, satellite_conditions: Optional[SatelliteConditions]=None)`

Physical quantum link between two nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `from_node` | str | — | Source node ID |
| `to` | str | — | Destination node ID |
| `distance_km` | float | — | Physical distance (km) |
| `base_fidelity` | float | — | Raw link entanglement fidelity |
| `generation_rate_hz` | float | — | Photon pair generation rate (Hz) |
| `link_type` | LinkType | Fiber | Physical medium |
| `satellite_conditions` | SatelliteConditions | None | Conditions if satellite link |

### Result Types

#### `SimulationResult(success: bool, latency_ms: float, final_fidelity: float, execution_path: List[str])`

Output from a single `request_entanglement()` call.

| Field | Type | Description |
|-------|------|-------------|
| `success` | bool | Whether entanglement was established |
| `latency_ms` | float | End-to-end latency (ms) |
| `final_fidelity` | float | Final entangled state fidelity |
| `execution_path` | List[str] | Node IDs traversed (inclusive) |

#### `MonteCarloStats(total_runs: int, empirical_success_rate: float, mean_latency_ms: float, mean_fidelity: float, aggregate_congestion_drops: int, link_utilization_heatmap: Dict[str, int])`

Output from a `simulate()` ensemble run.

| Field | Type | Description |
|-------|------|-------------|
| `total_runs` | int | Number of simulation runs executed |
| `empirical_success_rate` | float | Fraction of successful runs [0, 1] |
| `mean_latency_ms` | float | Mean latency across all runs (ms) |
| `mean_fidelity` | float | Mean fidelity across successful runs |
| `aggregate_congestion_drops` | int | Total drops due to qubit memory expiry |
| `link_utilization_heatmap` | Dict[str, int] | `{link_key: usage_count}` per link |

## Complete Import Map

Everything available after `from qnet_core import ...`:

**Classes (32)**

| Class | Purpose |
|-------|---------|
| `QNetEngine` | Main simulation engine |
| `SimulationConfig` | Timeline config (cutoff, step size) |
| `PhysicalConfig` | Physical layer constants |
| `NodeDefinition` | Node for runtime engine |
| `LinkDefinition` | Link for runtime engine |
| `StrategyType` | Routing strategy enum |
| `LinkType` | Physical medium enum (Fiber/Satellite) |
| `SatelliteConditions` | Atmospheric conditions helper |
| `SimulationResult` | Single-run result |
| `MonteCarloStats` | Ensemble result |
| `NetworkTopologyPayload` | Generated topology (nodes + links) from `generate_topology()` |
| `TopologyEndpoints` | Endpoint mapping for comparison |
| `TopologySnapshot` | Topology snapshot (for diffing) |
| `TopologyMetadata` | Snapshot metadata |
| `TopologyConfig` | Snapshot config (alpha_loss, gamma_swapping) |
| `TopologyDiff` | Diff between two snapshots |
| `QNetFile` | .qnet container (load/save/diff) |
| `QNetNode` | Node for .qnet format |
| `QNetLink` | Link for .qnet format |
| `QNetConfig` | Physics config for .qnet |
| `QNetConstraints` | Constraints for .qnet |
| `QNetMetadata` | Metadata for .qnet |
| `QNetNodeType` | Node type enum (Ground/Satellite/Repeater) |
| `QNetLinkType` | Link type enum (Fiber/Satellite) |
| `QNetSatelliteExtension` | Satellite extension for .qnet links |
| `QKDParameters` | QKD protocol parameters |
| `QKDResult` | QKD protocol result |
| `QKDStats` | QKD ensemble statistics |
| `TeleportationParameters` | Teleportation protocol parameters |
| `TeleportationOutcome` | Teleportation protocol result |
| `TeleportationStats` | Teleportation ensemble statistics |
| `BasisType` | Measurement basis enum (GHZ/Cluster/Graph) |
| `CoordinationTopology` | Distributed coordination pattern |
| `MeasurementBasis` | Measurement basis config |
| `DistributedComputingParameters` | Distributed computing parameters |
| `PartyOutcome` | Per-party measurement outcome |
| `DistributedComputingResult` | Distributed computing result |
| `DistributedComputingStats` | Distributed computing ensemble stats (planned) |

**Module-level functions (12)**

| Function | Returns | Purpose |
|----------|---------|---------|
| `from_qnet_file(filepath)` | `QNetEngine` | Load a .qnet file and return an initialized engine |
| `generate_topology(name: str)` | `NetworkTopologyPayload` | Generate a pre-built topology |
| `compare_topologies(...)` | `TopologyComparisonReport` | Compare topologies via Monte Carlo |
| `save_topology(engine, filepath)` | `None` | Save engine state to .qnet |
| `load_topology(engine, filepath)` | `None` | Load .qnet into engine |
| `validate(filepath)` | `dict` | Validate a .qnet file (returns errors) |
| `load_qnet_file(filepath)` | `QNetFile` | Load .qnet JSON as Python objects |
| `diff(file1, file2)` | `dict` | Diff two .qnet files on disk |
| `diff_topologies(snap1, snap2)` | `TopologyDiff` | Diff two topology snapshots programmatically |
| `qkd(engine, from_node, to_node, ...)` | `QKDResult` | Run BB84-style QKD (convenience) |
| `teleportation(engine, source, target, ...)` | `TeleportationOutcome` | Execute state teleportation (convenience) |
| `distributed_computation(engine, participants, ...)` | `DistributedComputingResult` | Run distributed quantum computation (convenience) |

## Running Tests

```bash
# Python tests
cd python
python test_example.py

# Rust core tests
cargo test
```
