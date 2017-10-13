// src/test/protocol/state.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports


// Local imports

use super::*;


// ===========================================================================
// Test State::handle_version()
// ===========================================================================


mod handle_version {
    use super::*;
    use protocol::Start;
    use siminau_rpc::message::CodeConvert;

    // --------------------
    // Tests
    // --------------------

    // Error if a response message received
    // TODO: this is an integration test, should it be here?
    #[test]
    fn method_non_u64()
    {
        // -----------------------------
        // GIVEN
        // A message w/ message type Request and
        // the message's 3rd argument is a non-u64 and
        // A state object
        // -----------------------------
        // Build Message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgcode = Value::from("hello");
        let msgargs = Value::Array(vec![]);
        let val = Value::Array(vec![msgtype, msgid, msgcode, msgargs]);
        let msg = Message::from(val).unwrap();

        // Create Start state object
        let mut test = Start::new();

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_version() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_version(&mut session_state.handle(), msg);

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

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }

    #[test]
    fn badmsg()
    {
        // -----------------------------------------------
        // GIVEN
        // A message w/ MessageType::Request and
        // only 3 parameters and
        // A state object
        // -----------------------------------------------
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(rpc::RequestMethod::Version.to_number());
        let msgval = Value::Array(vec![msgtype, msgid, msgmeth]);
        let msg = Message::from(msgval).unwrap();

        // Create Test state object
        let mut test = Start::new();

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // State::handle_version() is called with the response message
        // -----------------------------------------------------------
        let result = test.handle_version(&mut session_state.handle(), msg);

        // ---------------------------------------
        // THEN
        // an InvalidArrayLength error is returned
        // ---------------------------------------
        let value = match result {
            Err(e) => {
                match *e.kind() {
                    SasdErrorKind::Net(RpcErrorKind::InvalidArrayLength(ref msg)) => {
                        assert_eq!(msg, "expected array length of 4, got 3");
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        };
        assert!(value);

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }

    #[test]
    fn unknown_method_code()
    {
        // -----------------------------
        // GIVEN
        // A message w/ message type Request and
        // the message's 3rd argument is an unknown method code number and
        // A state object
        // -----------------------------
        // Build Message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgcode = Value::from(42);
        let msgargs = Value::Array(vec![Value::from(1)]);
        let val = Value::Array(vec![msgtype, msgid, msgcode, msgargs]);
        let msg = Message::from(val).unwrap();

        // Create Test state object
        let mut test = Test;

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_version() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_version(&mut session_state.handle(), msg);

        // ---------------------------------------
        // THEN
        // dispatch is called
        // ---------------------------------------
        let value = match result {
            Ok((None, None)) => true,
            _ => false,
        };

        assert!(value);

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }

    #[cfg(windows)]
    #[test]
    fn call_version()
    {
        // -----------------------------
        // GIVEN
        // A valid Request message and
        // A state object
        // -----------------------------
        let args = vec![Value::from(Protocol::V1.to_u64())];
        let request = Request::new(42, rpc::RequestMethod::Version, args);

        // Create Test state object
        let mut test = Test;

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_version() is called with the message
        // -----------------------------------------------------------
        let result =
            test.handle_version(&mut session_state.handle(), request.into());

        // ---------------------------------------
        // THEN
        // version() is called
        // ---------------------------------------
        let value = match result {
            Ok((Some(StateValue::V1(ref v)), None)) => v.is_initsession(),
            _ => false,
        };

        assert!(value);

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }

    #[cfg(unix)]
    #[test]
    fn call_version()
    {
        // -----------------------------
        // GIVEN
        // A valid Request message and
        // A state object
        // -----------------------------
        let args = vec![Value::from(Protocol::V1.to_u64())];
        let request = Request::new(42, rpc::RequestMethod::Version, args);

        // Create Test state object
        let mut test = Test;

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_version() is called with the message
        // -----------------------------------------------------------
        let result =
            test.handle_version(&mut session_state.handle(), request.into());

        // ---------------------------------------
        // THEN
        // version() is called
        // ---------------------------------------
        let value = match result {
            Ok((Some(StateValue::V1(ref v)), None)) => v.is_session(),
            _ => false,
        };

        assert!(value);

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }
}

// ===========================================================================
// Test State::handle_version()
// ===========================================================================


mod handle_done {
    use super::*;
    use protocol::Start;
    use siminau_rpc::message::CodeConvert;

    #[test]
    fn method_non_u64()
    {
        // -----------------------------
        // GIVEN
        // A message w/ message type Notification and
        // the message's 2nd argument is a non-u64 and
        // A state object
        // -----------------------------
        // Build Message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from("hello");
        let msgargs = Value::Array(vec![]);
        let val = Value::Array(vec![msgtype, msgcode, msgargs]);
        let msg = Message::from(val).unwrap();

        // Create Test state object
        let mut test = Test;

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_done() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_done(&mut session_state.handle(), msg);

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

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }

    #[test]
    fn badmsg()
    {
        // -----------------------------------------------
        // GIVEN
        // A message w/ MessageType::Request and
        // only 2 parameters and
        // A state object
        // -----------------------------------------------
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(rpc::Notice::Done.to_number());
        let msgargs = Value::Array(vec![]);
        let msgval = Value::Array(vec![msgtype, msgmeth, msgargs, msgid]);
        let msg = Message::from(msgval).unwrap();

        // Create Test state object
        let mut test = Test;

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // State::handle_done() is called with the response message
        // -----------------------------------------------------------
        let result = test.handle_done(&mut session_state.handle(), msg);

        // ---------------------------------------
        // THEN
        // an InvalidArrayLength error is returned
        // ---------------------------------------
        let value = match result {
            Err(e) => {
                match *e.kind() {
                    SasdErrorKind::Net(RpcErrorKind::InvalidArrayLength(ref msg)) => {
                        assert_eq!(msg, "expected array length of 3, got 4");
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        };
        assert!(value);

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }

    #[test]
    fn unknown_method_code()
    {
        // -----------------------------
        // GIVEN
        // A message w/ message type Notification and
        // the message's 2nd argument is an unknown method code number and
        // A state object
        // -----------------------------
        // Build Message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(42);
        let msgargs = Value::Array(vec![]);
        let val = Value::Array(vec![msgtype, msgcode, msgargs]);
        let msg = Message::from(val).unwrap();

        // Create Test state object
        let mut test = Test;

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_done() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_done(&mut session_state.handle(), msg);

        // ---------------------------------------
        // THEN
        // dispatch is called
        // ---------------------------------------
        let value = match result {
            Ok((None, None)) => true,
            _ => false,
        };

        assert!(value);

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }

    #[test]
    fn return_none()
    {
        // -----------------------------
        // GIVEN
        // A valid Info message and
        // A state object
        // -----------------------------
        let args = vec![Value::from(Protocol::V1.to_u64())];
        let info = Info::new(rpc::Notice::Done, args);

        // Create Test state object
        let mut test = Test;

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_done() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_done(&mut session_state.handle(), info.into());

        // ---------------------------------------
        // THEN
        // (None, None) is returned
        // ---------------------------------------
        let value = match result {
            Ok((None, None)) => true,
            _ => false,
        };

        assert!(value);

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }
}


// ===========================================================================
// Test State::change()
// ===========================================================================


mod change {
    use super::*;
    use protocol::Start;

    use siminau_rpc::message::response::RpcResponse;

    #[test]
    fn response_message()
    {
        // -----------------------------
        // GIVEN
        // A valid Response message and
        // A state object
        // -----------------------------
        let res = Value::from(true);
        let resp =
            Response::new(42, rpc::ResponseError::UnsupportedVersion, res);

        // Create Test state object
        let mut test = Test;

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // Test::change() is called with the message
        // -----------------------------------------------------------
        let result = test.change(session_state.handle(), resp.into());

        // ---------------------------------------
        // THEN
        // an UnexpectedMessage error is returned
        // ---------------------------------------
        let value = match result {
            Err(e) => {
                match *e.kind() {
                    SasdErrorKind::UnexpectedMessage => true,
                    _ => false,
                }
            }
            _ => false,
        };

        assert!(value);

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }

    #[test]
    fn call_handle_version()
    {
        // -----------------------------
        // GIVEN
        // A valid Request message and
        // A state object
        // -----------------------------
        let args = vec![Value::from(1)];
        let req = Request::new(42, rpc::RequestMethod::Version, args);

        struct Test;

        impl State for Test {
            fn dispatch(&mut self, _state: &mut SessionStateHandle, _msg: Message)
                -> SasdResult<(Option<StateValue>, Option<Message>)>
            {
                unreachable!()
            }

            fn handle_version(&mut self, _state: &mut SessionStateHandle, _msg: Message)
                -> SasdResult<(Option<StateValue>, Option<Message>)>
            {
                let res = Value::from("Hello world!");
                let resp = Response::new(
                    42,
                    rpc::ResponseError::UnsupportedVersion,
                    res,
                );
                Ok((None, Some(resp.into())))
            }
        }

        // Create Test state object
        let mut test = Test;

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // Test::change() is called with the message
        // -----------------------------------------------------------
        let result = test.change(session_state.handle(), req.into());

        // ---------------------------------------
        // THEN
        // Test::handle_version() is called
        // ---------------------------------------
        let value = match result {
            Ok((None, Some(msg))) => {
                let resp = Response::from(msg).unwrap();
                resp.result().as_str().unwrap() == "Hello world!"
            }
            _ => false,
        };

        assert!(value);

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }

    #[test]
    fn call_handle_done()
    {
        // -----------------------------
        // GIVEN
        // A valid Notice message and
        // A state object
        // -----------------------------
        let info = Info::new(rpc::Notice::Done, vec![]);

        struct Test;

        impl State for Test {
            fn dispatch(&mut self, _state: &mut SessionStateHandle, _msg: Message)
                -> SasdResult<(Option<StateValue>, Option<Message>)>
            {
                unreachable!()
            }

            fn handle_done(&mut self, _state: &mut SessionStateHandle, _msg: Message)
                -> SasdResult<(Option<StateValue>, Option<Message>)>
            {
                let res = Value::from("Answer 42");
                let resp = Response::new(
                    42,
                    rpc::ResponseError::UnsupportedVersion,
                    res,
                );
                Ok((None, Some(resp.into())))
            }
        }

        // Create Test state object
        let mut test = Test;

        // This dummy is only for testing since cannot access the
        // state that's attached to the session state
        let dummy = StateValue::Start(Start::new());

        // Create session state
        let mut session_state = dummy_session_state(dummy);

        // -----------------------------------------------------------
        // WHEN
        // Test::change() is called with the message
        // -----------------------------------------------------------
        let result = test.change(session_state.handle(), info.into());

        // ---------------------------------------
        // THEN
        // Test::handle_done() is called
        // ---------------------------------------
        let value = match result {
            Ok((None, Some(msg))) => {
                let resp = Response::from(msg).unwrap();
                resp.result().as_str().unwrap() == "Answer 42"
            }
            _ => false,
        };

        assert!(value);

        // --------------------
        // Cleanup
        // --------------------
        cleanup_settings(session_state);
    }
}

// ===========================================================================
//
// ===========================================================================
