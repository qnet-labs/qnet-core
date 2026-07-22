#!/usr/bin/env python3
"""
Example 7 — Teleportation with a Relay Node

Teleport a quantum state across a multi-hop path using a relay node.
Demonstrates the execute_teleportation() protocol with explicit relay_nodes
to bridge distance that would be too long for direct fidelity.

Run:
    python examples/07_teleportation_relay.py
"""

from qnet_core import (
    QNetEngine,
    NodeDefinition,
    LinkDefinition,
    TeleportationParameters,
)

# ── 1. Define a 3-node chain (source → relay → target) ───────────────
engine = QNetEngine()

nodes = [
    NodeDefinition(id="Alice", memory_lifetime_t2=1.0),   # source
    NodeDefinition(id="Charlie", memory_lifetime_t2=0.8),  # relay
    NodeDefinition(id="Bob",   memory_lifetime_t2=1.0),    # target
]

links = [
    LinkDefinition("Alice", "Charlie", distance_km=25.0, base_fidelity=0.90,
                   generation_rate_hz=500.0),
    LinkDefinition("Charlie", "Bob", distance_km=25.0, base_fidelity=0.90,
                   generation_rate_hz=500.0),
]

engine.define_network(nodes, links)

# ── 2. Configure teleportation with relay node ────────────────────────
teleport_params = TeleportationParameters(
    source_node="Alice",
    target_node="Bob",
    state_fidelity=0.95,       # desired output state fidelity
    classical_bandwidth_ms=100.0,
)
# Explicitly specify the relay node to bridge the two hops
teleport_params.relay_nodes = ["Charlie"]

# ── 3. Execute teleportation ───────────────────────────────────────────
outcome = engine.execute_teleportation(teleport_params)

print("=== Quantum State Teleportation ===")
print(f"Success:                {outcome.success}")
print(f"Teleportation fidelity: {outcome.teleportation_fidelity:.4f}")
print(f"Resource entanglement fidelity: {outcome.resource_entanglement_fidelity:.4f}")
print(f"Latency:                {outcome.latency_ms:.1f} ms")
print(f"Path:                   {' → '.join(outcome.path)}")
print(f"Classical bits sent:    {outcome.classical_bits_transferred}")

# ── 4. Compare with direct (no relay) attempt ─────────────────────────
print("\n--- Direct teleportation (no relay, longer hop) ---")
direct_links = [
    LinkDefinition("Alice", "Bob", distance_km=50.0, base_fidelity=0.75,
                   generation_rate_hz=300.0),  # long distance → lower fidelity
]
engine_direct = QNetEngine()
direct_nodes = [
    NodeDefinition(id="Alice", memory_lifetime_t2=1.0),
    NodeDefinition(id="Bob",   memory_lifetime_t2=1.0),
]
engine_direct.define_network(direct_nodes, direct_links)

direct_params = TeleportationParameters(
    source_node="Alice",
    target_node="Bob",
    state_fidelity=0.95,
    classical_bandwidth_ms=100.0,
)
# No relay nodes — single long hop
direct_outcome = engine_direct.execute_teleportation(direct_params)

print(f"Success:                {direct_outcome.success}")
print(f"Teleportation fidelity: {direct_outcome.teleportation_fidelity:.4f}")
print(f"Latency:                {direct_outcome.latency_ms:.1f} ms")
print("\n=> Relayed teleportation trades latency for higher output fidelity.")
