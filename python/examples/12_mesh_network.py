"""Simulate entanglement on a mesh topology with multiple path choices."""

from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

# --- 5-node mesh: A ── B ── D --> \    / <-- C ── D --> E --- | ---> | ---> | --- F --- --- G --- --- H --- I --- J ----
nodes = [NodeDefinition(id=n, memory_lifetime_t2=150.0) for n in ["A", "B", "C", "D", "E"]]

links = [
    # Left side paths to D
    LinkDefinition(from_node="A", to="B", distance_km=20.0, base_fidelity=0.94, generation_rate_hz=1000.0),
    LinkDefinition(from_node="B", to="D", distance_km=20.0, base_fidelity=0.93, generation_rate_hz=1000.0),
    # Right side paths to D
    LinkDefinition(from_node="A", to="C", distance_km=40.0, base_fidelity=0.90, generation_rate_hz=800.0),
    LinkDefinition(from_node="C", to="D", distance_km=40.0, base_fidelity=0.89, generation_rate_hz=800.0),
    # From D to destination
    LinkDefinition(from_node="D", to="E", distance_km=50.0, base_fidelity=0.87, generation_rate_hz=600.0),
]

# --- Strategy 1: Lowest Latency (should pick A->B->D->E) ---
engine = QNetEngine()
engine.define_network(nodes=nodes, links=links)

result_low_latency = engine.request_entanglement(
    from_node="A", to="E",
    fidelity_target=0.80, max_latency_ms=500.0,
    strategy=StrategyType.LowestLatency,
)
print("Lowest Latency:")
print(f"  Path:     {' -> '.join(result_low_latency.execution_path)}")
print(f"  Success:  {result_low_latency.success}")
print(f"  Latency:  {result_low_latency.latency_ms:.2f} ms")
print(f"  Fidelity: {result_low_latency.final_fidelity:.4f}")

# --- Strategy 2: Highest Fidelity (should pick A->B->D->E, higher fidelity links) ---
engine = QNetEngine()
engine.define_network(nodes=nodes, links=links)

result_high_fid = engine.request_entanglement(
    from_node="A", to="E",
    fidelity_target=0.85, max_latency_ms=500.0,
    strategy=StrategyType.HighestFidelity,
)
print()
print("Highest Fidelity:")
print(f"  Path:     {' -> '.join(result_high_fid.execution_path)}")
print(f"  Success:  {result_high_fid.success}")
print(f"  Latency:  {result_high_fid.latency_ms:.2f} ms")
print(f"  Fidelity: {result_high_fid.final_fidelity:.4f}")

# --- Strategy 3: Monte Carlo across all strategies ---
print()
print("Monte Carlo comparison (200 runs each):")
for strategy, label in [(StrategyType.LowestLatency, "Lowest Latency"),
                        (StrategyType.HighestFidelity, "Highest Fidelity")]:
    engine = QNetEngine()
    engine.define_network(nodes=nodes, links=links)

    stats = engine.simulate(
        from_node="A", to="E",
        fidelity_target=0.85, max_latency_ms=500.0,
        runs=200, strategy=strategy,
    )
    print(f"  {label:18s}  success={stats.empirical_success_rate:.1%} "
          f"latency={stats.mean_latency_ms:.1f} ms fidelity={stats.mean_fidelity:.4f}")
