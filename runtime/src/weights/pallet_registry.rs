// This file is part of CORD – https://cord.network

// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_registry`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-3-249`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/cord
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_registry
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-GPL3
// --output=./runtime/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_registry`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_registry::WeightInfo for WeightInfo<T> {
	/// Storage: Registry Registries (r:1 w:1)
	/// Proof: Registry Registries (max_values: None, max_size: Some(15544), added: 18019, mode: MaxEncodedLen)
	/// Storage: Registry Commits (r:1 w:1)
	/// Proof: Registry Commits (max_values: None, max_size: Some(69068), added: 71543, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 15360]`.
	fn create(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `72533`
		// Minimum execution time: 72_918_000 picoseconds.
		Weight::from_parts(75_187_914, 0)
			.saturating_add(Weight::from_parts(0, 72533))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Registry Registries (r:1 w:1)
	/// Proof: Registry Registries (max_values: None, max_size: Some(15544), added: 18019, mode: MaxEncodedLen)
	/// Storage: Registry Commits (r:1 w:1)
	/// Proof: Registry Commits (max_values: None, max_size: Some(69068), added: 71543, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 15360]`.
	fn update(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `15726`
		//  Estimated: `72533`
		// Minimum execution time: 60_648_000 picoseconds.
		Weight::from_parts(64_405_749, 0)
			.saturating_add(Weight::from_parts(0, 72533))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Registry Registries (r:1 w:1)
	/// Proof: Registry Registries (max_values: None, max_size: Some(15544), added: 18019, mode: MaxEncodedLen)
	/// Storage: Registry Commits (r:1 w:1)
	/// Proof: Registry Commits (max_values: None, max_size: Some(69068), added: 71543, mode: MaxEncodedLen)
	fn archive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `15726`
		//  Estimated: `72533`
		// Minimum execution time: 40_167_000 picoseconds.
		Weight::from_parts(41_760_000, 0)
			.saturating_add(Weight::from_parts(0, 72533))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Registry Registries (r:1 w:1)
	/// Proof: Registry Registries (max_values: None, max_size: Some(15544), added: 18019, mode: MaxEncodedLen)
	/// Storage: Registry Commits (r:1 w:1)
	/// Proof: Registry Commits (max_values: None, max_size: Some(69068), added: 71543, mode: MaxEncodedLen)
	fn restore() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `15795`
		//  Estimated: `72533`
		// Minimum execution time: 40_417_000 picoseconds.
		Weight::from_parts(41_773_000, 0)
			.saturating_add(Weight::from_parts(0, 72533))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Registry Registries (r:1 w:0)
	/// Proof: Registry Registries (max_values: None, max_size: Some(15544), added: 18019, mode: MaxEncodedLen)
	/// Storage: Registry Authorizations (r:1 w:1)
	/// Proof: Registry Authorizations (max_values: None, max_size: Some(203), added: 2678, mode: MaxEncodedLen)
	/// Storage: Registry Authorities (r:1 w:1)
	/// Proof: Registry Authorities (max_values: None, max_size: Some(320068), added: 322543, mode: MaxEncodedLen)
	/// Storage: Registry Commits (r:1 w:1)
	/// Proof: Registry Commits (max_values: None, max_size: Some(69068), added: 71543, mode: MaxEncodedLen)
	fn add_admin_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `15726`
		//  Estimated: `323533`
		// Minimum execution time: 44_592_000 picoseconds.
		Weight::from_parts(45_386_000, 0)
			.saturating_add(Weight::from_parts(0, 323533))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Registry Registries (r:1 w:0)
	/// Proof: Registry Registries (max_values: None, max_size: Some(15544), added: 18019, mode: MaxEncodedLen)
	/// Storage: Registry Authorizations (r:1 w:1)
	/// Proof: Registry Authorizations (max_values: None, max_size: Some(203), added: 2678, mode: MaxEncodedLen)
	/// Storage: Registry Commits (r:1 w:1)
	/// Proof: Registry Commits (max_values: None, max_size: Some(69068), added: 71543, mode: MaxEncodedLen)
	fn add_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `15726`
		//  Estimated: `72533`
		// Minimum execution time: 39_789_000 picoseconds.
		Weight::from_parts(40_268_000, 0)
			.saturating_add(Weight::from_parts(0, 72533))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Registry Registries (r:1 w:0)
	/// Proof: Registry Registries (max_values: None, max_size: Some(15544), added: 18019, mode: MaxEncodedLen)
	/// Storage: Registry Authorizations (r:1 w:1)
	/// Proof: Registry Authorizations (max_values: None, max_size: Some(203), added: 2678, mode: MaxEncodedLen)
	/// Storage: Registry Commits (r:1 w:1)
	/// Proof: Registry Commits (max_values: None, max_size: Some(69068), added: 71543, mode: MaxEncodedLen)
	fn remove_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16013`
		//  Estimated: `72533`
		// Minimum execution time: 37_002_000 picoseconds.
		Weight::from_parts(38_255_000, 0)
			.saturating_add(Weight::from_parts(0, 72533))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
