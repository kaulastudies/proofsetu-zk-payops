// ProofSetu ZK PayOps
//
// ZK Statement:
// A participant proves that their private eligibility score
// satisfies a public minimum workflow requirement.
//
// Private input:
// - eligibility_score
//
// Public input:
// - minimum_required_score
//
// The verifier learns:
// - The participant passed the public rule.
//
// The verifier does NOT learn:
// - The actual private eligibility score.
// - The underlying KYC/compliance record.

fn main(eligibility_score: u64, minimum_required_score: pub u64) {
    assert(eligibility_score >= minimum_required_score);
}
