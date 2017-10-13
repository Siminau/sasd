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

    mod from_value {
        use error::SasdErrorKind;
        use protocol::StateValue;
        use protocol::v1::{InitSession, Session, StateValue as V1StateValue};

        #[test]
        fn is_initsession()
        {

            // ------------------------
            // GIVEN
            // a StateValue::V1(InitSession) instance
            // ------------------------
            let value =
                StateValue::V1(V1StateValue::InitSession(InitSession::new()));

            // -------------------------------
            // WHEN
            // InitSession::from_value() is called with the StateValue value
            // -------------------------------
            let result = InitSession::from_value(value);

            // --------------------------------------------------
            // THEN
            // the value wrapped in the StateValue variant is returned
            // --------------------------------------------------
            let testval = match result {
                Ok(_) => true,
                _ => false,
            };
            assert!(testval);
        }

        #[test]
        fn not_initsession()
        {

            // ------------------------
            // GIVEN
            // a StateValue::V1(Session) instance
            // ------------------------
            let value = StateValue::V1(V1StateValue::Session(Session::new()));

            // -------------------------------
            // WHEN
            // InitSession::from_value() is called with the StateValue value
            // -------------------------------
            let result = InitSession::from_value(value);

            // --------------------------------------------------
            // THEN
            // a SasdErrorKind::InvalidStateValue error is returned
            // --------------------------------------------------
            let testval = match result {
                Err(e) => {
                    match e.kind() {
                        &SasdErrorKind::InvalidStateValue(_, _) => {
                            let expected = "Invalid StateValue: expected \
                                            StateValue::V1(InitSession), got \
                                            StateValue::V1(Session(Session)) \
                                            instead"
                                .to_owned();
                            assert_eq!(e.to_string(), expected);
                            true
                            // e.to_string() == expected
                        }
                        _ => false,
                    }
                }
                _ => false,
            };
            assert!(testval);
        }
    }

    mod can_skip_auth {
        use os::windows::protocol::SessionStore;
        use protocol::{State, StateValue};
        use protocol::v1::{InitSession, SessionRequest, SessionResponse,
                           StateValue as V1StateValue};
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
            let auth_token = "world".to_owned();
            let msgargs = vec![Value::from(auth_token.clone())];
            let request =
                SessionRequest::new(42, SessionMethod::Attach, msgargs);

            // Create state
            let mut init = InitSession::new();

            // Create session state
            let dummy =
                StateValue::V1(V1StateValue::InitSession(InitSession::new()));

            let settings = new_settings(
                1234,
                None,
                WindowsSection {
                    token_data_dir: PathBuf::from("/does/not/exist"),
                },
            );
            let settings_handle = new_settings_handle(settings);
            let session_store = SessionStore {
                auth_token: auth_token,
                auth_file: None,
            };
            let mut session_state =
                SessionState::new(session_store, settings_handle, dummy);
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

            assert!(state.is_v1());
            assert!(state.as_v1().unwrap().is_session());
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
            let dummy =
                StateValue::V1(V1StateValue::InitSession(InitSession::new()));

            let mut session_state = dummy_session_state(dummy);

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
            // the state is V1StateValue::AuthSession and
            // the response has Nil for its error and
            // the response has session and auth tokens for its result
            // --------------------------------------------------------
            let response = SessionResponse::from(msg).unwrap();

            assert!(state.is_v1());
            assert!(state.as_v1().unwrap().is_authsession());
            assert_eq!(response.error_code(), SessionError::Nil);

            // This is a &Vec<Value>
            let result = response.result().as_array().unwrap();

            assert_eq!(result.len(), 1);

            let auth_filepath = PathBuf::from(result[0].as_str().unwrap());

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

            // --------------------
            // CLEANUP
            // --------------------
            cleanup_settings(session_state);

            assert!(!auth_filepath.exists());
            assert!(!auth_filepath.parent().unwrap().exists());
        }
    }
}


mod authsession {

    mod check_msg_method {
        use error::SasdErrorKind;
        use protocol::{State, StateValue};
        use protocol::v1::{AuthSession, SessionRequest,
                           StateValue as V1StateValue};
        use quickcheck::TestResult;
        use rmpv::Value;
        use rpc::v1::SessionMethod;

        // Helpers

        use test::protocol::dummy_session_state_nofs;

        #[test]
        fn non_authattach_msg_error()
        {
            // -------------------------------------------------------
            // GIVEN
            // a SessionRequest message and
            // the message method is not AuthAttach and
            // an empty sessionstore and
            // an InitSession instance
            // -------------------------------------------------------
            let request =
                SessionRequest::new(42, SessionMethod::Attach, vec![]);

            // Create state
            let mut auth = AuthSession::new();

            // Create session state
            let dummy =
                StateValue::V1(V1StateValue::AuthSession(AuthSession::new()));

            let mut session_state = dummy_session_state_nofs(dummy);

            // ------------------------------------------------------------
            // WHEN
            // AuthSession::dispatch() is called with the sessionstore and
            // message
            // ------------------------------------------------------------
            let result = {
                let mut handle = session_state.handle();
                auth.dispatch(&mut handle, request.into())
            };

            // ----------------------------------------------------
            // THEN
            // An error is returned and
            // the error is UnexpectedMessage
            // ----------------------------------------------------
            let testval = match result {
                Ok(_) => false,
                Err(e) => {
                    match e.kind() {
                        &SasdErrorKind::UnexpectedMessage => true,
                        _ => false,
                    }
                }
            };
            assert!(testval);
        }

        quickcheck! {
            fn authattach_args_error(numargs: usize) -> TestResult
            {
                if numargs == 1 {
                    return TestResult::discard()
                }

                // -------------------------------------------------------
                // GIVEN
                // a SessionRequest message and
                // the message method is AuthAttach and
                // the message has a number of args != 1 and
                // an empty sessionstore and
                // an InitSession instance
                // -------------------------------------------------------
                // Setup args
                let mut args = Vec::new();
                for i in 0..numargs {
                    args.push(Value::from(i));
                }

                // Create request message
                let request =
                    SessionRequest::new(42, SessionMethod::AuthAttach, args);

                // Create state
                let mut auth = AuthSession::new();

                // Create session state
                let dummy =
                    StateValue::V1(V1StateValue::AuthSession(AuthSession::new()));

                let mut session_state = dummy_session_state_nofs(dummy);

                // ------------------------------------------------------------
                // WHEN
                // AuthSession::dispatch() is called with the sessionstore and
                // message
                // ------------------------------------------------------------
                let result = {
                    let mut handle = session_state.handle();
                    auth.dispatch(&mut handle, request.into())
                };

                // ----------------------------------------------------
                // THEN
                // An error is returned and
                // the error is InvalidMessage
                // ----------------------------------------------------
                let testval = match result {
                    Ok(_) => false,
                    Err(e) => {
                        match e.kind() {
                            &SasdErrorKind::InvalidMessage => true,
                            _ => false,
                        }
                    }
                };
                TestResult::from_bool(testval)
            }
        }

    }

    mod auth_attach {
        use protocol::{State, StateValue};
        use protocol::v1::{AuthSession, SessionRequest, SessionResponse,
                           StateValue as V1StateValue};
        use rmpv::{Utf8String, Value};
        use rpc::v1::{SessionError, SessionMethod};
        use siminau_rpc::message::response::RpcResponse;

        // Helpers

        use test::protocol::dummy_session_state_nofs;

        #[test]
        fn auth_token_nomatch()
        {
            // -------------------------------------------------------
            // GIVEN
            // an auth token and
            // a SessionRequest message and
            // the message method is AuthAttach and
            // a sessionstore containing a non-matching auth token and
            // an InitSession instance
            // -------------------------------------------------------
            let auth_token = "HELLO".to_owned();
            let request =
                SessionRequest::new(
                    42,
                    SessionMethod::AuthAttach,
                    vec![Value::String(Utf8String::from(&auth_token[..]))],
                );

            // Create state
            let mut auth = AuthSession::new();
            let dummy =
                StateValue::V1(V1StateValue::AuthSession(AuthSession::new()));

            // Create session state
            let mut session_state = dummy_session_state_nofs(dummy);

            // Assign tokens to session_state
            session_state.session_store().auth_token = "NOTCORRECT".to_owned();

            // ------------------------------------------------------------
            // WHEN
            // AuthSession::dispatch() is called with the sessionstore and
            // message
            // ------------------------------------------------------------
            let (_, msg) = {
                let mut handle = session_state.handle();
                auth.dispatch(&mut handle, request.into()).unwrap()
            };

            // ----------------------------------------------------
            // THEN
            // An error response is returned and
            // the response's error code is InvalidAttach and
            // the response's result is the str "auth token doesn't match"
            // ----------------------------------------------------
            let resp = SessionResponse::from(msg.unwrap()).unwrap();
            assert_eq!(resp.error_code(), SessionError::InvalidAttach);

            let result = resp.result().as_str().unwrap();
            assert_eq!(result, "auth token doesn't match");
        }

        #[test]
        fn auth_token_match()
        {
            // -------------------------------------------------------
            // GIVEN
            // an auth token and
            // a SessionRequest message and
            // the message method is AuthAttach and
            // a sessionstore containing a matching auth token and
            // an InitSession instance
            // -------------------------------------------------------
            let auth_token = "HELLO".to_owned();
            let request =
                SessionRequest::new(
                    42,
                    SessionMethod::AuthAttach,
                    vec![Value::String(Utf8String::from(&auth_token[..]))],
                );

            // Create state
            let mut auth = AuthSession::new();
            let dummy =
                StateValue::V1(V1StateValue::AuthSession(AuthSession::new()));

            // Create session state
            let mut session_state = dummy_session_state_nofs(dummy);

            // Assign tokens to session_state
            session_state.session_store().auth_token = auth_token;

            // ------------------------------------------------------------
            // WHEN
            // AuthSession::dispatch() is called with the sessionstore and
            // message
            // ------------------------------------------------------------
            let (state, msg) = {
                let mut handle = session_state.handle();
                auth.dispatch(&mut handle, request.into()).unwrap()
            };

            // ----------------------------------------------------
            // THEN
            // A non-error response is returned and
            // the response's error code is Nil and
            // the response's result is Nil
            // ----------------------------------------------------
            // Check response
            let resp = SessionResponse::from(msg.unwrap()).unwrap();
            assert_eq!(resp.error_code(), SessionError::Nil);
            assert!(resp.result().is_nil());

            // Check state
            let testval = match state {
                Some(StateValue::V1(ref v1)) => v1.is_session(),
                _ => false,
            };
            assert!(testval);
        }
    }
}


// ===========================================================================
//
// ===========================================================================
