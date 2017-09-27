// src/os/windows/protocol/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

use std::fs::File;

// Third-party imports

// Local imports


// ===========================================================================
// Modules
// ===========================================================================


pub mod v1;


// ===========================================================================
// SessionStore
// ===========================================================================


pub struct SessionStore {
    pub session_token: String,
    pub auth_token: String,
    pub auth_file: Option<File>,
}


impl SessionStore {
    pub fn new(session_token: String, auth_token: String) -> Self
    {
        Self {
            session_token: session_token,
            auth_token: auth_token,
            auth_file: None,
        }
    }
}


impl Default for SessionStore {
    fn default() -> Self
    {
        Self {
            session_token: String::with_capacity(64),
            auth_token: String::with_capacity(64),
            auth_file: None,
        }
    }
}


// ===========================================================================
//
// ===========================================================================
