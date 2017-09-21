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

extern crate config;

#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate matches;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate rmpv;
// extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate siminau_rpc;

#[macro_use]
extern crate siminau_rpc_derive;


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
