// Copyright (C) Parity Technologies (UK) Ltd.
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
//! DATE: 2023-04-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=pallet_proxy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	/// Storage: Proxy Proxies (r:1 w:0)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227 + p * (37 ±0)`
		//  Estimated: `4706`
		// Minimum execution time: 16_322_000 picoseconds.
		Weight::from_parts(16_647_215, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 665
			.saturating_add(Weight::from_parts(35_384, 0).saturating_mul(p.into()))
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
		//  Measured:  `554 + a * (68 ±0) + p * (37 ±0)`
		//  Estimated: `5698`
		// Minimum execution time: 40_900_000 picoseconds.
		Weight::from_parts(41_543_773, 0)
			.saturating_add(Weight::from_parts(0, 5698))
			// Standard Error: 4_079
			.saturating_add(Weight::from_parts(103_858, 0).saturating_mul(a.into()))
			// Standard Error: 4_214
			.saturating_add(Weight::from_parts(21_027, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `469 + a * (68 ±0)`
		//  Estimated: `5698`
		// Minimum execution time: 24_733_000 picoseconds.
		Weight::from_parts(25_557_471, 0)
			.saturating_add(Weight::from_parts(0, 5698))
			// Standard Error: 1_033
			.saturating_add(Weight::from_parts(133_245, 0).saturating_mul(a.into()))
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
		//  Measured:  `469 + a * (68 ±0)`
		//  Estimated: `5698`
		// Minimum execution time: 24_410_000 picoseconds.
		Weight::from_parts(25_514_144, 0)
			.saturating_add(Weight::from_parts(0, 5698))
			// Standard Error: 1_109
			.saturating_add(Weight::from_parts(131_095, 0).saturating_mul(a.into()))
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
		//  Measured:  `486 + a * (68 ±0) + p * (37 ±0)`
		//  Estimated: `5698`
		// Minimum execution time: 34_136_000 picoseconds.
		Weight::from_parts(37_134_885, 0)
			.saturating_add(Weight::from_parts(0, 5698))
			// Standard Error: 4_280
			.saturating_add(Weight::from_parts(124_510, 0).saturating_mul(a.into()))
			// Standard Error: 4_423
			.saturating_add(Weight::from_parts(18_117, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227 + p * (37 ±0)`
		//  Estimated: `4706`
		// Minimum execution time: 25_195_000 picoseconds.
		Weight::from_parts(25_886_778, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 840
			.saturating_add(Weight::from_parts(54_308, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227 + p * (37 ±0)`
		//  Estimated: `4706`
		// Minimum execution time: 25_191_000 picoseconds.
		Weight::from_parts(26_135_295, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 1_592
			.saturating_add(Weight::from_parts(56_159, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227 + p * (37 ±0)`
		//  Estimated: `4706`
		// Minimum execution time: 22_472_000 picoseconds.
		Weight::from_parts(23_237_113, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 956
			.saturating_add(Weight::from_parts(23_140, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `4706`
		// Minimum execution time: 27_176_000 picoseconds.
		Weight::from_parts(27_923_720, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 2_706
			.saturating_add(Weight::from_parts(2_006, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264 + p * (37 ±0)`
		//  Estimated: `4706`
		// Minimum execution time: 23_409_000 picoseconds.
		Weight::from_parts(23_984_022, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 768
			.saturating_add(Weight::from_parts(29_138, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
