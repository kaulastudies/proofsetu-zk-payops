# ProofSetu ZK PayOps Roadmap

## Phase 1 — Hackathon MVP

Goal: demonstrate a practical ZK + Stellar workflow payment use case.

Current components:

- private eligibility proof concept
- Noir-style proof circuit
- sample prover inputs
- Stellar/Soroban workflow contract
- frontend dashboard
- documentation and demo script

## Phase 2 — Full On-Chain Proof Verification

Goal: connect the proof flow directly to a Stellar verifier contract.

Planned work:

- generate Noir proof
- export proof and public inputs
- integrate UltraHonk verifier on Stellar
- call verifier from workflow contract
- update workflow state only after proof verification succeeds

## Phase 3 — Stellar USDC Settlement

Goal: release payment after verification.

Planned work:

- integrate Stellar USDC
- link verified workflow status to payment release
- create payer/participant settlement flow
- add payment history

## Phase 4 — Gasless UX

Goal: allow non-technical users to interact without needing XLM upfront.

Planned work:

- sponsored transactions
- fee-bump transactions
- relayer support
- passkey or embedded wallet exploration

## Phase 5 — Confidential Tokens

Goal: hide payment amounts and balances while preserving compliance controls.

Planned work:

- study Stellar Confidential Tokens developer preview
- explore private payment amount flows
- add auditor/selective disclosure model
- connect confidential payment state to ProofSetu workflows

## Phase 6 — CCTP Cross-Chain Settlement

Goal: support Stellar USDC to EVM USDC settlement.

Planned work:

- Stellar source CCTP flow
- attestation polling
- destination mint
- relayer fee model
- Base/Arbitrum settlement support

## Phase 7 — Anchors and Fiat Ramps

Goal: connect real-world fiat entry and exit.

Planned work:

- SEP-1 anchor discovery
- SEP-10 authentication
- SEP-24 deposit/withdrawal flow
- fiat on/off-ramp partner exploration

## Phase 8 — AI Compliance Assistant

Goal: help users prepare workflows before proof generation.

Planned work:

- document review assistant
- missing compliance checklist
- workflow risk flags
- plain-language explanation of proof/payment status
