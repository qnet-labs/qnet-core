#!/usr/bin/env python3
"""
Example 4 — Built-in Topology Generators Tour

Generate and inspect the three built-in topology templates:
- telecom_backbone: urban fiber network with intermediate repeaters
- repeater_chain: linear chain for long-distance point-to-point links
- hybrid_satellite_fiber: satellite uplink + fiber ground segments

These are factory functions that return fully-formed NetworkTopologyPayload
objects ready to pass to engine.define_network().

Run:
    python examples/04_builtin_topology_generators.py
"""

from qnet_core import (
    QNetEngine,
    generate_topology,
)


def inspect_payload(payload):
    """Pretty-print a generated topology."""
    print(f"  Nodes ({len(payload.nodes)}): {[n.id for n in payload.nodes]}")
    print(f"  Links ({len(payload.links)}):")
    for link in payload.links:
        link_type = getattr(link.link_type, "_name", "Fiber") or "Fiber"
        extra = ""
        if hasattr(link, "satellite_conditions") and link.satellite_conditions:
            cond = link.satellite_conditions
            extra = f" [satellite vis={cond.visibility:.1f} weather={cond.weather_factor:.1f}]"
        print(f"    {link.from_node} → {link.to}: {link.distance_km:.0f} km, "
              f"fidelity={link.base_fidelity:.2f}, rate={link.generation_rate_hz:.0f} Hz{extra}")


# ── 1. Generate and inspect each topology ──────────────────────────────
for name in ("telecom_backbone", "repeater_chain", "hybrid_satellite_fiber"):
    print(f"\n=== {name} ===")
    payload = generate_topology(name)
    inspect_payload(payload)

# ── 2. Use one in a quick simulation ───────────────────────────────────
print("\n--- Quick simulation on generated topology ---")
engine = QNetEngine()
payload = generate_topology("telecom_backbone")
engine.define_network(payload.nodes, payload.links)

nodes_ids = [n.id for n in payload.nodes]
if len(nodes_ids) >= 2:
    stats = engine.simulate(
        from_node=nodes_ids[0],
        to=nodes_ids[-1],
        fidelity_target=0.85,
        max_latency_ms=10_000.0,
        runs=100,
        seed=42,
    )
    print(f"Success rate: {stats.empirical_success_rate:.1%}")
    print(f"Mean latency: {stats.mean_latency_ms:.1f} ms")
    print(f"Mean fidelity: {stats.mean_fidelity:.4f}")
