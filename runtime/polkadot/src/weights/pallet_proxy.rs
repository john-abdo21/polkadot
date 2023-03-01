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
//! Autogenerated weights for `pallet_proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_proxy
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

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	/// Storage: Proxy Proxies (r:1 w:0)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn proxy(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + p * (37 ±0)`
		//  Estimated: `3716`
		// Minimum execution time: 16_045 nanoseconds.
		Weight::from_ref_time(17_922_796)
			.saturating_add(Weight::from_proof_size(3716))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: Proxy Proxies (r:1 w:0)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `650 + a * (68 ±0) + p * (37 ±0)`
		//  Estimated: `11027`
		// Minimum execution time: 34_619 nanoseconds.
		Weight::from_ref_time(35_338_676)
			.saturating_add(Weight::from_proof_size(11027))
			// Standard Error: 1_899
			.saturating_add(Weight::from_ref_time(162_971).saturating_mul(a.into()))
			// Standard Error: 1_963
			.saturating_add(Weight::from_ref_time(13_533).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533 + a * (68 ±0)`
		//  Estimated: `7311`
		// Minimum execution time: 21_156 nanoseconds.
		Weight::from_ref_time(22_277_542)
			.saturating_add(Weight::from_proof_size(7311))
			// Standard Error: 1_498
			.saturating_add(Weight::from_ref_time(164_253).saturating_mul(a.into()))
			// Standard Error: 1_548
			.saturating_add(Weight::from_ref_time(3_031).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn reject_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533 + a * (68 ±0)`
		//  Estimated: `7311`
		// Minimum execution time: 21_507 nanoseconds.
		Weight::from_ref_time(23_041_588)
			.saturating_add(Weight::from_proof_size(7311))
			// Standard Error: 1_515
			.saturating_add(Weight::from_ref_time(143_107).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Proxy Proxies (r:1 w:0)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `582 + a * (68 ±0) + p * (37 ±0)`
		//  Estimated: `11027`
		// Minimum execution time: 30_053 nanoseconds.
		Weight::from_ref_time(31_573_633)
			.saturating_add(Weight::from_proof_size(11027))
			// Standard Error: 2_251
			.saturating_add(Weight::from_ref_time(140_597).saturating_mul(a.into()))
			// Standard Error: 2_326
			.saturating_add(Weight::from_ref_time(33_286).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + p * (37 ±0)`
		//  Estimated: `3716`
		// Minimum execution time: 22_886 nanoseconds.
		Weight::from_ref_time(23_835_705)
			.saturating_add(Weight::from_proof_size(3716))
			// Standard Error: 1_804
			.saturating_add(Weight::from_ref_time(57_157).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + p * (37 ±0)`
		//  Estimated: `3716`
		// Minimum execution time: 22_234 nanoseconds.
		Weight::from_ref_time(23_637_414)
			.saturating_add(Weight::from_proof_size(3716))
			// Standard Error: 1_672
			.saturating_add(Weight::from_ref_time(65_808).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + p * (37 ±0)`
		//  Estimated: `3716`
		// Minimum execution time: 17_995 nanoseconds.
		Weight::from_ref_time(18_853_035)
			.saturating_add(Weight::from_proof_size(3716))
			// Standard Error: 2_823
			.saturating_add(Weight::from_ref_time(35_158).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `3716`
		// Minimum execution time: 24_710 nanoseconds.
		Weight::from_ref_time(25_539_249)
			.saturating_add(Weight::from_proof_size(3716))
			// Standard Error: 1_383
			.saturating_add(Weight::from_ref_time(10_689).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296 + p * (37 ±0)`
		//  Estimated: `3716`
		// Minimum execution time: 19_199 nanoseconds.
		Weight::from_ref_time(19_877_054)
			.saturating_add(Weight::from_proof_size(3716))
			// Standard Error: 1_043
			.saturating_add(Weight::from_ref_time(31_448).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
