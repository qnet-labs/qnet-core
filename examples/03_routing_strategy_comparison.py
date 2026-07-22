#!/usr/bin/env python3
"""
Example 3 — Routing Strategy Comparison

The same topology run under three routing strategies to show how strategy
choice changes outcomes: latency vs fidelity trade-offs.

Run:
    python examples/03_routing_strategy_comparison.py
"""

from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

# ── 1. Define a 3-hop network with asymmetric links ───────────────────
engine = QNetEngine()

nodes = [
    NodeDefinition(id="Alice", memory_lifetime_t2=1.0),
    NodeDefinition(id="R1",   memory_lifetime_t2=0.8),
    NodeDefinition(id="R2",   memory_lifetime_t2=0.6),
    NodeDefinition(id="Bob",  memory_lifetime_t2=1.0),
]

links = [
    # Short but noisy link (high fidelity, fast generation)
    LinkDefinition("Alice", "R1", distance_km=5.0, base_fidelity=0.97, generation_rate_hz=2_000.0),
    # Long but clean link (lower fidelity, slower generation)
    LinkDefinition("R1", "R2", distance_km=30.0, base_fidelity=0.80, generation_rate_hz=200.0),
    # Medium link
    LinkDefinition("R2", "Bob", distance_km=15.0, base_fidelity=0.90, generation_rate_hz=800.0),
]

engine.define_network(nodes, links)

# ── 2. Run under each strategy ─────────────────────────────────────────
strategies = [
    StrategyType.LowestLatency,
    StrategyType.HighestFidelity,
    StrategyType.HighestSuccess,
]

print(f"{'Strategy':<20} {'Success':>8} {'Latency':>10} {'Fidelity':>10}")
print("-" * 52)

for strategy in strategies:
    stats = engine.simulate(
        from_node="Alice",
        to="Bob",
        fidelity_target=0.75,
        max_latency_ms=10_000.0,
        runs=500,
        strategy=strategy,
        seed=42,
    )
    print(f"{strategy:<20} {stats.empirical_success_rate:>7.1%} "
          f"{stats.mean_latency_ms:>9.1f} ms {stats.mean_fidelity:>9.4f}")
