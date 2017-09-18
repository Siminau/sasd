// src/os/windows/protocol/v1/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

use siminau_rpc::message::Message;

// Local imports

use error::SasdResult;
use protocol::{State, StateKind};
use protocol::v1::V1StateKind;


// ===========================================================================
// Initialize session state
// ===========================================================================


pub struct InitSession;


impl State for InitSession {
    fn dispatch(&self, _msg: Message)
        -> SasdResult<(Option<Box<State>>, Option<Message>)>
    {
        unimplemented!()
    }

    fn kind(&self) -> StateKind
    {
        StateKind::V1(V1StateKind::InitSession)
    }
}


// ===========================================================================
// Tests
// ===========================================================================


#[cfg(test)]
mod tests {}


// ===========================================================================
//
// ===========================================================================
