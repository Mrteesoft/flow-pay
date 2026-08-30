use async_trait::async_trait;
use flowpay_chains::PreparedTransaction;
use flowpay_domain::{ApprovalId, RecoveryPlanId};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionClass {
    DeployCheckout,
    RecoverErc20,
    RecoverNative,
    SettleErc20,
    SettleNative,
}

impl TransactionClass {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeployCheckout => "DEPLOY_CHECKOUT",
            Self::RecoverErc20 => "RECOVER_ERC20",
            Self::RecoverNative => "RECOVER_NATIVE",
            Self::SettleErc20 => "SETTLE_ERC20",
            Self::SettleNative => "SETTLE_NATIVE",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApprovedSignerRequest {
    pub plan_id: RecoveryPlanId,
    pub approval_id: ApprovalId,
    pub approved_plan_hash: String,
    pub computed_plan_hash: String,
    pub approval_reserved_for_execution: bool,
    pub transaction_class: TransactionClass,
    pub transaction: PreparedTransaction,
    pub expected_factory: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettlementSignerRequest {
    pub payment_id: flowpay_domain::PaymentId,
    pub transaction_class: TransactionClass,
    pub transaction: PreparedTransaction,
    pub expected_factory: String,
    pub settlement_destination: String,
    pub configured_merchant_destination: String,
}

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("transaction class is not allowlisted")]
    ClassDenied,
    #[error("approved plan hash mismatch")]
    PlanHashMismatch,
    #[error("approval was not atomically reserved for this execution")]
    ApprovalNotReserved,
    #[error("transaction target is not the configured factory")]
    WrongTarget,
    #[error("transaction carries native value when class forbids it")]
    UnexpectedValue,
    #[error("settlement destination does not match merchant configuration")]
    WrongSettlementDestination,
    #[error("rpc error: {0}")]
    Rpc(String),
}

#[derive(Clone)]
pub struct SignerPolicy {
    pub allowed_classes: BTreeSet<TransactionClass>,
    pub factory_address: String,
}

impl SignerPolicy {
    pub fn validate(&self, request: &ApprovedSignerRequest) -> Result<(), SignerError> {
        if !self.allowed_classes.contains(&request.transaction_class) {
            return Err(SignerError::ClassDenied);
        }
        if request.approved_plan_hash != request.computed_plan_hash {
            return Err(SignerError::PlanHashMismatch);
        }
        if !request.approval_reserved_for_execution {
            return Err(SignerError::ApprovalNotReserved);
        }
        if !request
            .transaction
            .to
            .eq_ignore_ascii_case(&self.factory_address)
            || !request
                .expected_factory
                .eq_ignore_ascii_case(&self.factory_address)
        {
            return Err(SignerError::WrongTarget);
        }
        if matches!(
            request.transaction_class,
            TransactionClass::DeployCheckout
                | TransactionClass::RecoverErc20
                | TransactionClass::SettleErc20
        ) && !request.transaction.value.is_zero()
        {
            return Err(SignerError::UnexpectedValue);
        }
        Ok(())
    }

    pub fn validate_settlement(
        &self,
        request: &SettlementSignerRequest,
    ) -> Result<(), SignerError> {
        if !matches!(
            request.transaction_class,
            TransactionClass::SettleErc20 | TransactionClass::SettleNative
        ) || !self.allowed_classes.contains(&request.transaction_class)
        {
            return Err(SignerError::ClassDenied);
        }
        if !request
            .transaction
            .to
            .eq_ignore_ascii_case(&self.factory_address)
            || !request
                .expected_factory
                .eq_ignore_ascii_case(&self.factory_address)
        {
            return Err(SignerError::WrongTarget);
        }
        if !request
            .settlement_destination
            .eq_ignore_ascii_case(&request.configured_merchant_destination)
        {
            return Err(SignerError::WrongSettlementDestination);
        }
        if matches!(request.transaction_class, TransactionClass::SettleErc20)
            && !request.transaction.value.is_zero()
        {
            return Err(SignerError::UnexpectedValue);
        }
        Ok(())
    }
}

#[async_trait]
pub trait RestrictedSigner: Send + Sync {
    async fn submit_recovery(&self, request: &ApprovedSignerRequest)
        -> Result<String, SignerError>;
    async fn submit_settlement(
        &self,
        request: &SettlementSignerRequest,
    ) -> Result<String, SignerError>;
}

/// Local/test-chain signer. It relies on an unlocked Anvil account but still applies the
/// same plan-hash, class and target restrictions before asking the node to submit a tx.
#[derive(Clone)]
pub struct DevUnlockedSigner {
    pub policy: SignerPolicy,
    pub rpc_url: String,
    pub from: String,
    client: reqwest::Client,
}

impl DevUnlockedSigner {
    #[must_use]
    pub fn new(policy: SignerPolicy, rpc_url: impl Into<String>, from: impl Into<String>) -> Self {
        Self {
            policy,
            rpc_url: rpc_url.into(),
            from: from.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl RestrictedSigner for DevUnlockedSigner {
    async fn submit_recovery(
        &self,
        request: &ApprovedSignerRequest,
    ) -> Result<String, SignerError> {
        self.policy.validate(request)?;
        let result: Value = self.client.post(&self.rpc_url).json(&json!({"jsonrpc":"2.0","id":1,"method":"eth_sendTransaction","params":[{"from":self.from,"to":request.transaction.to,"value":format!("0x{}", request.transaction.value.inner().to_str_radix(16)),"data":request.transaction.calldata_hex}]})).send().await.map_err(|e| SignerError::Rpc(e.to_string()))?.json().await.map_err(|e| SignerError::Rpc(e.to_string()))?;
        if let Some(error) = result.get("error") {
            return Err(SignerError::Rpc(error.to_string()));
        }
        result
            .get("result")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| SignerError::Rpc("missing tx hash".into()))
    }

    async fn submit_settlement(
        &self,
        request: &SettlementSignerRequest,
    ) -> Result<String, SignerError> {
        self.policy.validate_settlement(request)?;
        let result: Value = self.client.post(&self.rpc_url).json(&json!({"jsonrpc":"2.0","id":1,"method":"eth_sendTransaction","params":[{"from":self.from,"to":request.transaction.to,"value":format!("0x{}", request.transaction.value.inner().to_str_radix(16)),"data":request.transaction.calldata_hex}]})).send().await.map_err(|e| SignerError::Rpc(e.to_string()))?.json().await.map_err(|e| SignerError::Rpc(e.to_string()))?;
        if let Some(error) = result.get("error") {
            return Err(SignerError::Rpc(error.to_string()));
        }
        result
            .get("result")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| SignerError::Rpc("missing tx hash".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flowpay_domain::{AtomicAmount, ChainKey};
    use std::str::FromStr;
    #[test]
    fn rejects_replayed_or_wrong_target_requests() {
        let policy = SignerPolicy {
            allowed_classes: [TransactionClass::RecoverErc20].into_iter().collect(),
            factory_address: "0xfac".into(),
        };
        let mut request = ApprovedSignerRequest {
            plan_id: RecoveryPlanId::new(),
            approval_id: ApprovalId::new(),
            approved_plan_hash: "abc".into(),
            computed_plan_hash: "abc".into(),
            approval_reserved_for_execution: true,
            transaction_class: TransactionClass::RecoverErc20,
            transaction: PreparedTransaction {
                transaction_class: "RECOVER_ERC20".into(),
                chain: ChainKey::Bsc,
                to: "0xfac".into(),
                value: AtomicAmount::from_str("0").unwrap(),
                calldata_hex: "0x1234".into(),
            },
            expected_factory: "0xfac".into(),
        };
        assert!(policy.validate(&request).is_ok());
        request.approval_reserved_for_execution = false;
        assert!(matches!(
            policy.validate(&request),
            Err(SignerError::ApprovalNotReserved)
        ));
        request.approval_reserved_for_execution = true;
        request.transaction.to = "0xevil".into();
        assert!(matches!(
            policy.validate(&request),
            Err(SignerError::WrongTarget)
        ));
    }
}
