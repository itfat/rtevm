# RTEVM - Rust Ethereum Virtual Machine

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Overview

RTEVM is a simplified Rust implementation of the Ethereum Virtual Machine (EVM). It simulates the execution environment for smart contracts on Ethereum, allowing developers to test and understand EVM behavior.

## Features

- Basic EVM opcodes:
  - Arithmetic: `ADD`, `SUB`, `MUL`, `DIV`, `SDIV`, `MOD`, `SMOD`, `ADDMOD`, `MULMOD`, `EXP`
  - Comparison: `LT`, `GT`, `SLT`, `SGT`, `EQ`, `ISZERO`
  - Bitwise: `AND`, `OR`, `XOR`, `NOT`, `BYTE`, `SHL`, `SHR`, `SAR`
  - Memory and Storage: `MSTORE`, `MLOAD`, `MSTORE8`, `SLOAD`, `SSTORE`
  - Logging: `LOG0`, `LOG1`, `LOG2`, `LOG3`, `LOG4`
  - Other: `PUSH1`, `STOP`, `KECCAK256`
- Stack and memory management
- Logging support
- Gas computation

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rtevm.git

2. Navigate to the directory:
   ```bash
   cd rtevm
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the program with the following command after adding input parameters to the evm program:

```bash
cargo run --release
```

## Contributing

This project welcomes contributions and suggestions. It is a learning experience, and we encourage you to help us make it even better!
For contributing, please follow these guidelines:

- Fork the repository.
- Create a branch (git checkout -b feature-xyz).
- Push your changes to the new branch.
- Open a pull request.

## License

This project is licensed under the MIT license.

