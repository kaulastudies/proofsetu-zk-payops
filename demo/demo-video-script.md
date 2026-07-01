# ProofSetu ZK PayOps Demo Video Script

Hi, this is ProofSetu ZK PayOps.

ProofSetu is a Stellar-based zero-knowledge prototype for private compliance and workflow payment eligibility.

In real-world payment workflows, a vendor, freelancer, hospital desk, claim agent, or business participant may need to prove they are eligible before receiving payment. This eligibility could depend on KYC, invoice approval, internal compliance checks, or business rules.

The problem is that these private records should not be exposed publicly on-chain.

ProofSetu uses zero-knowledge proofs to solve this. The participant proves they satisfy a public workflow rule without revealing the private data behind that proof.

For this hackathon MVP, the proof statement is simple:

A participant has a private eligibility score, and that score must be greater than or equal to a public minimum requirement.

In our sample circuit, the private eligibility score is 85 and the public minimum required score is 70. The proof confirms that the participant passed the rule, but the verifier does not learn the actual private score.

The repository includes the ZK circuit, sample prover inputs, proof design documentation, and a Stellar/Soroban workflow contract.

The Soroban workflow contract models the payment workflow state. A workflow can be created, a proof hash can be submitted, the workflow can be marked as verified, and then it can become eligible for future Stellar USDC settlement.

The frontend dashboard shows this complete flow visually: create workflow, generate proof, verify on Stellar, and mark the workflow as eligible for settlement.

The current MVP focuses on the load-bearing ZK eligibility proof and the Stellar workflow verification path. The roadmap includes full Noir and UltraHonk verifier integration on Stellar, Stellar USDC settlement, gasless UX with sponsored transactions, Confidential Tokens for private payment amounts, CCTP cross-chain settlement, anchor on/off-ramp support, and AI-assisted compliance workflows.

ProofSetu ZK PayOps is not production-ready and is not audited. It is a hackathon prototype showing how zero-knowledge proofs can support real-world private workflow payments on Stellar.
