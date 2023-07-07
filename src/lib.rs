#![no_std]

#[cfg(feature = "std")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod card;

pub mod character;

#[cfg(feature = "std")]
pub mod daily;

#[cfg(feature = "std")]
pub mod events;

pub mod filter;

#[cfg(feature = "std")]
pub mod map;

pub mod neow;

pub mod seed;

#[cfg(feature = "std")]
pub mod sieve;

pub mod unlock;
