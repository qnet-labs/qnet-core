"""Test boundary conditions where constraints violate fundamental physics (speed of light & decoherence)."""
from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

print("Testing stress-testing physical constraints (Failure Case)...")

engine = QNetEngine()

# Intentional short memory lifetime to guarantee high decoherence drops
nodes = [
    NodeDefinition(id="node_a", memory_lifetime_t2=5.0),
    NodeDefinition(id="node_b", memory_lifetime_t2=5.0),
]

# Massive intercontinental scale link
links = [
    LinkDefinition(from_node="node_a", to="node_b", distance_km=4500.0, base_fidelity=0.82, generation_rate_hz=100.0),
]

engine.define_network(nodes=nodes, links=links)

# Requesting an impossible 2.0 ms latency over 4,500 km (Speed of light in fiber needs ~22.5 ms one-way)
stats = engine.simulate(
    from_node="node_a",
    to="node_b",
    fidelity_target=0.95,  # Impossible to attain on a single link without purification support
    max_latency_ms=2.0,    # Physically impossible speed-of-light constraint
    runs=100,
    strategy=StrategyType.LowestLatency,
)

print(f"  Total runs: {stats.total_runs}")
print(f"  Success rate: {stats.empirical_success_rate:.2%}")
print(f"  Mean latency: {stats.mean_latency_ms:.2f} ms")
print(f"  Mean fidelity: {stats.mean_fidelity:.4f}")
print(f"  Congestion drops: {stats.aggregate_congestion_drops}")
print(f"  Link utilization: {stats.link_utilization_heatmap}")
print()
