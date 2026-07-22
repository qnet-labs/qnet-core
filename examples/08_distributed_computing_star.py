#!/usr/bin/env python3
"""
Example 8 — Distributed Quantum Computing (Star Topology)

A 3-party GHZ-basis computation using CoordinationTopology.star().
In the star topology, a central node coordinates all measurements and
collects classical results before final computation.

Run:
    python examples/08_distributed_computing_star.py
"""

from qnet_core import (
    QNetEngine,
    NodeDefinition,
    LinkDefinition,
    CoordinationTopology,
    MeasurementBasis,
    BasisType,
)

# ── 1. Define a star network: center + 3 peripheral nodes ─────────────
engine = QNetEngine()

nodes = [
    NodeDefinition(id="Center", memory_lifetime_t2=1.0),
    NodeDefinition(id="PartyA", memory_lifetime_t2=0.8),
    NodeDefinition(id="PartyB", memory_lifetime_t2=0.8),
    NodeDefinition(id="PartyC", memory_lifetime_t2=0.8),
]

links = [
    LinkDefinition("Center", "PartyA", distance_km=10.0, base_fidelity=0.95,
                   generation_rate_hz=1_000.0),
    LinkDefinition("Center", "PartyB", distance_km=10.0, base_fidelity=0.95,
                   generation_rate_hz=1_000.0),
    LinkDefinition("Center", "PartyC", distance_km=10.0, base_fidelity=0.92,
                   generation_rate_hz=800.0),
]

engine.define_network(nodes, links)

# ── 2. Configure distributed computation ──────────────────────────────
participants = ["PartyA", "PartyB", "PartyC"]
coordination = CoordinationTopology.star(center_node="Center")
basis = MeasurementBasis(basis_type=BasisType.GHZ, correlation_strength=0.85)

result = engine.run_distributed_computation(
    participants=participants,
    coordination_topology=coordination,
    measurement_basis=basis,
    classical_relay_latency_ms=5.0,
)

print("=== Distributed Quantum Computing (Star Topology) ===")
print(f"Success:             {result.success}")
print(f"Computation fidelity:{result.computation_fidelity:.4f}")
print(f"Total latency:       {result.total_latency_ms:.1f} ms")
print(f"Coordination overhead:{result.coordination_overhead_ms:.1f} ms")
print(f"\nResource links used:")
for link in result.resource_links_used:
    print(f"  - {link}")

print(f"\nParty outcomes:")
for party in result.party_results:
    status = "✓" if party.successful_measurement else "✗"
    print(f"  [{status}] {party.node_id}: local_fidelity={party.local_fidelity:.4f}")


# ── 3. Compare with mesh topology ──────────────────────────────────────
print("\n--- Mesh topology comparison ---")
mesh_coord = CoordinationTopology.mesh()

result_mesh = engine.run_distributed_computation(
    participants=participants,
    coordination_topology=mesh_coord,
    measurement_basis=basis,
    classical_relay_latency_ms=5.0,
)

print(f"Success:             {result_mesh.success}")
print(f"Computation fidelity:{result_mesh.computation_fidelity:.4f}")
print(f"Total latency:       {result_mesh.total_latency_ms:.1f} ms")
print("\n=> Star topology concentrates coordination at one node; mesh distributes it.")
