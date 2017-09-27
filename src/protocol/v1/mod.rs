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

#[cfg(windows)]
pub use os::windows::protocol::v1::InitSession;

use error::SasdResult;
use protocol::{State, StateKind};
use rpc::v1 as rpc1;

use super::SessionStateHandle;


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


#[cfg(target_family = "windows")]
#[derive(Debug, PartialEq, Clone)]
pub enum V1StateKind {
    InitSession,
    AuthSession,
    Session,
}


#[cfg(target_family = "unix")]
#[derive(Debug, PartialEq, Clone)]
pub enum V1StateKind {
    Session,
}


// ===========================================================================
// Helper
// ===========================================================================


macro_rules! v1state {
    ($t:ident) => (StateKind::V1(V1StateKind::$t));
}


// ===========================================================================
// Session
// ===========================================================================


pub struct Session;


impl Session {
    pub fn new() -> Self
    {
        Session
    }
}


impl State for Session {
    fn dispatch(&mut self, _state: &mut SessionStateHandle, _msg: Message)
        -> SasdResult<(Option<Box<State>>, Option<Message>)>
    {
        unimplemented!()
    }

    fn kind(&self) -> StateKind
    {
        v1state!(Session)
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
