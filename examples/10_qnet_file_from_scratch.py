#!/usr/bin/env python3
"""
Example 10 — Authoring a .qnet File from Scratch

Build a QNetFile by adding nodes and links programmatically, save to disk,
then reload it with from_qnet_file() into a fresh engine ready for simulation.

Run:
    python examples/10_qnet_file_from_scratch.py
"""

from qnet_core import (
    QNetEngine,
    from_qnet_file,
    load_qnet_file,
    save_qnet_file_wrapper,
)
from qnet_core.qnet_core import QNetFile, QNetNodeType

# ── 1. Build a network topology as a .qnet file ───────────────────────
print("=== Creating QNetFile from scratch ===")

qf = QNetFile(name="my_custom_network")

# Add metadata
qf.metadata.description = "Custom 3-node repeater chain"
qf.metadata.author = "demo"

# Add nodes with optional properties
qf.add_node("Alice", memory_lifetime_ms=1000.0, memory_capacity=4, node_type=QNetNodeType.Ground)
qf.add_node("R1",   memory_lifetime_ms=800.0,  memory_capacity=4, node_type=QNetNodeType.Repeater)
qf.add_node("Bob",  memory_lifetime_ms=1000.0, memory_capacity=4, node_type=QNetNodeType.Ground)

# Add links
qf.add_link(
    "link_01", src="Alice", to="R1",
    distance_km=15.0, base_fidelity=0.93, generation_rate_hz=800.0,
)
qf.add_link(
    "link_02", src="R1", to="Bob",
    distance_km=15.0, base_fidelity=0.90, generation_rate_hz=700.0,
)

print(f"  Nodes: {len(qf.nodes)}")
print(f"  Links: {len(qf.links)}")
print(f"  Version: {qf.version}")

# ── 2. Save to disk ───────────────────────────────────────────────────
filepath = "examples/network_demo.qnet"
save_qnet_file_wrapper(qf, filepath)
print(f"\n  Saved to: {filepath}")

# ── 3. Reload from disk ───────────────────────────────────────────────
print("\n=== Reloading from disk ===")
reloaded = load_qnet_file(filepath)
print(f"  Name:        {reloaded.metadata.name}")
print(f"  Nodes ({len(reloaded.nodes)}): {[n.id for n in reloaded.nodes]}")
print(f"  Links ({len(reloaded.links)}): {[f'{l.src}->{l.to}' for l in reloaded.links]}")

# ── 4. Load into engine and simulate ───────────────────────────────────
print("\n=== Simulation via from_qnet_file() ===")
engine = from_qnet_file(filepath)

stats = engine.simulate(
    from_node="Alice",
    to="Bob",
    fidelity_target=0.85,
    max_latency_ms=10_000.0,
    runs=200,
    seed=42,
)

print(f"  Success rate:   {stats.empirical_success_rate:.1%}")
print(f"  Mean latency:   {stats.mean_latency_ms:.1f} ms")
print(f"  Mean fidelity:  {stats.mean_fidelity:.4f}")
