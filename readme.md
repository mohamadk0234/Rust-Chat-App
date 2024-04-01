# Project: Simple Chat Application in Rust

## Objective
Develop a basic chat application that enables multiple clients to connect to a server and exchange messages in real-time. This project focuses on networking, concurrency, and handling multiple client connections simultaneously.

## Key Concepts to Learn
- **Networking with Rust**: Understanding how to create TCP connections using Rust's standard library.
- **Concurrency**: Leveraging Rust's concurrency model to handle multiple client connections. This includes understanding threads, the tokio crate for asynchronous operations, or any other async runtime of your choice.
- **Serialization and Deserialization**: Using crates like serde to serialize and deserialize data structures for communication over the network.
- **Error Handling**: Learning Rust's approach to handling errors in a robust and safe manner.

## Project Breakdown

### Setup Rust Development Environment
1. Ensure you have Rust and Cargo installed.
2. Familiarize yourself with Cargo's project management.

### Create the Server
1. Initialize a new Cargo project for the server.
2. Implement TCP listener to accept client connections.
3. Use threads or async/await to handle multiple connections concurrently.
4. Design a simple protocol for your chat (e.g., JSON-based messages).

### Develop the Client
1. Create another Cargo project for the client application.
2. Implement functionality to connect to the server using TCP.
3. Design the client to send and receive messages.
4. Create a simple user interface in the terminal or consider a GUI with a crate like druid or iced.

### Implement Chat Features
1. Allow clients to set a username upon connection.
2. Broadcast messages from one client to all connected clients.
