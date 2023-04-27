
//! Autogenerated weights for `pallet_guild_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 19.0.0
//! DATE: 2023-04-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `turbineblade`, CPU: `AMD Ryzen 5 3600 6-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/gn-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_guild_identity
// --extrinsic
// *
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./gn-pallets/pallet-guild-identity/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn register() -> Weight;
	fn deregister() -> Weight;
	fn authorize() -> Weight;
	fn link_address() -> Weight;
	fn unlink_address() -> Weight;
	fn remove_addresses() -> Weight;
	fn link_identity() -> Weight;
	fn unlink_identity() -> Weight;
}

/// Weight functions for `pallet_guild_identity`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: GuildIdentity Authorities (r:1 w:1)
	/// Proof Skipped: GuildIdentity Authorities (max_values: None, max_size: None, mode: Measured)
	/// Storage: GuildIdentity Identities (r:0 w:1)
	/// Proof Skipped: GuildIdentity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: GuildIdentity Addresses (r:0 w:1)
	/// Proof Skipped: GuildIdentity Addresses (max_values: None, max_size: None, mode: Measured)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `2493`
		// Minimum execution time: 20_398 nanoseconds.
		Weight::from_parts(21_100_000, 2493)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: GuildIdentity Authorities (r:1 w:1)
	/// Proof Skipped: GuildIdentity Authorities (max_values: None, max_size: None, mode: Measured)
	/// Storage: GuildIdentity Identities (r:0 w:1)
	/// Proof Skipped: GuildIdentity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: GuildIdentity Addresses (r:0 w:1)
	/// Proof Skipped: GuildIdentity Addresses (max_values: None, max_size: None, mode: Measured)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `3018`
		// Minimum execution time: 24_576 nanoseconds.
		Weight::from_parts(25_047_000, 3018)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: GuildIdentity Authorities (r:1 w:1)
	/// Proof Skipped: GuildIdentity Authorities (max_values: None, max_size: None, mode: Measured)
	fn authorize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `248`
		//  Estimated: `2723`
		// Minimum execution time: 17_573 nanoseconds.
		Weight::from_parts(18_024_000, 2723)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: GuildIdentity Authorities (r:1 w:0)
	/// Proof Skipped: GuildIdentity Authorities (max_values: None, max_size: None, mode: Measured)
	/// Storage: GuildIdentity Addresses (r:1 w:1)
	/// Proof Skipped: GuildIdentity Addresses (max_values: None, max_size: None, mode: Measured)
	fn link_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `5520`
		// Minimum execution time: 2_923_976 nanoseconds.
		Weight::from_parts(2_945_867_000, 5520)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: GuildIdentity Addresses (r:1 w:1)
	/// Proof Skipped: GuildIdentity Addresses (max_values: None, max_size: None, mode: Measured)
	fn unlink_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `225`
		//  Estimated: `2700`
		// Minimum execution time: 19_607 nanoseconds.
		Weight::from_parts(20_398_000, 2700)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: GuildIdentity Addresses (r:1 w:1)
	/// Proof Skipped: GuildIdentity Addresses (max_values: None, max_size: None, mode: Measured)
	fn remove_addresses() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `2765`
		// Minimum execution time: 20_448 nanoseconds.
		Weight::from_parts(21_210_000, 2765)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: GuildIdentity Identities (r:1 w:0)
	/// Proof Skipped: GuildIdentity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Oracle ActiveOperators (r:1 w:0)
	/// Proof Skipped: Oracle ActiveOperators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle NextOperator (r:1 w:1)
	/// Proof Skipped: Oracle NextOperator (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle NextRequestIdentifier (r:1 w:1)
	/// Proof Skipped: Oracle NextRequestIdentifier (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle Requests (r:0 w:1)
	/// Proof Skipped: Oracle Requests (max_values: None, max_size: None, mode: Measured)
	fn link_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `267`
		//  Estimated: `5295`
		// Minimum execution time: 32_882 nanoseconds.
		Weight::from_parts(33_834_000, 5295)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: GuildIdentity Identities (r:1 w:1)
	/// Proof Skipped: GuildIdentity Identities (max_values: None, max_size: None, mode: Measured)
	fn unlink_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `187`
		//  Estimated: `2662`
		// Minimum execution time: 18_815 nanoseconds.
		Weight::from_parts(19_527_000, 2662)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

impl WeightInfo for () {
	/// Storage: GuildIdentity Authorities (r:1 w:1)
	/// Proof Skipped: GuildIdentity Authorities (max_values: None, max_size: None, mode: Measured)
	/// Storage: GuildIdentity Identities (r:0 w:1)
	/// Proof Skipped: GuildIdentity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: GuildIdentity Addresses (r:0 w:1)
	/// Proof Skipped: GuildIdentity Addresses (max_values: None, max_size: None, mode: Measured)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `2493`
		// Minimum execution time: 20_398 nanoseconds.
		Weight::from_parts(21_100_000, 2493)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	/// Storage: GuildIdentity Authorities (r:1 w:1)
	/// Proof Skipped: GuildIdentity Authorities (max_values: None, max_size: None, mode: Measured)
	/// Storage: GuildIdentity Identities (r:0 w:1)
	/// Proof Skipped: GuildIdentity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: GuildIdentity Addresses (r:0 w:1)
	/// Proof Skipped: GuildIdentity Addresses (max_values: None, max_size: None, mode: Measured)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `3018`
		// Minimum execution time: 24_576 nanoseconds.
		Weight::from_parts(25_047_000, 3018)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	/// Storage: GuildIdentity Authorities (r:1 w:1)
	/// Proof Skipped: GuildIdentity Authorities (max_values: None, max_size: None, mode: Measured)
	fn authorize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `248`
		//  Estimated: `2723`
		// Minimum execution time: 17_573 nanoseconds.
		Weight::from_parts(18_024_000, 2723)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: GuildIdentity Authorities (r:1 w:0)
	/// Proof Skipped: GuildIdentity Authorities (max_values: None, max_size: None, mode: Measured)
	/// Storage: GuildIdentity Addresses (r:1 w:1)
	/// Proof Skipped: GuildIdentity Addresses (max_values: None, max_size: None, mode: Measured)
	fn link_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `5520`
		// Minimum execution time: 2_923_976 nanoseconds.
		Weight::from_parts(2_945_867_000, 5520)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: GuildIdentity Addresses (r:1 w:1)
	/// Proof Skipped: GuildIdentity Addresses (max_values: None, max_size: None, mode: Measured)
	fn unlink_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `225`
		//  Estimated: `2700`
		// Minimum execution time: 19_607 nanoseconds.
		Weight::from_parts(20_398_000, 2700)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: GuildIdentity Addresses (r:1 w:1)
	/// Proof Skipped: GuildIdentity Addresses (max_values: None, max_size: None, mode: Measured)
	fn remove_addresses() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `2765`
		// Minimum execution time: 20_448 nanoseconds.
		Weight::from_parts(21_210_000, 2765)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: GuildIdentity Identities (r:1 w:0)
	/// Proof Skipped: GuildIdentity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Oracle ActiveOperators (r:1 w:0)
	/// Proof Skipped: Oracle ActiveOperators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle NextOperator (r:1 w:1)
	/// Proof Skipped: Oracle NextOperator (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle NextRequestIdentifier (r:1 w:1)
	/// Proof Skipped: Oracle NextRequestIdentifier (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle Requests (r:0 w:1)
	/// Proof Skipped: Oracle Requests (max_values: None, max_size: None, mode: Measured)
	fn link_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `267`
		//  Estimated: `5295`
		// Minimum execution time: 32_882 nanoseconds.
		Weight::from_parts(33_834_000, 5295)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	/// Storage: GuildIdentity Identities (r:1 w:1)
	/// Proof Skipped: GuildIdentity Identities (max_values: None, max_size: None, mode: Measured)
	fn unlink_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `187`
		//  Estimated: `2662`
		// Minimum execution time: 18_815 nanoseconds.
		Weight::from_parts(19_527_000, 2662)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}
