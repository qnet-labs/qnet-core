#!/usr/bin/env python3
"""
Example 1 — Basic Entanglement Request (Minimal End-to-End Flow)

Two nodes connected by a single fiber link, one request_entanglement() call.
This is the minimal complete workflow: define → simulate → inspect result.

Run:
    pip install qnet-core
    python examples/01_basic_entanglement.py
"""

from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

# ── 1. Define network topology ────────────────────────────────────────
engine = QNetEngine()

nodes = [
    NodeDefinition(id="Alice", memory_lifetime_t2=0.5),   # 500 ms coherence
    NodeDefinition(id="Bob",   memory_lifetime_t2=0.5),
]

links = [
    LinkDefinition(
        from_node="Alice",
        to="Bob",
        distance_km=10.0,            # 10 km fiber
        base_fidelity=0.95,          # high-quality link
        generation_rate_hz=1_000.0,  # 1 kHz generation rate
    ),
]

engine.define_network(nodes, links)

# ── 2. Request a single entanglement distribution ─────────────────────
result = engine.request_entanglement(
    from_node="Alice",
    to="Bob",
    fidelity_target=0.90,          # minimum acceptable fidelity
    max_latency_ms=5_000.0,        # 5-second timeout
    strategy=StrategyType.HighestFidelity,
)

# ── 3. Inspect the result ─────────────────────────────────────────────
print(f"Success:      {result.success}")
print(f"Latency:      {result.latency_ms:.1f} ms")
print(f"Fidelity:     {result.final_fidelity:.4f}")
print(f"Path:         {' → '.join(result.execution_path)}")
