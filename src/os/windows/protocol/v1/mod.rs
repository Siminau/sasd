// src/os/windows/protocol/v1/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::os::windows::prelude::*;
use std::path::{Path, PathBuf};
// use std::rc::Rc;

// Third-party imports

use hex;
use rand::{OsRng, Rng};
use rmpv::{Utf8String, Value};
use siminau_rpc::message::{CodeConvert, Message, MessageType, RpcMessage};
use siminau_rpc::message::request::RpcRequest;
use winapi;

// Local imports

use error::{SasdErrorKind, SasdResult, SasdResultExt};
use protocol::{State, StateValue};
use protocol::v1::{Session, SessionRequest, SessionResponse,
                   StateValue as V1StateValue};
use rpc::v1 as rpc1;

// Grab SessionStore from parent module

use super::SessionStore;
use state::SessionStateHandle;


// ===========================================================================
// SessionState
// ===========================================================================


pub trait SessionState {
    fn check_msg(&self, msg: Message) -> SasdResult<SessionRequest>
    {
        // Check request method value
        let code = msg.as_vec()[2].as_u64().ok_or(
            SasdErrorKind::InvalidMessage,
        )?;

        rpc1::SessionMethod::from_u64(code).chain_err(|| {
            SasdErrorKind::InvalidMessage
        })?;

        let req = SessionRequest::from(msg).chain_err(
            || SasdErrorKind::InvalidMessage,
        )?;

        self.check_msg_method(req)
    }

    fn check_msg_method(&self, req: SessionRequest)
        -> SasdResult<SessionRequest>;
}


// ===========================================================================
// Initialize session state
// ===========================================================================


// TODO: add private members to hold client session token and auth token
// Tokens should use protected memory
#[derive(Debug)]
pub struct InitSession;


// Implement From and Into traits
impl From<InitSession> for StateValue {
    fn from(s: InitSession) -> StateValue
    {
        StateValue::V1(V1StateValue::InitSession(s))
    }
}


impl SessionState for InitSession {
    fn check_msg_method(&self, req: SessionRequest)
        -> SasdResult<SessionRequest>
    {
        match req.message_method() {
            rpc1::SessionMethod::Attach => {
                let numargs = req.message_args().len();
                match numargs {
                    0 | 1 => Ok(req),
                    _ => Err(SasdErrorKind::InvalidMessage.into()),
                }
            }
            _ => Err(SasdErrorKind::UnexpectedMessage.into()),
        }
    }
}


impl InitSession {
    pub fn new() -> InitSession
    {
        InitSession
    }

    pub fn from_value(v: StateValue) -> SasdResult<Self>
    {
        match v {
            StateValue::V1(V1StateValue::InitSession(s)) => Ok(s),
            _ => {
                let expected = format!("StateValue::V1(InitSession)");
                let value = format!("StateValue::{:?}", v);
                Err(SasdErrorKind::InvalidStateValue(expected, value).into())
            }
        }
    }

    fn make_random_hexstr(&self, len: usize) -> String
    {
        let mut rng = OsRng::new().expect("could not create rng");
        let mut bytes = vec![0u8; len];
        rng.fill_bytes(&mut bytes[..]);

        // Encode bytes into lower case ascii hex values
        hex::encode(bytes)
    }

    // TODO: use values from config object
    // TODO: can secure memory be used here?
    fn make_auth_file(&self, filepath: &Path) -> File
    {
        // Create and return a temp file
        OpenOptions::new()
            .read(true)
            .write(true)
            .share_mode(winapi::FILE_SHARE_READ)
            .custom_flags(winapi::FILE_FLAG_DELETE_ON_CLOSE)
            .create_new(true)
            .attributes(winapi::FILE_ATTRIBUTE_HIDDEN)
            .open(filepath)
            .expect("tmp file create failed")
    }

    fn write_auth_token(&self, tok: String, f: &mut File)
    {
        // Write auth token to the temp file
        f.write_all(tok.as_bytes()).expect(
            "write to temp file failed",
        );

        // Seek back to 0
        f.seek(SeekFrom::Start(0)).expect("tmp file seek 0 failed");
    }

    fn make_response(&self, skip_auth: bool, req: SessionRequest, filepath: Option<&Path>)
        -> SasdResult<SessionResponse>
    {
        let result = if !skip_auth {
            // Create SessionResponse w/ session token and file location as args
            let filepath = {
                let filepath =
                    filepath
                        .unwrap()
                        .as_os_str()
                        .to_os_string()
                        .into_string()
                        .map_err(|_| "Unable to convert filepath to string")?;
                Utf8String::from(filepath)
            };
            Value::Array(vec![Value::String(filepath)])
        } else {
            Value::Nil
        };

        let resp = SessionResponse::new(
            req.message_id(),
            rpc1::SessionError::Nil,
            result,
        );

        // Return SessionResponse
        Ok(resp)
    }

    // TODO: use config object
    fn attach(&mut self, state: &mut SessionStateHandle, req: SessionRequest)
        -> SasdResult<SessionResponse>
    {
        // Create auth token and store it in the session store
        let auth_token = self.make_random_hexstr(32);
        state.session_store().auth_token.push_str(&auth_token[..]);

        // Create custom name
        let mut filepath = {
            let config = state.server_settings().read().expect(
                "failed to read server \
                 settings",
            );
            PathBuf::from(&config.windows().token_data_dir)
        };
        let filename = self.make_random_hexstr(8);
        filepath.push(filename);

        // Create hidden temporary file in secure file location
        let mut tmpfile = self.make_auth_file(filepath.as_path());

        // Write auth token to file
        self.write_auth_token(auth_token, &mut tmpfile);

        // Store the temporary file in the session store
        state.session_store().auth_file = Some(tmpfile);

        // Create SessionResponse w/ file location as arg
        self.make_response(false, req, Some(filepath.as_path()))
    }

    // Used by dispatch() method to check if can skip calling the attach method
    fn can_skip_auth(&self, session_store: &SessionStore, req: &SessionRequest)
        -> bool
    {
        let args = req.message_args();
        if args.len() == 0 {
            return false;
        }

        let auth_token = match args[0].as_str() {
            Some(s) => String::from(s),
            None => return false,
        };

        &auth_token == &session_store.auth_token
    }
}


impl State for InitSession {
    fn dispatch(&mut self, state: &mut SessionStateHandle, msg: Message)
        -> SasdResult<(Option<StateValue>, Option<Message>)>
    {
        match msg.message_type() {
            MessageType::Request => {
                let req = self.check_msg(msg)?;
                let (resp, next): (SessionResponse, StateValue) =
                    if self.can_skip_auth(state.session_store(), &req) {
                        let resp = self.make_response(true, req, None)?;
                        (
                            resp,
                            StateValue::V1(V1StateValue::Session(Session::new())),
                        )
                    } else {
                        let resp = self.attach(state, req)?;
                        (
                            resp,
                            StateValue::V1(
                                V1StateValue::AuthSession(AuthSession::new()),
                            ),
                        )
                    };
                Ok((Some(next), Some(resp.into())))
            }
            MessageType::Notification => {
                bail!(SasdErrorKind::UnexpectedMessage)
            }
            MessageType::Response => unreachable!(),
        }
    }
}


// ===========================================================================
// Authorize session state
// ===========================================================================


// TODO: Tokens should use protected memory
#[derive(Debug)]
pub struct AuthSession;


// Implement From and Into traits
impl From<AuthSession> for StateValue {
    fn from(s: AuthSession) -> StateValue
    {
        StateValue::V1(V1StateValue::AuthSession(s))
    }
}


impl SessionState for AuthSession {
    fn check_msg_method(&self, req: SessionRequest)
        -> SasdResult<SessionRequest>
    {
        match req.message_method() {
            rpc1::SessionMethod::AuthAttach => {
                if req.message_args().len() != 1 {
                    Err(SasdErrorKind::InvalidMessage.into())
                } else {
                    Ok(req)
                }
            }
            _ => Err(SasdErrorKind::UnexpectedMessage.into()),
        }
    }
}


impl AuthSession {
    pub fn new() -> AuthSession
    {
        AuthSession
    }

    fn auth_attach(&self, state: &mut SessionStateHandle, req: SessionRequest)
        -> SasdResult<(Option<Session>, Option<Message>)>
    {
        // Get expected token str slices
        let auth_token = &state.session_store().auth_token[..];

        // Get auth token from request message
        let args = req.message_args();
        let req_auth_token = args[0].as_str().unwrap();

        // Compare token
        let errmsg = {
            if auth_token != req_auth_token {
                Some("auth token doesn't match")
            } else {
                None
            }
        };

        // Values needed to create response
        let (err, result, state) = match errmsg {
            Some(msg) => (
                rpc1::SessionError::InvalidAttach,
                Value::String(Utf8String::from(msg)),
                None,
            ),
            None => (rpc1::SessionError::Nil, Value::Nil, Some(Session::new())),
        };

        // Create response
        let resp = SessionResponse::new(req.message_id(), err, result);

        Ok((state, Some(resp.into())))
    }
}


impl State for AuthSession {
    fn dispatch(&mut self, state: &mut SessionStateHandle, msg: Message)
        -> SasdResult<(Option<StateValue>, Option<Message>)>
    {
        match msg.message_type() {
            MessageType::Request => {
                let req = self.check_msg(msg)?;
                let (newstate, resp) = self.auth_attach(state, req)?;
                let ret = match newstate {
                    Some(s) => (Some(s.into()), resp),
                    None => (None, resp),
                };
                Ok(ret)
            }
            MessageType::Notification => {
                bail!(SasdErrorKind::UnexpectedMessage)
            }
            MessageType::Response => unreachable!(),
        }
    }
}


// ===========================================================================
// Tests
// ===========================================================================


#[cfg(test)]
mod test {

    mod initsession {

        mod make_random_hexstr {
            use os::windows::protocol::v1::InitSession;

            #[test]
            fn token_len_64()
            {
                let state = InitSession::new();
                let tok = state.make_random_hexstr(32);
                assert_eq!(tok.len(), 64);
            }
        }

        mod make_auth_file {
            use os::windows::protocol::v1::InitSession;
            use std::path::PathBuf;

            use tempdir::TempDir;

            // TODO: this is an integration test, should it stay here?
            #[test]
            fn creates_file()
            {
                // ----------------------------------------------------------
                // GIVEN
                // an InitSession object and
                // InitSession has been initialized with a SessionStore and
                // a random filename
                // ----------------------------------------------------------
                let state = InitSession::new();
                let filename = state.make_random_hexstr(8);

                // Create the expected file path
                let tempdir = TempDir::new("sasd").unwrap();
                let mut filepath = PathBuf::from(tempdir.path());

                // Create full path to new file
                filepath.push(&filename);

                // -------------------------------------------------------
                // WHEN
                // InitSession::make_auth_file is called with the filename
                // -------------------------------------------------------
                let handle = state.make_auth_file(filepath.as_path());

                // ----------------------------------------------
                // THEN
                // the file is created in the expected directory
                // ----------------------------------------------
                assert!(filepath.exists());
                assert!(filepath.is_file());

                // --------------------
                // Cleanup
                // --------------------
                drop(handle);

                // File should have been deleted
                assert!(!filepath.exists());
            }
        }
    }
}


// ===========================================================================
//
// ===========================================================================
