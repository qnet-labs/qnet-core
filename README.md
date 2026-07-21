# qnet-core
### Design the quantum internet before it exists

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.8+-blue.svg)](https://www.python.org/)
[![Release](https://img.shields.io/github/v/release/qnet-labs/qnet-core)](https://github.com/qnet-labs/qnet-core/releases)

Quantum network entanglement distribution simulator. Models quantum repeater networks, link generation protocols, fidelity purification (BBPSSW), and routing strategies for quantum communication — with both Rust library and Python bindings.

## Features

- **Event-driven simulation** — timeline coordination with binary heap scheduler
- **Entanglement purification** — BBPSSW distillation protocol
- **Three routing strategies** — lowest latency, highest fidelity, highest success rate
- **Monte Carlo ensembles** — statistical analysis across thousands of runs
- **Pre-built topologies** — telecom backbone, repeater chain, hybrid satellite-fiber
- **Topology comparison & diffing** — compare and version your network designs
- **.qnet file format** — load/save/diff network topologies
- **Python bindings** — full API via PyO3

## Installation

### Python (from source)

```bash
# Install build dependencies
pip install qnet-core
```
## Quick Start

```python
from qnet_core import QNetEngine, generate_topology

topology = generate_topology("hybrid_satellite_fiber")

engine = QNetEngine()
engine.load_topology(topology)

stats = engine.simulate("Toronto", "London", fidelity_target=0.9, max_latency_ms=200, runs=100)

print(stats.empirical_success_rate)
```

## Documentation
- [Documnets main page](https://qnet-labs.github.io/qnet-examples/intro)
- [Python API Reference Detailed](https://qnet-labs.github.io/qnet-examples/category/api-reference)
- [Python API Reference](python/README.md) — complete type signatures and function docs
- [Changelog](CHANGELOG.md) — version history

## Tutorials
[qnet-core tutorials](https://qnet-labs.github.io/qnet-examples/category/tutorials)

## Examples
[Examples](https://github.com/qnet-labs/qnet-examples)

[![Open in Colab](https://colab.research.google.com/assets/colab-badge.svg)]
(https://colab.research.google.com/github/qnet-labs/qnet-core/blob/main/notebooks/01_gothamq_validation.ipynb)


## Roadmap
- [x] Integration hooks for higher-level protocols (QKD, teleportation, distributed computing)
- [ ] Expanded noise/decoherence modeling
- [ ] Additional purification protocols
- [ ] Parallel simulation support (also planned for the hosted simulation API)

# Build and install in development mode
maturin develop --features python
```

### Rust

```bash
cargo add qnet-core
```

## Building
```bash
# Rust only
cargo build

# Python bindings
maturin develop --features python

# Format & lint
cargo fmt && cargo clippy

# Run tests
cargo test
cd python && python test_example.py
```

## Architecture

```
src/
├── api/          # Public API boundary (request/response types)
├── routing/      # Pathfinding + strategy selection
├── protocols.rs  # BBPSSW purification
├── scheduler.rs  # Timeline orchestration
├── simulation.rs # Event-driven runtime
├── network.rs    # Quantum graph model
├── memory.rs     # Qubit register tracking
├── metrics.rs    # Telemetry
└── swapping.rs   # Bell-state transformations
python/           # Python bindings + examples
```

## License

MIT — see [LICENSE](LICENSE).

## Contributing

Contributions welcome! Please open an issue or submit a pull request.

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit changes (`git commit -am 'Add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request

---

Built with Rust + PyO3. Powered by quantum simulation.
