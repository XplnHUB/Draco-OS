mod core;
mod brain;
mod system;
mod memory;
mod execution;
mod ipc;

use tokio::sync::mpsc;
use core::engine::Engine;
use ipc::server::start_server;

#[tokio::main]
async fn main() {
    println!("Starting Draco Core...");

    let (tx, rx) = mpsc::channel(100);

    let engine = Engine { rx };

    // Engine thread
    tokio::spawn(async move {
        engine.run().await;
    });

    // IPC server (main thread)
    println!("IPC Server starting on 127.0.0.1:8080");
    start_server(tx).await;
}
