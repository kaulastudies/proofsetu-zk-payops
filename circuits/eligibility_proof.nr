// ProofSetu ZK PayOps
// Private eligibility proof concept
//
// Private input:
// - eligibility_score
//
// Public input:
// - minimum_required_score
//
// ZK statement:
// Prove eligibility_score >= minimum_required_score
// without revealing eligibility_score.

fn main(eligibility_score: u64, minimum_required_score: pub u64) {
    assert(eligibility_score >= minimum_required_score);
}
