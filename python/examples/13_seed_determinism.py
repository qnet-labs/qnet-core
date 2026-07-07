"""Demonstrate that setting a seed makes Monte Carlo results fully reproducible."""

from qnet_core import QNetEngine, NodeDefinition, LinkDefinition

# --- Define a network with some randomness in link generation ---
nodes = [NodeDefinition(id="X", memory_lifetime_t2=150.0),
         NodeDefinition(id="Y", memory_lifetime_t2=150.0)]
links = [LinkDefinition(from_node="X", to="Y", distance_km=10.0,
                       base_fidelity=0.93, generation_rate_hz=800.0)]

# --- Run 3 times with the same seed — results must match exactly ---
print("Same seed (42) x 3 runs:")
results = []
for i in range(3):
    engine = QNetEngine()
    engine.define_network(nodes=nodes, links=links)
    stats = engine.simulate(from_node="X", to="Y", fidelity_target=0.85,
                            max_latency_ms=200.0, runs=100, seed=42)
    results.append(stats)
    print(f"  Run {i+1}: success={stats.empirical_success_rate:.2%} "
          f"latency={stats.mean_latency_ms:.2f} ms fidelity={stats.mean_fidelity:.4f}")

# Verify determinism
assert results[0].empirical_success_rate == results[1].empirical_success_rate
assert results[1].empirical_success_rate == results[2].empirical_success_rate
print("\n  All three runs produced identical results (deterministic OK)")

# --- Now run without seed — results should differ ---
print("\nNo seed (random) x 3 runs:")
results = []
for i in range(3):
    engine = QNetEngine()
    engine.define_network(nodes=nodes, links=links)
    stats = engine.simulate(from_node="X", to="Y", fidelity_target=0.85,
                            max_latency_ms=200.0, runs=100)
    results.append(stats)
    print(f"  Run {i+1}: success={stats.empirical_success_rate:.2%} "
          f"latency={stats.mean_latency_ms:.2f} ms fidelity={stats.mean_fidelity:.4f}")

print("\n  Results differ across runs (random OK)")

# --- Practical tip: use seeds when you want to compare configs ---
print("\nPractical comparison — seed = control variable:")
engine1 = QNetEngine()
engine1.define_network(nodes=nodes, links=links)
s1 = engine1.simulate("X", "Y", 0.85, 200.0, 100, seed=42)

nodes2 = [NodeDefinition(id="X", memory_lifetime_t2=300.0),  # double the memory
          NodeDefinition(id="Y", memory_lifetime_t2=300.0)]
links2 = [LinkDefinition(from_node="X", to="Y", distance_km=10.0,
                        base_fidelity=0.93, generation_rate_hz=800.0)]

engine2 = QNetEngine()
engine2.define_network(nodes=nodes2, links=links2)
s2 = engine2.simulate("X", "Y", 0.85, 200.0, 100, seed=42)

print(f"  T2=150ms: success={s1.empirical_success_rate:.2%}")
print(f"  T2=300ms: success={s2.empirical_success_rate:.2%}")
print("  ↑ Same seed lets you attribute difference to the config change, not randomness")
