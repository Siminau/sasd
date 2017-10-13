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
pub use os::windows::protocol::v1::{AuthSession, InitSession};

use error::SasdResult;
use protocol;
use protocol::State;
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


#[cfg(windows)]
#[derive(Debug)]
pub enum StateValue {
    InitSession(InitSession),
    AuthSession(AuthSession),
    Session(Session),
}


#[cfg(unix)]
#[derive(Debug)]
pub enum StateValue {
    Session(Session),
}


impl StateValue {
    // --------------------
    // is methods
    // --------------------
    #[cfg(unix)]
    pub fn is_session(&self) -> bool
    {
        true
    }

    #[cfg(windows)]
    pub fn is_session(&self) -> bool
    {
        match self {
            &StateValue::Session(_) => true,
            _ => false,
        }
    }

    #[cfg(windows)]
    pub fn is_initsession(&self) -> bool
    {
        match self {
            &StateValue::InitSession(_) => true,
            _ => false,
        }
    }

    #[cfg(windows)]
    pub fn is_authsession(&self) -> bool
    {
        match self {
            &StateValue::AuthSession(_) => true,
            _ => false,
        }
    }

    // --------------------
    // as methods
    // --------------------
    #[cfg(unix)]
    pub fn as_session(&self) -> Option<&Session>
    {
        let &StateValue::Session(ref s) = self;
        Some(s)
    }

    #[cfg(windows)]
    pub fn as_session(&self) -> Option<&Session>
    {
        match self {
            &StateValue::Session(ref s) => Some(s),
            _ => None,
        }
    }

    #[cfg(windows)]
    pub fn as_initsession(&self) -> Option<&InitSession>
    {
        match self {
            &StateValue::InitSession(ref s) => Some(s),
            _ => None,
        }
    }

    #[cfg(windows)]
    pub fn as_authsession(&self) -> Option<&AuthSession>
    {
        match self {
            &StateValue::AuthSession(ref s) => Some(s),
            _ => None,
        }
    }

    // --------------------
    // to methods
    // --------------------
    #[cfg(unix)]
    pub fn to_session(self) -> Option<Session>
    {
        let StateValue::Session(s) = self;
        Some(s)
    }

    #[cfg(windows)]
    pub fn to_session(self) -> Option<Session>
    {
        match self {
            StateValue::Session(s) => Some(s),
            _ => None,
        }
    }

    #[cfg(windows)]
    pub fn to_initsession(self) -> Option<InitSession>
    {
        match self {
            StateValue::InitSession(s) => Some(s),
            _ => None,
        }
    }

    #[cfg(windows)]
    pub fn to_authsession(self) -> Option<AuthSession>
    {
        match self {
            StateValue::AuthSession(s) => Some(s),
            _ => None,
        }
    }
}


// ===========================================================================
// Session
// ===========================================================================


#[derive(Debug)]
pub struct Session;


// Implement From and Into traits
impl From<Session> for protocol::StateValue {
    fn from(s: Session) -> protocol::StateValue
    {
        protocol::StateValue::V1(StateValue::Session(s))
    }
}


impl Session {
    pub fn new() -> Self
    {
        Session
    }
}


impl State for Session {
    fn dispatch(&mut self, _state: &mut SessionStateHandle, _msg: Message)
        -> SasdResult<(Option<protocol::StateValue>, Option<Message>)>
    {
        unimplemented!()
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
