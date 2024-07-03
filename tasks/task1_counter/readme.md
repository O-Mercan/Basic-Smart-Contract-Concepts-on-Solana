# Counter Solana Smart Contract

This project is a simple counter smart contract written in Rust for the Solana blockchain. The contract allows you to increment, decrement, reset, or update the counter value.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Homework Task](#homework-task)
- [Testing](#testing)
- [License](#license)

## Introduction

The Counter Solana Smart Contract is designed to demonstrate a basic understanding of writing and deploying smart contracts on the Solana blockchain using the Rust programming language. It provides basic operations on a counter, which can be incremented, decremented, reset, or updated to a specific value.

## Features

- **Increment**: Increase the counter value.
- **Decrement**: Decrease the counter value.
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

## Homework Task

In this homework, you will modify the functionality of increment and decrement functions. Right now, they are incrementing/decrementing the state by 1. Let’s make them get input from the user and increment/decrement the state accordingly.

### Step 1

Add a `u32` argument to both `Increment` and `Decrement` in the `CounterInstructions` enum. They should get a value as a parameter, just like the `Update` function.

### Step 2

In the `unpack` function, unpack the `u32` value for both `Increment` and `Decrement`, just like what we are doing in the `Update` function.

### Step 3

In `lib.rs`, modify the following lines, so that instead of incrementing/decrementing by 1, it should increment/decrement by the `u32` parameter value.

#### Original Code

```rust
CounterInstructions::Increment => {
  counter_account.counter += 1;
}
CounterInstructions::Decrement => {
  counter_account.counter -= 1;
}
