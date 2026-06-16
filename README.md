# ♻️ Plastic Credit & Circular Economy Registry

**A blockchain-based Measurement, Reporting, and Verification (MRV) and tokenization protocol for plastic waste collection and recycling, built on Stellar / Soroban.**

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-7D00FF)](https://stellar.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](#license)
[![Status](https://img.shields.io/badge/status-pre--alpha-orange)](#roadmap)

---

## Table of Contents

1. [Overview](#overview)
2. [Problem Statement](#problem-statement)
3. [Core Concept](#core-concept)
4. [Why Stellar / Soroban](#why-stellar--soroban)
5. [System Architecture](#system-architecture)
6. [Credit Lifecycle](#credit-lifecycle)
7. [MRV (Measurement, Reporting, Verification)](#mrv-measurement-reporting-verification)
8. [Smart Contract Design](#smart-contract-design)
9. [Token Model](#token-model)
10. [Actors & Roles](#actors--roles)
11. [Retirement & Attribution](#retirement--attribution)
12. [Data Flow](#data-flow)
13. [Tech Stack](#tech-stack)
14. [Repository Structure](#repository-structure)
15. [Getting Started](#getting-started)
16. [Testing](#testing)
17. [Security Considerations](#security-considerations)
18. [Governance](#governance)
19. [Roadmap](#roadmap)
20. [Contributing](#contributing)
21. [FAQ](#faq)
22. [License](#license)

---

## Overview

The **Plastic Credit & Circular Economy Registry** is a decentralized protocol for tokenizing verified plastic waste collection and recycling activity. Each token represents a quantifiable, auditable unit of environmental impact — one tonne of plastic either removed from the environment or mechanically recycled — anchored to real-world proof data and issued on a public, low-cost, high-throughput blockchain (Stellar, with smart contract logic powered by Soroban).

The registry is designed to give corporations, NGOs, and individuals a transparent, fraud-resistant way to fund and claim plastic circularity outcomes, while giving waste collectors and recyclers — particularly in informal and emerging-market supply chains — direct, traceable access to global capital.

---

## Problem Statement

Voluntary plastic credit markets today suffer from several structural weaknesses:

- **Opaque verification** — Many existing schemes rely on self-reported or loosely audited data, making double-counting and fraud difficult to detect.
- **Centralized registries** — Credit issuance and retirement are typically controlled by a single organization, creating single points of failure and trust bottlenecks.
- **Poor traceability** — It is often impossible to verify whether a claimed "recycled" credit corresponds to an actual end product.
- **Exclusion of informal collectors** — Waste pickers and small aggregators, who do the majority of physical collection work in many regions, are frequently disconnected from the financial value their work generates.
- **High transaction overhead** — Traditional carbon/plastic credit infrastructure (legal contracts, manual audits, intermediary fees) makes small, granular credits economically unviable to issue.

This protocol addresses these issues using on-chain verification anchoring, low-cost tokenization, and transparent, programmable retirement logic.

---

## Core Concept

| Element | Definition |
|---|---|
| **Credit unit** | 1 credit = 1 tonne of plastic waste collected from the environment **or** mechanically recycled |
| **Credit types** | `COLLECTION` credit and `RECYCLING` credit — tracked as distinct, separately issued asset classes |
| **Backing** | Each credit is minted only after MRV proof (GPS, weighbridge, facility certification) is submitted and validated |
| **Retirement** | Credits are permanently burned/locked when claimed, optionally bound to a specific end product or claim statement |
| **Dual retirement** | A single physical batch of plastic can generate and retire both a collection credit and a recycling credit, reflecting two distinct value-chain stages |

---

## Why Stellar / Soroban

Stellar and its Soroban smart contract platform are well-suited to this use case for several concrete reasons:

- **Low and predictable fees** — Base fees are fractions of a cent, which matters when credits are issued in small, frequent batches (a single informal collector may submit sub-tonne batches daily).
- **Fast finality** — 3–5 second settlement supports near-real-time issuance as MRV data is submitted from the field.
- **Native asset support** — Stellar's built-in asset/trustline model allows `COLLECTION` and `RECYCLING` credits to exist as first-class, exchange-compatible tokens without needing to build ERC-20-equivalent logic from scratch.
- **Soroban for programmable logic** — Rust-based, WASM-compiled smart contracts handle the conditional minting, registry storage, and retirement/attribution logic that pure asset issuance can't express alone.
- **Anchor network** — Stellar's existing anchor infrastructure (regulated on/off ramps and identity-verified institutions) can double as a verification layer for collection partners and recycling facilities.
- **Built-in compliance primitives** — Features like clawback, asset authorization flags, and multi-signature accounts support regulatory and auditability requirements without custom contract code.
- **Sustainability alignment** — Stellar's consensus mechanism (Stellar Consensus Protocol) has a negligible energy footprint relative to proof-of-work chains, which avoids the optics problem of running an environmental-impact protocol on a high-emissions network.

---

## System Architecture

```
┌──────────────────────────────────────────────────────────────────────┐
│                          FIELD DATA LAYER                              │
│  GPS collection events │ Weighbridge logs │ Facility certifications    │
└───────────────────────────────┬──────────────────────────────────────┘
                                 │ (signed attestations)
                                 ▼
┌──────────────────────────────────────────────────────────────────────┐
│                        ORACLE / ATTESTATION LAYER                      │
│   Verified field agents, IoT weighbridges, certified facility partners │
│   submit hashed proof bundles (stored off-chain, hash committed on-chain)│
└───────────────────────────────┬──────────────────────────────────────┘
                                 │
                                 ▼
┌──────────────────────────────────────────────────────────────────────┐
│                         SOROBAN CONTRACT LAYER                         │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────────────┐  │
│  │ Issuance        │  │ Registry        │  │ Retirement /            │  │
│  │ Contract        │  │ Contract        │  │ Attribution Contract    │  │
│  │ (mint on proof) │  │ (batch metadata)│  │ (burn + claim binding)  │  │
│  └────────────────┘  └────────────────┘  └────────────────────────┘  │
└───────────────────────────────┬──────────────────────────────────────┘
                                 │
                                 ▼
┌──────────────────────────────────────────────────────────────────────┐
│                       STELLAR ASSET LAYER                              │
│      COLLECT-PLC token        │        RECYCLE-PLC token               │
│      (Stellar Asset Contract wrapping classic Stellar assets)          │
└───────────────────────────────┬──────────────────────────────────────┘
                                 │
                                 ▼
┌──────────────────────────────────────────────────────────────────────┐
│                          APPLICATION LAYER                              │
│   Buyer dashboard │ Collector/recycler portal │ Public registry explorer│
└──────────────────────────────────────────────────────────────────────┘
```

---

## Credit Lifecycle

1. **Collection event occurs** — A waste picker, aggregator, or partner organization collects plastic waste. A mobile app or IoT device logs GPS coordinates and timestamp.
2. **Weighing & logging** — The batch is weighed at a registered collection point or weighbridge; the reading is digitally signed by the operator or device.
3. **Attestation submission** — The signed data bundle (GPS + weight + photos/metadata) is hashed and submitted to the **Registry Contract**, with the full bundle pinned to off-chain storage (IPFS/Arweave).
4. **Validation** — The **Issuance Contract** checks the attestation against registered collector/facility credentials and anti-fraud rules (e.g., duplicate hash detection, GPS plausibility, rate limits).
5. **Minting** — On successful validation, a `COLLECTION` credit is minted to the collector's (or sponsoring organization's) Stellar account, denominated 1:1 with verified tonnage.
6. **Recycling event occurs** — The same (or a downstream) batch is processed at a certified recycling facility; the facility submits its own attestation (input weight, output weight, process certification).
7. **Second minting** — A `RECYCLING` credit is minted, separately tracked, representing the recycling-stage value add.
8. **Trading (optional)** — Credits may be transferred or traded on Stellar's native DEX or via OTC agreements before retirement.
9. **Retirement** — A buyer (brand, manufacturer, NGO, or individual) retires one or both credit types, optionally attaching an end-product claim (e.g., "Bottle SKU #4471, Batch #2026-0614").
10. **Public record** — The retirement event, including any end-product binding, is permanently and publicly queryable via the registry explorer.

---

## MRV (Measurement, Reporting, Verification)

MRV is the foundation of credit integrity. The protocol does not attempt to verify physical reality on-chain — it anchors and timestamps **already-verified** off-chain data and makes tampering or duplication detectable.

### Data Captured Per Batch

| Field | Source | Purpose |
|---|---|---|
| GPS coordinates + timestamp | Mobile app / IoT tracker | Confirms location and time plausibility of collection |
| Weight (kg) | Digital weighbridge or calibrated scale | Determines credit quantity |
| Collector ID | Registered identity credential | Links batch to an accountable party |
| Facility certification | Recycling facility's compliance documents | Confirms recycling claim legitimacy |
| Photo/video evidence | Field app capture | Supplementary fraud deterrence |
| Output product data | Facility records | Supports end-product attribution |

### Anti-Fraud Mechanisms

- **Hash commitments** — Full proof bundles live off-chain (IPFS/Arweave); only their cryptographic hash is stored on-chain, making any later tampering detectable.
- **Duplicate detection** — The Registry Contract rejects attestations whose hash, GPS+timestamp combination, or batch ID has already been submitted.
- **Rate and plausibility limits** — Per-collector daily tonnage caps and GPS-speed plausibility checks (e.g., a single device cannot log collection events 50km apart within minutes).
- **Multi-party attestation** — High-value batches may require co-signature from a second verifier (e.g., a regional NGO partner) before minting is authorized.
- **Independent audits** — Periodic third-party audits sample physical sites against the on-chain record; audit results are themselves recorded on-chain as a trust signal.

---

## Smart Contract Design

The protocol is composed of three primary Soroban contracts, designed to be modular and independently upgradeable behind a governance-controlled proxy pattern.

### 1. Issuance Contract

Responsible for validating attestations and minting credits.

```rust
pub trait IssuanceContract {
    /// Mint a collection credit after attestation validation
    fn mint_collection_credit(
        env: Env,
        collector: Address,
        batch_id: BytesN<32>,
        weight_kg: u64,
        proof_hash: BytesN<32>,
        gps_lat: i64,
        gps_lng: i64,
        timestamp: u64,
    ) -> Result<(), Error>;

    /// Mint a recycling credit after facility attestation validation
    fn mint_recycling_credit(
        env: Env,
        facility: Address,
        batch_id: BytesN<32>,
        input_weight_kg: u64,
        output_weight_kg: u64,
        proof_hash: BytesN<32>,
        timestamp: u64,
    ) -> Result<(), Error>;

    /// Register a new verified collector or facility
    fn register_participant(
        env: Env,
        participant: Address,
        role: ParticipantRole,
        credentials_hash: BytesN<32>,
    ) -> Result<(), Error>;
}
```

### 2. Registry Contract

Stores immutable batch metadata and exposes public queryability.

```rust
pub trait RegistryContract {
    /// Store metadata for a batch, keyed by batch_id
    fn record_batch(
        env: Env,
        batch_id: BytesN<32>,
        metadata: BatchMetadata,
    ) -> Result<(), Error>;

    /// Retrieve full batch history (collection -> recycling -> retirement)
    fn get_batch_history(env: Env, batch_id: BytesN<32>) -> Vec<BatchEvent>;

    /// Link a collection batch to a downstream recycling batch
    fn link_batches(
        env: Env,
        collection_batch_id: BytesN<32>,
        recycling_batch_id: BytesN<32>,
    ) -> Result<(), Error>;
}
```

### 3. Retirement & Attribution Contract

Handles permanent credit burning and end-product claim binding.

```rust
pub trait RetirementContract {
    /// Retire a credit, optionally binding to an end-product claim
    fn retire_credit(
        env: Env,
        holder: Address,
        credit_type: CreditType,
        amount: u64,
        claim_reference: Option<BytesN<32>>,
    ) -> Result<RetirementReceipt, Error>;

    /// Dual retirement: retire matched collection + recycling credits together
    fn dual_retire(
        env: Env,
        holder: Address,
        batch_id: BytesN<32>,
        claim_reference: Option<BytesN<32>>,
    ) -> Result<RetirementReceipt, Error>;

    /// Query all retirements bound to a given end-product claim
    fn get_retirements_by_claim(env: Env, claim_reference: BytesN<32>) -> Vec<RetirementReceipt>;
}
```

### Core Data Structures

```rust
pub struct BatchMetadata {
    pub batch_id: BytesN<32>,
    pub origin_collector: Address,
    pub weight_kg: u64,
    pub gps_lat: i64,
    pub gps_lng: i64,
    pub timestamp: u64,
    pub proof_hash: BytesN<32>,
    pub status: BatchStatus,
}

pub enum BatchStatus {
    Collected,
    InTransit,
    Recycled,
    Retired,
}

pub enum CreditType {
    Collection,
    Recycling,
}

pub enum ParticipantRole {
    Collector,
    Aggregator,
    Facility,
    Verifier,
    Auditor,
}

pub struct RetirementReceipt {
    pub holder: Address,
    pub credit_type: CreditType,
    pub amount: u64,
    pub claim_reference: Option<BytesN<32>>,
    pub timestamp: u64,
    pub retirement_id: BytesN<32>,
}
```

---

## Token Model

| Token | Symbol | Backing | Mint Trigger | Burn Trigger |
|---|---|---|---|---|
| Collection Credit | `COLLECT-PLC` | 1 tonne verified collection | Validated collection attestation | Retirement |
| Recycling Credit | `RECYCLE-PLC` | 1 tonne verified mechanical recycling | Validated facility attestation | Retirement |
| Governance Token (optional) | `PLC-GOV` | N/A | Protocol launch allocation / contribution rewards | N/A (used for voting) |

Both `COLLECT-PLC` and `RECYCLE-PLC` are implemented as Stellar Asset Contract (SAC)–wrapped classic Stellar assets, giving them native compatibility with Stellar wallets, the Stellar DEX, and Soroban contract logic simultaneously.

**Decimal precision:** 7 decimal places (Stellar standard), allowing fractional-tonne credits (e.g., 0.0015000 tonnes for very small informal-collector batches).

---

## Actors & Roles

| Role | Description | On-Chain Permissions |
|---|---|---|
| **Collector** | Individual or small aggregator performing physical collection | Submit collection attestations |
| **Aggregator** | Organization consolidating batches from multiple collectors | Submit/relay attestations, bundle batches |
| **Recycling Facility** | Certified mechanical recycling operation | Submit recycling attestations |
| **Verifier** | Independent body co-signing high-value attestations | Co-sign attestations, flag disputes |
| **Auditor** | Third party conducting periodic physical/process audits | Publish audit results on-chain |
| **Buyer/Retiree** | Brand, manufacturer, NGO, or individual purchasing and retiring credits | Hold, transfer, retire credits |
| **Governance Council** | Multi-sig body overseeing contract upgrades and parameter changes | Upgrade contracts, adjust fraud thresholds, manage participant registry |

---

## Retirement & Attribution

Retirement is the point at which a credit is permanently removed from circulation and counted as a claimed environmental outcome.

- **Standard retirement** — A holder burns a credit with no specific claim attached (a general environmental contribution claim).
- **Attributed retirement** — A holder burns a credit and binds it to a `claim_reference` (e.g., a hash representing a specific SKU, batch, or marketing claim such as *"This bottle contains verified ocean-bound plastic, Batch #2026-0614"*). This binding is publicly queryable, allowing any third party to verify the claim against the underlying batch history.
- **Dual retirement** — Both a `COLLECT-PLC` and a matched `RECYCLE-PLC` credit (linked via shared or chained `batch_id`) are retired together in a single transaction, producing a combined receipt that reflects the full collection-to-recycling value chain for that tonnage.

Every retirement emits an on-chain `RetirementReceipt`, permanently queryable by anyone — preventing the same physical tonne from being claimed twice by different parties.

---

## Data Flow

```
Field Collection Event
        │
        ▼
Mobile/IoT App ──(GPS + weight + timestamp + signature)──▶ Off-chain storage (IPFS)
        │                                                          │
        │                                                          ▼
        └────────────────(proof_hash)───────────────────▶ Soroban Issuance Contract
                                                                    │
                                                          validate & mint
                                                                    │
                                                                    ▼
                                                     COLLECT-PLC credited to Collector
                                                                    │
                                          (batch moves to recycling facility)
                                                                    │
                                                                    ▼
                                                    Facility submits recycling proof
                                                                    │
                                                          validate & mint
                                                                    │
                                                                    ▼
                                                    RECYCLE-PLC credited to Facility
                                                                    │
                                         (credits transferred/sold to Buyer via DEX or OTC)
                                                                    │
                                                                    ▼
                                                  Buyer calls retire_credit / dual_retire
                                                                    │
                                                                    ▼
                                          RetirementReceipt published — publicly auditable
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| Smart contracts | Rust + Soroban SDK (WASM) |
| Blockchain | Stellar (Mainnet/Testnet/Futurenet) |
| Asset standard | Stellar Classic Assets + Stellar Asset Contract (SAC) |
| Off-chain proof storage | IPFS / Arweave |
| Oracle / attestation relay | Custom signer service + Stellar multi-sig |
| Backend API | Node.js / TypeScript, Soroban RPC client |
| Frontend (buyer dashboard, registry explorer) | React, Stellar SDK (`@stellar/stellar-sdk`) |
| Mobile field app | React Native (offline-first, syncs attestations when connectivity available) |
| Identity/credentialing | Stellar anchor-style KYC integration for collectors & facilities |
| Testing | Soroban CLI test harness, Rust unit tests, Futurenet integration tests |

---

## Repository Structure

```
plastic-credit-registry/
├── contracts/
│   ├── issuance/             # Issuance contract (Rust/Soroban)
│   ├── registry/              # Registry contract (Rust/Soroban)
│   └── retirement/            # Retirement & attribution contract (Rust/Soroban)
├── sdk/
│   └── ts/                   # TypeScript SDK for contract interaction
├── backend/
│   ├── attestation-relay/     # Service that relays field data + hashes to chain
│   └── api/                  # REST/GraphQL API for dashboards
├── frontend/
│   ├── buyer-dashboard/       # Web app for purchasing/retiring credits
│   └── registry-explorer/     # Public block explorer-style UI for batch history
├── mobile/
│   └── field-app/            # React Native app for collectors
├── scripts/
│   ├── deploy.sh              # Contract deployment scripts
│   └── seed-testnet.sh        # Testnet data seeding
├── tests/
│   ├── unit/
│   └── integration/
├── docs/
│   ├── architecture.md
│   ├── contributing.md
│   ├── mrv-spec.md
│   └── governance.md
├── ISSUES.md
├── .env.example
├── Cargo.toml
└── README.md
```

---

## Getting Started

### Prerequisites

- Rust (stable) + `wasm32-unknown-unknown` target
- [Soroban CLI](https://developers.stellar.org/docs/tools/developer-tools)
- Node.js ≥ 18
- A funded Stellar Testnet account ([Friendbot](https://friendbot.stellar.org))

### Installation

```bash
git clone https://github.com/your-org/plastic-credit-registry.git
cd plastic-credit-registry

# Install Rust target for Soroban contracts
rustup target add wasm32-unknown-unknown

# Build contracts
cd contracts/issuance && cargo build --target wasm32-unknown-unknown --release
cd ../registry && cargo build --target wasm32-unknown-unknown --release
cd ../retirement && cargo build --target wasm32-unknown-unknown --release

# Install backend & frontend dependencies
cd ../../backend/api && npm install
cd ../../frontend/buyer-dashboard && npm install
```

### Deploying to Testnet

```bash
soroban contract deploy \
  --wasm contracts/issuance/target/wasm32-unknown-unknown/release/issuance.wasm \
  --source <your-secret-key> \
  --network testnet
```

### Environment Variables

```bash
cp .env.example .env
# Fill in:
# STELLAR_NETWORK=testnet
# SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
# ISSUANCE_CONTRACT_ID=
# REGISTRY_CONTRACT_ID=
# RETIREMENT_CONTRACT_ID=
# IPFS_API_KEY=
```

---

## Testing

```bash
# Rust contract unit tests
cargo test --workspace

# Soroban integration tests against local sandbox
soroban contract invoke --id <contract_id> --network testnet -- mint_collection_credit ...

# Frontend/backend tests
npm test
```

Test coverage should include: attestation validation edge cases, duplicate-hash rejection, GPS plausibility boundaries, dual-retirement matching logic, and contract upgrade safety.

---

## Security Considerations

- **Oracle trust** — The protocol's weakest link is the off-chain-to-on-chain bridge; attestation signers must be vetted, rotated, and ideally distributed across multiple independent verifiers rather than a single relay.
- **Sybil resistance** — Collector registration should require identity credentialing (even lightweight, e.g., phone number + local NGO vouching) to prevent fake-collector farming of credits.
- **Replay protection** — All attestations include a nonce and timestamp to prevent resubmission.
- **Upgrade governance** — Contract upgrades should require multi-sig governance approval with a time-locked execution delay to prevent unilateral or rushed changes.
- **Audit trail immutability** — Once recorded, batch metadata and retirement receipts must be append-only; corrections require new linked records, never overwrites.
- **Recommended external audit** — Before mainnet deployment, contracts should undergo an independent security audit (e.g., via firms experienced in Soroban/Rust contract review).

---

## Governance

The protocol is intended to transition from a founding-team-operated registry to a multi-stakeholder governed system:

1. **Phase 1 — Curated** — Founding team manages participant registration and fraud parameters directly.
2. **Phase 2 — Council** — A multi-sig council (founding team + NGO partners + auditor representatives) governs parameter changes and contract upgrades.
3. **Phase 3 — Token-weighted governance (optional)** — If a `PLC-GOV` token is introduced, governance decisions may shift to on-chain voting weighted by stake and/or verified contribution history.

---

## Roadmap

| Phase | Milestone |
|---|---|
| **Q3 2026** | Finalize contract specs, testnet deployment, MRV pilot with 1–2 collection partners |
| **Q4 2026** | Field app beta, first dual-retirement pilot transaction, third-party audit of contracts |
| **Q1 2027** | Public testnet registry explorer launch, onboarding additional recycling facility partners |
| **Q2 2027** | Mainnet launch, first corporate buyer integrations |
| **H2 2027** | Governance council formation, expansion to additional waste streams (e.g., textile, e-waste) |

---

## Contributing

Contributions are welcome. Please:

1. Fork the repository and create a feature branch.
2. Ensure all contract changes include corresponding unit tests.
3. Run `cargo fmt` and `cargo clippy` before submitting.
4. Open a pull request with a clear description of the change and its motivation.
5. For MRV methodology changes, include supporting rationale in `docs/mrv-spec.md`.

---

## FAQ

**Q: How is double-counting prevented across different registries?**
A: Currently scoped to this registry only; cross-registry deduplication would require either an industry-wide shared ledger standard or bilateral hash-sharing agreements, which is a known limitation flagged for future work.

**Q: What happens if a collector submits fraudulent GPS/weight data?**
A: Plausibility checks and multi-party co-signature requirements reduce risk, but the system cannot fully eliminate fraud at the physical layer — this is why periodic independent audits and reputation-based participant scoring are built into governance.

**Q: Can credits be partially retired?**
A: Yes, given the 7-decimal precision of the underlying Stellar asset, fractional-tonne retirement is supported.

**Q: Why two separate credit types instead of one?**
A: Collection and recycling represent distinct value-chain stages with different verification methods and different economic actors; separating them allows each stage to be independently priced, traded, and claimed.

---

## License

This project is released under the [MIT License](LICENSE) unless otherwise noted in individual subdirectories.

---

*Built for a circular economy. Verified on-chain. Powered by Stellar.*
