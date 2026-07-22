#!/usr/bin/env python3
"""
Example 14 — Tuning Physical-Layer Constants

Vary PhysicalConfig parameters (fiber loss, speed of light in fiber) and show
how fidelity and success rate shift. Useful as a sensitivity-style demo to
understand which physical parameters dominate performance.

Run:
    python examples/14_tuning_physical_constants.py
"""

from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

# ── 1. Define baseline network ─────────────────────────────────────────
nodes = [
    NodeDefinition(id="Alice", memory_lifetime_t2=1.0),
    NodeDefinition(id="Bob",   memory_lifetime_t2=1.0),
]

links = [
    LinkDefinition("Alice", "Bob", distance_km=20.0, base_fidelity=0.90,
                   generation_rate_hz=1_000.0),
]


def run_with_alpha(alpha_db_per_km):
    """Run a simulation with a given fiber loss coefficient."""
    config = QNetEngine().__class__.__bases__[0].__dict__  # skip — use constructor param

    # PhysicalConfig controls baseline_purify_factor which is alpha_loss / 10.0
    from qnet_core import PhysicalConfig, SimulationConfig
    phys = PhysicalConfig(alpha_loss_db_km=alpha_db_per_km)
    sim_config = SimulationConfig(
        total_time_cutoff_ms=10_000.0,
        step_resolution_ms=0.1,
        alpha_loss_db_km=alpha_db_per_km,
    )
    eng = QNetEngine(config=sim_config)
    eng.define_network(nodes, links)

    stats = eng.simulate(
        from_node="Alice",
        to="Bob",
        fidelity_target=0.85,
        max_latency_ms=10_000.0,
        runs=300,
        strategy=StrategyType.HighestFidelity,
        seed=42,
    )
    return stats


# ── 2. Sweep fiber loss coefficient ────────────────────────────────────
print("=== Sensitivity: Fiber Loss Coefficient (alpha_loss_db_km) ===\n")
print(f"{'alpha (dB/km)':<16} {'Success':>9} {'Fidelity':>10} {'Latency':>12}")
print("-" * 52)

alphas = [0.10, 0.15, 0.20, 0.25, 0.30, 0.40]  # realistic range for telecom fiber

for alpha in alphas:
    stats = run_with_alpha(alpha)
    print(f"{alpha:<16.2f} {stats.empirical_success_rate:>8.1%} "
          f"{stats.mean_fidelity:>9.4f} {stats.mean_latency_ms:>11.1f} ms")


# ── 3. Sweep memory lifetime (T2) — another key physical parameter ─────
print("\n=== Sensitivity: Qubit Memory Lifetime (T2) ===\n")
print(f"{'T2 (s)':<14} {'Success':>9} {'Fidelity':>10} {'Latency':>12}")
print("-" * 48)

from qnet_core import SimulationConfig

t2_values = [0.1, 0.3, 0.5, 0.8, 1.0, 2.0]

for t2 in t2_values:
    nodes_t2 = [
        NodeDefinition(id="Alice", memory_lifetime_t2=t2),
        NodeDefinition(id="Bob",   memory_lifetime_t2=t2),
    ]
    sim_config = SimulationConfig(total_time_cutoff_ms=10_000.0, step_resolution_ms=0.1)
    eng = QNetEngine(config=sim_config)
    eng.define_network(nodes_t2, links)

    stats = eng.simulate(
        from_node="Alice",
        to="Bob",
        fidelity_target=0.85,
        max_latency_ms=10_000.0,
        runs=300,
        strategy=StrategyType.HighestFidelity,
        seed=42,
    )
    print(f"{t2:<14.1f} {stats.empirical_success_rate:>8.1%} "
          f"{stats.mean_fidelity:>9.4f} {stats.mean_latency_ms:>11.1f} ms")


# ── 4. Sweep generation rate ───────────────────────────────────────────
print("\n=== Sensitivity: Link Generation Rate (Hz) ===\n")
print(f"{'Rate (Hz)':<12} {'Success':>9} {'Fidelity':>10} {'Latency':>12}")
print("-" * 48)

rates = [50, 100, 300, 500, 1_000, 3_000]

for rate in rates:
    links_rate = [
        LinkDefinition("Alice", "Bob", distance_km=20.0, base_fidelity=0.90,
                       generation_rate_hz=rate),
    ]
    sim_config = SimulationConfig(total_time_cutoff_ms=10_000.0, step_resolution_ms=0.1)
    eng = QNetEngine(config=sim_config)
    eng.define_network(nodes, links_rate)

    stats = eng.simulate(
        from_node="Alice",
        to="Bob",
        fidelity_target=0.85,
        max_latency_ms=10_000.0,
        runs=300,
        strategy=StrategyType.HighestFidelity,
        seed=42,
    )
    print(f"{rate:<12,d} {stats.empirical_success_rate:>8.1%} "
          f"{stats.mean_fidelity:>9.4f} {stats.mean_latency_ms:>11.1f} ms")


print("\n=> Key insights:")
print("   - Higher alpha (more loss) → lower fidelity, more congestion drops")
print("   - Longer T2 → higher success rate for long-distance paths")
print("   - Higher generation rate → faster convergence but same fidelity floor")
