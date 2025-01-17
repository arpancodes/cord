//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-14, STEPS: [3, ], REPEAT: 2, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Interpreted, CHAIN: Some("dev"), DB
//! CACHE: 128

// Executed Command:
// ./target/release/cord
// benchmark
// --chain=dev
// --execution=wasm
// --pallet=pallet_utility
// --extrinsic=*
// --steps=3
// --repeat=2
// --output=./runtime/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_utility.
pub trait WeightInfo {
	fn batch(c: u32, ) -> Weight;
	fn as_derivative() -> Weight;
	fn batch_all(c: u32, ) -> Weight;
	fn dispatch_as() -> Weight;
}

/// Weights for pallet_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn batch(c: u32, ) -> Weight {
		(18_293_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((5_530_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(3_387_000 as Weight)
	}
	fn batch_all(c: u32, ) -> Weight {
		(19_223_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((5_998_000 as Weight).saturating_mul(c as Weight))
	}
	fn dispatch_as() -> Weight {
		(14_340_000 as Weight)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn batch(c: u32, ) -> Weight {
		(18_293_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((5_530_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(3_387_000 as Weight)
	}
	fn batch_all(c: u32, ) -> Weight {
		(19_223_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((5_998_000 as Weight).saturating_mul(c as Weight))
	}
	fn dispatch_as() -> Weight {
		(14_340_000 as Weight)
	}
}
