"""Test routing optimization prioritizing lowest latency across a multi-hop network."""
from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

print("Testing latency-optimized routing...")

engine = QNetEngine()

# Define a multi-hop repeater chain network
nodes = [
    NodeDefinition(id="node_a", memory_lifetime_t2=150.0),
    NodeDefinition(id="node_b", memory_lifetime_t2=150.0),
    NodeDefinition(id="node_c", memory_lifetime_t2=150.0),
]

links = [
    LinkDefinition(from_node="node_a", to="node_b", distance_km=25.0, base_fidelity=0.95, generation_rate_hz=1200.0),
    LinkDefinition(from_node="node_b", to="node_c", distance_km=25.0, base_fidelity=0.95, generation_rate_hz=1200.0),
]

engine.define_network(nodes=nodes, links=links)

# Simulate routing with an emphasis on speed
stats = engine.simulate(
    from_node="node_a",
    to="node_c",
    fidelity_target=0.85,
    max_latency_ms=150.0,
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
