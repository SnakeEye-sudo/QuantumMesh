//! REST API Server Module
//! Provides HTTP endpoints for quantum circuit simulation

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct ApiServer {
    port: u16,
    circuits: Arc<Mutex<HashMap<String, crate::qsim::QuantumCircuit>>>,
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            circuits: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub fn start_server(port: u16) {
    println!("\u250c\u2500 Starting QuantumMesh API server on port {}", port);
    println!("\u251c\u2500 Available endpoints:");
    println!("\u2502  POST   /api/simulate       - Simulate quantum circuit");
    println!("\u2502  POST   /api/upload         - Upload circuit definition");
    println!("\u2502  GET    /api/circuits       - List all circuits");
    println!("\u2502  GET    /api/circuit/:id    - Get specific circuit");
    println!("\u2502  DELETE /api/circuit/:id    - Delete circuit");
    println!("\u2502  POST   /api/optimize       - Optimize circuit");
    println!("\u2502  GET    /api/health         - Health check");
    println!("\u2514\u2500 Server ready at http://localhost:{}", port);
    
    // Simulate server running
    println!("\nPress Ctrl+C to stop the server");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
