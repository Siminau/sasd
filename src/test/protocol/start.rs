// src/test/protocol/start.rs
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
            // a Start state object
            // -----------------------------
            // Build Message
            let msgtype = Value::from(MessageType::Request.to_number());
            let msgid = Value::from(42);
            let msgcode = Value::from(code);
            let msgargs = Value::Array(vec![Value::from(1)]);
            let val = Value::Array(vec![msgtype, msgid, msgcode, msgargs]);
            let msg = Message::from(val).unwrap();

            // Create Start state object
            let start = Box::new(Start);

            // -----------------------------------------------------------
            // WHEN
            // Start::change() is called with the message
            // -----------------------------------------------------------
            let result = start.change(msg);

            // ---------------------------------------
            // THEN
            // dispatch is called
            // ---------------------------------------
            let value = match result {
                Err(err) => matches!(*err.kind(), SasdErrorKind::UnexpectedMessage),
                _ => false,
            };

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
        let start = Start;

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
