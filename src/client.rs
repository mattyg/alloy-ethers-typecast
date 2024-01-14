use ethers::middleware::gas_oracle::{GasOracleMiddleware, ProviderOracle};
use ethers::middleware::{MiddlewareBuilder, SignerMiddleware};
use ethers::prelude::{Http, Provider};
use ethers_signers::{HDPath, Ledger};

pub struct LedgerClient {
    pub client: GasOracleMiddleware<
        SignerMiddleware<Provider<Http>, Ledger>,
        ProviderOracle<Provider<Http>>,
    >,
}

impl LedgerClient {
    pub async fn new(
        ledger_derivation_path: Option<usize>,
        chain_id: u64,
        rpc_url: String,
    ) -> anyhow::Result<Self> {
        let wallet = Ledger::new(
            HDPath::LedgerLive(ledger_derivation_path.unwrap_or(0)),
            chain_id,
        )
        .await?;
        let provider = Provider::<Http>::try_from(rpc_url.clone())?;
        let client = provider
            .clone()
            .with_signer(wallet)
            .wrap_into(|s| GasOracleMiddleware::new(s, ProviderOracle::new(provider)));

        Ok(Self { client })
    }
}
