use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::bundles::RpcSimulateBundleConfig;
use solana_sdk::bundle::VersionedBundle;
use dotenv::{dotenv, var};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
		dotenv()?;
		let rpc_url = var("RPC_URL")?;
    let rpc_client = RpcClient::new(rpc_url);
 
    let version = rpc_client.get_version().await.unwrap();
    println!("running version {}", version.solana_core);

    let identity= rpc_client.get_identity().await.unwrap();
    println!("running version {}", identity);

    println!("running simulate_bundle");
    let res = rpc_client
        .simulate_bundle_with_config(
            &VersionedBundle {
                transactions: vec![],
            },
            RpcSimulateBundleConfig {
                pre_execution_accounts_configs: vec![],
                post_execution_accounts_configs: vec![],
                transaction_encoding: None,
                simulation_bank: None,
                skip_sig_verify: true,
                replace_recent_blockhash: true,
            },
        )
        .await.unwrap();
      println!("result {}", res);

      return Ok(());
}
