#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;

pub type CommonId = u64;

pub use invarch_pallet_primitives::AnyId;
