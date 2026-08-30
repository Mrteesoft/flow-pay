use anyhow::{anyhow, Context};
use flowpay_domain::ChainKey;
use std::{collections::HashMap, env, str::FromStr};

#[derive(Clone, Debug)]
pub struct ChainConfig {
    pub chain: ChainKey,
    pub rpc_url: String,
    pub numeric_chain_id: u64,
    pub factory_address: String,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub bind: String,
    pub database_url: String,
    pub environment: String,
    pub checkout_base_url: String,
    pub api_key_pepper: String,
    pub proxy_creation_code_hash: String,
    pub factory_runtime_code_hash: Option<String>,
    pub operator_address: String,
    pub faucet_address: Option<String>,
    pub evidence_dir: String,
    pub webhook_encryption_key: Vec<u8>,
    pub chains: HashMap<ChainKey, ChainConfig>,
    pub agent_mode: String,
    pub model_provider: String,
    pub openai_api_key: Option<String>,
    pub openai_model: String,
    pub openai_endpoint: String,
    pub agent_max_steps: usize,
    pub agent_retry_budget: usize,
    pub rabbitmq_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let factory = required("FLOWPAY_FACTORY_ADDRESS")?;
        let mut chains = HashMap::new();
        chains.insert(
            ChainKey::Base,
            ChainConfig {
                chain: ChainKey::Base,
                rpc_url: env::var("BASE_RPC_URL")
                    .unwrap_or_else(|_| "http://127.0.0.1:8545".into()),
                numeric_chain_id: parse_u64("BASE_CHAIN_ID", 31337)?,
                factory_address: factory.clone(),
            },
        );
        if let Ok(url) = env::var("BSC_RPC_URL") {
            chains.insert(
                ChainKey::Bsc,
                ChainConfig {
                    chain: ChainKey::Bsc,
                    rpc_url: url,
                    numeric_chain_id: parse_u64("BSC_CHAIN_ID", 31338)?,
                    factory_address: factory,
                },
            );
        }
        let model_provider = parse_model_provider()?;
        let default_model = if model_provider == "ollama" {
            "qwen2.5-coder:7b"
        } else {
            "gpt-5"
        };
        let default_endpoint = if model_provider == "ollama" {
            "http://127.0.0.1:11434/api/chat"
        } else {
            "https://api.openai.com/v1/responses"
        };
        Ok(Self {
            bind: env::var("FLOWPAY_BIND").unwrap_or_else(|_| "0.0.0.0:8080".into()),
            database_url: required("DATABASE_URL")?,
            environment: env::var("FLOWPAY_ENV").unwrap_or_else(|_| "local".into()),
            checkout_base_url: env::var("FLOWPAY_CHECKOUT_BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3001".into())
                .trim_end_matches('/')
                .to_owned(),
            api_key_pepper: required("FLOWPAY_API_KEY_HASH_PEPPER")?,
            proxy_creation_code_hash: required("FLOWPAY_PROXY_CREATION_CODE_HASH")?,
            factory_runtime_code_hash: env::var("FLOWPAY_FACTORY_RUNTIME_CODE_HASH").ok(),
            operator_address: required("FLOWPAY_OPERATOR_ADDRESS")?,
            faucet_address: env::var("FLOWPAY_FAUCET_ADDRESS").ok(),
            evidence_dir: env::var("FLOWPAY_EVIDENCE_DIR")
                .unwrap_or_else(|_| "./runtime/evidence".into()),
            webhook_encryption_key: parse_hex32("FLOWPAY_WEBHOOK_ENCRYPTION_KEY")?.to_vec(),
            chains,
            agent_mode: parse_agent_mode()?,
            model_provider,
            openai_api_key: env::var("OPENAI_API_KEY")
                .ok()
                .filter(|v| !v.trim().is_empty()),
            openai_model: env::var("FLOWPAY_AGENT_MODEL").unwrap_or_else(|_| default_model.into()),
            openai_endpoint: env::var("FLOWPAY_MODEL_ENDPOINT")
                .or_else(|_| env::var("FLOWPAY_OPENAI_RESPONSES_URL"))
                .unwrap_or_else(|_| default_endpoint.into()),
            agent_max_steps: parse_usize("FLOWPAY_AGENT_MAX_STEPS", 12)?,
            agent_retry_budget: parse_usize("FLOWPAY_AGENT_RETRY_BUDGET", 3)?.clamp(1, 10),
            rabbitmq_url: env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://guest:guest@127.0.0.1:5672/%2f".into()),
        })
    }
}
fn required(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("missing {key}"))
}
fn parse_u64(key: &str, default: u64) -> anyhow::Result<u64> {
    match env::var(key) {
        Ok(v) => u64::from_str(&v).map_err(|e| anyhow!("invalid {key}: {e}")),
        Err(_) => Ok(default),
    }
}
fn parse_usize(key: &str, default: usize) -> anyhow::Result<usize> {
    match env::var(key) {
        Ok(v) => usize::from_str(&v).map_err(|e| anyhow!("invalid {key}: {e}")),
        Err(_) => Ok(default),
    }
}

fn parse_hex32(key: &str) -> anyhow::Result<[u8; 32]> {
    let value = required(key)?;
    let bytes = hex::decode(value.trim_start_matches("0x"))
        .with_context(|| format!("{key} must be hex"))?;
    bytes
        .try_into()
        .map_err(|_| anyhow!("{key} must be 32 bytes"))
}

fn parse_agent_mode() -> anyhow::Result<String> {
    let value = env::var("FLOWPAY_AGENT_MODE")
        .unwrap_or_else(|_| "model".into())
        .to_ascii_lowercase();
    match value.as_str() {
        "model" | "deterministic" | "baseline" => Ok(value),
        _ => Err(anyhow!(
            "FLOWPAY_AGENT_MODE must be model, deterministic, or baseline"
        )),
    }
}
fn parse_model_provider() -> anyhow::Result<String> {
    let value = env::var("FLOWPAY_MODEL_PROVIDER")
        .unwrap_or_else(|_| "openai".into())
        .to_ascii_lowercase();
    match value.as_str() {
        "openai" | "ollama" => Ok(value),
        _ => Err(anyhow!("FLOWPAY_MODEL_PROVIDER must be openai or ollama")),
    }
}
