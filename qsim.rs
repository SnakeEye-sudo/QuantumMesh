//! Quantum Simulation Module
//! Core quantum circuit simulation logic

use std::fs;
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::gpu_ops::{GpuStateVector, Complex, RotationAxis};

/// Quantum circuit definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub gates: Vec<QuantumGate>,
}

/// Quantum gate types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum QuantumGate {
    Hadamard { qubit: usize },
    PauliX { qubit: usize },
    PauliY { qubit: usize },
    PauliZ { qubit: usize },
    Phase { qubit: usize, angle: f64 },
    CNOT { control: usize, target: usize },
    SWAP { qubit1: usize, qubit2: usize },
    Toffoli { control1: usize, control2: usize, target: usize },
    RotationX { qubit: usize, angle: f64 },
    RotationY { qubit: usize, angle: f64 },
    RotationZ { qubit: usize, angle: f64 },
    Measurement { qubit: usize },
}

/// Quantum simulator state
pub struct QuantumSimulator {
    pub num_qubits: usize,
    state: GpuStateVector,
}

impl QuantumSimulator {
    /// Create a new quantum simulator
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            state: GpuStateVector::new(num_qubits),
        }
    }

    /// Apply a quantum gate
    pub fn apply_gate(&mut self, gate: &QuantumGate) {
        match gate {
            QuantumGate::Hadamard { qubit } => self.apply_hadamard(*qubit),
            QuantumGate::PauliX { qubit } => self.apply_x(*qubit),
            QuantumGate::PauliY { qubit } => self.apply_y(*qubit),
            QuantumGate::PauliZ { qubit } => self.apply_z(*qubit),
            QuantumGate::Phase { qubit, angle } => self.apply_phase(*qubit, *angle),
            QuantumGate::CNOT { control, target } => self.apply_cnot(*control, *target),
            QuantumGate::SWAP { qubit1, qubit2 } => self.apply_swap(*qubit1, *qubit2),
            QuantumGate::Toffoli { control1, control2, target } => {
                self.apply_toffoli(*control1, *control2, *target)
            }
            QuantumGate::RotationX { qubit, angle } => self.apply_rx(*qubit, *angle),
            QuantumGate::RotationY { qubit, angle } => self.apply_ry(*qubit, *angle),
            QuantumGate::RotationZ { qubit, angle } => self.apply_rz(*qubit, *angle),
            QuantumGate::Measurement { qubit } => {
                // Measurement is handled separately
            }
        }
    }

    /// Apply Hadamard gate
    pub fn apply_hadamard(&mut self, qubit: usize) {
        self.state.apply_hadamard_gpu(qubit);
    }

    /// Apply Pauli-X gate
    pub fn apply_x(&mut self, qubit: usize) {
        self.state.apply_x_gpu(qubit);
    }

    /// Apply Pauli-Y gate
    pub fn apply_y(&mut self, qubit: usize) {
        self.state.apply_y_gpu(qubit);
    }

    /// Apply Pauli-Z gate
    pub fn apply_z(&mut self, qubit: usize) {
        self.state.apply_z_gpu(qubit);
    }

    /// Apply Phase gate
    pub fn apply_phase(&mut self, qubit: usize, angle: f64) {
        self.state.apply_phase_gpu(qubit, angle);
    }

    /// Apply CNOT gate
    pub fn apply_cnot(&mut self, control: usize, target: usize) {
        self.state.apply_cnot_gpu(control, target);
    }

    /// Apply SWAP gate
    pub fn apply_swap(&mut self, qubit1: usize, qubit2: usize) {
        self.apply_cnot(qubit1, qubit2);
        self.apply_cnot(qubit2, qubit1);
        self.apply_cnot(qubit1, qubit2);
    }

    /// Apply Toffoli (CCNOT) gate
    pub fn apply_toffoli(&mut self, control1: usize, control2: usize, target: usize) {
        // Simplified Toffoli implementation
        // In practice, this would be decomposed into more basic gates
        self.apply_hadamard(target);
        self.apply_cnot(control2, target);
        self.apply_rz(target, -std::f64::consts::PI / 4.0);
        self.apply_cnot(control1, target);
        self.apply_rz(target, std::f64::consts::PI / 4.0);
        self.apply_cnot(control2, target);
        self.apply_rz(target, -std::f64::consts::PI / 4.0);
        self.apply_cnot(control1, target);
        self.apply_rz(target, std::f64::consts::PI / 4.0);
        self.apply_hadamard(target);
    }

    /// Apply RX rotation
    pub fn apply_rx(&mut self, qubit: usize, angle: f64) {
        self.state.apply_rotation_gpu(qubit, RotationAxis::X, angle);
    }

    /// Apply RY rotation
    pub fn apply_ry(&mut self, qubit: usize, angle: f64) {
        self.state.apply_rotation_gpu(qubit, RotationAxis::Y, angle);
    }

    /// Apply RZ rotation
    pub fn apply_rz(&mut self, qubit: usize, angle: f64) {
        self.state.apply_rotation_gpu(qubit, RotationAxis::Z, angle);
    }

    /// Measure all qubits
    pub fn measure_all(&self) -> Vec<f64> {
        self.state.measure_all_gpu()
    }

    /// Measure single qubit
    pub fn measure_qubit(&self, qubit: usize) -> f64 {
        let probabilities = self.measure_all();
        let mut prob = 0.0;
        let mask = 1 << qubit;
        for (i, p) in probabilities.iter().enumerate() {
            if i & mask != 0 {
                prob += p;
            }
        }
        prob
    }

    /// Get quantum state vector
    pub fn get_state(&self) -> &[Complex] {
        self.state.get_data()
    }
}

/// Load quantum circuit from JSON file
pub fn load_circuit(path: &str) -> Result<QuantumCircuit, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let circuit: QuantumCircuit = serde_json::from_str(&contents)?;
    Ok(circuit)
}

/// Save quantum circuit to JSON file
pub fn save_circuit(circuit: &QuantumCircuit, path: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(circuit)?;
    fs::write(path, json)?;
    Ok(())
}

/// Optimize quantum circuit by removing redundant gates
pub fn optimize(circuit: QuantumCircuit) -> QuantumCircuit {
    let mut optimized_gates = Vec::new();
    let mut skip_next = false;

    for (i, gate) in circuit.gates.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        // Remove consecutive Hadamard gates on same qubit
        if let QuantumGate::Hadamard { qubit } = gate {
            if i + 1 < circuit.gates.len() {
                if let QuantumGate::Hadamard { qubit: next_qubit } = &circuit.gates[i + 1] {
                    if qubit == next_qubit {
                        skip_next = true;
                        continue;
                    }
                }
            }
        }

        // Remove consecutive Pauli-X gates on same qubit
        if let QuantumGate::PauliX { qubit } = gate {
            if i + 1 < circuit.gates.len() {
                if let QuantumGate::PauliX { qubit: next_qubit } = &circuit.gates[i + 1] {
                    if qubit == next_qubit {
                        skip_next = true;
                        continue;
                    }
                }
            }
        }

        optimized_gates.push(gate.clone());
    }

    QuantumCircuit {
        num_qubits: circuit.num_qubits,
        gates: optimized_gates,
    }
}

/// Create Bell state circuit
pub fn create_bell_state() -> QuantumCircuit {
    QuantumCircuit {
        num_qubits: 2,
        gates: vec![
            QuantumGate::Hadamard { qubit: 0 },
            QuantumGate::CNOT { control: 0, target: 1 },
        ],
    }
}

/// Create GHZ state circuit
pub fn create_ghz_state(num_qubits: usize) -> QuantumCircuit {
    let mut gates = vec![QuantumGate::Hadamard { qubit: 0 }];
    for i in 1..num_qubits {
        gates.push(QuantumGate::CNOT { control: 0, target: i });
    }
    QuantumCircuit { num_qubits, gates }
}

/// Create quantum Fourier transform circuit
pub fn create_qft_circuit(num_qubits: usize) -> QuantumCircuit {
    let mut gates = Vec::new();
    for j in (0..num_qubits).rev() {
        gates.push(QuantumGate::Hadamard { qubit: j });
        for k in (0..j).rev() {
            let angle = std::f64::consts::PI / 2.0_f64.powi((j - k) as i32);
            gates.push(QuantumGate::Phase { qubit: j, angle });
        }
    }
    QuantumCircuit { num_qubits, gates }
}
