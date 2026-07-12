"""Test using generate_topology function with different topologies."""
from qnet_core import QNetEngine, StrategyType, generate_topology

print("Testing generate_topology function...")

# Generate a hybrid satellite-fiber topology
net = generate_topology("hybrid_satellite_fiber")
print(f"  Generated a hybrid topology with {len(net.nodes)} nodes and {len(net.links)} links")

# Define engine with generated topology
engine = QNetEngine()
engine.define_network(nodes=net.nodes, links=net.links)

# Run simulation on the generated topology
stats = engine.simulate(
    from_node="Toronto",
    to="London",
    fidelity_target=0.75,
    max_latency_ms=5000.0,
    runs=100,
    strategy=StrategyType.HighestFidelity,
)

print(f"  Success rate: {stats.empirical_success_rate:.2%}")
print(f"  Mean latency: {stats.mean_latency_ms:.2f} ms")
print(f"  Mean fidelity: {stats.mean_fidelity:.4f}")
print()

# Generate a telecom backbone topology
net = generate_topology("telecom_backbone")
print(f"  Generated telecom backbone with {len(net.nodes)} nodes and {len(net.links)} links")

# Generate a repeater chain topology
net = generate_topology("repeater_chain")
print(f"  Generated repeater chain with {len(net.nodes)} nodes and {len(net.links)} links")
print()


# nodes = [
#     NodeDefinition("A", 120.0),  # Edge device (low memory)
#     NodeDefinition("B", 150.0),  # Metro core
#     NodeDefinition("C", 200.0),  # Regional hub
#     NodeDefinition("D", 200.0),  # Backbone node
#     NodeDefinition("E", 250.0),  # Intercontinental gateway
#     NodeDefinition("F", 180.0),  # Alternate relay
# ]

# links = [
#     # Metro (fast, low loss)
#     LinkDefinition("A", "B", distance_km=5, base_fidelity=0.97, generation_rate_hz=2000),

#     # Regional backbone
#     LinkDefinition("B", "C", distance_km=80, base_fidelity=0.93, generation_rate_hz=1200),

#     # Long-haul backbone
#     LinkDefinition("C", "D", distance_km=600, base_fidelity=0.88, generation_rate_hz=600),

#     # Transatlantic (hard link)
#     LinkDefinition("D", "E", distance_km=5500, base_fidelity=0.75, generation_rate_hz=150),

#     # Alternative path (more stable but longer)
#     LinkDefinition("C", "F", distance_km=900, base_fidelity=0.91, generation_rate_hz=500),
#     LinkDefinition("F", "E", distance_km=4800, base_fidelity=0.80, generation_rate_hz=200),

#     # Shortcut (fewer hops but worse physics)
#     LinkDefinition("B", "F", distance_km=1200, base_fidelity=0.85, generation_rate_hz=400),
# ]

# net = generate_topology("hybrid_satellite_fiber")
# print(f"  Generated hybrid topology with {len(net.nodes)} nodes and {len(net.links)} links")

# # engine.load_network(net)

# # result = engine.simulate(
# #     from_node="Toronto",
# #     to="London",
# #     runs=500,
# #     strategy=StrategyType.HighestFidelity
# # )

# engine = QNetEngine()
# engine.define_network(nodes=net.nodes, links=net.links)
# # engine.load_network(net)

# stats = engine.simulate(
#     from_node="A",
#     to="E",
#     fidelity_target=0.50,
#     max_latency_ms=150,
#     runs=1500,
#     strategy=StrategyType.LowestLatency
# )

# print(f"  Total runs: {stats.total_runs}")
# print(f"  Success rate: {stats.empirical_success_rate * 100:.2f}%")
# print(f"  Mean latency: {stats.mean_latency_ms:.2f} ms")
# print(f"  Mean fidelity: {stats.mean_fidelity:.4f}")
# print(f"  Congestion drops: {stats.aggregate_congestion_drops}")
# print(f"  Link utilization: {stats.link_utilization_heatmap}")
# print()
