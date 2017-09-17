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
    use siminau_rpc::message::CodeConvert;

    // Error if a response message received
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

        // Create Test state object
        let test = Box::new(Test);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_version() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_version(msg);

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

    #[test]
    fn badmsg()
    {
        // -----------------------------------------------
        // GIVEN
        // A message w/ MessageType::Request and
        // only 3 parameters and
        // A state object
        // -----------------------------------------------
        let test = Box::new(Test);

        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(rpc::RequestMethod::Version.to_number());
        let msgval = Value::Array(vec![msgtype, msgid, msgmeth]);
        let msg = Message::from(msgval).unwrap();

        // -----------------------------------------------------------
        // WHEN
        // State::handle_version() is called with the response message
        // -----------------------------------------------------------
        let result = test.handle_version(msg);

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
        let test = Box::new(Test);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_version() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_version(msg);

        // ---------------------------------------
        // THEN
        // dispatch is called
        // ---------------------------------------
        let value = match result {
            Ok((None, None)) => true,
            _ => false,
        };

        assert!(value);
    }

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
        let test = Box::new(Test);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_version() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_version(request.into());

        // ---------------------------------------
        // THEN
        // version() is called
        // ---------------------------------------
        let value = match result {
            Ok((Some(s), None)) => {
                s.kind() == StateKind::V1(v1::V1StateKind::InitSession)
            }
            _ => false,
        };

        assert!(value);
    }

}

// ===========================================================================
// Test State::handle_version()
// ===========================================================================


mod handle_done {
    use super::*;
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
        let test = Box::new(Test);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_done() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_done(msg);

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

    #[test]
    fn badmsg()
    {
        // -----------------------------------------------
        // GIVEN
        // A message w/ MessageType::Request and
        // only 2 parameters and
        // A state object
        // -----------------------------------------------
        let test = Box::new(Test);

        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(rpc::Notice::Done.to_number());
        let msgargs = Value::Array(vec![]);
        let msgval = Value::Array(vec![msgtype, msgmeth, msgargs, msgid]);
        let msg = Message::from(msgval).unwrap();

        // -----------------------------------------------------------
        // WHEN
        // State::handle_done() is called with the response message
        // -----------------------------------------------------------
        let result = test.handle_done(msg);

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
        let test = Box::new(Test);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_done() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_done(msg);

        // ---------------------------------------
        // THEN
        // dispatch is called
        // ---------------------------------------
        let value = match result {
            Ok((None, None)) => true,
            _ => false,
        };

        assert!(value);
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
        let test = Box::new(Test);

        // -----------------------------------------------------------
        // WHEN
        // Test::handle_done() is called with the message
        // -----------------------------------------------------------
        let result = test.handle_done(info.into());

        // ---------------------------------------
        // THEN
        // (None, None) is returned
        // ---------------------------------------
        let value = match result {
            Ok((None, None)) => true,
            _ => false,
        };

        assert!(value);
    }

}


// ===========================================================================
// Test State::change()
// ===========================================================================


mod change {
    use super::*;
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
        let test = Box::new(Test);

        // -----------------------------------------------------------
        // WHEN
        // Test::change() is called with the message
        // -----------------------------------------------------------
        let result = test.change(resp.into());

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
            fn dispatch(&self, _msg: Message)
                -> SasdResult<(Option<BoxState>, Option<Message>)>
            {
                unreachable!()
            }

            fn kind(&self) -> StateKind
            {
                unreachable!()
            }

            fn handle_version(&self, _msg: Message)
                -> SasdResult<(Option<BoxState>, Option<Message>)>
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
        let test = Box::new(Test);

        // -----------------------------------------------------------
        // WHEN
        // Test::change() is called with the message
        // -----------------------------------------------------------
        let result = test.change(req.into());

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
            fn dispatch(&self, _msg: Message)
                -> SasdResult<(Option<BoxState>, Option<Message>)>
            {
                unreachable!()
            }

            fn kind(&self) -> StateKind
            {
                unreachable!()
            }

            fn handle_done(&self, _msg: Message)
                -> SasdResult<(Option<BoxState>, Option<Message>)>
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
        let test = Box::new(Test);

        // -----------------------------------------------------------
        // WHEN
        // Test::change() is called with the message
        // -----------------------------------------------------------
        let result = test.change(info.into());

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
    }
}

// ===========================================================================
//
// ===========================================================================
