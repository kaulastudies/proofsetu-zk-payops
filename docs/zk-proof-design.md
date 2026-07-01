# ProofSetu ZK Proof Design

## Project

ProofSetu ZK PayOps

## Core Idea

ProofSetu ZK PayOps lets a workflow participant prove they are eligible for a payment or business process without revealing the private compliance data behind that eligibility.

## ZK Statement

The participant proves:

"I have a private eligibility score that satisfies the public workflow rule."

## Example

Private input:

- eligibility_score = 85

Public input:

- minimum_required_score = 70

The proof verifies that:

eligibility_score >= minimum_required_score

without revealing the actual eligibility_score.

## Why This Matters

In real-world payment workflows, a vendor, freelancer, hospital desk, agent, or business participant may need to prove KYC/compliance/payment eligibility before settlement.

Publishing private documents, KYC records, approval IDs, or internal scores on-chain is not acceptable.

Zero-knowledge allows the workflow to prove eligibility without exposing the underlying private data.

## Stellar Integration

The proof is generated off-chain.

A Stellar/Soroban verifier contract verifies the proof or records the verified proof result.

The workflow dashboard then marks the payment workflow as:

- Pending
- Proof Generated
- Verified
- Eligible for Settlement

## Current MVP Scope

This hackathon prototype focuses on:

- A simple private eligibility proof
- Stellar/Soroban verifier integration path
- Workflow payment status dashboard
- Clear roadmap toward Stellar USDC, gasless UX, Confidential Tokens, CCTP, and anchors

## Not Production Ready

This is a hackathon prototype. It is not audited and must not be used with real funds or sensitive user data.
