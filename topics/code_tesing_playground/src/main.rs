use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

fn main() {
    let rpc = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let pubkey = Pubkey::from_str("AeeLowqmEW3wyaJbPY3BUqRLvZHRrnqutJcsFfL2x9pt").unwrap();
    let bal = rpc.get_balance(&pubkey).unwrap();
    println!("Balance is : {}", bal);
}
