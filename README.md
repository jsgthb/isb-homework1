# Information Security Basics - Homework Symmetric Cryptography

This repository contains an implementation of the four main round transformations of the AES-128 encryption algorithm. The code focuses on performing a single encryption round on a 128-bit plaintext block using a modified round key. This project was completed as part of an assignment. Do not use this code in a production environment.

## Project Overview

The main task was to:
- Implement the four AES round transformations: add round key, byte substitution, shift rows, mix columns
- Apply these transformations in sequence to encrypt a single round of AES-128 on a provided plaintext block.

This round-reduced version of AES-128 performs only a single encryption round using the transformations, rather than the full 10 rounds specified in standard AES.

## Implementation Details

The `Matrix` struct is used as syntactic sugar for matrix operations. The four AES transformations were implemented as follows using the `AES` struct:
- `add_round_key` XORs the current state with the provided round key, incorporating the key into the encryption process.
- `sub_bytes` Substitutes each byte in the state matrix using the AES S-box to provide non-linearity.
- `shift_rows` Shifts the rows of the state matrix by different offsets to achieve inter-column diffusion.
- `mix_columns` Mixes the data within each column using Galois Field multiplication to ensure diffusion.

These transformations are then applied in sequence within the `round_trans` function.