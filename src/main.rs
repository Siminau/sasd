// src/main.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Features
// ===========================================================================

// #![feature(use_extern_macros)]

// ===========================================================================
// Externs
// ===========================================================================

// Third-party externs

extern crate appdirs;
extern crate config;

#[macro_use]
extern crate error_chain;

#[cfg(windows)]
extern crate hex;

#[cfg(test)]
#[macro_use]
extern crate matches;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(windows)]
extern crate rand;
extern crate rmpv;
// extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate siminau_rpc;

#[macro_use]
extern crate siminau_rpc_derive;

#[cfg(test)]
extern crate tempdir;

#[cfg(windows)]
extern crate winapi;


// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

// Local imports

// ===========================================================================
// Modules
// ===========================================================================


pub mod error;
pub mod rpc;
pub mod os;
pub mod protocol;
pub mod settings;
pub mod state;

#[cfg(test)]
mod test;

// ===========================================================================
// Main
// ===========================================================================


fn main()
{
    println!("Hello, world!");
}


// ===========================================================================
//
// ===========================================================================
