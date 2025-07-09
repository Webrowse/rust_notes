//Exercises:

// Serialize and deserialize the User struct, compare with original.

// Serialize a Vec<User>, deserialize, confirm data integrity.

// Create and serialize all variants of AccountType, deserialize each, assert match.

// Nest User in Profile, use None and Some, serialize both, deserialize, confirm values.

// Mutate one field in Profile, serialize, deserialize, confirm only the intended field changed.

// Corrupt serialized VaultState by removing bytes, attempt deserialization, observe error.

// Manually parse partial byte slice of Profile, confirm failure, understand boundary sensitivity.

// Extend VaultState with a new field, serialize with new, deserialize with old, observe failure.

// Serialize using Borsh, deserialize using Serde (or vice versa), confirm incompatibility.

// Write a function taking [u8] as input, returning a User, usable in Solana processor logic.