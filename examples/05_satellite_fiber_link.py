#!/usr/bin/env python3
"""
Example 5 — Hybrid Satellite-Fiber Link

A satellite-to-ground link combined with fiber segments. Demonstrates:
- Using SatelliteConditions (visibility, weather) to degrade the effective rate
- Comparing clear-sky vs degraded atmospheric conditions

Run:
    python examples/05_satellite_fiber_link.py
"""

from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType


def build_satellite_network(sat_conditions=None):
    """Build a network with one satellite uplink and two fiber segments."""
    engine = QNetEngine()

    nodes = [
        NodeDefinition(id="Sat",  memory_lifetime_t2=0.3),   # satellite node
        NodeDefinition(id="G1",  memory_lifetime_t2=1.0),     # ground station 1
        NodeDefinition(id="G2",  memory_lifetime_t2=1.0),     # ground station 2
    ]

    links = [
        LinkDefinition(
            "Sat", "G1", distance_km=500.0, base_fidelity=0.85,
            generation_rate_hz=100.0, link_type="Satellite",
            satellite_conditions=sat_conditions,
        ),
        LinkDefinition("G1", "G2", distance_km=50.0, base_fidelity=0.93,
                       generation_rate_hz=1_000.0),
    ]

    engine.define_network(nodes, links)
    return engine


def run_scenario(label, sat_conditions):
    """Run a satellite-fiber scenario and print results."""
    engine = build_satellite_network(sat_conditions)

    stats = engine.simulate(
        from_node="Sat",
        to="G2",
        fidelity_target=0.80,
        max_latency_ms=30_000.0,
        runs=500,
        strategy=StrategyType.HighestFidelity,
        seed=42,
    )

    effective_rate = (
        sat_conditions.effective_rate(100.0) if sat_conditions else 100.0
    )
    print(f"\n{label}")
    print(f"  Effective link rate: {effective_rate:.1f} Hz")
    print(f"  Success rate:      {stats.empirical_success_rate:.1%}")
    print(f"  Mean latency:      {stats.mean_latency_ms:.1f} ms")
    print(f"  Mean fidelity:     {stats.mean_fidelity:.4f}")


# ── 1. Clear sky (visibility=1.0, weather_factor=1.0) ──────────────────
run_scenario(
    "=== Scenario A: Clear Sky ===",
    sat_conditions=None,
)

# ── 2. Partial cloud cover ─────────────────────────────────────────────
cloudy = type("SatCond", (), {
    "visibility": 0.7,
    "weather_factor": 0.8,
    "effective_rate": lambda self, base: base * self.visibility * self.weather_factor,
})()
run_scenario(
    "\n=== Scenario B: Partial Cloud Cover (vis=0.7, weather=0.8) ===",
    sat_conditions=cloudy,
)

# ── 3. Heavy storm degradation ─────────────────────────────────────────
storm = type("SatCond", (), {
    "visibility": 0.4,
    "weather_factor": 0.5,
    "effective_rate": lambda self, base: base * self.visibility * self.weather_factor,
})()
run_scenario(
    "\n=== Scenario C: Heavy Storm (vis=0.4, weather=0.5) ===",
    sat_conditions=storm,
)
