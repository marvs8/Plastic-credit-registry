# Contributing to Plastic Credit Registry

This repository is designed for rapid Soroban smart contract development while preserving the MRV and tokenization model described in `README.md`.

## Getting Started

1. Fork the repository.
2. Clone your fork locally.
3. Install Rust and the `wasm32-unknown-unknown` target.
4. Install the Soroban CLI.

```bash
git clone https://github.com/your-org/plastic-credit-registry.git
cd plastic-credit-registry
rustup target add wasm32-unknown-unknown
cargo test --workspace
```

## Repository Layout

- `contracts/issuance` — Issuance contract package
- `contracts/registry` — Registry contract package
- `contracts/retirement` — Retirement contract package
- `docs/` — Design documents and contributor guidance
- `ISSUES.md` — Planned tasks for the next development phases

## Workflow

- Create a feature branch for each change.
- Keep changes focused; align every contract update with the README's protocol scope.
- Add tests for contract behavior and data-model invariants.
- Run `cargo fmt` and `cargo clippy` before opening a pull request.

## Coding Standards

- Use explicit type names for credits, batches, and attestations.
- Keep contract logic decoupled: issuance should never be responsible for retirement semantics.
- Document the intended behavior of each contract in `docs/architecture.md`.
- Avoid adding frontend or off-chain features that are not required for the core protocol.

## Pull Request Checklist

- [ ] The implementation follows the registry, issuance, or retirement contract boundaries.
- [ ] Unit tests cover the main configuration or validation logic.
- [ ] The README still matches the repository layout.
- [ ] Design documents are updated if the contract model changed.
