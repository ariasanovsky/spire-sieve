#![no_std]

#[cfg(feature = "std")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
pub mod card;
#[cfg(feature = "std")]
pub mod character;
#[cfg(feature = "std")]
pub mod daily;
#[cfg(feature = "std")]
pub mod events;
#[cfg(feature = "std")]
pub mod filter;
#[cfg(feature = "std")]
pub mod map;
#[cfg(feature = "std")]
pub mod neow;
#[cfg(feature = "std")]
pub mod seed;
#[cfg(feature = "std")]
pub mod sieve;
#[cfg(feature = "std")]
pub mod unlock;
