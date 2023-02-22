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
//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-ehxwxxsd-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `64 + r * (57 ±0)`
		//  Estimated: `1636`
		// Minimum execution time: 10_611 nanoseconds.
		Weight::from_ref_time(11_211_314)
			.saturating_add(Weight::from_proof_size(1636))
			// Standard Error: 2_369
			.saturating_add(Weight::from_ref_time(104_039).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `474 + r * (5 ±0)`
		//  Estimated: `10013`
		// Minimum execution time: 26_860 nanoseconds.
		Weight::from_ref_time(25_070_850)
			.saturating_add(Weight::from_proof_size(10013))
			// Standard Error: 4_502
			.saturating_add(Weight::from_ref_time(174_383).saturating_mul(r.into()))
			// Standard Error: 878
			.saturating_add(Weight::from_ref_time(347_063).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:100 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `15746 + s * (2589 ±0)`
		// Minimum execution time: 7_593 nanoseconds.
		Weight::from_ref_time(18_167_650)
			.saturating_add(Weight::from_proof_size(15746))
			// Standard Error: 4_550
			.saturating_add(Weight::from_ref_time(2_724_075).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_proof_size(2589).saturating_mul(s.into()))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226 + p * (32 ±0)`
		//  Estimated: `15746`
		// Minimum execution time: 7_411 nanoseconds.
		Weight::from_ref_time(18_378_377)
			.saturating_add(Weight::from_proof_size(15746))
			// Standard Error: 2_903
			.saturating_add(Weight::from_ref_time(1_158_126).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533 + r * (5 ±0) + s * (32 ±0) + x * (66 ±0)`
		//  Estimated: `15746`
		// Minimum execution time: 42_456 nanoseconds.
		Weight::from_ref_time(24_492_231)
			.saturating_add(Weight::from_proof_size(15746))
			// Standard Error: 9_059
			.saturating_add(Weight::from_ref_time(133_735).saturating_mul(r.into()))
			// Standard Error: 1_769
			.saturating_add(Weight::from_ref_time(1_150_131).saturating_mul(s.into()))
			// Standard Error: 1_769
			.saturating_add(Weight::from_ref_time(197_199).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + r * (57 ±0) + x * (66 ±0)`
		//  Estimated: `11649`
		// Minimum execution time: 26_590 nanoseconds.
		Weight::from_ref_time(25_283_680)
			.saturating_add(Weight::from_proof_size(11649))
			// Standard Error: 4_460
			.saturating_add(Weight::from_ref_time(171_973).saturating_mul(r.into()))
			// Standard Error: 870
			.saturating_add(Weight::from_ref_time(367_613).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430 + x * (66 ±0)`
		//  Estimated: `10013`
		// Minimum execution time: 24_287 nanoseconds.
		Weight::from_ref_time(23_412_854)
			.saturating_add(Weight::from_proof_size(10013))
			// Standard Error: 3_591
			.saturating_add(Weight::from_ref_time(128_659).saturating_mul(r.into()))
			// Standard Error: 700
			.saturating_add(Weight::from_ref_time(359_832).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `121 + r * (57 ±0)`
		//  Estimated: `1636`
		// Minimum execution time: 6_482 nanoseconds.
		Weight::from_ref_time(6_993_218)
			.saturating_add(Weight::from_proof_size(1636))
			// Standard Error: 2_092
			.saturating_add(Weight::from_ref_time(81_037).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `121 + r * (57 ±0)`
		//  Estimated: `1636`
		// Minimum execution time: 6_174 nanoseconds.
		Weight::from_ref_time(6_728_319)
			.saturating_add(Weight::from_proof_size(1636))
			// Standard Error: 1_499
			.saturating_add(Weight::from_ref_time(95_351).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `121 + r * (57 ±0)`
		//  Estimated: `1636`
		// Minimum execution time: 6_161 nanoseconds.
		Weight::from_ref_time(6_659_333)
			.saturating_add(Weight::from_proof_size(1636))
			// Standard Error: 1_438
			.saturating_add(Weight::from_ref_time(91_975).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509 + r * (57 ±0) + x * (66 ±0)`
		//  Estimated: `11649`
		// Minimum execution time: 20_593 nanoseconds.
		Weight::from_ref_time(19_801_143)
			.saturating_add(Weight::from_proof_size(11649))
			// Standard Error: 4_504
			.saturating_add(Weight::from_ref_time(148_263).saturating_mul(r.into()))
			// Standard Error: 833
			.saturating_add(Weight::from_ref_time(624_696).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `772 + r * (5 ±0) + s * (32 ±0) + x * (66 ±0)`
		//  Estimated: `18349`
		// Minimum execution time: 57_066 nanoseconds.
		Weight::from_ref_time(36_503_317)
			.saturating_add(Weight::from_proof_size(18349))
			// Standard Error: 9_982
			.saturating_add(Weight::from_ref_time(215_418).saturating_mul(r.into()))
			// Standard Error: 1_949
			.saturating_add(Weight::from_ref_time(1_172_228).saturating_mul(s.into()))
			// Standard Error: 1_949
			.saturating_add(Weight::from_ref_time(204_944).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `507 + s * (36 ±0)`
		//  Estimated: `18335`
		// Minimum execution time: 22_951 nanoseconds.
		Weight::from_ref_time(27_792_668)
			.saturating_add(Weight::from_proof_size(18335))
			// Standard Error: 1_560
			.saturating_add(Weight::from_ref_time(101_551).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `623 + s * (3 ±0)`
		//  Estimated: `12602`
		// Minimum execution time: 10_721 nanoseconds.
		Weight::from_ref_time(12_690_006)
			.saturating_add(Weight::from_proof_size(12602))
			// Standard Error: 609
			.saturating_add(Weight::from_ref_time(40_906).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `702 + s * (35 ±0)`
		//  Estimated: `18335`
		// Minimum execution time: 26_552 nanoseconds.
		Weight::from_ref_time(29_055_574)
			.saturating_add(Weight::from_proof_size(18335))
			// Standard Error: 1_075
			.saturating_add(Weight::from_ref_time(96_558).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `628 + s * (37 ±0)`
		//  Estimated: `8322`
		// Minimum execution time: 17_719 nanoseconds.
		Weight::from_ref_time(20_115_353)
			.saturating_add(Weight::from_proof_size(8322))
			// Standard Error: 1_074
			.saturating_add(Weight::from_ref_time(86_193).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
