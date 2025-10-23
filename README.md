# ⚛️ QuantumMesh

A distributed quantum circuit simulator in Rust with GPU acceleration, distributed execution, and a clean CLI/API for building and testing circuits up to 40+ qubits.

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Active-success.svg)]()

---

## Table of Contents
- Overview
- Features
- Architecture
- Quick Start
- Usage
  - CLI
  - REST API
- Configuration
- Examples
- Development

---

## Overview
QuantumMesh simulates quantum circuits using a dense state vector model with optional GPU acceleration. It supports essential gates (H, X, Y, Z, CNOT, SWAP, Toffoli, rotations, phase), circuit optimization, JSON-based circuit I/O, and an HTTP API for remote control.

---

## Features
- GPU-accelerated state vector operations (simulated interface; swappable backend)
- Distributed-ready design for multi-node orchestration
- Circuit optimizer (remove redundant gate pairs, fusion-ready)
- JSON circuit I/O (load/save)
- CLI and REST API for automation
- Benchmarking utilities and sample circuits (Bell, GHZ, QFT)

---

## Architecture
```
QuantumMesh/
├── main.rs        # entry point and command router
├── qsim.rs        # quantum logic and circuit ops
├── gpu_ops.rs     # GPU-accelerated state vector ops
├── api_server.rs  # REST API (WIP lightweight mock handler)
├── cli.rs         # CLI utilities (status, visualization)
├── config.toml    # simulator configuration
└── testdata.json  # sample circuit
```
High-level flow:
- CLI/API -> main.rs routes to qsim.rs operations
- qsim.rs uses gpu_ops.rs for accelerated kernels
- api_server.rs exposes health/simulate/optimize endpoints (mock loop in this minimal repo)

---

## Quick Start
Prerequisites: Rust 1.75+, cargo

Clone:
```bash
git clone https://github.com/SnakeEye-sudo/QuantumMesh
cd QuantumMesh
```
Build:
```bash
cargo build
```
Run a simulation (using sample testdata.json):
```bash
cargo run -- simulate testdata.json
```
Run API server:
```bash
cargo run -- serve 8080
```
Benchmark 20 qubits:
```bash
cargo run -- benchmark 20
```

---

## Usage
### CLI
- Simulate: `quantummesh simulate <circuit.json>`
- Serve API: `quantummesh serve [port]`
- Benchmark: `quantummesh benchmark <qubits>`
- Visualize circuit: `quantummesh visualize <circuit.json>`
- Optimize circuit: `quantummesh optimize <circuit.json>`
- Status: `quantummesh status`

Example:
```bash
quantummesh simulate testdata.json
```
Output (truncated):
```
┌─ Loading circuit from: testdata.json
├─ Circuit loaded: 3 qubits, 6 gates
├─ Initializing quantum simulator...
├─ Applying quantum gates...
├─ Simulation complete!
├─ Measuring quantum state...
└─ Measurement results:
  |000⟩  12.50% █████
  |001⟩   0.00% 
  ...
```

### REST API
Start server:
```bash
quantummesh serve 8080
```
Health check:
```bash
curl http://localhost:8080/api/health
```
Simulate (mock):
```bash
curl -X POST http://localhost:8080/api/simulate \
  -H 'Content-Type: application/json' \
  -d @testdata.json
```
Note: The bundled api_server.rs prints endpoints and runs a loop to simulate a server. Swap in a real framework (Axum/Actix) for production.

---

## Configuration
Edit config.toml:
```toml
[simulation]
max_qubits = 40
default_backend = "gpu"
fault_tolerance = true
precision = "double"

[gpu]
enabled = true
max_memory_mb = 8192
thread_block_size = 256

[optimization]
circuit_optimization = true
remove_redundant_gates = true
optimization_level = 2

[api]
port = 8080
host = "0.0.0.0"
```

---

## Examples
Create Bell state (programmatic):
```rust
use quantummesh::qsim::{create_bell_state, QuantumSimulator};
let circuit = create_bell_state();
let sim = QuantumSimulator::new(circuit.num_qubits);
```
Create GHZ(5):
```rust
use quantummesh::qsim::create_ghz_state;
let circuit = create_ghz_state(5);
```
Quantum Fourier Transform(4):
```rust
use quantummesh::qsim::create_qft_circuit;
let circuit = create_qft_circuit(4);
```

---

## Development
- Format: `cargo fmt`
- Lint: `cargo clippy`
- Build: `cargo build`
- Test: `cargo test` (add tests as needed)

Roadmap:
- Replace mock API loop with Axum/Actix implementation
- Add distributed executor and sharding
- Add custom GPU kernels via CUDA/OpenCL

License: MIT
