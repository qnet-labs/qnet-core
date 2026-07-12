"""Inspect link utilization heatmap to see which links are bottlenecked in a mesh."""

from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

# --- 4-node diamond: X -- A --> D   /   X -- B --> D --- | ---> | ---> E ---
nodes = [NodeDefinition(id=n, memory_lifetime_t2=150.0) for n in ["X", "A", "B", "D"]]

links = [
    LinkDefinition(from_node="X", to="A", distance_km=10.0,
                   base_fidelity=0.96, generation_rate_hz=2000.0),  # fast short
    LinkDefinition(from_node="A", to="D", distance_km=50.0,
                   base_fidelity=0.88, generation_rate_hz=400.0),   # medium
    LinkDefinition(from_node="X", to="B", distance_km=60.0,
                   base_fidelity=0.87, generation_rate_hz=300.0),   # slow long
    LinkDefinition(from_node="B", to="D", distance_km=10.0,
                   base_fidelity=0.95, generation_rate_hz=1500.0),  # fast short
]

print("Network: X -- A --> D")
for lnk in links:
    print(f"  {lnk.from_node} -> {lnk.to}: {lnk.distance_km} km, "
          f"fidelity={lnk.base_fidelity}, rate={lnk.generation_rate_hz} Hz")
print()

engine = QNetEngine()
engine.define_network(nodes=nodes, links=links)

stats = engine.simulate(
    from_node="X", to="D",
    fidelity_target=0.85, max_latency_ms=500.0,
    runs=200, strategy=StrategyType.HighestFidelity,
)

print("Monte Carlo (200 runs, X -> D):")
print(f"  Success rate:   {stats.empirical_success_rate:.2%}")
print(f"  Mean latency:   {stats.mean_latency_ms:.2f} ms")
print(f"  Mean fidelity:  {stats.mean_fidelity:.4f}")

print()
print("Link utilization heatmap (absolute use counts):")
max_count = max(stats.link_utilization_heatmap.values()) if stats.link_utilization_heatmap else 1
for link_key, count in stats.link_utilization_heatmap.items():
    bar_len = int(count / max_count * 30)
    bar = "#" * max(bar_len, 1)
    print(f"  {link_key:15s} {count:6d}  {bar}")

print()
print("Interpretation:")
print("  In a HighestFidelity strategy, the simulator picks X->A->D (shorter total")
print("  distance = less decoherence). The bottom path X->B->D is underused.")
