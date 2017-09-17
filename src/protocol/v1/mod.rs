// src/protocol/v1/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

use siminau_rpc::message::Message;
use siminau_rpc::message::request::RequestMessage;
use siminau_rpc::message::response::ResponseMessage;

// Local imports

use error::SasdResult;
use protocol::{State, StateKind};
use rpc::v1 as rpc1;


// ===========================================================================
// Modules
// ===========================================================================


pub mod plaintext;


// ===========================================================================
// Messages
// ===========================================================================


pub type SessionRequest = RequestMessage<rpc1::SessionMethod>;


pub type SessionResponse = ResponseMessage<rpc1::SessionError>;


// pub type Info = NotificationMessage<rpc::Notice>;


// ===========================================================================
// StateType
// ===========================================================================


#[derive(Debug, PartialEq, Clone)]
pub enum V1StateKind {
    InitSession,
}


// ===========================================================================
// Helper
// ===========================================================================


macro_rules! v1state {
    ($t:ident) => (StateKind::V1(V1StateKind::$t));
}


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
        v1state!(InitSession)
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
