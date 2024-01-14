use ethers::middleware::gas_oracle::{BlockNative, GasCategory, GasOracleMiddleware};
use ethers::middleware::{MiddlewareBuilder, SignerMiddleware};
use ethers::prelude::{Http, Provider};
use ethers_signers::{HDPath, Ledger};

pub struct LedgerClient {
    pub client: SignerMiddleware<GasOracleMiddleware<Provider<Http>, BlockNative>, Ledger>,
}

impl LedgerClient {
    pub async fn new(
        ledger_derivation_path: Option<usize>,
        chain_id: u64,
        rpc_url: String,
        gas_priority: Option<GasCategory>,
    ) -> anyhow::Result<Self> {
        let wallet = Ledger::new(
            HDPath::LedgerLive(ledger_derivation_path.unwrap_or(0)),
            chain_id,
        )
        .await?;

        let api_key: Option<String> = std::env::var("BLOCK_NATIVE_API_KEY").ok();
        let blocknative =
            BlockNative::new(api_key).category(gas_priority.unwrap_or(GasCategory::Standard));

        let provider = Provider::<Http>::try_from(rpc_url.clone())?;
        let client = provider
            .clone()
            .wrap_into(|s| GasOracleMiddleware::new(s, blocknative))
            .with_signer(wallet);

        Ok(Self { client })
    }
}
