# Issue Backlog for Days 2-4

This backlog captures the Day 2-4 development tasks outlined in `Plan.md`.

## Day 2: Contract Design and Core Data Model

- Define shared contract interfaces and data models.
- Implement `CreditType`, `BatchRecord`, `ProofHash`, and `Attestation` types.
- Create storage helpers for batch lookup and duplicate detection.
- Add unit tests for batch registration and metadata invariants.
- Ensure separate issuance paths for `COLLECTION` and `RECYCLING` credits.

## Day 3: Issuance and Registry Logic

- Implement `mint_collection_credit` and `mint_recycling_credit`.
- Add attestation validation and duplicate rejection logic.
- Simulate realistic MRV flows in contract-level tests.
- Document issuance contract behavior and validation rules.

## Day 4: Retirement and Attribution

- Implement `retire_credit` with optional claim metadata.
- Record retirement receipts and prevent double-retirement.
- Add retirement tests and failure cases.
- Create contributor-facing onboarding and milestone guidance.

## Future Enhancements

- Governance-controlled contract upgrades.
- Integration test harness against Soroban local sandbox.
- Off-chain SDK or relay service prototypes.
- Frontend registry explorer and buyer portal wireframes.
