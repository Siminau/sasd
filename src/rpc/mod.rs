// src/rpc/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

use siminau_rpc::error::{RpcErrorKind, RpcResult};
use siminau_rpc::message::CodeConvert;

// Local imports


// ===========================================================================
// Modules
// ===========================================================================


pub mod v1;


// ===========================================================================
//
// ===========================================================================


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum Notice {
    // No more requests will be made
    Done = 1,
}


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum RequestMethod {
    // Initiate client session by requesting an API version
    // Single argument: unsigned integer
    Version = 2,
}


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum RequestError {
    // API version is unsupported by the server
    // Result argument: unsigned integer
    VersionUnsupported = 3,
}


// ===========================================================================
//
// ===========================================================================
