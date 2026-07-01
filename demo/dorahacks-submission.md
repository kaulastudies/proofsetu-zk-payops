# DoraHacks Submission

## Project Name

ProofSetu ZK PayOps

## Short Description

Private compliance proofs for Stellar workflow payments using zero-knowledge.

## Long Description

ProofSetu ZK PayOps is a Stellar-based hackathon prototype that uses zero-knowledge proofs to verify private workflow payment eligibility without exposing sensitive compliance, KYC, invoice, or approval data publicly.

In many real-world payment workflows, a participant must prove they are eligible before payment can be released. This may involve KYC approval, vendor verification, invoice approval, internal compliance checks, or business rule validation. Publishing those private records directly on-chain is not acceptable for regulated or business use cases.

ProofSetu solves this by using a ZK proof to prove that a participant satisfies a public workflow rule while keeping the private eligibility data hidden.

The MVP demonstrates a simple private eligibility proof:

A participant proves that their private eligibility score is greater than or equal to a public minimum required score.

The repository includes:

* a Noir-style eligibility proof circuit,
* sample prover input files,
* ZK proof design documentation,
* a Stellar/Soroban workflow contract,
* a frontend dashboard demo,
* demo script and submission documentation.

The Stellar/Soroban contract models the workflow payment state. A workflow can be created, a proof hash can be submitted, proof verification can be recorded, and the workflow can become eligible for future Stellar USDC settlement.

The current hackathon prototype focuses on the load-bearing ZK proof concept and Stellar workflow verification path. The roadmap includes full Noir + UltraHonk verifier integration on Stellar, Stellar USDC settlement, gasless UX, Confidential Token support for private payment amounts, CCTP cross-chain settlement, anchor on/off-ramp integrations, and AI-assisted compliance review.

ProofSetu ZK PayOps is designed for real-world use cases such as vendor payments, freelancer milestones, hospital/insurance workflows, B2B compliance workflows, payroll eligibility, private invoicing, and institutional settlement preparation.

## Tags

Stellar, Soroban, ZK, Zero Knowledge, Noir, UltraHonk, Payments, Compliance, USDC, Workflow, Privacy, Hackathon

## Hackathon Scope

This is a hackathon prototype. It is not audited, not production-ready, and must not be used with real funds or real sensitive user data.
