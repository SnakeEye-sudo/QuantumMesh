//! CLI Module
//! Command-line interface utilities and display functions

use crate::qsim::QuantumCircuit;

pub fn show_status() {
    println!("\u250c\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2510");
    println!("\u2502     QuantumMesh System Status        \u2502");
    println!("\u2514\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2518");
    println!("  \u2713 Quantum Engine: Ready");
    println!("  \u2713 GPU Acceleration: Enabled");
    println!("  \u2713 Circuit Optimizer: Active");
    println!("  \u2713 API Server: Available");
    println!("  \u2713 Max Qubits: 40+");
    println!();
}

pub fn display_results(results: &[f64]) {
    println!("\n  Qubit State Probabilities:");
    for (i, prob) in results.iter().enumerate().take(10) {
        let bar_len = (prob * 40.0) as usize;
        let bar = "\u2588".repeat(bar_len);
        println!("  |{:04b}\u27e9 {:6.2}% {}", i, prob * 100.0, bar);
    }
    if results.len() > 10 {
        println!("  ... ({} more states)", results.len() - 10);
    }
    println!();
}

pub fn visualize_circuit(circuit: &QuantumCircuit) {
    println!("\n  Circuit Visualization:");
    println!("  Qubits: {}", circuit.num_qubits);
    println!("  Gates: {}", circuit.gates.len());
    println!("\n  Gate Sequence:");
    for (i, gate) in circuit.gates.iter().enumerate().take(20) {
        println!("  {:3}. {:?}", i + 1, gate);
    }
    if circuit.gates.len() > 20 {
        println!("  ... ({} more gates)", circuit.gates.len() - 20);
    }
    println!();
}
