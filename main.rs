//! QuantumMesh - Distributed Quantum Circuit Simulator
//! Main entry point for the quantum simulation engine

use std::env;
use std::process;

mod qsim;
mod gpu_ops;
mod api_server;
mod cli;

/// Main entry point for QuantumMesh
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "simulate" => {
            if args.len() < 3 {
                eprintln!("Error: simulate requires circuit file path");
                process::exit(1);
            }
            simulate_circuit(&args[2]);
        }
        "serve" => {
            let port = if args.len() > 2 {
                args[2].parse::<u16>().unwrap_or(8080)
            } else {
                8080
            };
            api_server::start_server(port);
        }
        "benchmark" => {
            if args.len() < 3 {
                eprintln!("Error: benchmark requires number of qubits");
                process::exit(1);
            }
            let qubits = args[2].parse::<usize>().unwrap_or(10);
            run_benchmark(qubits);
        }
        "visualize" => {
            if args.len() < 3 {
                eprintln!("Error: visualize requires circuit file path");
                process::exit(1);
            }
            visualize_circuit(&args[2]);
        }
        "optimize" => {
            if args.len() < 3 {
                eprintln!("Error: optimize requires circuit file path");
                process::exit(1);
            }
            optimize_circuit(&args[2]);
        }
        "status" => {
            cli::show_status();
        }
        "version" | "--version" | "-v" => {
            println!("QuantumMesh v{}", env!("CARGO_PKG_VERSION"));
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
            process::exit(1);
        }
    }
}

/// Print help information
fn print_help() {
    println!(r#"
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃      QuantumMesh - Quantum Circuit Simulator     ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

Usage: quantummesh <command> [options]

Commands:
  simulate <file>     Simulate quantum circuit from JSON file
  serve [port]        Start REST API server (default: 8080)
  benchmark <qubits>  Run benchmark with N qubits
  visualize <file>    Visualize circuit structure
  optimize <file>     Optimize circuit gates
  status              Show system status
  version             Show version information
  help                Show this help message

Examples:
  quantummesh simulate circuit.json
  quantummesh serve 8080
  quantummesh benchmark 30
  quantummesh visualize circuit.json
  quantummesh optimize circuit.json
"#);
}

/// Simulate a quantum circuit from file
fn simulate_circuit(file_path: &str) {
    println!("┌─ Loading circuit from: {}", file_path);
    
    match qsim::load_circuit(file_path) {
        Ok(circuit) => {
            println!("├─ Circuit loaded: {} qubits, {} gates", 
                     circuit.num_qubits, circuit.gates.len());
            println!("├─ Initializing quantum simulator...");
            
            let mut simulator = qsim::QuantumSimulator::new(circuit.num_qubits);
            
            println!("├─ Applying quantum gates...");
            for (i, gate) in circuit.gates.iter().enumerate() {
                simulator.apply_gate(gate);
                if (i + 1) % 100 == 0 {
                    println!("│  Progress: {}/{} gates", i + 1, circuit.gates.len());
                }
            }
            
            println!("├─ Simulation complete!");
            println!("├─ Measuring quantum state...");
            
            let results = simulator.measure_all();
            println!("└─ Measurement results:");
            
            cli::display_results(&results);
        }
        Err(e) => {
            eprintln!("Error loading circuit: {}", e);
            process::exit(1);
        }
    }
}

/// Run performance benchmark
fn run_benchmark(qubits: usize) {
    println!("┌─ Running benchmark with {} qubits", qubits);
    
    let mut simulator = qsim::QuantumSimulator::new(qubits);
    
    println!("├─ Applying Hadamard gates...");
    let start = std::time::Instant::now();
    
    for i in 0..qubits {
        simulator.apply_hadamard(i);
    }
    
    let hadamard_time = start.elapsed();
    
    println!("├─ Applying CNOT gates...");
    let start = std::time::Instant::now();
    
    for i in 0..qubits-1 {
        simulator.apply_cnot(i, i+1);
    }
    
    let cnot_time = start.elapsed();
    
    println!("├─ Measuring state...");
    let start = std::time::Instant::now();
    
    let _results = simulator.measure_all();
    
    let measure_time = start.elapsed();
    
    println!("└─ Benchmark results:");
    println!("   Hadamard gates: {:?}", hadamard_time);
    println!("   CNOT gates: {:?}", cnot_time);
    println!("   Measurement: {:?}", measure_time);
    println!("   Total time: {:?}", hadamard_time + cnot_time + measure_time);
}

/// Visualize circuit structure
fn visualize_circuit(file_path: &str) {
    match qsim::load_circuit(file_path) {
        Ok(circuit) => {
            cli::visualize_circuit(&circuit);
        }
        Err(e) => {
            eprintln!("Error loading circuit: {}", e);
            process::exit(1);
        }
    }
}

/// Optimize circuit gates
fn optimize_circuit(file_path: &str) {
    match qsim::load_circuit(file_path) {
        Ok(circuit) => {
            println!("Original circuit: {} gates", circuit.gates.len());
            let optimized = qsim::optimize(circuit);
            println!("Optimized circuit: {} gates", optimized.gates.len());
            println!("Reduction: {}%", 
                     ((circuit.gates.len() - optimized.gates.len()) * 100) / circuit.gates.len());
        }
        Err(e) => {
            eprintln!("Error loading circuit: {}", e);
            process::exit(1);
        }
    }
}
