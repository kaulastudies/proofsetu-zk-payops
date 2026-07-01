# Frontend Demo

This folder contains a simple static dashboard for ProofSetu ZK PayOps.

## Purpose

The frontend demonstrates the user-facing workflow:

1. Create a payment workflow.
2. Generate a private eligibility proof.
3. Verify proof through the Stellar/Soroban verification path.
4. Mark the workflow as eligible for future Stellar USDC settlement.

## Current Status

This is a lightweight hackathon demo UI.

The current dashboard uses static/demo state to explain the product flow. The ZK circuit, proof design, and Soroban workflow contract are included in the repository separately.

## How to Run

Open `index.html` in a browser.

No build step is required.

## Future Work

- Connect frontend to Stellar Wallets Kit
- Connect to deployed Soroban workflow contract
- Display real proof verification result
- Add Stellar USDC settlement after verification
- Add gasless UX using sponsored transactions
