# ProofSetu ZK PayOps Architecture

## Flow

1. A workflow payment request is created.
2. The participant has private eligibility/compliance data.
3. A ZK proof is generated off-chain.
4. The proof proves the participant satisfies the public rule.
5. A Stellar/Soroban verifier checks the proof or records the verification result.
6. The dashboard marks the workflow as verified.
7. Payment settlement can happen later through Stellar USDC.

## Components

### ZK Circuit

Defines the private eligibility rule.

### Proof Generator

Generates proof off-chain using Noir/UltraHonk or another supported ZK stack.

### Soroban Verifier

Verifies the proof or links the proof result to a workflow state on Stellar.

### Workflow Dashboard

Shows payment workflow status without exposing private compliance data.

## Roadmap

1. Full Noir + UltraHonk verifier integration
2. Stellar testnet verification
3. Stellar USDC settlement
4. Gasless UX with sponsored fees
5. Confidential Tokens for private payment amounts
6. CCTP for Stellar-to-EVM settlement
7. Anchor/on-off-ramp support
