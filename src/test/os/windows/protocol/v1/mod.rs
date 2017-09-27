// src/test/os/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.


// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

// Local imports

// use os::windows::protocol::v1::InitSession;


// ===========================================================================
// Tests
// ===========================================================================


mod sessionstate {

    mod check_msg {
        use error::{SasdErrorKind, SasdResult};
        use os::windows::protocol::v1::SessionState;
        use protocol::v1::SessionRequest;
        use rmpv::Value;
        use rpc::v1 as rpc1;

        use siminau_rpc::message::{CodeConvert, Message, MessageType};

        // --------------------
        // Helper
        // --------------------
        struct Test;

        impl SessionState for Test {
            fn check_msg_method(&self, _req: SessionRequest)
                -> SasdResult<SessionRequest>
            {
                unreachable!()
            }
        }

        #[test]
        fn non_u64_method()
        {
            // --------------------------------------------------------
            // GIVEN
            // a Message instance and
            // the message has a non-u64 value for the method argument and
            // a type implementing the SessionState trait
            // --------------------------------------------------------
            let msgtype = Value::from(MessageType::Request.to_number());
            let msgid = Value::from(42);
            let msgcode = Value::from("hello");
            let msgargs = Value::Array(vec![]);
            let value = Value::Array(vec![msgtype, msgid, msgcode, msgargs]);
            let msg = Message::from(value).unwrap();

            let test = Test;

            // ----------------------------------------------------
            // WHEN
            // SessionState::check_msg() is called with the message
            // ----------------------------------------------------
            let result = test.check_msg(msg);

            // -------------------------------------
            // THEN
            // An InvalidMessage error is generated
            // -------------------------------------

            let val = match result {
                Err(e) => {
                    match e.kind() {
                        &SasdErrorKind::InvalidMessage => true,
                        _ => false,
                    }
                }
                _ => false,
            };
            assert!(val);
        }

        #[test]
        fn invalid_method_code()
        {
            // --------------------------------------------------------
            // GIVEN
            // a Message instance and
            // a u64 value that is not valid for SessionMethod and
            // a type implementing the SessionState trait
            // --------------------------------------------------------
            let msgtype = Value::from(MessageType::Request.to_number());
            let msgid = Value::from(42);
            let msgcode = Value::from(9999);
            let msgargs = Value::Array(vec![]);
            let value = Value::Array(vec![msgtype, msgid, msgcode, msgargs]);
            let msg = Message::from(value).unwrap();

            let test = Test;

            // ----------------------------------------------------
            // WHEN
            // SessionState::check_msg() is called with the message
            // ----------------------------------------------------
            let result = test.check_msg(msg);

            // -------------------------------------
            // THEN
            // An InvalidMessage error is generated
            // -------------------------------------

            let val = match result {
                Err(e) => {
                    match e.kind() {
                        &SasdErrorKind::InvalidMessage => true,
                        _ => false,
                    }
                }
                _ => false,
            };
            assert!(val);
        }

        #[test]
        fn invalid_request_message()
        {
            // --------------------------------------------------------
            // GIVEN
            // a Message instance and
            // the message's 4th parameter as a non-vector
            // --------------------------------------------------------
            let msgtype = Value::from(MessageType::Request.to_number());
            let msgid = Value::from(42);
            let msgcode = Value::from(rpc1::SessionMethod::Attach.to_number());
            let msgargs = Value::from(42);
            let value = Value::Array(vec![msgtype, msgid, msgcode, msgargs]);
            let msg = Message::from(value).unwrap();

            let test = Test;

            // ----------------------------------------------------
            // WHEN
            // SessionState::check_msg() is called with the message
            // ----------------------------------------------------
            let result = test.check_msg(msg);

            // -------------------------------------
            // THEN
            // An InvalidMessage error is generated
            // -------------------------------------

            let val = match result {
                Err(e) => {
                    match e.kind() {
                        &SasdErrorKind::InvalidMessage => true,
                        _ => false,
                    }
                }
                _ => false,
            };
            assert!(val);
        }

        #[test]
        fn call_check_msg_method()
        {
            let expected = String::from("called");

            // -----------------------------------------------------
            // GIVEN
            // a type implementing SessionState and
            // the type's check_msg_method() method raises an error and
            // a Message that can be turned into a SessionRequest
            // -----------------------------------------------------
            struct Test;

            impl SessionState for Test {
                fn check_msg_method(&self, _req: SessionRequest)
                    -> SasdResult<SessionRequest>
                {
                    bail!("called")
                }
            }
            let test = Test;

            // Message
            let msgtype = Value::from(MessageType::Request.to_number());
            let msgid = Value::from(42);
            let msgcode = Value::from(rpc1::SessionMethod::Attach.to_number());
            let msgargs = Value::Array(vec![]);
            let value = Value::Array(vec![msgtype, msgid, msgcode, msgargs]);
            let msg = Message::from(value).unwrap();

            // ----------------------------------------------------------
            // WHEN
            // the SessionState type's check_msg() method is called with
            // the message
            // ----------------------------------------------------------
            let result = test.check_msg(msg);

            // -------------------------------
            // THEN
            // check_msg_method() is called
            // -------------------------------
            let val = match result {
                Err(e) => {
                    match e.kind() {
                        &SasdErrorKind::Msg(ref msg) => msg == &expected,
                        _ => false,
                    }
                }
                _ => false,
            };
            assert!(val);
        }
    }
}


mod initsession {

    mod kind {
        use protocol::{State, StateKind};
        use protocol::v1::{InitSession, V1StateKind};

        #[test]
        fn is_initsession()
        {
            let expected = StateKind::V1(V1StateKind::InitSession);

            // ------------------------
            // GIVEN
            // an InitSession instance
            // ------------------------
            let init = InitSession::new();

            // -------------------------------
            // WHEN
            // InitSession::kind() is called
            // -------------------------------
            let result = init.kind();

            // --------------------------------------------------
            // THEN
            // the V1StateKind::InitSession variant is returned
            // --------------------------------------------------
            assert_eq!(result, expected);
        }
    }

    mod can_skip_auth {
        use os::windows::protocol::SessionStore;
        use protocol::{State, StateKind};
        use protocol::v1::{InitSession, SessionRequest, SessionResponse,
                           V1StateKind};
        use rmpv::Value;
        use rpc::v1::{SessionError, SessionMethod};
        use siminau_rpc::message::response::RpcResponse;
        use state::SessionState;
        use std::fs::OpenOptions;
        use std::io::Read;
        use std::path::PathBuf;

        // Helpers

        use settings::{WindowsSection, new_settings_handle};
        use settings::test::helper::new_settings;

        use test::protocol::{cleanup_settings, dummy_session_state};

        // TODO
        // This matches on both session and auth tokens.
        // Once the state machine has been fleshed out, this should be changed
        // so that only the auth token is checked
        #[test]
        fn skip_auth_on_matching_tokens()
        {

            // -------------------------------------------------------
            // GIVEN
            // a valid SessionRequest message and
            // the message contains session and auth tokens as args and
            // a sessionstore that holds a session and auth token and
            // the sessionstore tokens match the message tokens and
            // an InitSession instance
            // -------------------------------------------------------
            // Create tokens and request message
            let session_token = "hello".to_owned();
            let auth_token = "world".to_owned();
            let msgargs = vec![
                Value::from(session_token.clone()),
                Value::from(auth_token.clone()),
            ];
            let request =
                SessionRequest::new(42, SessionMethod::Attach, msgargs);

            // Create state
            let mut init = InitSession::new();

            // Create session state
            let dummy = InitSession::new();

            let settings = new_settings(
                1234,
                None,
                WindowsSection {
                    token_data_dir: PathBuf::from("/does/not/exist"),
                },
            );
            let settings_handle = new_settings_handle(settings);
            let session_store = SessionStore {
                session_token: session_token,
                auth_token: auth_token,
                auth_file: None,
            };
            let mut session_state = SessionState::new(
                session_store,
                settings_handle,
                Box::new(dummy),
            );
            let mut handle = session_state.handle();

            // ------------------------------------------------------------
            // WHEN
            // InitSession::dispatch() is called with the sessionstore and
            // message
            // ------------------------------------------------------------
            let result = init.dispatch(&mut handle, request.into()).unwrap();
            let (state, msg) = match result {
                (Some(s), Some(m)) => (s, m),
                _ => unreachable!(),
            };

            // ----------------------------------------------------
            // THEN
            // A (State, SessionResponse) tuple is returned and
            // the state is V1StateKind::Session and
            // the response has Nil for its error and
            // the response has Nil for its result
            // ----------------------------------------------------
            let response = SessionResponse::from(msg).unwrap();

            assert_eq!(state.kind(), StateKind::V1(V1StateKind::Session));
            assert_eq!(response.error_code(), SessionError::Nil);
            assert_eq!(response.result(), &Value::Nil);
        }

        // TODO
        // it doesn't check if the token_data_dir already contains a
        // file with the same name as the generated file name.
        #[test]
        fn do_auth()
        {
            // -------------------------------------------------------
            // GIVEN
            // a valid SessionRequest message and
            // an empty sessionstore and
            // an InitSession instance
            // -------------------------------------------------------
            let request =
                SessionRequest::new(42, SessionMethod::Attach, vec![]);

            // Create state
            let mut init = InitSession::new();

            // Create session state
            let dummy = InitSession::new();

            let mut session_state = dummy_session_state(Box::new(dummy));

            // ------------------------------------------------------------
            // WHEN
            // InitSession::dispatch() is called with the sessionstore and
            // message
            // ------------------------------------------------------------
            let (state, msg) = {
                let mut handle = session_state.handle();
                let result = init.dispatch(&mut handle, request.into())
                    .unwrap();
                match result {
                    (Some(s), Some(m)) => (s, m),
                    _ => unreachable!(),
                }
            };

            // --------------------------------------------------------
            // THEN
            // A (State, SessionResponse) tuple is returned and
            // the state is V1StateKind::AuthSession and
            // the response has Nil for its error and
            // the response has session and auth tokens for its result
            // --------------------------------------------------------
            let response = SessionResponse::from(msg).unwrap();

            assert_eq!(state.kind(), StateKind::V1(V1StateKind::AuthSession));
            assert_eq!(response.error_code(), SessionError::Nil);

            // This is a &Vec<Value>
            let result = response.result().as_array().unwrap();

            assert_eq!(result.len(), 2);

            let session_token = String::from(result[0].as_str().unwrap());
            let auth_filepath = PathBuf::from(result[1].as_str().unwrap());

            assert_eq!(session_token.len(), 64);
            assert!(auth_filepath.exists());
            assert!(auth_filepath.is_file());

            // Read the file
            let auth_token = {
                let mut buf: Vec<u8> = Vec::new();
                let mut f = OpenOptions::new()
                    .read(true)
                    .open(auth_filepath.as_path())
                    .unwrap();
                let numbytes = f.read_to_end(&mut buf).unwrap();
                assert!(numbytes > 0);

                // Return the auth token contained in the file
                String::from_utf8(buf).unwrap()
            };
            assert_eq!(auth_token.len(), 64);
            assert!(session_token != auth_token);

            // --------------------
            // CLEANUP
            // --------------------
            cleanup_settings(session_state);

            assert!(!auth_filepath.exists());
            assert!(!auth_filepath.parent().unwrap().exists());
        }
    }
}


// ===========================================================================
//
// ===========================================================================
