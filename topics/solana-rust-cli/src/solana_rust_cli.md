# 🧠 Solana Rust Client CLI – Core Notes

This document explains how to build a **Rust CLI** that interacts with a Solana smart contract (written in Anchor).

Your CLI will:
- Derive PDAs
- Load wallet
- Build and send instructions
- Use Solana RPC to talk to devnet

---

## 🧱 High-Level Structure

- On-chain Program: Written in Anchor (`lib.rs`)
- Off-chain CLI: Written in Rust (`vault-cli/main.rs`)
- Communication: JSON-RPC using `solana-client`

---

## 📦 Crates You'll Use

| Crate            | Purpose                                |
|------------------|----------------------------------------|
| `clap`           | CLI argument parsing                   |
| `solana-sdk`     | Keypair, Pubkey, Instruction, Tx       |
| `solana-client`  | Connect to devnet, send tx             |
| `solana-program` | Access `system_program::ID` etc.       |
| `bincode`        | Serialize instruction args             |
| `anyhow`         | Handle errors cleanly                  |
| `dirs`           | Get user's home directory              |

---

## 🔐 1. Load Wallet Keypair

```rust
use solana_sdk::signature::read_keypair_file;

let payer = read_keypair_file("~/.config/solana/id.json")?;
```

- This loads your default keypair used by `solana-cli`
- You'll use this to sign every transaction

---

## 🌐 2. Connect to Devnet RPC

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

let rpc = RpcClient::new_with_commitment(
    "https://api.devnet.solana.com".to_string(),
    CommitmentConfig::confirmed(),
);
```

- All on-chain interactions (get blockhash, send tx) use this client

---

## 🧠 3. Derive PDA (Program Derived Address)

```rust
use solana_sdk::pubkey::Pubkey;

let (vault_pda, bump) = Pubkey::find_program_address(
    &[b"vault", user_pubkey.as_ref()],
    &program_id,
);
```

- PDAs are special addresses **owned by your program**
- You use a PDA to create a **user-specific vault account**
- PDAs are predictable and verifiable

---

## 🛠️ 4. Build an Instruction

```rust
use solana_sdk::instruction::{AccountMeta, Instruction};

let instruction = Instruction {
    program_id,
    accounts: vec![
        AccountMeta::new(vault_pda, false),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new_readonly(system_program::ID, false),
    ],
    data: instruction_data, // e.g. encoded "initialize" call
};
```

---

## 🎛️ 5. Encode Anchor Instruction Data

Anchor expects the instruction like this:

```
<discriminator: [u8; 8]> + <args: bincode>
```

Example:

```rust
use anchor_lang::InstructionData;
use bincode::serialize;

// Optional if using Anchor's derive macros
let ix_data = YourInstruction { arg1, arg2 }.data(); 

// Manual version:
let mut data = vec![];
let discriminator = anchor_lang::solana_program::hash::hashv(&[b"global:initialize"]);
data.extend_from_slice(&discriminator.to_bytes()[..8]);
data.extend_from_slice(&bincode::serialize(&YourArgsStruct {})?);
```

---

## 🚀 6. Send Transaction

```rust
use solana_sdk::{transaction::Transaction, message::Message};

let tx = Transaction::new_signed_with_payer(
    &[instruction],
    Some(&payer.pubkey()),
    &[&payer],
    rpc.get_latest_blockhash()?,
);

rpc.send_and_confirm_transaction(&tx)?;
```

---

## ✅ Summary: Full Client Flow

1. Load wallet keypair from `id.json`
2. Connect to Solana RPC
3. Derive PDA for vault
4. Create instruction with encoded data
5. Sign and send transaction
6. Confirm result or handle error

---

## 💡 Tips

- Use `println!()` to debug PDAs and pubkeys
- Keep your program ID in a `.env` or constants file
- Log instructions using `rpc.simulate_transaction()` for debug

---

## 🧠 Example Flow for `vault-cli init`

- Load wallet
- Derive `vault` PDA using `[b"vault", user_pubkey]`
- Build `initialize` instruction with `payer`, `vault_pda`, `system_program`
- Encode `initialize` using Anchor's discriminator
- Send transaction to devnet

This is how every CLI wallet, game, faucet, and DeFi client works under the hood.

---
