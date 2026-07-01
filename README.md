# ProofSetu ZK PayOps

**Private compliance proofs for Stellar workflow payments.**

ProofSetu ZK PayOps is a Stellar-based hackathon prototype that uses zero-knowledge proofs to verify workflow payment eligibility without exposing private compliance or KYC data publicly.

## Problem

Real-world payment workflows often require sensitive checks before settlement:

* KYC or identity approval
* vendor/freelancer eligibility
* invoice approval
* internal compliance rules
* payment authorization
* hospital, insurance, or B2B workflow verification

Putting this private information directly on-chain is not acceptable for businesses, institutions, users, or regulated workflows.

## Solution

ProofSetu ZK PayOps allows a participant to prove they are eligible for a payment workflow without revealing the private data behind that eligibility.

The project demonstrates a simple private eligibility proof:

> A participant proves that their private eligibility score satisfies a public workflow rule.

The public can verify that the participant passed the rule, but the private eligibility score and underlying compliance data are not revealed.

## What ZK Does

The zero-knowledge proof proves the following statement:

> “The participant has private eligibility data that satisfies the public workflow requirement.”

Example:

* Private input: `eligibility_score = 85`
* Public input: `minimum_required_score = 70`
* ZK proof: `eligibility_score >= minimum_required_score`

The verifier learns that the participant passed the rule, but does not learn the actual private score.

## What Stellar Does

Stellar/Soroban is used as the workflow verification and settlement layer.

The intended flow is:

1. A workflow payment request is created.
2. The participant generates a ZK proof off-chain.
3. A Stellar/Soroban verifier contract verifies the proof or records the verified proof result.
4. The workflow status becomes verified.
5. The payment becomes eligible for future Stellar USDC settlement.

## Current MVP

This hackathon prototype includes:

* private eligibility proof design,
* basic Noir-style proof circuit,
* Stellar/Soroban verifier integration plan,
* workflow payment architecture,
* documentation for how ZK is used,
* roadmap for Stellar USDC, gasless UX, Confidential Tokens, CCTP, and anchors.

## Repository Structure

```text
contracts/   Stellar/Soroban verifier and workflow contract files
circuits/    ZK proof circuit files
frontend/    Demo dashboard files
scripts/     Helper scripts for proof generation, deployment, and verification
docs/        Architecture, ZK design, and roadmap
demo/        Demo script, screenshots, and video notes
```

## Demo Flow

The hackathon demo follows this flow:

1. Create a payment workflow.
2. Generate or use a sample private eligibility proof.
3. Verify the proof through the Stellar/Soroban verification path.
4. Mark the workflow as verified.
5. Show that private compliance data is not revealed publicly.

## Use Cases

ProofSetu ZK PayOps can be extended to:

* vendor payment approval,
* freelancer milestone settlement,
* payroll eligibility,
* private invoice verification,
* hospital or insurance workflow checks,
* B2B compliance workflows,
* institutional settlement preparation,
* private credential or KYC-passed proofs.

## Why Stellar

Stellar is well-suited for real-world payment workflows because of:

* fast and low-cost transactions,
* Stellar USDC,
* Soroban smart contracts,
* support for ZK-friendly primitives,
* emerging Confidential Token and privacy tooling,
* future CCTP support for cross-chain USDC settlement,
* anchor ecosystem for fiat on/off-ramp integrations.

## Roadmap

### Phase 1 — Hackathon MVP

* Private eligibility proof circuit
* Stellar/Soroban verification path
* Workflow status dashboard
* Documentation and demo video

### Phase 2 — Stellar USDC Settlement

* Add Stellar USDC payment settlement after proof verification
* Link verified workflow state to payment release

### Phase 3 — Gasless UX

* Add sponsored transactions
* Explore OpenZeppelin Relayer or fee-bump transaction support
* Allow users to interact without needing XLM upfront

### Phase 4 — Confidential Tokens

* Explore Stellar Confidential Tokens developer preview
* Hide payment amounts and balances while preserving compliance visibility
* Add auditor/selective disclosure concepts

### Phase 5 — CCTP Cross-Chain Settlement

* Add Stellar-to-EVM USDC settlement using CCTP
* Support Base, Arbitrum, or other destination chains in future versions

### Phase 6 — Anchors and Fiat Ramps

* Explore SEP-based anchor integrations
* Add fiat on/off-ramp support for real-world business users

### Phase 7 — AI Compliance Assistant

* Add AI-assisted document and workflow review
* Help users understand missing compliance steps before payment settlement

## Hackathon Scope

This project is built for the Stellar Hacks: Real-World ZK hackathon.

The current goal is to demonstrate a practical ZK + Stellar use case where the ZK proof is meaningful and connected to a real-world workflow payment scenario.

## Disclaimer

This is a hackathon prototype.

It is not audited, not production-ready, and must not be used with real funds, real private documents, or sensitive user data.
