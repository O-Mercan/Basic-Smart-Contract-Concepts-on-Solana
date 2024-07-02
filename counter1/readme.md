# Counter Solana Smart Contract

This project is a simple counter smart contract written in Rust for the Solana blockchain. The contract allows you to increment, decrement, reset, or update the counter value.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [License](#license)

## Introduction

The Counter Solana Smart Contract is designed to demonstrate a basic understanding of writing and deploying smart contracts on the Solana blockchain using the Rust programming language. It provides basic operations on a counter, which can be incremented, decremented, reset, or updated to a specific value.

## Features

- **Increment**: Increase the counter value by 1.
- **Decrement**: Decrease the counter value by 1.
- **Reset**: Set the counter value to 0.
- **Update**: Update the counter value to a specific number.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust programming language installed. Follow the instructions [here](https://www.rust-lang.org/tools/install) to install Rust.
- Solana CLI installed. Follow the instructions [here](https://docs.solana.com/cli/install-solana-cli-tools) to install Solana CLI.

## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/counter-solana-smart-contract.git
    cd counter-solana-smart-contract
    ```

2. Install dependencies:

    ```sh
    cargo build-bpf
    ```

## Usage

To use this project, follow these steps:

1. Build the project:

    ```sh
    cargo build-bpf
    ```

2. Deploy the program to the Solana blockchain. Ensure you have Solana CLI set up and are connected to the desired network:

    ```sh
    solana program deploy /path/to/your/compiled/program
    ```

3. Interact with the deployed contract using the Solana CLI or your preferred method to send transactions.

## Testing

To run the tests for this project, use the following command:

```sh
cargo test -- --nocapture
