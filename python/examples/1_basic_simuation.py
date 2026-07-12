from qnet_core import QNetEngine, NodeDefinition, LinkDefinition, StrategyType

print("Example 1: Routing Strategy Trade-offs (Latency vs. Fidelity)")

# Initialize the engine
engine = QNetEngine()

# Define a 3-node repeater chain with realistic T2 coherence (150ms)
nodes = [
    NodeDefinition(id="Alice", memory_lifetime_t2=150.0),
    NodeDefinition(id="Repeater_1", memory_lifetime_t2=150.0),
    NodeDefinition(id="Bob", memory_lifetime_t2=150.0),
]

# Define two fiber links
links = [
    LinkDefinition(from_node="Alice", to="Repeater_1", distance_km=25.0, base_fidelity=0.88, generation_rate_hz=1000.0),
    LinkDefinition(from_node="Repeater_1", to="Bob", distance_km=25.0, base_fidelity=0.88, generation_rate_hz=1000.0),
]

engine.define_network(nodes=nodes, links=links)

# --- Cell 1: Optimize for Speed ---
print("--- Strategy: Lowest Latency ---")
fast_result = engine.request_entanglement(
    from_node="Alice",
    to="Bob",
    fidelity_target=0.70, # Relaxed target
    max_latency_ms=50.0,
    strategy=StrategyType.LowestLatency,
)
print(f"  Success: {fast_result.success}")
print(f"  Latency: {fast_result.latency_ms:.2f} ms")
print(f"  Fidelity: {fast_result.final_fidelity:.4f}")
print(f"  Path: {' -> '.join(fast_result.execution_path)}\n")

# --- Cell 2: Optimize for Quality (Triggers Purification) ---
print("--- Strategy: Highest Fidelity ---")
quality_result = engine.request_entanglement(
    from_node="Alice",
    to="Bob",
    fidelity_target=0.85, # Strict target requiring distillation
    max_latency_ms=200.0,
    strategy=StrategyType.HighestFidelity,
)
print(f"  Success: {quality_result.success}")
print(f"  Latency: {quality_result.latency_ms:.2f} ms")
print(f"  Fidelity: {quality_result.final_fidelity:.4f}")
print(f"  Path: {' -> '.join(quality_result.execution_path)}")
