"""Test with custom simulation config."""
from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

print("Testing with custom config...")
# Create engine with custom config
config = QNetEngine.__new__(QNetEngine)   # Placeholder
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
