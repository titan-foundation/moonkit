// Copyright Moonsong Labs
// This file is part of Moonkit.

// Moonkit is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonkit is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonkit.  If not, see <http://www.gnu.org/licenses/>.

#![cfg(feature = "runtime-benchmarks")]

use crate::{Call, Config, Pallet};
use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;
use nimbus_primitives::CanAuthor;
use nimbus_primitives::SlotBeacon;
benchmarks! {
	kick_off_authorship_validation {
		// The slot inserted needs to be higher than that already in storage
		T::SlotBeacon::set_slot(100);
		Pallet::<T>::set_eligible_author(&T::SlotBeacon::slot());
	}: _(RawOrigin::None)
}
