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
            let mut test = Start::new();

            // This dummy is only for testing since cannot access the
            // state that's attached to the session state
            let dummy = StateValue::Start(Start::new());

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


mod from {
    use super::*;
    use protocol::Start;

    #[test]
    fn convert_to_statevalue()
    {
        // -----------------------------
        // GIVEN
        // a Start state object
        // -----------------------------
        // Create Start state object
        let start = Start::new();

        // -----------------------------------------------------------
        // WHEN
        // StateValue::from() is called
        // -----------------------------------------------------------
        let result = StateValue::from(start);

        // ---------------------------------------
        // THEN
        // StateKind::Start is returned
        // ---------------------------------------
        assert!(matches!(result, StateValue::Start(_)));
    }
}


mod statevalue_is_start {
    use protocol::{Start, StateValue};
    use protocol::v1;

    #[test]
    fn is_start_true()
    {
        // --------------------------
        // GIVEN
        // a StateValue::Start value
        // --------------------------
        let val = StateValue::Start(Start::new());

        // --------------------
        // WHEN
        // StateValue::is_start() is called
        // --------------------
        let result = val.is_start();

        // --------------------
        // THEN
        // true is returned
        // --------------------
        assert!(result);
    }

    #[test]
    fn is_start_false()
    {
        // --------------------------
        // GIVEN
        // a StateValue::V1(v1::StateValue::Session) value
        // --------------------------
        let state = v1::Session::new();
        let val = StateValue::V1(v1::StateValue::Session(state));

        // --------------------
        // WHEN
        // StateValue::is_start() is called
        // --------------------
        let result = val.is_start();

        // --------------------
        // THEN
        // false is returned
        // --------------------
        assert!(!result);
    }
}


// ===========================================================================
//
// ===========================================================================
