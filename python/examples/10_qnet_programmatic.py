"""Build a .qnet file programmatically, save it, then load and simulate from it."""

import os
from qnet_core import (
    PyQNetFile, PyQNetNodeType, PyQNetLinkType, PyQNetSatelliteExtension,
    PyQNetConfig, PyQNetConstraints, validate, from_qnet_file,
)

base = os.path.dirname(os.path.abspath(__file__))
path = os.path.join(base, "tmp_programmatic.qnet")

# --- Build a network from scratch in .qnet format ---
f = PyQNetFile("my_star_network")
f.metadata.description = "Star topology with one satellite node"
f.metadata.author = "example"

# Center hub
f.add_node("hub", memory_lifetime_ms=200.0, memory_capacity=64,
           node_type=PyQNetNodeType.Ground)

# Three edge nodes
f.add_node("sat_1", memory_lifetime_ms=300.0, memory_capacity=128,
           node_type=PyQNetNodeType.Satellite)
f.add_node("site_b", memory_lifetime_ms=150.0, memory_capacity=32,
           node_type=PyQNetNodeType.Ground)
f.add_node("sat_2", memory_lifetime_ms=300.0, memory_capacity=128,
           node_type=PyQNetNodeType.Satellite)

# Links with satellite extension on one link
f.add_link("", "hub", "sat_1", 500.0, 0.95, 50.0,
           link_type=PyQNetLinkType.Satellite,
           satellite=PyQNetSatelliteExtension(visibility=0.90, weather_factor=0.95))
f.add_link("", "hub", "site_b", 75.0, 0.92, 1000.0)
f.add_link("", "hub", "sat_2", 600.0, 0.94, 40.0,
           link_type=PyQNetLinkType.Satellite,
           satellite=PyQNetSatelliteExtension(visibility=0.85, weather_factor=0.90))

# Global config
f.config = PyQNetConfig(alpha_loss=0.045, gamma_swapping=0.92)
f.constraints = PyQNetConstraints(fidelity_target=0.80, max_latency_ms=3000.0)

# --- Save first, then validate ---
f.save(path)
result = validate(path)
print("Validation: valid={}, errors={}".format(result['valid'], result['errors']))
print("Saved {} with {} nodes, {} links".format(path, len(f.nodes), len(f.links)))

# --- Load it back and simulate ---
engine = from_qnet_file(path)

stats = engine.simulate(
    from_node="sat_1",
    to="site_b",
    fidelity_target=0.80,
    max_latency_ms=3000.0,
    runs=100,
)

print()
print("Sat_1 -> site_b:")
print(f"  Success rate: {stats.empirical_success_rate:.2%}")
print(f"  Mean latency: {stats.mean_latency_ms:.2f} ms")
print(f"  Mean fidelity: {stats.mean_fidelity:.4f}")

# Clean up
os.remove(path)
