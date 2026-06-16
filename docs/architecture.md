# Architecture Overview

This repository implements the foundation for the Plastic Credit & Circular Economy Registry on Stellar / Soroban.

## High-Level Architecture

The protocol is organized into three modular smart contracts:

- `issuance` — validates MRV attestation data and mints credits for `COLLECTION` and `RECYCLING` events.
- `registry` — stores batch metadata, proof hashes, and duplicate-detection state.
- `retirement` — retires credits, anchors claim metadata, and records final attribution.

These contracts are designed to operate together as a set of independent but interoperable modules, allowing future upgrades or governance wrappers.

### Data Flow

1. A collection or recycling event is logged by a field operator.
2. The event is packaged into an off-chain proof bundle and hashed.
3. The proof hash is submitted to the `registry` contract.
4. The `issuance` contract verifies attestations, checks duplicates, and mints the appropriate credit.
5. A buyer may later send a retirement request to the `retirement` contract.

## Contract Boundaries

### Issuance contract

Responsibilities:
- Validate collector and facility credentials.
- Verify attestation metadata is complete and non-duplicated.
- Mint a credit token for the specified party.
- Emit a reference to the registry batch record.

### Registry contract

Responsibilities:
- Store hashes of submitted attestations.
- Enforce one-time batch registration and duplicate rejection.
- Maintain a mapping from batch IDs to proof metadata.
- Expose query patterns for batch history and audit.

### Retirement contract

Responsibilities:
- Burn or lock issued credits.
- Record claim metadata and retirement attribution.
- Provide a queryable retirement receipt.
- Prevent double-retirement of the same credit unit.

## Initial Data Model

### Core types

- `CreditType`
  - `COLLECTION`
  - `RECYCLING`

- `BatchRecord`
  - `batch_id`
  - `location`
  - `timestamp`
  - `weight_kg`
  - `proof_hash`
  - `collector`
  - `facility` (optional)
  - `credit_type`

- `Attestation`
  - `proof_hash`
  - `off_chain_uri`
  - `signatures`
  - `metadata`

- `RetirementRecord`
  - `credit_id`
  - `retiring_account`
  - `claim_statement`
  - `retired_at`

## Developer Considerations

- Keep the contract interface minimal for the first release.
- Prefer explicit attestation hashes and batch IDs over freeform text.
- Design storage with auditability and append-only semantics.
- Support separate issuance paths for collection and recycling credits.
