"""Test performance drop-offs by sweeping the fidelity target parameter across multiple batches."""
from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

print("Testing sensitivity analysis...")

engine = QNetEngine()

nodes = [
    NodeDefinition(id="node_a", memory_lifetime_t2=120.0),
    NodeDefinition(id="node_b", memory_lifetime_t2=120.0),
]

links = [
    LinkDefinition(from_node="node_a", to="node_b", distance_km=15.0, base_fidelity=0.93, generation_rate_hz=800.0),
]

engine.define_network(nodes=nodes, links=links)

# Iteratively sweep fidelity expectations to trace the constraint breakdown curve
fidelity_sweeps = [0.80, 0.85, 0.90, 0.95]

for target in fidelity_sweeps:
    stats = engine.simulate(
        from_node="node_a",
        to="node_b",
        fidelity_target=target,
        max_latency_ms=100.0,
        runs=50,
        strategy=StrategyType.HighestFidelity,
    )

    print(f"  Fidelity Target Sweep: {target:.2f}")
    print(f"    Success rate: {stats.empirical_success_rate:.2%}")
    print(f"    Mean latency: {stats.mean_latency_ms:.2f} ms")
    print(f"    Mean fidelity: {stats.mean_fidelity:.4f}")
print()
