//Exercises:

use borsh::{BorshDeserialize, BorshSerialize};
use std::io::Result;

// 1. Serialize and deserialize the User struct, compare with original.
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct BorshBasic {
    name: String,
    age: u8,
}
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
struct User {
    id: u8,
    username: String,
    is_active: bool,
}
pub fn ex1() -> Result<()> {
    let borsh_basic = BorshBasic {
        name: "Adarsh".to_string(),
        age: 22,
    };
    let borsh_serial = borsh_basic.try_to_vec()?;
    let borsh_deserial = BorshBasic::try_from_slice(&borsh_serial)?;
    println!("Exercise 1:\nRound trip: {:?}", borsh_deserial);
    Ok(())
}

// 2. Serialize a Vec<User>, deserialize, confirm data integrity.
pub fn ex2() -> Result<()> {
    let vec2: Vec<User> = vec![
        User {
            id: 01,
            username: "exercise2".to_string(),
            is_active: true,
        },
        User {
            id: 02,
            username: "exe2".to_string(),
            is_active: true,
        },
    ];

    let ser_vec_user = vec2.try_to_vec()?;
    let des_vec_user: Vec<User> = Vec::<User>::try_from_slice(&ser_vec_user)?;

    assert_eq!(vec2, des_vec_user);
    println!("Exercise 2 passed");
    Ok(())
}
// 3. Create and serialize all variants of AccountType, deserialize each, assert match.
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum AccountType {
    Uninitialized,
    Basic { bal: u64 },
    Premium { bal: u64, expires: u64 },
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct AtSerDes {
    type_of_account: AccountType,
}

pub fn ex3() -> Result<()> {
    let user1 = AtSerDes {
        type_of_account: AccountType::Uninitialized,
    };
    let ser_un = user1.try_to_vec()?;
    let der_un = AtSerDes::try_from_slice(&ser_un)?;
    assert_eq!(user1, der_un);

    let user2 = AtSerDes {
        type_of_account: AccountType::Basic { bal: 2 },
    };
    let ser_un = user2.try_to_vec()?;
    let der_un = AtSerDes::try_from_slice(&ser_un)?;
    assert_eq!(user2, der_un);

    let user3 = AtSerDes {
        type_of_account: AccountType::Premium {
            bal: 3,
            expires: 365,
        },
    };
    let ser_un = user3.try_to_vec()?;
    let der_un = AtSerDes::try_from_slice(&ser_un)?;
    assert_eq!(user3, der_un);

    println!("Exercise 3 passed");

    Ok(())
}

// 4. Nest User in Profile, use None and Some, serialize both, deserialize, confirm values.

#[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq)]
struct Profile {
    user: User,
    nickname: Option<String>,
}
pub fn ex4() -> Result<()> {
    let user1 = Profile {
        user: User {
            id: 01,
            username: "F_name".to_string(),
            is_active: true,
        },
        nickname: Some("Frisky".to_string()),
    };

    let ser1 = user1.try_to_vec()?;
    let des1 = Profile::try_from_slice(&ser1)?;
    assert_eq!(user1, des1);

    let user2 = Profile {
        user: User {
            id: 01,
            username: "F_name".to_string(),
            is_active: true,
        },
        nickname: None,
    };

    let ser2 = user2.try_to_vec()?;
    let des2 = Profile::try_from_slice(&ser2)?;
    assert_eq!(user2, des2);
    println!("Exercise 4 passed");
    Ok(())
}
// 5. Mutate one field in Profile, serialize, deserialize, confirm only the intended field changed.

// 6. Corrupt serialized VaultState by removing bytes, attempt deserialization, observe error.

// 7. Manually parse partial byte slice of Profile, confirm failure, understand boundary sensitivity.

// 8. Extend VaultState with a new field, serialize with new, deserialize with old, observe failure.

// 9. Serialize using Borsh, deserialize using Serde (or vice versa), confirm incompatibility.

// 10. Write a function taking [u8] as input, returning a User, usable in Solana processor logic.
