#!/usr/bin/env python3
"""
Example 9 — Distributed Computing: Mesh vs Ring

Same 3-party GHZ computation compared across two coordination topologies
(mesh and ring) to show the trade-off: mesh has more direct links (higher
resource cost) but lower coordination overhead; ring has fewer links but
requires sequential message passing.

Run:
    python examples/09_distributed_mesh_ring_comparison.py
"""

from qnet_core import (
    QNetEngine,
    NodeDefinition,
    LinkDefinition,
    CoordinationTopology,
    MeasurementBasis,
    BasisType,
)

# ── 1. Define the same underlying network for both topologies ──────────
# Mesh topology needs all pairwise links among parties
nodes = [
    NodeDefinition(id="Alice", memory_lifetime_t2=0.8),
    NodeDefinition(id="Bob",   memory_lifetime_t2=0.8),
    NodeDefinition(id="Charlie", memory_lifetime_t2=0.8),
]

# In mesh: all pairs connected
mesh_links = [
    LinkDefinition("Alice", "Bob", distance_km=10.0, base_fidelity=0.95,
                   generation_rate_hz=1_000.0),
    LinkDefinition("Bob", "Charlie", distance_km=10.0, base_fidelity=0.93,
                   generation_rate_hz=1_000.0),
    LinkDefinition("Charlie", "Alice", distance_km=12.0, base_fidelity=0.92,
                   generation_rate_hz=900.0),
]

# In ring: only a cycle (fewer links)
ring_links = [
    LinkDefinition("Alice", "Bob", distance_km=10.0, base_fidelity=0.95,
                   generation_rate_hz=1_000.0),
    LinkDefinition("Bob", "Charlie", distance_km=10.0, base_fidelity=0.93,
                   generation_rate_hz=1_000.0),
]

participants = ["Alice", "Bob", "Charlie"]
basis = MeasurementBasis(basis_type=BasisType.GHZ, correlation_strength=0.85)


def run_comparison(label, link_list):
    engine = QNetEngine()
    engine.define_network(nodes, link_list)

    ring_coord = CoordinationTopology.ring() if "ring" in label.lower() else CoordinationTopology.mesh()

    result = engine.run_distributed_computation(
        participants=participants,
        coordination_topology=ring_coord,
        measurement_basis=basis,
        classical_relay_latency_ms=5.0,
    )

    print(f"\n=== {label} ===")
    print(f"Success:               {result.success}")
    print(f"Computation fidelity:  {result.computation_fidelity:.4f}")
    print(f"Total latency:         {result.total_latency_ms:.1f} ms")
    print(f"Coordination overhead: {result.coordination_overhead_ms:.1f} ms")
    print(f"Links used:            {len(result.resource_links_used)}")
    for link in result.resource_links_used:
        print(f"  - {link}")


# ── 2. Run both topologies ─────────────────────────────────────────────
run_comparison("Mesh Topology", mesh_links)
run_comparison("Ring Topology", ring_links)

print("\n=> Mesh provides direct pairwise links (more overhead, lower latency).")
print("   Ring uses fewer links but adds sequential coordination delay.")
