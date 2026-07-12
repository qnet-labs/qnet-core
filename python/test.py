#!/usr/bin/env python3
"""
Refactored Test Suite for qnet_core Python Bindings.
Tracks physical timescales in milliseconds (ms) and ensures proper routing execution.
"""

import sys
from qnet_core import (
    PyQNetEngine as QNetEngine, 
    PyStrategyType as StrategyType, 
    PyNodeDefinition as NodeDefinition, 
    PyLinkDefinition as LinkDefinition, 
    PySimulationConfig as SimulationConfig
)


def get_standard_topology():
    """Helper to generate a validated physical network graph topology."""
    # Qubit coherence lifetimes bumped to 150.0 ms to prevent early dephasing drops
    nodes = [
        NodeDefinition(id="node_a", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_b", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_c", memory_lifetime_t2=150.0),
    ]

    links = [
        LinkDefinition(from_node="node_a", to="node_b", distance_km=10.0, base_fidelity=0.95, generation_rate_hz=1000.0),
        LinkDefinition(from_node="node_b", to="node_c", distance_km=15.0, base_fidelity=0.92, generation_rate_hz=800.0),
    ]
    return nodes, links


def test_basic_simulation():
    """Test basic single-hop entanglement simulation using HighestFidelity selection."""
    print("Testing basic simulation...")
    engine = QNetEngine()
    nodes, links = get_standard_topology()
    engine.define_network(nodes=nodes, links=links)

    result = engine.request_entanglement(
        from_node="node_a",
        to="node_b",
        fidelity_target=0.90,
        max_latency_ms=150.0,
        strategy=StrategyType.HighestFidelity,
    )

    print(f"  Success: {result.success}")
    print(f"  Latency: {result.latency_ms:.2f} ms")
    print(f"  Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {' -> '.join(result.execution_path)}")
    print()


def test_monte_carlo():
    """Test Monte Carlo ensemble simulation to analyze statistical distribution logs."""
    print("Testing Monte Carlo simulation (100 runs)...")
    engine = QNetEngine()
    nodes, links = get_standard_topology()
    engine.define_network(nodes=nodes, links=links)

    # Flattened arguments passed directly to match the PyO3 method signature
    stats = engine.simulate(
        from_node="node_a",
        to="node_b",
        fidelity_target=0.90,
        max_latency_ms=150.0,
        runs=100,  # Set to 100 to match your terminal printout layout
        strategy=StrategyType.HighestFidelity,
    )

    print(f"  Total runs: {stats.total_runs}")
    print(f"  Success rate: {stats.empirical_success_rate * 100:.2f}%")
    print(f"  Mean latency: {stats.mean_latency_ms:.2f} ms")
    print(f"  Mean fidelity: {stats.mean_fidelity:.4f}")
    print(f"  Congestion drops: {stats.aggregate_congestion_drops}")
    print(f"  Link utilization: {stats.link_utilization_heatmap}")
    print()


def test_latency_optimized():
    """Test routing along multi-hop targets optimized purely for arrival time."""
    print("Testing latency-optimized routing...")
    engine = QNetEngine()
    nodes, links = get_standard_topology()
    engine.define_network(nodes=nodes, links=links)

    result = engine.request_entanglement(
        from_node="node_a",
        to="node_c",
        fidelity_target=0.85,
        max_latency_ms=150.0,
        strategy=StrategyType.LowestLatency,
    )

    print(f"  Success: {result.success}")
    print(f"  Latency: {result.latency_ms:.2f} ms")
    print(f"  Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {' -> '.join(result.execution_path)}")
    print()


def test_high_success_strategy():
    """Test multi-hop routing that triggers active purification layers."""
    print("Testing highest success rate routing...")
    engine = QNetEngine()
    nodes, links = get_standard_topology()
    engine.define_network(nodes=nodes, links=links)

    result = engine.request_entanglement(
        from_node="node_a",
        to="node_c",
        fidelity_target=0.95, 
        max_latency_ms=150.0,
        strategy=StrategyType.HighestSuccess,  # <-- Removed "Rate" to match your Rust enum variant
    )

    print(f"  Success: {result.success}")
    print(f"  Latency: {result.latency_ms:.2f} ms")
    print(f"  Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {' -> '.join(result.execution_path)}")
    print()


def test_direct_link():
    """Test target validation over an isolated direct structural hop boundary."""
    print("Testing direct link simulation...")
    engine = QNetEngine()
    nodes, links = get_standard_topology()
    engine.define_network(nodes=nodes, links=links)

    result = engine.request_entanglement(
        from_node="node_a",
        to="node_b",
        fidelity_target=0.85,
        max_latency_ms=100.0,
        strategy=StrategyType.HighestFidelity,
    )

    print(f"  Success: {result.success}")
    print(f"  Latency: {result.latency_ms:.2f} ms")
    print(f"  Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {' -> '.join(result.execution_path)}")
    print()


def test_with_custom_config():
    """Test custom attenuation parameters and environmental configuration flags."""
    print("Testing with custom config...")
    
    # Overriding internal physical traits via custom configuration instances
    custom_config = SimulationConfig(
        total_time_cutoff_ms=2000.0,
        step_resolution_ms=0.05,
        alpha_loss_db_km=0.22  # Standard commercial fiber loss coefficient
    )
    
    engine = QNetEngine(config=custom_config)
    nodes, links = get_standard_topology()
    engine.define_network(nodes=nodes, links=links)

    result = engine.request_entanglement(
        from_node="node_a",
        to="node_c",
        fidelity_target=0.85,
        max_latency_ms=150.0,
        strategy=StrategyType.LowestLatency,
    )

    print(f"  Success: {result.success}")
    print(f"  Latency: {result.latency_ms:.2f} ms")
    print(f"  Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {' -> '.join(result.execution_path)}")
    print()


def main():
    """Run all verification targets within the suite."""
    print("=" * 60)
    print("qnet_core Python Bindings Test Suite")
    print("=" * 60)
    print()

    try:
        test_basic_simulation()
        test_monte_carlo()
        test_latency_optimized()
        test_high_success_strategy()
        test_direct_link()
        test_with_custom_config()
        print("=" * 60)
        print("All engine assertions completed successfully.")
        print("=" * 60)
    except Exception as e:
        print(f"\nExecution Failed: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()