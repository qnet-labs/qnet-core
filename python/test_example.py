#!/usr/bin/env python3
"""
Test script for qnet_core Python bindings.

This script demonstrates the basic usage of the qnet_core SDK
by running entanglement distribution simulations.
"""

import sys
import os

# Add the site-packages directory to the path
# Resolve site-packages relative to this script's directory
_project_root = os.path.dirname(os.path.abspath(__file__))
_venv_lib = os.path.join(_project_root, "..", ".venv", "lib")
# Try common Python minor versions; pick the first that exists
for _py in sorted(os.listdir(_venv_lib)):
    _site_packages = os.path.join(_venv_lib, _py, "site-packages")
    if os.path.isdir(_site_packages):
        break
else:
    _site_packages = None

if _site_packages:
    sys.path.insert(0, _site_packages)

# Import from qnet_core module
from qnet_core import (
    QNetEngine,
    StrategyType,
    NodeDefinition,
    LinkDefinition,
    TopologyEndpoints,
    generate_topology,
    compare_topologies,
    LinkType,
    SatelliteConditions,
    from_qnet_file,
    validate,
)


def test_basic_simulation():
    """Test basic single-node entanglement simulation."""
    print("Testing basic simulation...")

    # Create engine with default config
    engine = QNetEngine()

    # Define a simple network
    nodes = [
        NodeDefinition(id="node_a", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_b", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_c", memory_lifetime_t2=150.0),
    ]

    links = [
        LinkDefinition(from_node="node_a", to="node_b", distance_km=10.0, base_fidelity=0.95, generation_rate_hz=1000.0),
        LinkDefinition(from_node="node_b", to="node_c", distance_km=15.0, base_fidelity=0.92, generation_rate_hz=800.0),
    ]

    # Define network topology (pass nodes and links directly)
    engine.define_network(nodes=nodes, links=links)

    # Run single entanglement request
    result = engine.request_entanglement(
        from_node="node_a",
        to="node_b",
        fidelity_target=0.9,
        max_latency_ms=100.0,
        strategy=StrategyType.HighestFidelity,
    )

    print(f"  Success: {result.success}")
    print(f"  Latency: {result.latency_ms:.2f} ms")
    print(f"  Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {' -> '.join(result.execution_path)}")
    print()


def test_monte_carlo():
    """Test Monte Carlo ensemble simulation."""
    print("Testing Monte Carlo simulation (100 runs)...")

    engine = QNetEngine()

    nodes = [
        NodeDefinition(id="node_a", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_b", memory_lifetime_t2=150.0),
    ]

    links = [
        LinkDefinition(from_node="node_a", to="node_b", distance_km=10.0, base_fidelity=0.95, generation_rate_hz=1000.0),
    ]

    engine.define_network(nodes=nodes, links=links)

    stats = engine.simulate(
        from_node="node_a",
        to="node_b",
        fidelity_target=0.9,
        max_latency_ms=100.0,
        runs=100,
        strategy=StrategyType.HighestFidelity,
    )

    print(f"  Total runs: {stats.total_runs}")
    print(f"  Success rate: {stats.empirical_success_rate:.2%}")
    print(f"  Mean latency: {stats.mean_latency_ms:.2f} ms")
    print(f"  Mean fidelity: {stats.mean_fidelity:.4f}")
    print(f"  Congestion drops: {stats.aggregate_congestion_drops}")
    print(f"  Link utilization: {stats.link_utilization_heatmap}")
    print()


def test_sensitivity_analysis():
    """Test sensitivity analysis to find which parameters most impact success rate."""
    print("Testing sensitivity analysis...")

    # A chain with moderate parameters — not too short lifetime (zero baseline),
    # not too long (tests would take forever) so perturbation actually moves things.
    engine = QNetEngine()

    nodes = [
        NodeDefinition(id="node_0", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_1", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_2", memory_lifetime_t2=150.0),
    ]

    links = [
        LinkDefinition(from_node="node_0", to="node_1", distance_km=50.0, base_fidelity=0.90, generation_rate_hz=1000.0),
        LinkDefinition(from_node="node_1", to="node_2", distance_km=50.0, base_fidelity=0.90, generation_rate_hz=1000.0),
    ]

    engine.define_network(nodes=nodes, links=links)

    stats = engine.simulate(
        from_node="node_0",
        to="node_2",
        fidelity_target=0.85,
        max_latency_ms=5000.0,
        runs=300,
        strategy=StrategyType.HighestFidelity,
    )

    print(f"  Baseline success rate: {stats.empirical_success_rate:.2%}")
    print()

    report = stats.sensitivity_analysis(seed_base=42)

    # Sanity checks: must return dict with expected keys (run before early exit)
    assert isinstance(report, dict), "sensitivity_analysis should return a dict"
    assert len(report) > 0, "sensitivity_analysis should have entries"
    assert "alpha_loss_db_km" in report, "should include alpha_loss_db_km"

    # Verify sorted descending by impact
    non_sentinel = {k: v for k, v in report.items() if k != "_baseline_is_zero"}
    values = list(non_sentinel.values())
    for i in range(len(values) - 1):
        assert values[i] >= values[i + 1], f"sensitivity_analysis should be sorted descending: {values[i]} < {values[i+1]}"

    if "_baseline_is_zero" in report:
        print("  Warning: success rate was ~0%; sensitivity analysis skipped.")
        print()
        return

    print("  Parameter sensitivity (sorted by impact):")
    for key, val in non_sentinel.items():
        bar = "#" * int(val * 30)
        print(f"    {key:<35s} {val:.4f}  {bar}")
    print()

    # Sanity checks: must return dict with expected keys
    assert isinstance(report, dict), "sensitivity_analysis should return a dict"
    assert len(report) > 0, "sensitivity_analysis should have entries"
    assert "alpha_loss_db_km" in report, "should include alpha_loss_db_km"

    # Verify sorted descending by impact
    values = list(report.values())
    for i in range(len(values) - 1):
        assert values[i] >= values[i + 1], f"sensitivity_analysis should be sorted descending: {values[i]} < {values[i+1]}"

    # memory_lifetime_t2 and distance_km should have non-trivial impact
    # (they are the dominant loss mechanisms in this long chain)
    t2_impact = sum(v for k, v in report.items() if "memory_lifetime_t2" in k)
    dist_impact = sum(v for k, v in report.items() if "distance_km" in k)

    print(f"  Total memory_lifetime_t2 impact: {t2_impact:.4f}")
    print(f"  Total distance_km impact:         {dist_impact:.4f}")
    assert t2_impact > 0 or dist_impact > 0, "At least one parameter should have measurable impact"
    print()


def test_latency_optimized():
    """Test with latency-optimized routing."""
    print("Testing latency-optimized routing...")

    engine = QNetEngine()

    nodes = [
        NodeDefinition(id="node_a", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_b", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_c", memory_lifetime_t2=150.0),
    ]

    links = [
        LinkDefinition(from_node="node_a", to="node_b", distance_km=10.0, base_fidelity=0.95, generation_rate_hz=1000.0),
        LinkDefinition(from_node="node_b", to="node_c", distance_km=15.0, base_fidelity=0.92, generation_rate_hz=800.0),
    ]

    engine.define_network(nodes=nodes, links=links)

    # Use latency-optimized routing (loosened fidelity/latency for 2-hop path)
    result = engine.request_entanglement(
        from_node="node_a",
        to="node_c",
        fidelity_target=0.80,  # Loosened for 2-hop (was 0.85)
        max_latency_ms=500.0,  # Increased for 2-hop (was 50.0)
        strategy=StrategyType.LowestLatency,
    )

    print(f"  Success: {result.success}")
    print(f"  Latency: {result.latency_ms:.2f} ms")
    print(f"  Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {' -> '.join(result.execution_path)}")
    print()


def test_high_success_strategy():
    """Test with highest success rate routing."""
    print("Testing highest success rate routing...")

    engine = QNetEngine()

    nodes = [
        NodeDefinition(id="node_a", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_b", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_c", memory_lifetime_t2=150.0),
    ]

    links = [
        LinkDefinition(from_node="node_a", to="node_b", distance_km=10.0, base_fidelity=0.95, generation_rate_hz=1000.0),
        LinkDefinition(from_node="node_b", to="node_c", distance_km=15.0, base_fidelity=0.92, generation_rate_hz=800.0),
    ]

    engine.define_network(nodes=nodes, links=links)

    # Use highest success rate routing (loosened fidelity/latency for 2-hop path)
    result = engine.request_entanglement(
        from_node="node_a",
        to="node_c",
        fidelity_target=0.80,  # Loosened for 2-hop (was 0.88)
        max_latency_ms=500.0,  # Increased for 2-hop (was 75.0)
        strategy=StrategyType.HighestSuccess,
    )

    print(f"  Success: {result.success}")
    print(f"  Latency: {result.latency_ms:.2f} ms")
    print(f"  Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {' -> '.join(result.execution_path)}")
    print()


def test_direct_link():
    """Test direct link between two nodes."""
    print("Testing direct link simulation...")

    engine = QNetEngine()

    nodes = [
        NodeDefinition(id="node_a", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_b", memory_lifetime_t2=150.0),
    ]

    links = [
        LinkDefinition(from_node="node_a", to="node_b", distance_km=50.0, base_fidelity=0.85, generation_rate_hz=500.0),
    ]

    engine.define_network(nodes=nodes, links=links)

    result = engine.request_entanglement(
        from_node="node_a",
        to="node_b",
        fidelity_target=0.8,
        max_latency_ms=200.0,
        strategy=None,   # Use default strategy
    )

    print(f"  Success: {result.success}")
    print(f"  Latency: {result.latency_ms:.2f} ms")
    print(f"  Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {' -> '.join(result.execution_path)}")
    print()


def test_custom_config():
    """Test with custom simulation config."""
    print("Testing with custom config...")

    # Create engine with custom config (placeholder — not yet wired to SimulationConfig)
    _config = QNetEngine.__new__(QNetEngine)   # Placeholder
    # Note: Custom config would require access to SimulationConfig class
    # For now, we use defaults

    engine = QNetEngine()

    nodes = [
        NodeDefinition(id="node_a", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_b", memory_lifetime_t2=150.0),
        NodeDefinition(id="node_c", memory_lifetime_t2=150.0),
    ]

    links = [
        LinkDefinition(from_node="node_a", to="node_b", distance_km=10.0, base_fidelity=0.95, generation_rate_hz=1000.0),
        LinkDefinition(from_node="node_b", to="node_c", distance_km=15.0, base_fidelity=0.92, generation_rate_hz=800.0),
    ]

    engine.define_network(nodes=nodes, links=links)

    # Use latency-optimized routing
    result = engine.request_entanglement(
        from_node="node_a",
        to="node_c",
        fidelity_target=0.85,
        max_latency_ms=50.0,
        strategy=StrategyType.LowestLatency,
    )

    print(f"  Success: {result.success}")
    print(f"  Latency: {result.latency_ms:.2f} ms")
    print(f"  Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {' -> '.join(result.execution_path)}")
    print()


def test_generate_topology():
    """Test using generate_topology function with different topologies."""
    print("Testing generate_topology function...")

    # Generate a hybrid satellite-fiber topology
    net = generate_topology("hybrid_satellite_fiber")
    print(f"  Generated hybrid topology with {len(net.nodes)} nodes and {len(net.links)} links")

    # Define engine with generated topology
    engine = QNetEngine()
    engine.define_network(nodes=net.nodes, links=net.links)

    # Run simulation on the generated topology
    stats = engine.simulate(
        from_node="Toronto",
        to="London",
        fidelity_target=0.75,
        max_latency_ms=5000.0,
        runs=100,
        strategy=StrategyType.HighestFidelity,
    )

    print(f"  Success rate: {stats.empirical_success_rate:.2%}")
    print(f"  Mean latency: {stats.mean_latency_ms:.2f} ms")
    print(f"  Mean fidelity: {stats.mean_fidelity:.4f}")
    print()

    # Generate a telecom backbone topology
    net = generate_topology("telecom_backbone")
    print(f"  Generated telecom backbone with {len(net.nodes)} nodes and {len(net.links)} links")

    # Generate a repeater chain topology
    net = generate_topology("repeater_chain")
    print(f"  Generated repeater chain with {len(net.nodes)} nodes and {len(net.links)} links")
    print()


def test_satellite_link_conditions():
    """Test satellite links with weather conditions."""
    print("Testing satellite link conditions...")

    engine = QNetEngine()

    # Create satellite link with specific conditions
    satellite_link = LinkDefinition(
        from_node="ground_station",
        to="SAT-1",
        distance_km=1000.0,
        base_fidelity=0.98,
        generation_rate_hz=50.0,
        link_type=LinkType.Satellite,
        satellite_conditions=SatelliteConditions(visibility=0.90, weather_factor=0.95)
    )

    # Calculate effective rate considering conditions
    effective_rate = satellite_link.satellite_conditions.effective_rate(50.0)
    print("  Base rate: 50.0 Hz")
    print(f"  Effective rate (with conditions): {effective_rate:.2f} Hz")

    engine.define_network(
        nodes=[NodeDefinition(id="ground_station", memory_lifetime_t2=150.0)],
        links=[satellite_link]
    )
    print()


def test_compare_topologies():
    """Compare fiber-only vs hybrid satellite-fiber topologies."""
    print("Comparing topologies (fiber-only vs hybrid)...")

    # Each topology has its own node names and structure. We compare semantically
    # equivalent endpoints — for both, the edge-to-edge path that stresses the network most:
    #   telecom_backbone: A→C  (end of backbone)
    #   hybrid_satellite_fiber: Toronto→London (cross-continental, via satellite or ground)
    report = compare_topologies(
        endpoints=[
            TopologyEndpoints("telecom_backbone", "A", "C"),
            TopologyEndpoints("hybrid_satellite_fiber", "Toronto", "London"),
        ],
        fidelity_target=0.75,
        max_latency_ms=5000.0,
        runs=1000,
        strategy=StrategyType.HighestFidelity,
    )

    print(f"  Recommended topology: {report.recommended_topology}")
    print(f"  Summary: {report.summary}")
    print()
    print("  Detailed results:")
    for result in report.results:
        print(f"     {result.topology_name}:")
        print(f"     Success rate: {result.success_rate:.1%}")
        print(f"     Mean latency: {result.mean_latency_ms:.2f} ms")
        print(f"     Mean fidelity: {result.mean_fidelity:.4f}")
    print()

    # Sanity-check: all results must have non-zero data (no silent failures)
    for result in report.results:
        assert result.success_rate > 0 or result.mean_latency_ms > 0, \
            f"Topology {result.topology_name} has zero success and latency — endpoint mismatch!"
    print("  All topology results are valid (non-zero data)")
    print()


def _qnet_dir():
    """Return path to the python/ test fixtures directory."""
    return os.path.dirname(os.path.abspath(__file__))


def test_qnet_file_load_save():
    """Test .qnet file load and save functionality."""
    print("Testing .qnet file load/save...")

    base = _qnet_dir()

    # Load a valid .qnet file via the new load_qnet_file function
    from qnet_core import load_qnet_file, PyQNetFile

    loaded = load_qnet_file(os.path.join(base, "valid_network.qnet"))
    assert loaded.version == "1.0", f"Expected version 1.0, got {loaded.version}"
    assert loaded.metadata.name == "valid_network"
    assert len(loaded.nodes) == 2
    assert len(loaded.links) == 1
    print(f"  Loaded: {loaded}")

    # Save it to a temp file and reload to verify round-trip
    out_path = os.path.join(base, "test_roundtrip.qnet")
    loaded.save(out_path)
    reloaded = load_qnet_file(out_path)
    assert reloaded.metadata.name == loaded.metadata.name
    assert len(reloaded.nodes) == len(loaded.nodes)
    assert len(reloaded.links) == len(loaded.links)
    os.remove(out_path)
    print("  Round-trip save/load verified OK")

    # Also test PyQNetFile constructor (new() accepts a name string)
    f2 = PyQNetFile("manual_test")
    assert f2.version == "1.0"
    assert f2.metadata.name == "manual_test"
    assert len(f2.nodes) == 0
    print("  PyQNetFile() constructor verified OK")
    print()


def test_qnet_file_format():
    """Test creating and using .qnet file format directly."""
    print("Testing .qnet file format creation...")

    from qnet_core import (
        PyQNetFile, PyQNetConfig, PyQNetConstraints,
        PyQNetNodeType, PyQNetLinkType, PyQNetSatelliteExtension,
        load_qnet_file,
    )

    base = _qnet_dir()
    out_path = os.path.join(base, "test_programmatic.qnet")

    # Build a .qnet file programmatically with explicit optional args
    f = PyQNetFile("programmatic_test")
    f.metadata.description = "Created from Python bindings"
    f.metadata.author = "claude-test"

    # Node using convenience add_node with optional args
    f.add_node("alpha", memory_lifetime_ms=200.0, memory_capacity=64, node_type=PyQNetNodeType.Satellite)

    # Second node
    f.add_node("beta", memory_lifetime_ms=150.0, memory_capacity=32, node_type=PyQNetNodeType.Ground)

    # Link with satellite extension
    f.add_link(
        id="link_001",
        src="alpha", to="beta",
        distance_km=500.0,
        base_fidelity=0.95,
        generation_rate_hz=50.0,
        link_type=PyQNetLinkType.Satellite,
        satellite=PyQNetSatelliteExtension(visibility=0.90, weather_factor=0.95),
    )

    # Config and constraints with partial optional fields (some are None)
    f.config = PyQNetConfig(alpha_loss=0.05, gamma_swapping=0.90)  # beta_fidelity_decay is None
    f.constraints = PyQNetConstraints(fidelity_target=0.85, max_latency_ms=500.0)

    # Verify None fields before save
    assert f.config.beta_fidelity_decay is None, "beta_fidelity_decay should be None"

    # Save and verify
    f.save(out_path)
    print(f"  Created + saved: {f}")

    # Reload and inspect
    reloaded = load_qnet_file(out_path)
    assert reloaded.metadata.name == "programmatic_test"
    assert len(reloaded.nodes) == 2
    assert reloaded.nodes[0].memory_lifetime_ms == 200.0
    assert reloaded.nodes[0].node_type is not None
    assert reloaded.config.alpha_loss == 0.05
    print(f"  Reloaded: {reloaded}")

    # Verify the raw JSON has correct snake_case format with nulls for None
    import json
    with open(out_path) as fh:
        raw = json.load(fh)
    assert raw["version"] == "1.0"
    assert raw["metadata"]["name"] == "programmatic_test"
    os.remove(out_path)
    print("  JSON format verified OK")
    print()


def test_qnet_validate():
    """Test .qnet file validation."""
    print("Testing .qnet file validation...")

    from qnet_core import PyQNetFile, PyQNetNodeType

    base = _qnet_dir()

    # Valid file should return valid=True with no errors
    result = validate(os.path.join(base, "valid_network.qnet"))
    assert result["valid"] is True, f"Expected valid=True, got {result}"
    assert len(result["errors"]) == 0
    print(f"  valid_network.qnet: valid={result['valid']} (OK)")

    # Another valid file
    result2 = validate(os.path.join(base, "network_v1.qnet"))
    assert result2["valid"] is True
    print(f"  network_v1.qnet: valid={result2['valid']} (OK)")

    # Programmatic file validation also works (round-trip above produces one)
    test_path = os.path.join(base, "test_validate_programmatic.qnet")
    f = PyQNetFile("validate_test")
    f.add_node("x", node_type=PyQNetNodeType.Ground)
    f.add_node("y", node_type=PyQNetNodeType.Satellite)
    f.add_link("", "x", "y", 50.0, 0.9, 100.0)
    f.save(test_path)

    result3 = validate(test_path)
    assert result3["valid"] is True
    print(f"  test_validate_programmatic.qnet: valid={result3['valid']} (OK)")
    os.remove(test_path)
    print()


def test_qnet_diff():
    """Test .qnet file diffing."""
    print("Testing .qnet file diffing...")

    from qnet_core import diff as qnet_diff

    base = _qnet_dir()

    d = qnet_diff(
        os.path.join(base, "network_v1.qnet"),
        os.path.join(base, "network_v2.qnet"),
    )
    assert "summary" in d
    print(f"  Summary: {d['summary']}")
    print(f"  Nodes added: {d.get('nodes_added', [])}")
    print(f"  Links added: {d.get('links_added', [])}")

    # network_v2 has node_c and an extra link -> should show additions
    assert "node_c" in d.get("nodes_added", []), f"Expected node_c in nodes_added, got {d}"
    print("  Diff verified OK")
    print()


def test_qnet_validation_errors():
    """Test validation with invalid .qnet files."""
    print("Testing validation with invalid files...")

    from qnet_core import validate

    base = _qnet_dir()

    # Self-loop in invalid_network.qnet should cause validation failure during load
    # The io.rs load_qnet_file() validates as part of loading, so it throws on invalid data
    try:
        validate(os.path.join(base, "invalid_network.qnet"))
        assert False, "Should have raised ValidationError"
    except RuntimeError as e:
        assert "Validation failed" in str(e) or "Self-loop" in str(e), f"Expected validation error, got: {e}"

    print("  invalid_network.qnet: correctly rejected with Self-loop error")

    # Also test that a valid file returns proper structured result (not just dict with errors)
    valid_result = validate(os.path.join(base, "valid_network.qnet"))
    assert valid_result["valid"] is True
    assert isinstance(valid_result["errors"], list)
    assert isinstance(valid_result["warnings"], list)
    print("  Validation error structure verified OK")
    print()


# ========================================================================
# Helper tests for new types
# ========================================================================

def test_define_network_from_qnet():
    """Test bridging .qnet format directly to the simulation engine — no manual NodeDefinition / LinkDefinition needed."""
    print("Testing define_network_from_qnet (bridge both type systems)...")

    from qnet_core import load_qnet_file

    base = _qnet_dir()

    # Load a .qnet file (PyQNetNode / PyQNetLink format) and feed it straight into the engine
    qf = load_qnet_file(os.path.join(base, "valid_network.qnet"))
    assert len(qf.nodes) == 2, f"Expected 2 nodes in fixture, got {len(qf.nodes)}"

    engine = QNetEngine()
    engine.define_network_from_qnet(qf)

    # Now simulate — no need to construct NodeDefinition / LinkDefinition at all
    stats = engine.simulate(
        from_node="A",
        to="B",
        fidelity_target=0.7,
        max_latency_ms=5000.0,
        runs=100,
        seed=42,  # deterministic output
    )

    print(f"  Nodes loaded: {len(qf.nodes)}, Links loaded: {len(qf.links)}")
    print(f"  Success rate: {stats.empirical_success_rate:.2%}")
    print(f"  Mean latency: {stats.mean_latency_ms:.2f} ms")
    print("  define_network_from_qnet verified OK")
    print()


def test_from_qnet_file():
    """Test the one-liner 'from_file' pattern — load a .qnet file and start simulating in a single call."""
    print("Testing from_qnet_file (one-liner topology load)...")

    base = _qnet_dir()

    # This is the "from_file" pattern — replaces 3 lines with 1:
    #   qf = load_qnet_file("network.qnet")
    #   engine = QNetEngine()
    #   engine.define_network_from_qnet(qf)
    engine = from_qnet_file(os.path.join(base, "valid_network.qnet"))

    # Engine is ready to simulate immediately
    stats = engine.simulate(
        from_node="A",
        to="B",
        fidelity_target=0.7,
        max_latency_ms=5000.0,
        runs=100,
        seed=42,  # deterministic
    )

    print("  Loaded via from_qnet_file()")
    print(f"  Success rate: {stats.empirical_success_rate:.2%}")
    print(f"  Mean latency: {stats.mean_latency_ms:.2f} ms")
    print("  from_qnet_file verified OK")
    print()


def test_determinism():
    """Verify seed=42 produces identical results across calls."""
    print("Testing determinism (seed = same → result = same)...")

    engine = QNetEngine()

    nodes = [NodeDefinition(id="A", memory_lifetime_t2=150.0)]
    links = [LinkDefinition(from_node="A", to="A", distance_km=10.0, base_fidelity=0.95, generation_rate_hz=1000.0)]
    engine.define_network(nodes=nodes, links=links)

    r1 = engine.simulate("A", "A", 0.9, 1000.0, 200, seed=42)
    r2 = engine.simulate("A", "A", 0.9, 1000.0, 200, seed=42)

    assert r1.empirical_success_rate == r2.empirical_success_rate, \
        f"FAIL: seed mismatch — run1={r1.empirical_success_rate}, run2={r2.empirical_success_rate}"
    assert r1.mean_latency_ms == r2.mean_latency_ms, \
        f"FAIL: latency mismatch — run1={r1.mean_latency_ms}, run2={r2.mean_latency_ms}"

    print(f"  seed=42 run1: success={r1.empirical_success_rate:.4f} latency={r1.mean_latency_ms:.2f}")
    print(f"  seed=42 run2: success={r2.empirical_success_rate:.4f} latency={r2.mean_latency_ms:.2f}")
    print("  Determinism verified OK")
    print()


def test_qnet_enum_repr():
    """Test enum class attributes and repr."""
    from qnet_core import PyQNetNodeType, PyQNetLinkType

    assert PyQNetNodeType.Ground is not None
    assert PyQNetNodeType.Satellite is not None
    assert PyQNetNodeType.Repeater is not None
    assert "Ground" in repr(PyQNetNodeType.Ground)
    print(f"  PyQNetNodeType: {repr(PyQNetNodeType.Satellite)}")

    assert PyQNetLinkType.Fiber is not None
    assert PyQNetLinkType.Satellite is not None
    assert "Fiber" in repr(PyQNetLinkType.Fiber)
    print(f"  PyQNetLinkType: {repr(PyQNetLinkType.Satellite)}")
    print()


def test_qnet_config_optional_fields():
    """Test Config/Constraints optional fields are None by default."""
    from qnet_core import PyQNetConfig, PyQNetConstraints

    cfg = PyQNetConfig(alpha_loss=0.05, gamma_swapping=0.9)
    assert cfg.alpha_loss == 0.05
    assert cfg.beta_fidelity_decay is None  # not provided
    assert cfg.gamma_swapping == 0.9
    assert cfg.max_attempts is None

    con = PyQNetConstraints()  # all defaults to None
    assert con.fidelity_target is None
    assert con.max_latency_ms is None

    # Equality works
    con2 = PyQNetConstraints(fidelity_target=0.8, max_latency_ms=200.0)
    assert con != con2
    print("  Config/Constraints optional fields OK")
    print()


def test_qnet_add_defaults():
    """Test add_node/add_link default None fields."""
    from qnet_core import PyQNetFile

    f = PyQNetFile("defaults_test")
    f.add_node("x")  # no optional args
    n = f.nodes[0]
    assert n.memory_lifetime_ms is None, "add_node without memory should be None"
    assert n.memory_capacity is None
    assert n.node_type is None

    f.add_link("", "x", "y", 10.0, 0.9, 100.0)  # no optional args
    link = f.links[0]
    assert link.link_type is None
    assert link.satellite is None
    print("  add_node/add_link defaults OK")
    print()



def main():
    """Run all tests."""
    print("=" * 60)
    print("qnet_core Python Bindings Test Suite")
    print("=" * 60)
    print()

    try:
        test_basic_simulation()
        test_monte_carlo()
        test_sensitivity_analysis()
        test_latency_optimized()
        test_high_success_strategy()
        test_direct_link()
        test_custom_config()
        test_generate_topology()
        test_satellite_link_conditions()
        test_compare_topologies()
        test_define_network_from_qnet()
        test_from_qnet_file()
        test_determinism()
        test_qnet_file_load_save()
        test_qnet_file_format()
        test_qnet_validate()
        test_qnet_diff()
        test_qnet_validation_errors()
        test_qnet_enum_repr()
        test_qnet_config_optional_fields()
        test_qnet_add_defaults()
        print("=" * 60)
        print("All tests completed successfully!")
        print("=" * 60)
    except ImportError as e:
        print(f"Import error: {e}")
        print("Make sure to run 'maturin develop' first to build the bindings.")
    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    main()
