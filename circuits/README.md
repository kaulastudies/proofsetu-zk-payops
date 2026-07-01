# Circuits

This folder contains the zero-knowledge circuit layer for ProofSetu ZK PayOps.

## Current Circuit

`eligibility_proof`

This circuit proves that a participant satisfies a public workflow payment eligibility rule without revealing the private eligibility value.

## ZK Statement

The participant proves:

```text
eligibility_score >= minimum_required_score
