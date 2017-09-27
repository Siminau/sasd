// src/test/protocol/start.rs
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

// Local imports

// use protocol::ConfigHandle;

use super::*;
// use util::create_config;

// ===========================================================================
// Test Start::dispatch()
// ===========================================================================


mod dispatch {
    use super::*;
    use protocol::Start;
    use quickcheck::TestResult;
    use siminau_rpc::message::CodeConvert;

    quickcheck! {
        fn error_on_unexpected_message(code: u64) -> TestResult
        {
            if code <= MessageType::max_number() {
                return TestResult::discard()
            }

            // -----------------------------
            // GIVEN
            // A message w/ message type Request and
            // the message's 3rd argument is an unknown method code number and
            // a Start state object and
            // an empty Config object
            // -----------------------------
            // Build Message
            let msgtype = Value::from(MessageType::Request.to_number());
            let msgid = Value::from(42);
            let msgcode = Value::from(code);
            let msgargs = Value::Array(vec![Value::from(1)]);
            let val = Value::Array(vec![msgtype, msgid, msgcode, msgargs]);
            let msg = Message::from(val).unwrap();

            // Create Test state object
            let mut test = Box::new(Start);

            // This dummy is only for testing since cannot access the
            // state that's attached to the session state
            let dummy = Box::new(Start);

            // Create session state
            let mut session_state = dummy_session_state(dummy);

            // -----------------------------------------------------------
            // WHEN
            // Start::change() is called with the message
            // -----------------------------------------------------------
            let result = test.change(session_state.handle(), msg);

            // ---------------------------------------
            // THEN
            // dispatch is called
            // ---------------------------------------
            let value = match result {
                Err(err) => matches!(*err.kind(), SasdErrorKind::UnexpectedMessage),
                _ => false,
            };

            // --------------------
            // Cleanup
            // --------------------
            cleanup_settings(session_state);

            TestResult::from_bool(value)
        }
    }
}


mod kind {
    use super::*;
    use protocol::Start;

    #[test]
    fn kind_is_start()
    {
        // -----------------------------
        // GIVEN
        // a Start state object
        // -----------------------------
        // Create Start state object
        let start = Start::new();

        // -----------------------------------------------------------
        // WHEN
        // Start::kind() is called
        // -----------------------------------------------------------
        let result = start.kind();

        // ---------------------------------------
        // THEN
        // StateKind::Start is returned
        // ---------------------------------------
        assert!(matches!(result, StateKind::Start));
    }
}


// ===========================================================================
//
// ===========================================================================
