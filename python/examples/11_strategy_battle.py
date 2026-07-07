"""Run all three routing strategies side-by-side on the same network and compare."""

from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

# --- Define a 3-node chain (two equal-length fiber links) ---
nodes = [
    NodeDefinition(id="Alice", memory_lifetime_t2=150.0),
    NodeDefinition(id="Repeater", memory_lifetime_t2=150.0),
    NodeDefinition(id="Bob", memory_lifetime_t2=150.0),
]

links = [
    LinkDefinition(from_node="Alice", to="Repeater", distance_km=30.0,
                   base_fidelity=0.90, generation_rate_hz=800.0),
    LinkDefinition(from_node="Repeater", to="Bob", distance_km=30.0,
                   base_fidelity=0.90, generation_rate_hz=800.0),
]

strategies = [
    (StrategyType.LowestLatency, "Lowest Latency"),
    (StrategyType.HighestFidelity, "Highest Fidelity"),
    (StrategyType.HighestSuccess, "Highest Success"),
]

print(f"Network: {' -> '.join(n.id for n in nodes)}")
for lnk in links:
    print(f"  {lnk.from_node} -> {lnk.to}: {lnk.distance_km} km, "
          f"fidelity={lnk.base_fidelity}, rate={lnk.generation_rate_hz} Hz")
print()

# Run each strategy and compare
for strategy, label in strategies:
    engine = QNetEngine()
    engine.define_network(nodes=nodes, links=links)

    stats = engine.simulate(
        from_node="Alice",
        to="Bob",
        fidelity_target=0.85,
        max_latency_ms=500.0,
        runs=200,
        strategy=strategy,
    )

    print(f"{label}:")
    print(f"  Success rate: {stats.empirical_success_rate:.2%}")
    print(f"  Mean latency:   {stats.mean_latency_ms:.2f} ms")
    print(f"  Mean fidelity:  {stats.mean_fidelity:.4f}")
    print(f"  Congestion drops: {stats.aggregate_congestion_drops}")
    print()
