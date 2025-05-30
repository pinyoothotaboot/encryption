# Rust Encryption Library

A secure and efficient Rust library for asymmetric encryption, providing robust tools for generating and managing public/private key pairs. This library implements industry-standard encryption algorithms to ensure secure data transmission and storage.

## Features
- **RSA Encryption**
  - Generate secure public/private key pairs
  - Encrypt and decrypt messages using RSA
  - Support for different key sizes (2048, 3072, 4096 bits)
  - PKCS#1 v1.5 padding scheme

## Getting Started

## Features
The rustify supported generate encryption
- RSA Encryption (Public/Private Key)

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
```sh
sudo cargo run
```

## Development

### Running Locally
To run the project locally:
```sh
sudo cargo test
```

### Building the Project
To build the project in release mode:
```sh
sudo cargo build --release
```

## Roadmap
Planned features and improvements:
- One-time public key encryption
- Support for additional encryption algorithms (Ed25519, ECDSA)
- Key management utilities
- WASM compatibility for browser usage

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## Author
[Pinyoo Thotaboot](https://github.com/pinyoothotaboot)

## License
[MIT](./LICENSE)