
//! Autogenerated weights for `pallet_verifier`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-12, STEPS: `40`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bundeMBP.lan`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_verifier
// --extrinsic
// *
// --steps
// 40
// --repeat
// 20
// --output
// ./pallets/verifier/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_verifier`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_verifier::WeightInfo for WeightInfo<T> {
	// Storage: Verifier Vkey (r:0 w:1)
	// Storage: Verifier PubSignal (r:0 w:1)
	fn set_zk_keys_benchmark() -> Weight {
		// Minimum execution time: 491_000 nanoseconds.
		Weight::from_ref_time(494_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Verifier PubSignal (r:1 w:0)
	// Storage: Verifier Vkey (r:1 w:0)
	fn verify_benchmark() -> Weight {
		// Minimum execution time: 3_650_000 nanoseconds.
		Weight::from_ref_time(3_732_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
}