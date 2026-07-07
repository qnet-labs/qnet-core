To build the Python SDK after making changes to the Rust files, you need to run:

  maturin develop

  This command will:
  1. Rebuild the Rust library with your changes
  2. Build the Python bindings
  3. Install the updated package into your Python virtual environment
  
  Typical workflow:

  1. Make your changes to the Rust source files (e.g., src/engine.rs, src/routing/strategy.rs, etc.)
  2. Build the bindings:
  maturin develop
  3. Test your changes in Python:
  python python/test_example.py

  Alternative options:

  - Build wheel only (for distribution):
  maturin build
  - Build and upload to PyPI:
  maturin publish
  - Force a rebuild (if you're having issues):
  cargo clean && maturin develop

  The maturin develop command uses your project's pyproject.toml configuration to determine how to build and install the package. It will
  automatically detect changes to your Rust code and rebuild accordingly.

To run a Python example with the qnet_core library, you need to:

  1. Build the Python bindings using maturin:
  maturin develop
  2. Create your Python script (e.g., test1.py) and use the library:

  Read 1 file (ctrl+o to expand)

⏺ Here's the existing test example. To create your own test1.py:

  Step 1: Build the bindings
  cd /path/to/qnet-core
  maturin develop
  
  Step 2: Create your test script (test1.py)

  from qnet_core import QNetEngine, StrategyType, NodeDefinition, LinkDefinition

  # Create engine
  engine = QNetEngine()

  # Define network
  nodes = [
      NodeDefinition(id="node_a", memory_lifetime_t2=1.0),
      NodeDefinition(id="node_b", memory_lifetime_t2=1.0),
      NodeDefinition(id="node_c", memory_lifetime_t2=1.0),
  ]

  links = [
      LinkDefinition(from_node="node_a", to="node_b", distance_km=10.0, base_fidelity=0.95, generation_rate_hz=1000.0),
      LinkDefinition(from_node="node_b", to="node_c", distance_km=15.0, base_fidelity=0.92, generation_rate_hz=800.0),
  
  engine.define_network(nodes=nodes, links=links)
  
  # Run single simulation
  result = engine.request_entanglement(
      from_node="node_a",
      to="node_c",
      fidelity_target=0.9,
      max_latency_ms=100.0,
      strategy=StrategyType.HighestFidelity,
  )   
  
  print(f"Success: {result.success}")
  print(f"Latency: {result.latency_ms:.2f} ms")
  print(f"Fidelity: {result.final_fidelity:.4f}")
  print(f"Path: {' -> '.join(result.execution_path)}")
  
  Step 3: Run your script
  python test1.py
  
  The key points:
  - maturin develop builds the Rust library and installs it as a Python package in your current virtual environment
  - After that, import qnet_core works like any other Python package
  - Available types: QNetEngine, StrategyType (enum with LowestLatency, HighestFidelity, HighestSuccess), NodeDefinition, LinkDefinition
  - Available methods on QNetEngine: define_network(), request_entanglement(), simulate() for Monte Carlo ensembles