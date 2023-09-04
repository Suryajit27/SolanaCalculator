# Solana Rust-Based Calculator Program

## Overview
This Rust program serves as a Solana smart contract designed to perform arithmetic operations based on user-provided instructions and two numerical values. It offers the flexibility to either add or subtract the provided numbers, making it a versatile tool for various computational tasks.

## Introduction
This Solana smart contract is coded in Rust, a language well-suited for developing smart contracts on the Solana blockchain. It efficiently processes user inputs, enabling the addition or subtraction of numerical values. Additionally, a client interface is available to facilitate testing and interaction with the smart contract.

## Getting Started
To utilize this program, follow the steps below:

### Execution

1. Clone the repository.
2. Build and deploy the program using the provided commands.

### Build

```shell
cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=dist/program
```

### Deploy

```shell
solana program deploy dist/program/helloworld.so
```

3. After completing the above steps, navigate to the 'scripts' directory and execute 'npm install' to install the necessary dependencies.
4. To interact with the contract, use the following commands:

#### Add two numbers

```shell
npm run start -- add <number1> <number2>
```

#### Subtract two numbers

```shell
npm run start -- sub <number1> <number2>
```

These commands enable you to perform addition or subtraction operations with ease, enhancing your computational capabilities on the Solana blockchain.
