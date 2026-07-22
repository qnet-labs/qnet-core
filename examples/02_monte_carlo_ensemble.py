#!/usr/bin/env python3
"""
Example 2 — Monte Carlo Ensemble

Run simulate() over 1,000 trials and report success rate, mean latency,
and mean fidelity. Demonstrates the stochastic nature of link generation
in a quantum repeater network.

Run:
    python examples/02_monte_carlo_ensemble.py
"""

from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

# ── 1. Define a 4-node repeater chain ─────────────────────────────────
engine = QNetEngine()

nodes = [
    NodeDefinition(id="Alice", memory_lifetime_t2=1.0),
    NodeDefinition(id="R1",   memory_lifetime_t2=0.8),
    NodeDefinition(id="R2",   memory_lifetime_t2=0.8),
    NodeDefinition(id="Bob",  memory_lifetime_t2=1.0),
]

links = [
    LinkDefinition("Alice", "R1", distance_km=15.0, base_fidelity=0.92, generation_rate_hz=500.0),
    LinkDefinition("R1",   "R2", distance_km=15.0, base_fidelity=0.90, generation_rate_hz=500.0),
    LinkDefinition("R2",   "Bob", distance_km=15.0, base_fidelity=0.88, generation_rate_hz=500.0),
]

engine.define_network(nodes, links)

# ── 2. Run Monte Carlo ensemble ───────────────────────────────────────
stats = engine.simulate(
    from_node="Alice",
    to="Bob",
    fidelity_target=0.85,
    max_latency_ms=10_000.0,
    runs=1_000,
    strategy=StrategyType.HighestFidelity,
    seed=42,
)

# ── 3. Report results ─────────────────────────────────────────────────
print(f"Total runs:      {stats.total_runs}")
print(f"Success rate:    {stats.empirical_success_rate:.1%}")
print(f"Mean latency:    {stats.mean_latency_ms:.1f} ms")
print(f"Mean fidelity:   {stats.mean_fidelity:.4f}")
print(f"Congestion drops:{stats.aggregate_congestion_drops}")

# ── 4. Per-link utilization heatmap ─────────────────────────────────────
print("\nLink utilization (hit counts):")
for link_key, count in sorted(stats.link_utilization_heatmap.items()):
    print(f"  {link_key}: {count} hits")
