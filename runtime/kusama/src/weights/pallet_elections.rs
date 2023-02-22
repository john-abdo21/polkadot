// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_elections`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_elections
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_elections`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections::WeightInfo for WeightInfo<T> {
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Minimum execution time: 33_181 nanoseconds.
		Weight::from_ref_time(34_284_379)
			// Standard Error: 4_703
			.saturating_add(Weight::from_ref_time(193_150).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Minimum execution time: 43_559 nanoseconds.
		Weight::from_ref_time(45_014_304)
			// Standard Error: 6_002
			.saturating_add(Weight::from_ref_time(232_544).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Minimum execution time: 43_465 nanoseconds.
		Weight::from_ref_time(45_027_590)
			// Standard Error: 6_393
			.saturating_add(Weight::from_ref_time(218_368).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn remove_voter() -> Weight {
		// Minimum execution time: 42_363 nanoseconds.
		Weight::from_ref_time(43_806_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Minimum execution time: 43_408 nanoseconds.
		Weight::from_ref_time(37_013_988)
			// Standard Error: 1_189
			.saturating_add(Weight::from_ref_time(111_132).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Elections Candidates (r:1 w:1)

	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 40_256 nanoseconds.
		Weight::from_ref_time(32_461_064)
			// Standard Error: 1_225
			.saturating_add(Weight::from_ref_time(87_968).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn renounce_candidacy_members() -> Weight {
		// Minimum execution time: 54_566 nanoseconds.
		Weight::from_ref_time(56_705_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Elections RunnersUp (r:1 w:1)
	fn renounce_candidacy_runners_up() -> Weight {
		// Minimum execution time: 42_417 nanoseconds.
		Weight::from_ref_time(43_317_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn remove_member_without_replacement() -> Weight {
		// Minimum execution time: 2_000_000_000 nanoseconds.
		Weight::from_ref_time(2_000_000_000_000)
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn remove_member_with_replacement() -> Weight {
		// Minimum execution time: 72_982 nanoseconds.
		Weight::from_ref_time(74_640_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Elections Voting (r:5001 w:5000)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Balances Locks (r:5000 w:5000)
	// Storage: System Account (r:5000 w:5000)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Minimum execution time: 306_719_494 nanoseconds.
		Weight::from_ref_time(307_744_759_000)
			// Standard Error: 264_406
			.saturating_add(Weight::from_ref_time(37_950_413).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:10001 w:0)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn pre_solve_election(_c: u32, v: u32, e: u32, ) -> Weight {
		// Minimum execution time: 6_399_000 nanoseconds.
		Weight::from_ref_time(1_581_352_360 as u64)
			// Standard Error: 55_251
			.saturating_add(Weight::from_ref_time(8_030_872 as u64).saturating_mul(v as u64))
			// Standard Error: 3_683
			.saturating_add(Weight::from_ref_time(13_203 as u64).saturating_mul(e as u64))
			.saturating_add(T::DbWeight::get().reads(296 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Elections ElectionRounds (r:1 w:1)
	// Storage: Elections Candidates (r:0 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn post_solve_election(c: u32, v: u32, e: u32, ) -> Weight {
		// Minimum execution time: 730_000 nanoseconds.
		Weight::from_ref_time(747_000_000 as u64)
			// Standard Error: 50_403
			.saturating_add(Weight::from_ref_time(12_670_099 as u64).saturating_mul(c as u64))
			// Standard Error: 5_039
			.saturating_add(Weight::from_ref_time(342_861 as u64).saturating_mul(v as u64))
			// Standard Error: 323
			.saturating_add(Weight::from_ref_time(127 as u64).saturating_mul(e as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}