// vault_cli.rs

// -------------------- Dependencies --------------------
use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
    pubkey::Pubkey,
    system_program,
    instruction::{Instruction, AccountMeta},
    transaction::Transaction,
    message::Message,
};
use anchor_lang::{InstructionData, solana_program::hash::hashv};
use bincode::serialize;
use anyhow::Result;

// -------------------- CLI Definition --------------------
#[derive(Parser, Debug)]
#[command(name = "vault-cli")]
#[command(about = "Solana CLI client for Vault program")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
}

// -------------------- Constants --------------------
const PROGRAM_ID: &str = "AeeLowqmEW3wyaJbPY3BUqRLvZHRrnqutJcsFfL2x9pt"; // replace with actual
const RPC_URL: &str = "https://api.devnet.solana.com";

// -------------------- Entry --------------------
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.commands {
        Commands::Init => initialize_vault()?,
    }
    Ok(())
}

// -------------------- Load Wallet --------------------
fn load_keypair() -> Result<Keypair> {
    let path = dirs::home_dir()
        .expect("no home dir")
        .join(".config/solana/id.json");
    let payer = read_keypair_file(path)?;
    Ok(payer)
}

// -------------------- Initialize Vault --------------------
fn initialize_vault() -> Result<()> {
    let payer = load_keypair()?; // wallet signer
    let program_id = Pubkey::from(PROGRAM_ID)?; // program
    let user_pubkey = payer.pubkey();

    // connect to devnet
    let rpc = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

    // derive PDA: seeds = ["vault", user]
    let (vault_pda, _bump) = Pubkey::find_program_address(
        &[b"vault", user_pubkey.as_ref()],
        &program_id,
    );

    // prepare instruction data
    let mut data = vec![];

    // discriminator = first 8 bytes of hash of "global:initialize"
    let discriminator = hashv(&[b"global:initialize"]);
    data.extend_from_slice(&discriminator.to_bytes()[..8]);

    // encode args if any (here, none)
    let args = (); // replace with struct if needed
    data.extend_from_slice(&serialize(&args)?);

    // build instruction: program, accounts, data
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(vault_pda, false), // PDA
            AccountMeta::new(user_pubkey, true), // signer
            AccountMeta::new_readonly(system_program::ID, false), // system
        ],
        data,
    };

    // latest blockhash
    let blockhash = rpc.get_latest_blockhash()?;

    // create tx and sign
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&user_pubkey),
        &[&payer],
        blockhash,
    );

    // send tx
    let sig = rpc.send_and_confirm_transaction(&tx)?;
    println!("Transaction successful: {}", sig);
    Ok(())
}
