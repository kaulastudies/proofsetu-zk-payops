# Contracts

This folder contains the Stellar/Soroban contract layer for ProofSetu ZK PayOps.

## Current Contract

`proofsetu_workflow`

This contract models the workflow/payment state for the hackathon MVP.

## Purpose

The contract links a private ZK eligibility proof to a Stellar workflow/payment process.

## Current Flow

1. `create_workflow`
   - Creates a workflow payment request.
   - Stores payer, participant, public rule, and status.

2. `submit_proof_hash`
   - Stores a hash/reference of an off-chain generated ZK proof.
   - Moves workflow status to `ProofSubmitted`.

3. `mark_verified`
   - Represents successful proof verification.
   - In the full version, this should be connected directly to a Stellar verifier contract.

4. `mark_eligible_for_settlement`
   - Marks the workflow as ready for future Stellar USDC settlement.

5. `get_workflow`
   - Reads workflow state.

6. `get_proof_hash`
   - Reads proof hash/reference.

## ZK Integration Roadmap

The current workflow contract is designed to connect with a verifier contract.

The intended final path is:

1. Generate ZK proof off-chain.
2. Verify proof using Noir + UltraHonk or Circom + Groth16 verifier on Stellar.
3. If verification succeeds, update the workflow status to `Verified`.
4. Allow future Stellar USDC settlement.

## Hackathon Note

This contract is part of the hackathon MVP. It shows how ZK proof verification can be connected to a real-world Stellar workflow payment system.

It is not audited and not production-ready.
