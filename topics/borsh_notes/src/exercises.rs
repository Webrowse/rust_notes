//Exercises:

use borsh::{BorshDeserialize, BorshSerialize};
use std::io::Result;

// use borsh_derive;
// Serialize and deserialize the User struct, compare with original.
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct BorshBasic{
    name: String,
    age: u8,
}
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
struct User{
    id: u8,
    username: String,
    is_active: bool,
}
pub fn ex1()-> Result<()>{
    let borsh_basic = BorshBasic{name: "Adarsh".to_string(), age: 22 };
    let borsh_serial = borsh_basic.try_to_vec()?;
    let borsh_deserial = BorshBasic::try_from_slice(&borsh_serial)?;
    println!("Exercise 1:\nRound trip: {:?}", borsh_deserial);
    Ok(())
}
// Serialize a Vec<User>, deserialize, confirm data integrity.
pub fn ex2() -> Result<()> {
    let vec2: Vec<User> = vec![
    User{ id: 01, username: "exercise2".to_string(), is_active: true},
    User{ id: 02, username: "exe2".to_string(), is_active: true},
    ];
    
    let ser_vec_user = vec2.try_to_vec()?;
    let des_vec_user: Vec<User> = Vec::<User>::try_from_slice(&ser_vec_user)?;
    // println!("Exercise 2:\n{:?}, {:?}", ser_vec_user, des_vec_user);

    assert_eq!(vec2, des_vec_user);
    
    Ok(())
}
// Create and serialize all variants of AccountType, deserialize each, assert match.

// Nest User in Profile, use None and Some, serialize both, deserialize, confirm values.

// Mutate one field in Profile, serialize, deserialize, confirm only the intended field changed.

// Corrupt serialized VaultState by removing bytes, attempt deserialization, observe error.

// Manually parse partial byte slice of Profile, confirm failure, understand boundary sensitivity.

// Extend VaultState with a new field, serialize with new, deserialize with old, observe failure.

// Serialize using Borsh, deserialize using Serde (or vice versa), confirm incompatibility.

// Write a function taking [u8] as input, returning a User, usable in Solana processor logic.