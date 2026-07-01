#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, BytesN, Env,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowStatus {
    Pending,
    ProofSubmitted,
    Verified,
    Rejected,
    EligibleForSettlement,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Workflow {
    pub id: u64,
    pub payer: Address,
    pub participant: Address,
    pub minimum_required_score: u64,
    pub status: WorkflowStatus,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Workflow(u64),
    ProofHash(u64),
}

#[contract]
pub struct ProofSetuWorkflow;

#[contractimpl]
impl ProofSetuWorkflow {
    /// Creates a payment/workflow request.
    ///
    /// In the real ProofSetu flow, this represents a vendor, freelancer,
    /// hospital desk, claim agent, or business participant becoming part of
    /// a payment workflow.
    pub fn create_workflow(
        env: Env,
        id: u64,
        payer: Address,
        participant: Address,
        minimum_required_score: u64,
    ) -> Workflow {
        payer.require_auth();

        let key = DataKey::Workflow(id);

        if env.storage().persistent().has(&key) {
            panic!("workflow already exists");
        }

        let workflow = Workflow {
            id,
            payer,
            participant,
            minimum_required_score,
            status: WorkflowStatus::Pending,
        };

        env.storage().persistent().set(&key, &workflow);

        workflow
    }

    /// Stores a proof hash for the workflow.
    ///
    /// The actual ZK proof is generated off-chain. For the hackathon MVP,
    /// this function links a proof reference/hash to a Stellar workflow.
    pub fn submit_proof_hash(
        env: Env,
        id: u64,
        participant: Address,
        proof_hash: BytesN<32>,
    ) -> Workflow {
        participant.require_auth();

        let key = DataKey::Workflow(id);

        let mut workflow: Workflow = env
            .storage()
            .persistent()
            .get(&key)
            .expect("workflow not found");

        if workflow.participant != participant {
            panic!("only participant can submit proof");
        }

        workflow.status = WorkflowStatus::ProofSubmitted;

        env.storage()
            .persistent()
            .set(&DataKey::ProofHash(id), &proof_hash);

        env.storage().persistent().set(&key, &workflow);

        workflow
    }

    /// Marks a workflow as verified.
    ///
    /// In the full version, this should be called only after the Noir /
    /// UltraHonk / Groth16 verifier confirms the proof.
    ///
    /// For the hackathon demo, this shows the workflow state transition
    /// after proof verification.
    pub fn mark_verified(env: Env, id: u64, verifier: Address) -> Workflow {
        verifier.require_auth();

        let key = DataKey::Workflow(id);

        let mut workflow: Workflow = env
            .storage()
            .persistent()
            .get(&key)
            .expect("workflow not found");

        workflow.status = WorkflowStatus::Verified;

        env.storage().persistent().set(&key, &workflow);

        workflow
    }

    /// Marks the workflow as eligible for future Stellar USDC settlement.
    pub fn mark_eligible_for_settlement(
        env: Env,
        id: u64,
        payer: Address,
    ) -> Workflow {
        payer.require_auth();

        let key = DataKey::Workflow(id);

        let mut workflow: Workflow = env
            .storage()
            .persistent()
            .get(&key)
            .expect("workflow not found");

        if workflow.payer != payer {
            panic!("only payer can mark settlement eligibility");
        }

        if workflow.status != WorkflowStatus::Verified {
            panic!("workflow must be verified first");
        }

        workflow.status = WorkflowStatus::EligibleForSettlement;

        env.storage().persistent().set(&key, &workflow);

        workflow
    }

    /// Reads a workflow by ID.
    pub fn get_workflow(env: Env, id: u64) -> Workflow {
        env.storage()
            .persistent()
            .get(&DataKey::Workflow(id))
            .expect("workflow not found")
    }

    /// Reads the submitted proof hash for a workflow.
    pub fn get_proof_hash(env: Env, id: u64) -> BytesN<32> {
        env.storage()
            .persistent()
            .get(&DataKey::ProofHash(id))
            .expect("proof hash not found")
    }
}
