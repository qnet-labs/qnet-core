# qnet-core Examples

Standalone Python examples demonstrating every feature of the qnet-core library. Each file is self-contained and runnable after `pip install qnet-core`.

## Setup

```bash
pip install qnet-core
```

Then run any example directly:

```bash
python examples/01_basic_entanglement.py
```

## Example List

| # | File | Description |
|---|------|-------------|
| 1 | [01_basic_entanglement.py](./01_basic_entanglement.py) | **Basic entanglement request** — two nodes, one link, a single `request_entanglement()` call to show the minimal end-to-end flow. |
| 2 | [02_monte_carlo_ensemble.py](./02_monte_carlo_ensemble.py) | **Monte Carlo ensemble** — run `simulate()` over 1,000 trials and report success rate, mean latency, and mean fidelity. |
| 3 | [03_routing_strategy_comparison.py](./03_routing_strategy_comparison.py) | **Routing strategy comparison** — the same topology run under `LowestLatency`, `HighestFidelity`, and `HighestSuccess` to show how strategy changes outcomes. |
| 4 | [04_builtin_topology_generators.py](./04_builtin_topology_generators.py) | **Built-in topology generators tour** — generate and inspect `telecom_backbone`, `repeater_chain`, and `hybrid_satellite_fiber`. |
| 5 | [05_satellite_fiber_link.py](./05_satellite_fiber_link.py) | **Hybrid satellite-fiber link** — a satellite link with `SatelliteConditions` (visibility/weather) showing how atmospheric factors degrade effective rate. |
| 6 | [06_qkd_key_exchange.py](./06_qkd_key_exchange.py) | **QKD key exchange (BB84)** — run `run_qkd()` between two nodes and inspect QBER, key length, and efficiency. |
| 7 | [07_teleportation_relay.py](./07_teleportation_relay.py) | **Teleportation with a relay node** — teleport a state across a multi-hop path using `relay_nodes`. |
| 8 | [08_distributed_computing_star.py](./08_distributed_computing_star.py) | **Distributed quantum computing (star topology)** — a 3-party GHZ-basis computation using `CoordinationTopology.star()`. |
| 9 | [09_distributed_mesh_ring_comparison.py](./09_distributed_mesh_ring_comparison.py) | **Distributed quantum computing (mesh vs ring)** — same computation compared across two coordination topologies to show the tradeoff. |
| 10 | [10_qnet_file_from_scratch.py](./10_qnet_file_from_scratch.py) | **Authoring a .qnet file from scratch** — build a `QNetFile`, add nodes/links, save to disk, then reload it with `from_qnet_file()`. |
| 11 | [11_qnet_validation.py](./11_qnet_validation.py) | **Validating a .qnet file** — intentionally introduce an error (e.g. a self-loop link) and show `validate()` catching it before simulation. |
| 12 | [12_qnet_diffing.py](./12_qnet_diffing.py) | **Diffing two topology versions** — use `diff()` to show what changed between a v1 and v2 network design. |
| 13 | [13_compare_candidate_topologies.py](./13_compare_candidate_topologies.py) | **Comparing candidate topologies for a route** — compare two named topologies for the same source→target pair, picking a winner by success rate. |
| 14 | [14_tuning_physical_constants.py](./14_tuning_physical_constants.py) | **Tuning physical-layer constants** — vary `PhysicalConfig` (fiber loss, T2 memory lifetime, generation rate) and show how fidelity/success rate shift, useful as a sensitivity-style demo. |

## Organization

- **Examples 01–03**: Core simulation (`QNetEngine`, strategies, Monte Carlo)
- **Examples 04–05**: Topology generators and satellite links
- **Examples 06–08**: Quantum protocols (QKD BB84, state teleportation, distributed computing)
- **Examples 09–10**: Distributed computing topologies and .qnet file authoring
- **Examples 11–13**: .qnet file management (validation, diffing, topology comparison)
- **Example 14**: Sensitivity analysis for physical-layer parameters

## Prerequisites

All examples use only the public qnet-core Python bindings:

```python
from qnet_core import (
    QNetEngine, NodeDefinition, LinkDefinition, StrategyType,
    PhysicalConfig, SimulationConfig, SatelliteConditions,
    CoordinationTopology, MeasurementBasis, BasisType,
    QKDParameters, TeleportationParameters,
    QNetFile, QNetNodeType,
    generate_topology, compare_topologies, validate, diff,
    from_qnet_file, load_qnet_file, save_qnet_file_wrapper,
)
```

No additional dependencies are required — each example is self-contained.
