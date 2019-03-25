//! A Rust re-write of the Stockfish chess engine.
//!
//! This crate is not intended to be used by other crates as a dependency, as it's a mostly useful as a direct
//! executable.
//!
//! If you are interested in using the direct chess library functions (The Boards, move generation, etc), please
//! checkout the core library, `pleco`, available on [on crates.io](https://crates.io/crates/pleco).
//!
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(test, allow(dead_code))]
#![allow(dead_code)]
#![feature(ptr_internals)]
#![feature(integer_atomics)]
#![feature(test)]
#![feature(allocator_api)]
#![feature(trusted_len)]
#![feature(const_fn)]
#![feature(box_into_raw_non_null)]
#![feature(alloc_layout_extra)]
#![feature(thread_spawn_unchecked)]

//#![crate_type = "staticlib"]

extern crate chrono;
extern crate num_cpus;
extern crate pleco;
extern crate prefetch;
extern crate rand;

pub mod consts;
pub mod engine;
pub mod movepick;
pub mod root_moves;
pub mod search;
pub mod sync;
pub mod tables;
pub mod threadpool;
pub mod time;
pub mod uci;

pub use consts::*;
