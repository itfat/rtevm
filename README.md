# RTEVM - Rust Ethereum Virtual Machine

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project)
- [Modules](#modules)
- [Contributing](#contributing)
- [License](#license)

## Overview
The Ethereum Virtual Machine (EVM) is the runtime environment for smart contracts in Ethereum. It is responsible for executing bytecode and managing the state of the blockchain. This project implements a basic version of the EVM using Rust RTEVM, demonstrating the core functionalities and architecture of the EVM.

## Features

- Basic EVM opcodes:
  - Arithmetic: `ADD`, `SUB`, `MUL`, `DIV`, `SDIV`, `MOD`, `SMOD`, `ADDMOD`, `MULMOD`, `EXP`
  - Comparison: `LT`, `GT`, `SLT`, `SGT`, `EQ`, `ISZERO`
  - Bitwise: `AND`, `OR`, `XOR`, `NOT`, `BYTE`, `SHL`, `SHR`, `SAR`
  - Memory and Storage: `MSTORE`, `MLOAD`, `MSTORE8`, `SLOAD`, `SSTORE`
  - Logging: `LOG0`, `LOG1`, `LOG2`, `LOG3`, `LOG4`
  - Duplicate: `DUP1`, `DUP2`, `DUP3`, `DUP4`, `DUP5`, `DUP6`, `DUP7`, `DUP8`, `DUP9`, `DUP10`, `DUP11`, `DUP12`, `DUP13`, `DUP14`, `DUP15`, `DUP16`
  - Swap: `SWAP1`, `SWAP2`, `SWAP3`, `SWAP4`, `SWAP5`, `SWAP6`, `SWAP7`, `SWAP8`, `SWAP9`, `SWAP10`, `SWAP11`, `SWAP12`, `SWAP13`, `SWAP14`, `SWAP15`, `SWAP16`
  - Push: `PUSH1`, `PUSH2`, `PUSH3`, `PUSH4`, `PUSH5`, `PUSH6`, `PUSH7`, `PUSH8`, 
  `PUSH9`, `PUSH10`, `PUSH11`, `PUSH12`, `PUSH13`, `PUSH14`, `PUSH15`, `PUSH16`,
  `PUSH17`, `PUSH18`, `PUSH19`, `PUSH20`, `PUSH21`, `PUSH22`, `PUSH23`, `PUSH24`,
  `PUSH25`, `PUSH26`, `PUSH27`, `PUSH28`, `PUSH29`, `PUSH30`, `PUSH31`, `PUSH32`
  - Jump: `JUMP`, `JUMPI`, `PC`, `JUMPDEST`
  - Other: `POP`, `STOP`, `KECCAK256`
- Stack and memory management
- Logging support
- Gas computation - static and dynamic
   Computed dynamic gas cost based on memory expansion cost, word size, and topic count in the transaction. The implementation of dynamic gas calculation is in the helper module.

## Installation


1. Clone the repository:

    ```sh
    git clone https://github.com/itfat/rtevm.git
    cd rtevm
    ```

2. Build the project:

    ```sh
    cargo build
    ```

3. Run the project:

    ```sh
    cargo run
    ```

## Usage

The main entry point for the project is the `main.rs` file. It initializes an instance of the EVM and executes a sample bytecode program.

```rust
fn main() {
    println!("--------EVM--------");

    let program = vec![
        0x60, 0x01, // PUSH1 0x01
        0x60, 0x42, // PUSH1 0x42
        0x5D,       // TSTORE
        0x60, 0x01, // PUSH1 0x01
        0x5C,    // TLOAD
        0x00        // STOP
    ];

    let call_data = vec![];
    let sender = H160::zero();  
    let gas = 5000;  
    let value = 0;   

    let mut evm = EVM::new(sender, gas, value, program, call_data);
    println!("Initial EVM state: {:#?}", evm);

    evm.run();

    println!("Final EVM state: {:#?}", evm);
}
```

## Project Structure
The project is structured into several modules, each handling different aspects of the EVM:

```
rtevm/
├── Cargo.toml
├── evm/
│   ├── opcode_instructions.rs
├── src/
│   ├── main.rs
│   ├── evm.rs
│   ├── memory.rs
│   ├── stack.rs
│   ├── storage.rs
│   ├── transient.rs
│   ├── opcodes.rs
│   ├── helper.rs


```

## Modules
- `main.rs:` Entry point of the application, initializes and runs the EVM.
- `evm.rs:` Core EVM logic, including opcode fetching, execution, and state management.
- `opcode_instructions.rs:` Defines opcode instructions and their execution logic.
- `memory.rs:` Manages memory operations.
- `stack.rs:` Implements stack operations.
- `storage.rs:` Handles persistent storage.
- `transient.rs:` Manages transient (temporary) storage as in EIP-1153.
- `opcodes.rs:` Defines supported opcodes and their execution logic.
- `helper.rs:` Contains helper functions and utilities.

## Contributing

This project welcomes contributions and suggestions. It is a learning experience, and we encourage you to help us make it even better!
For contributing, please follow these guidelines:

- Fork the repository.
- Create a branch (git checkout -b feature-xyz).
- Push your changes to the new branch.
- Open a pull request.

## License

This project is licensed under the MIT license.

