// src/protocol/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// use std::rc::Rc;
// use std::sync::RwLock;

// Third-party imports

// use config::Config;

use rmpv::Value;
use siminau_rpc::error::{RpcErrorKind, RpcResult};
use siminau_rpc::message::{CodeConvert, Message, MessageType, RpcMessage};
use siminau_rpc::message::notify::{NotificationMessage, RpcNotice};
use siminau_rpc::message::request::{RequestMessage, RpcRequest};
use siminau_rpc::message::response::ResponseMessage;

// Local imports

use error::{SasdErrorKind, SasdResult};
use rpc;
use state::SessionStateHandle;

// Re-exports

#[cfg(windows)]
pub use os::windows::protocol::SessionStore;


// ===========================================================================
// Modules
// ===========================================================================


pub mod v1;


// ===========================================================================
// StateType
// ===========================================================================


#[derive(Debug, PartialEq, Clone)]
pub enum StateKind {
    Start,
    V1(v1::V1StateKind),
}


// ===========================================================================
// Supported protocol versions
// ===========================================================================


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum Protocol {
    V1 = 1,
}


// ===========================================================================
// Messages
// ===========================================================================


pub type Request = RequestMessage<rpc::RequestMethod>;


pub type Response = ResponseMessage<rpc::ResponseError>;


pub type Info = NotificationMessage<rpc::Notice>;


// ===========================================================================
// State
// ===========================================================================


pub type BoxState = Box<State>;


#[cfg(unix)]
fn first_state(rpcver: Protocol) -> BoxState
{
    match rpcver {
        Protocol::V1 => Box::new(v1::Session::new()),
    }
}


#[cfg(windows)]
fn first_state(rpcver: Protocol) -> BoxState
{
    match rpcver {
        Protocol::V1 => Box::new(v1::InitSession::new()),
    }
}


fn version(req: Request) -> SasdResult<(Option<BoxState>, Option<Message>)>
{
    let request_args = req.message_args();
    if request_args.len() != 1 {
        bail!(SasdErrorKind::InvalidMessage)
    }

    // Convert argument to a u64
    let ver = request_args[0].as_u64().ok_or(
        SasdErrorKind::InvalidMessage,
    )?;

    // Convert number to protocol version
    let rpcver = match Protocol::from_u64(ver) {
        Ok(v) => v,
        Err(_) => {
            let resp = Response::new(
                req.message_id(),
                rpc::ResponseError::UnsupportedVersion,
                Value::Nil,
            );
            let msg: Message = resp.into();
            let ret = (None, Some(msg));
            return Ok(ret);
        }
    };

    let val = first_state(rpcver);
    Ok((Some(val), None))
}


pub trait State {
    fn handle_version(&mut self, state: &mut SessionStateHandle, msg: Message)
        -> SasdResult<(Option<BoxState>, Option<Message>)>
    {
        // Check request method value
        let code = msg.as_vec()[2].as_u64().ok_or(
            SasdErrorKind::InvalidMessage,
        )?;

        match rpc::RequestMethod::from_u64(code) {
            Ok(_) => {
                let request = Request::from(msg)?;

                // Disconnect if get any method except Version
                match request.message_method() {
                    rpc::RequestMethod::Version => version(request),
                    // _ => bail!(SasdErrorKind::UnexpectedMessage),
                }
            }
            Err(_) => self.dispatch(state, msg),
        }
    }

    fn handle_done(&mut self, state: &mut SessionStateHandle, msg: Message)
        -> SasdResult<(Option<BoxState>, Option<Message>)>
    {
        // Check notification code value
        let code = msg.as_vec()[1].as_u64().ok_or(
            SasdErrorKind::InvalidMessage,
        )?;

        match rpc::Notice::from_u64(code) {
            Ok(_) => {
                let info = Info::from(msg)?;

                // Disconnect if get any method except Done
                match info.message_code() {
                    rpc::Notice::Done => Ok((None, None)),
                    // _ => bail!(SasdErrorKind::UnexpectedMessage),
                }
            }
            Err(_) => self.dispatch(state, msg),
        }
    }

    // Accepts a RequestMessage, and returns (State, ResponseMessage)
    fn change(&mut self, mut state: SessionStateHandle, msg: Message)
        -> SasdResult<(Option<BoxState>, Option<Message>)>
    {
        match msg.message_type() {
            MessageType::Request => self.handle_version(&mut state, msg),
            MessageType::Notification => self.handle_done(&mut state, msg),
            MessageType::Response => bail!(SasdErrorKind::UnexpectedMessage),
        }
    }

    fn dispatch(&mut self, state: &mut SessionStateHandle, msg: Message)
        -> SasdResult<(Option<BoxState>, Option<Message>)>;

    fn kind(&self) -> StateKind;
}


// ===========================================================================
// Start state
// ===========================================================================


pub struct Start;


impl Start {
    pub fn new() -> Self
    {
        Start
    }
}


impl State for Start {
    fn dispatch(&mut self, _state: &mut SessionStateHandle, _msg: Message)
        -> SasdResult<(Option<BoxState>, Option<Message>)>
    {
        bail!(SasdErrorKind::UnexpectedMessage)
    }

    fn kind(&self) -> StateKind
    {
        StateKind::Start
    }
}


// ===========================================================================
// Tests
// ===========================================================================


#[cfg(test)]
mod test {

    // Stdlib imports

    // Third-party imports

    use quickcheck::TestResult;
    use rmpv::Value;
    use siminau_rpc::message::CodeConvert;

    // Local imports

    use super::{Request, Response, version};
    use error::SasdErrorKind;
    use protocol::{Protocol, StateKind};
    use protocol::v1;
    use rpc;

    mod version {
        use super::*;
        use siminau_rpc::message::response::RpcResponse;

        quickcheck! {

            fn request_bad_numargs(numargs: u8) -> TestResult {
                if numargs == 1 {
                    return TestResult::discard()
                }

                // -----------------------------
                // GIVEN
                // A valid request message w/ RequestMethod::Version and
                // args w/ length != 1
                // -----------------------------
                let mut args = Vec::with_capacity(numargs as usize);
                for _ in 0..numargs {
                    args.push(Value::from(1u8));
                }
                let request = Request::new(42, rpc::RequestMethod::Version, args);

                // -----------------------------------------------------------
                // WHEN
                // version() is called with the request message
                // -----------------------------------------------------------
                let result = version(request);

                // ---------------------------------------
                // THEN
                // an InvalidMessage error is returned
                // ---------------------------------------
                let value = match result {
                    Err(e) => {
                        match *e.kind() {
                            SasdErrorKind::InvalidMessage => true,
                            _ => false,
                        }
                    }
                    _ => false
                };
                TestResult::from_bool(value)
            }
        }

        #[test]
        fn request_bad_argvaltype()
        {
            // -----------------------------
            // GIVEN
            // A valid request message w/ RequestMethod::Version and
            // args w/ length == 1 and
            // string arg
            // -----------------------------
            let args = vec![Value::from("1")];
            let request = Request::new(42, rpc::RequestMethod::Version, args);

            // -----------------------------------------------------------
            // WHEN
            // version() is called with the request message
            // -----------------------------------------------------------
            let result = version(request);

            // ---------------------------------------
            // THEN
            // an InvalidMessage error is returned
            // ---------------------------------------
            let value = match result {
                Err(e) => {
                    match *e.kind() {
                        SasdErrorKind::InvalidMessage => true,
                        _ => false,
                    }
                }
                _ => false,
            };
            assert!(value);
        }

        quickcheck! {
            fn request_bad_argval(val: u64) -> TestResult
            {
                if val <= Protocol::max_number() {
                    return TestResult::discard()
                }

                // -----------------------------
                // GIVEN
                // A valid request message w/ RequestMethod::Version and
                // args w/ length == 1 and
                // arg > Protocol::max_number()
                // -----------------------------
                let args = vec![Value::from(val)];
                let request = Request::new(42, rpc::RequestMethod::Version, args);

                // -----------------------------------------------------------
                // WHEN
                // version() is called with the request message
                // -----------------------------------------------------------
                let result = version(request);

                // ---------------------------------------
                // THEN
                // (None, Response) is returned
                // ---------------------------------------
                let value = match result {
                    Ok((None, Some(msg))) => {
                        // Convert Message into Response
                        let resp = Response::from(msg).unwrap();
                        assert_eq!(resp.message_id(), 42);
                        assert_eq!(
                            resp.error_code(),
                            rpc::ResponseError::UnsupportedVersion
                        );
                        assert_eq!(resp.result(), &Value::Nil);
                        true
                    }
                    _ => false,
                };
                TestResult::from_bool(value)
            }
        }

        #[test]
        fn request_val_notexist()
        {
            // -----------------------------
            // GIVEN
            // A valid request message w/ RequestMethod::Version and
            // args w/ length == 1 and
            // arg == Protocol::max_value()+1
            // -----------------------------
            let args = vec![Value::from(Protocol::max_number() + 1)];
            let request = Request::new(42, rpc::RequestMethod::Version, args);

            // -----------------------------------------------------------
            // WHEN
            // version() is called with the request message
            // -----------------------------------------------------------
            let result = version(request);

            // ---------------------------------------
            // THEN
            // (None, Response) is returned
            // ---------------------------------------
            let value = match result {
                Ok((None, Some(msg))) => {
                    let resp = Response::from(msg).unwrap();
                    assert_eq!(resp.message_id(), 42);
                    assert_eq!(
                        resp.error_code(),
                        rpc::ResponseError::UnsupportedVersion
                    );
                    assert_eq!(resp.result(), &Value::Nil);
                    true
                }
                _ => false,
            };
            assert!(value);
        }

        quickcheck! {
            #[cfg(target_family = "windows")]
            fn request_val_good(val: <Protocol as CodeConvert<Protocol>>::int_type) -> TestResult
            {
                if val as u64 > Protocol::max_number() || val == 0 {
                    return TestResult::discard()
                }

                // -----------------------------
                // GIVEN
                // A valid request message w/ RequestMethod::Version and
                // args w/ length == 1 and
                // arg == valid Protocol number value
                // -----------------------------
                let args = vec![Value::from(val)];
                let request = Request::new(42, rpc::RequestMethod::Version, args);

                // -----------------------------------------------------------
                // WHEN
                // version() is called with the request message
                // -----------------------------------------------------------
                let result = version(request);

                // ---------------------------------------
                // THEN
                // (Some(InitSession), None) is returned
                // ---------------------------------------
                let value = match result {
                    Ok((Some(state), None)) => {
                        state.kind() == StateKind::V1(v1::V1StateKind::InitSession)
                    }
                    _ => false,
                };
                TestResult::from_bool(value)
            }

            #[cfg(target_family = "unix")]
            fn request_val_good(val: <Protocol as CodeConvert<Protocol>>::int_type) -> TestResult
            {
                if val as u64 > Protocol::max_number() || val == 0 {
                    return TestResult::discard()
                }

                // -----------------------------
                // GIVEN
                // A valid request message w/ RequestMethod::Version and
                // args w/ length == 1 and
                // arg == valid Protocol number value
                // -----------------------------
                let args = vec![Value::from(val)];
                let request = Request::new(42, rpc::RequestMethod::Version, args);

                // -----------------------------------------------------------
                // WHEN
                // version() is called with the request message
                // -----------------------------------------------------------
                let result = version(request);

                // ---------------------------------------
                // THEN
                // (Some(Session), None) is returned
                // ---------------------------------------
                let value = match result {
                    Ok((Some(state), None)) => {
                        state.kind() == StateKind::V1(v1::V1StateKind::Session)
                    }
                    _ => false,
                };
                TestResult::from_bool(value)
            }
        }
    }
}


// ===========================================================================
//
// ===========================================================================
