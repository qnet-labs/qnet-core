from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

print("Testing Physical Constraint Boundaries (Decoherence & Latency)...")

engine = QNetEngine()

# Scenario: Transatlantic link with poor memory hardware
nodes = [
    NodeDefinition(id="NewYork", memory_lifetime_t2=2.0), # Very short memory (2ms)
    NodeDefinition(id="MidAtlantic", memory_lifetime_t2=2.0),
    NodeDefinition(id="London", memory_lifetime_t2=2.0),
]

links = [
    LinkDefinition(from_node="NewYork", to="MidAtlantic", distance_km=2500.0, base_fidelity=0.90, generation_rate_hz=500.0),
    LinkDefinition(from_node="MidAtlantic", to="London", distance_km=3000.0, base_fidelity=0.90, generation_rate_hz=500.0),
]

engine.define_network(nodes=nodes, links=links)

# Attempt to entangle across the ocean with a 10ms hard timeout
print("Attempting impossible entanglement request...")
result = engine.request_entanglement(
    from_node="NewYork",
    to="London",
    fidelity_target=0.80,
    max_latency_ms=10.0, # Physically impossible (speed of light takes > 25ms one-way)
    strategy=StrategyType.HighestSuccess,
)

if not result.success:
    print("  Status: Request Rejected (Expected Behavior)")
    print("  Reason: Route failed to satisfy physical latency or coherence constraints.")
    print(f"  Returned Latency: {result.latency_ms:.2f} ms")
    print(f"  Returned Fidelity: {result.final_fidelity:.4f}")
    print(f"  Path: {result.execution_path}")
else:
    print("  Status: Success (Warning: Physics constraints breached!)")
