"""Explore pre-built network topologies and simulate entanglement across them."""

from qnet_core import QNetEngine, StrategyType, generate_topology, compare_topologies, TopologyEndpoints

# --- Build a hybrid satellite-fiber network from scratch ---
net = generate_topology("hybrid_satellite_fiber")
engine = QNetEngine()
engine.define_network(nodes=net.nodes, links=net.links)

print("Hybrid satellite-fiber topology:")
print(f"  Nodes: {len(net.nodes)}  Links: {len(net.links)}")
node_names = [n.id for n in net.nodes]
link_pairs = [(lnk.from_node, lnk.to) for lnk in net.links]
print(f"  Node names: {node_names}")
print(f"  Links: {link_pairs}")

stats = engine.simulate(
    from_node="Toronto",
    to="London",
    fidelity_target=0.75,
    max_latency_ms=5000.0,
    runs=100,
    strategy=StrategyType.HighestFidelity,
)
print("  Toronto -> London:")
print(f"    Success rate: {stats.empirical_success_rate:.2%}")
print(f"    Mean latency: {stats.mean_latency_ms:.2f} ms")
print(f"    Mean fidelity: {stats.mean_fidelity:.4f}")
print()

# --- Build a telecom backbone network ---
net = generate_topology("telecom_backbone")
engine = QNetEngine()
engine.define_network(nodes=net.nodes, links=net.links)

print("Telecom backbone topology:")
print(f"  Nodes: {len(net.nodes)}  Links: {len(net.links)}")
node_names = [n.id for n in net.nodes]
link_pairs = [(lnk.from_node, lnk.to) for lnk in net.links]
print(f"  Node names: {node_names}")
print(f"  Links: {link_pairs}")

stats = engine.simulate(
    from_node="A",
    to="C",
    fidelity_target=0.75,
    max_latency_ms=5000.0,
    runs=100,
    strategy=StrategyType.HighestFidelity,
)
print("  A -> C:")
print(f"    Success rate: {stats.empirical_success_rate:.2%}")
print(f"    Mean latency: {stats.mean_latency_ms:.2f} ms")
print(f"    Mean fidelity: {stats.mean_fidelity:.4f}")
print()

# --- Build a repeater chain topology ---
net = generate_topology("repeater_chain")
engine = QNetEngine()
engine.define_network(nodes=net.nodes, links=net.links)

print("Repeater chain topology:")
print(f"  Nodes: {len(net.nodes)}  Links: {len(net.links)}")
node_names = [n.id for n in net.nodes]
link_pairs = [(lnk.from_node, lnk.to) for lnk in net.links]
print(f"  Node names: {node_names}")
print(f"  Links: {link_pairs}")

stats = engine.simulate(
    from_node="0",
    to=str(len(net.nodes) - 1),
    fidelity_target=0.75,
    max_latency_ms=5000.0,
    runs=100,
    strategy=StrategyType.HighestFidelity,
)
print(f"  0 -> {len(net.nodes) - 1}:")
print(f"    Success rate: {stats.empirical_success_rate:.2%}")
print(f"    Mean latency: {stats.mean_latency_ms:.2f} ms")
print(f"    Mean fidelity: {stats.mean_fidelity:.4f}")
print()

# --- Compare topologies programmatically ---
report = compare_topologies(
    endpoints=[
        TopologyEndpoints("hybrid_satellite_fiber", "Toronto", "London"),
        TopologyEndpoints("telecom_backbone", "A", "C"),
    ],
    fidelity_target=0.75,
    max_latency_ms=5000.0,
    runs=100,
    strategy=StrategyType.HighestFidelity,
)

print(f"Topology comparison recommended: {report.recommended_topology}")
for result in report.results:
    print(f"  {result.topology_name}: success={result.success_rate:.1%} "
          f"latency={result.mean_latency_ms:.2f} ms fidelity={result.mean_fidelity:.4f}")
