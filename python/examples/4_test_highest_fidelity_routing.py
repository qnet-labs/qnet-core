"""Test routing optimization prioritizing high fidelity (triggering purification loops)."""
from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

print("Testing highest success rate / fidelity routing...")

engine = QNetEngine()

nodes = [
    NodeDefinition(id="node_a", memory_lifetime_t2=250.0),
    NodeDefinition(id="node_b", memory_lifetime_t2=250.0),
    NodeDefinition(id="node_c", memory_lifetime_t2=250.0),
]

# Links with high generation rates supporting purification cycles
links = [
    LinkDefinition(from_node="node_a", to="node_b", distance_km=40.0, base_fidelity=0.96, generation_rate_hz=1500.0),
    LinkDefinition(from_node="node_b", to="node_c", distance_km=40.0, base_fidelity=0.96, generation_rate_hz=1500.0),
]

engine.define_network(nodes=nodes, links=links)

# Requesting a strict target that requires purification across multi-hop swapping
stats = engine.simulate(
    from_node="node_a",
    to="node_c",
    fidelity_target=0.93,
    max_latency_ms=250.0,
    runs=100,
    strategy=StrategyType.HighestFidelity,
)

print(f"  Total runs: {stats.total_runs}")
print(f"  Success rate: {stats.empirical_success_rate:.2%}")
print(f"  Mean latency: {stats.mean_latency_ms:.2f} ms")
print(f"  Mean fidelity: {stats.mean_fidelity:.4f}")
print(f"  Congestion drops: {stats.aggregate_congestion_drops}")
print(f"  Link utilization: {stats.link_utilization_heatmap}")
print()
