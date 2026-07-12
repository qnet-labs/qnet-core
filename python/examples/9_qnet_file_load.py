"""Load a .qnet topology file and run simulation — no manual NodeDefinition / LinkDefinition needed."""

import os
from qnet_core import from_qnet_file, validate, PyQNetFile

base = os.path.dirname(os.path.abspath(__file__))

# --- Load network_v1.qnet (simple 2-node fiber link) ---
v1_path = os.path.join(base, "../network_v1.qnet")

result = validate(v1_path)
print(f"network_v1.qnet — valid={result['valid']}, errors={result['errors']}")

engine = from_qnet_file(v1_path)

# Node names come directly from the .qnet file (snake_case in this fixture)
stats = engine.simulate(
    from_node="node_a",
    to="node_b",
    fidelity_target=0.7,
    max_latency_ms=5000.0,  # generous timeout for a 100 km link
    runs=100,
)

print("\nLoaded network_v1.qnet (100 km fiber, base_fidelity=0.9):")
print(f"  Success rate:   {stats.empirical_success_rate:.2%}")
print(f"  Mean latency:   {stats.mean_latency_ms:.2f} ms")
print(f"  Mean fidelity:  {stats.mean_fidelity:.4f}")

# --- Load the Toronto-London satellite-fiber topology ---
toronto_path = os.path.join(base, "../toronto_london.qnet")
engine = from_qnet_file(toronto_path)

stats = engine.simulate(
    from_node="Toronto",
    to="London",
    fidelity_target=0.75,
    max_latency_ms=5000.0,
    runs=100,
)
print("\nLoaded toronto_london.qnet (4-node hybrid satellite/fiber):")
print(f"  Success rate:   {stats.empirical_success_rate:.2%}")
print(f"  Mean latency:   {stats.mean_latency_ms:.2f} ms")
print(f"  Mean fidelity:  {stats.mean_fidelity:.4f}")

# --- Round-trip: save a programmatic topology, reload it ---
roundtrip_path = os.path.join(base, "tmp_roundtrip.qnet")
f = PyQNetFile("my_custom_network")
f.add_node("alpha", memory_lifetime_ms=200.0)
f.add_node("beta", memory_lifetime_ms=200.0)
f.add_link("", "alpha", "beta", 25.0, 0.94, 1000.0)
f.save(roundtrip_path)

engine2 = from_qnet_file(roundtrip_path)
stats = engine2.simulate(
    from_node="alpha", to="beta",
    fidelity_target=0.85, max_latency_ms=500.0,
    runs=100,
)
print("\nRound-trip (saved -> loaded):")
print(f"  Success rate:   {stats.empirical_success_rate:.2%}")
print(f"  Mean latency:   {stats.mean_latency_ms:.2f} ms")

os.remove(roundtrip_path)
