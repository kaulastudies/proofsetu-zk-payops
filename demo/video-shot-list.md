# Demo Video Shot List

Target length: 2 to 3 minutes.

## 0:00 - 0:20 — Intro

Show project title:

ProofSetu ZK PayOps  
Private compliance proofs for Stellar workflow payments.

Explain the problem briefly:

Real-world payment workflows need eligibility, KYC, invoice, or compliance checks, but private records should not be exposed on-chain.

## 0:20 - 0:50 — ZK Explanation

Show `docs/zk-proof-design.md`.

Explain:

The participant proves they satisfy a payment eligibility rule without revealing private compliance data.

ZK statement:

eligibility_score >= minimum_required_score

## 0:50 - 1:20 — Circuit

Show:

`circuits/eligibility_proof/src/main.nr`

Explain:

Private input: eligibility_score  
Public input: minimum_required_score  
Proof result: participant passed the rule without exposing the actual score.

## 1:20 - 1:50 — Stellar Contract

Show:

`contracts/proofsetu_workflow/src/lib.rs`

Explain:

The Soroban contract connects proof status to workflow payment state.

Functions to mention:

- create_workflow
- submit_proof_hash
- mark_verified
- mark_eligible_for_settlement

## 1:50 - 2:20 — Frontend Demo

Open:

`frontend/index.html`

Click:

Run Demo Flow

Show statuses changing:

- Proof generated
- Stellar verified
- Eligible for settlement

## 2:20 - 2:50 — Roadmap

Show README roadmap.

Mention:

- full Noir/UltraHonk verifier integration
- Stellar USDC settlement
- gasless UX
- Confidential Tokens
- CCTP
- anchors
- AI compliance assistant

## 2:50 - 3:00 — Closing

Say:

ProofSetu ZK PayOps demonstrates a practical ZK + Stellar use case for private compliance and workflow payments.
