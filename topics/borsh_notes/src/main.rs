use borsh::{BorshDeserialize, BorshSerialize};
use std::io::Result;

mod exercises;
/// 1. Basic Struct Serialization and Deserialization

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct User {
    id: u32,
    username: String,
    active: bool,
}

fn _basic_borsh_example() -> Result<()> {
    let user = User {
        id: 1,
        username: "romy_solana".to_string(),
        active: true,
    };

    // Serialize the struct to bytes
    let serialized = user.try_to_vec()?; // Vec<u8>

    // Deserialize back to struct
    let deserialized = User::try_from_slice(&serialized)?;

    println!("Deserialized: {:?}", deserialized);
    Ok(())
}

/// 2. Borsh with Enums (Important: Enum variants must be defined explicitly)

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum AccountType {
    Uninitialized,
    Basic { balance: u64 },
    Premium { balance: u64, expires: u64 },
}

/// 3. Nesting Structs and Using Option<T>

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Profile {
    user: User,
    nickname: Option<String>, // Borsh supports Option
}

/// 4. Manual Byte Parsing (Simulating Solana Account Data)

fn _simulate_solana_account_parsing() -> Result<()> {
    // Imagine this is account data from Solana runtime
    let account_data = Profile {
        user: User {
            id: 42,
            username: "solana_dev".to_string(),
            active: true,
        },
        nickname: Some("audit_warrior".to_string()),
    }
    .try_to_vec()?; // Send this to a Solana account

    // Deserialize manually from byte slice (as Solana programs do)
    let data: &[u8] = &account_data;
    let parsed = Profile::try_from_slice(data)?;
    println!("Parsed from account data: {:?}", parsed);
    Ok(())
}

/// 5. How Solana Programs Use Borsh

/// Imagine this struct is stored inside a Solana account.
/// You read the account's `.data` bytes and deserialize.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VaultState {
    pub owner: [u8; 32],    // Pubkey as byte array
    pub total_deposit: u64, // lamports deposited
    pub is_active: bool,    // flag
}

fn _simulate_solana_vault() -> Result<()> {
    let vault = VaultState {
        owner: [0u8; 32],
        total_deposit: 1_000_000,
        is_active: true,
    };

    let raw = vault.try_to_vec()?;
    let restored = VaultState::try_from_slice(&raw)?;
    println!("VaultState roundtrip: {:?}", restored);
    Ok(())
}

/// 6. Common Pitfalls
/// - Borsh does not store field names. Just order matters.
/// - Adding/removing fields without migration breaks deserialization.
/// - No versioning: handle migrations yourself.

fn main() -> Result<()> {
    // println!("Running Borsh examples...");

    // basic_borsh_example()?;
    // simulate_solana_account_parsing()?;
    // simulate_solana_vault()?;
    let _ = exercises::ex1();
    let _ = exercises::ex2();
    Ok(())
}


//Exercises