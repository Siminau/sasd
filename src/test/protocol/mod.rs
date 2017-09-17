// src/test/protocol/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================


mod state;
mod start;


// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

use rmpv::Value;
use siminau_rpc::error::RpcErrorKind;
use siminau_rpc::message::{Message, MessageType};

// Local imports

use error::{SasdErrorKind, SasdResult};
use protocol::{BoxState, Info, Protocol, Request, Response, State, StateKind};
use protocol::v1;
use rpc;


// ===========================================================================
// Helpers
// ===========================================================================


struct Test;


impl State for Test {
    fn dispatch(&self, _msg: Message)
        -> SasdResult<(Option<BoxState>, Option<Message>)>
    {
        Ok((None, None))
    }

    fn kind(&self) -> StateKind
    {
        StateKind::Start
    }
}


// ===========================================================================
//
// ===========================================================================
