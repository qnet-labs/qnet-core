# qnet-core Python SDK

Python bindings for the **qnet-core** quantum network simulation engine. Simulates entanglement distribution across repeater networks, models link generation protocols, fidelity purification (BBPSSW), and routing strategies for quantum communication.

## Installation

```bash
# Install build dependencies
pip install qnet-core

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

The SDK ships with three ready-to-use topology generators:

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

Physical layer constants.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `baseline_purify_factor` | float | `alpha/10` | Purification baseline factor |
| `speed_of_light_in_fiber_km_ms` | float | 200.0 | Light speed in fiber (km/ms) |

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
from qnet_core import diff_topologies, PyTopologySnapshot, PyTopologyMetadata, PyTopologyConfig

snap1 = PyTopologySnapshot(
    metadata=PyTopologyMetadata(name="v1", version="1.0"),
    nodes=[...],
    links=[...],
    config=PyTopologyConfig(alpha_loss=0.22, gamma_swapping=0.9),
)

snap2 = PyTopologySnapshot(...)

diff = diff_topologies(snap1, snap2)
print(diff.summary)
# Access: diff.nodes_added, diff.nodes_removed, diff.nodes_modified
#         diff.links_added, diff.links_removed, diff.links_modified
```

| Class | Fields |
|-------|--------|
| `PyTopologyMetadata` | `name: str`, `version: str` |
| `PyTopologyConfig` | `alpha_loss: float`, `gamma_swapping: float` |
| `PyTopologySnapshot` | `metadata`, `nodes: List[NodeDefinition]`, `links: List[LinkDefinition]`, `config` |
| **PyTopologyDiff** | `name`, `nodes_added`, `nodes_removed`, `nodes_modified`, `links_added`, `links_removed`, `links_modified`, `summary` (all `List[str]`) |

### .qnet File Format — Load / Save / Validate

Save and load network topologies as JSON files.

#### Programmatic creation & save

```python
from qnet_core import PyQNetFile, QNetNodeType, QNetLinkType, PyQNetSatelliteExtension

qf = PyQNetFile(name="my-network")
qf.metadata.description = "A hybrid satellite-fiber network"
qf.metadata.author = "Alice"

qf.add_node("Toronto", memory_lifetime_ms=2000.0, memory_capacity=10, node_type=QNetNodeType.Ground)
qf.add_node("London", memory_lifetime_ms=2000.0, node_type=QNetNodeType.Ground)
qf.add_link(
    id="link-1", src="Toronto", to="London",
    distance_km=5600.0, base_fidelity=0.85, generation_rate_hz=500.0,
    link_type=QNetLinkType.Satellite,
    satellite=PyQNetSatelliteExtension(visibility=0.9, weather_factor=0.8),
)
qf.save("output.qnet")
```

#### Load & validate

```python
from qnet_core import load_qnet_file, validate

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
| `load_qnet_file(filepath)` | `(str) -> PyQNetFile` | Load and parse a .qnet JSON file |
| `save_topology(engine, filepath)` | `(PyQNetEngine, str) -> None` | Persist engine's current topology to .qnet |
| `load_topology(engine, filepath)` | `(PyQNetEngine, str) -> None` | Load a .qnet file into an existing engine |
| `validate(filepath)` | `(str) -> dict` | Returns `{"valid", "filepath", "errors", "warnings"}` — **does not raise** |
| `diff(file1, file2)` | `(str, str) -> dict` | Returns diff keys (`summary`, `_added`, `_removed`, `_modified`) or `{"error": ...}` |

#### `.qnet` container types

| Class | Constructor | Key fields |
|-------|------------|------------|
| **PyQNetFile** | `(name: str)` | `version`, `metadata` (PyQNetMetadata), `nodes` (List[PyQNetNode]), `links` (List[PyQNetLink]), `config` (Optional[PyQNetConfig]), `constraints` (Optional[PyQNetConstraints]) |
| **PyQNetFile.add_node(...)** | `(id, memory_lifetime_ms?, memory_capacity?, node_type?)` | — | Push a node onto the file |
| **PyQNetFile.add_link(...)** | `(id, src, to, distance_km, base_fidelity, generation_rate_hz, link_type?, satellite?)` | — | Push a link onto the file |
| **PyQNetFile.save(...)** | `(filepath: str)` | `None` | Write pretty-printed JSON to disk |
| **PyQNetNode** | `(id, memory_lifetime_ms?, memory_capacity?, node_type?)` | `id`, `memory_lifetime_ms`, `memory_capacity`, `node_type` |
| **PyQNetLink** | `(id, src, to, distance_km, base_fidelity, generation_rate_hz, link_type?, satellite?)` | `id`, `src`, `to`, `distance_km`, `base_fidelity`, `generation_rate_hz`, `link_type`, `satellite` (PyQNetSatelliteExtension) |
| **PyQNetConfig** | `(alpha_loss?, beta_fidelity_decay?, gamma_swapping?, max_attempts?)` | `alpha_loss`, `beta_fidelity_decay`, `gamma_swapping`, `max_attempts` |
| **PyQNetConstraints** | `(fidelity_target?, max_latency_ms?)` | `fidelity_target`, `max_latency_ms` — supports `__eq__` |
| **PyQNetMetadata** | `(name, description?, author?, created_at?)` | `name`, `description`, `author`, `created_at` |

## Complete Import Map

Everything available after `from qnet_core import ...`:

**Classes (23)**

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
| `TopologyEndpoints` | Endpoint mapping for comparison |
| `PyTopologySnapshot` | Topology snapshot (for diffing) |
| `PyTopologyMetadata` | Snapshot metadata |
| `PyTopologyConfig` | Snapshot config (alpha_loss, gamma_swapping) |
| `PyTopologyDiff` | Diff between two snapshots |
| `PyQNetFile` | .qnet container (load/save/diff) |
| `PyQNetNode` | Node for .qnet format |
| `PyQNetLink` | Link for .qnet format |
| `PyQNetConfig` | Physics config for .qnet |
| `PyQNetConstraints` | Constraints for .qnet |
| `PyQNetMetadata` | Metadata for .qnet |
| `PyQNetNodeType` | Node type enum (Ground/Satellite/Repeater) |
| `PyQNetLinkType` | Link type enum (Fiber/Satellite) |
| `PyQNetSatelliteExtension` | Satellite extension for .qnet links |

**Module-level functions (8)**

| Function | Returns | Purpose |
|----------|---------|---------|
| `generate_topology(name: str)` | `NetworkTopologyPayload` | Generate a pre-built topology |
| `compare_topologies(...)` | `TopologyComparisonReport` | Compare topologies via Monte Carlo |
| `save_topology(engine, filepath)` | `None` | Save engine state to .qnet |
| `load_topology(engine, filepath)` | `None` | Load .qnet into engine |
| `validate(filepath)` | `dict` | Validate a .qnet file (returns errors) |
| `load_qnet_file(filepath)` | `PyQNetFile` | Load .qnet JSON as Python objects |
| `diff(file1, file2)` | `dict` | Diff two .qnet files on disk |
| `diff_topologies(snap1, snap2)` | `PyTopologyDiff` | Diff two topology snapshots programmatically |

## Running Tests

```bash
cd python
python test_example.py
```
