#!/usr/bin/env python3
"""
Example 6 — QKD Key Exchange (BB84)

Run run_qkd() between two nodes and inspect QBER, key length, and efficiency.
QKD relies on the underlying entanglement distribution to establish a shared
secret key using the BB84 protocol.

Run:
    python examples/06_qkd_key_exchange.py
"""

from qnet_core import (
    QNetEngine,
    NodeDefinition,
    LinkDefinition,
    QKDParameters,
)

# ── 1. Define a simple 2-node network ─────────────────────────────────
engine = QNetEngine()

nodes = [
    NodeDefinition(id="Alice", memory_lifetime_t2=1.0),
    NodeDefinition(id="Bob",   memory_lifetime_t2=1.0),
]

links = [
    LinkDefinition("Alice", "Bob", distance_km=10.0, base_fidelity=0.95,
                   generation_rate_hz=1_000.0),
]

engine.define_network(nodes, links)

# ── 2. Configure QKD parameters ───────────────────────────────────────
qkd_params = QKDParameters(
    from_node="Alice",
    to_node="Bob",
    fidelity_target=0.90,
    max_latency_ms=10_000.0,
    rounds=200,                # number of BB84 signal rounds
    error_rate_tolerance=0.11,  # BB84 threshold for abort
    sifting_overhead_ratio=0.5,
    privacy_amplification_factor=0.8,
)

# ── 3. Execute QKD protocol ────────────────────────────────────────────
result = engine.run_qkd(qkd_params)

print("=== BB84 QKD Key Exchange ===")
print(f"Success:          {result.success}")
print(f"Secret key length: {result.secret_key_length_bits} bits")
print(f"Efficiency rate:   {result.efficiency_rate:.4f}")
print(f"QBER:              {result.qber:.4f}")
print(f"Latency:           {result.latency_ms:.1f} ms")
print(f"Path:              {' → '.join(result.execution_path)}")
print(f"Rounds completed:  {result.rounds_completed} / {qkd_params.rounds}")
print(f"Rounds failed:     {result.rounds_failed}")

# ── 4. Run multiple QKD trials ────────────────────────────────────────
print("\n=== Multi-Round QKD Stats ===")
from qnet_core import QKDStats
qkd_stats = engine.simulate(
    from_node="Alice",
    to="Bob",
    fidelity_target=0.90,
    max_latency_ms=10_000.0,
    runs=50,
    seed=42,
)
print(f"QKD trial success rate: {qkd_stats.empirical_success_rate:.1%}")
