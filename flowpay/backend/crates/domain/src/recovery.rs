use crate::{AddressRef, AtomicAmount, ChainKey, ClaimId, PaymentId, RecoveryPlanId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SimulationStatus {
    NotRun,
    Succeeded,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskFlag {
    CustodialSender,
    FactoryMismatch,
    UnsupportedTokenBehavior,
    BalanceChanged,
    RpcInconsistency,
    AmbiguousOwnership,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecoveryPolicyDecision {
    Allowed,
    Denied,
    RequiresEscalation,
    NeedsFunding,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlan {
    pub id: RecoveryPlanId,
    pub claim_id: ClaimId,
    pub payment_id: PaymentId,
    pub source_chain: ChainKey,
    pub token_contract: Option<String>,
    pub asset_symbol: String,
    pub amount: AtomicAmount,
    pub checkout_address: AddressRef,
    pub recovery_destination: AddressRef,
    pub receiver_deployment_required: bool,
    pub estimated_gas_atomic: AtomicAmount,
    pub policy_version: String,
    pub policy_decision: RecoveryPolicyDecision,
    pub simulation_status: SimulationStatus,
    pub risk_flags: Vec<RiskFlag>,
    pub required_approval: bool,
}
