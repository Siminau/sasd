// src/test/protocol/statevalue.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

// Local imports


mod statevalue {

    mod is_v1 {
        use protocol::{Start, StateValue};
        use protocol::v1;

        #[test]
        fn return_true()
        {
            // --------------------
            // GIVEN
            // a StateValue::V1 value
            // --------------------
            let val =
                StateValue::V1(v1::StateValue::Session(v1::Session::new()));

            // --------------------
            // WHEN
            // StateValue::is_v1() is called
            // --------------------
            let result = val.is_v1();

            // --------------------
            // THEN
            // true is returned
            // --------------------
            assert!(result);
        }

        #[test]
        fn return_false()
        {
            // --------------------
            // GIVEN
            // a StateValue::Start value
            // --------------------
            let val = StateValue::Start(Start::new());

            // --------------------
            // WHEN
            // StateValue::is_v1() is called
            // --------------------
            let result = val.is_v1();

            // --------------------
            // THEN
            // false is returned
            // --------------------
            assert!(!result);
        }
    }

    mod is_start {
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

    mod as_v1 {
        use protocol::{Start, StateValue};
        use protocol::v1;

        #[test]
        fn return_v1_statevalue()
        {
            // --------------------------
            // GIVEN
            // a StateValue::V1(v1::StateValue::Session) value
            // --------------------------
            let state = v1::Session::new();
            let val = StateValue::V1(v1::StateValue::Session(state));
            let sessref = match val {
                StateValue::V1(ref s) => s,
                _ => unreachable!(),
            };

            // --------------------
            // WHEN
            // StateValue::as_v1() is called
            // --------------------
            let result = val.as_v1();

            // --------------------
            // THEN
            // a reference to the v1::StateValue::Session object is returned
            // --------------------
            let testval = match result {
                None => false,

                // This is testing pointer equality to determine if the
                // references are pointing to the same object
                Some(v) => sessref as *const _ == v as *const _,
            };

            assert!(testval);
        }

        #[test]
        fn return_none()
        {
            // --------------------------
            // GIVEN
            // a StateValue::Start value
            // --------------------------
            let val = StateValue::Start(Start::new());

            // --------------------
            // WHEN
            // StateValue::as_v1() is called
            // --------------------
            let result = val.as_v1();

            // --------------------
            // THEN
            // None is returned
            // --------------------
            let testval = match result {
                None => true,
                Some(_) => false,
            };

            assert!(testval);
        }
    }

    mod as_start {
        use protocol::{Start, StateValue};
        use protocol::v1;

        #[test]
        fn return_ref_to_start()
        {
            // --------------------------
            // GIVEN
            // a StateValue::Start value
            // --------------------------
            let val = StateValue::Start(Start::new());
            let stateref = match val {
                StateValue::Start(ref s) => s,
                _ => unreachable!(),
            };

            // --------------------
            // WHEN
            // StateValue::as_start() is called
            // --------------------
            let result = val.as_start();

            // --------------------
            // THEN
            // a reference to the Start object is returned
            // --------------------
            let testval = match result {
                None => false,

                // This is testing pointer equality to determine if the
                // references are pointing to the same object
                Some(v) => stateref as *const _ == v as *const _,
            };

            assert!(testval);
        }

        #[test]
        fn return_none()
        {
            // --------------------------
            // GIVEN
            // a StateValue::V1(v1::StateValue::Session) value
            // --------------------------
            let state = v1::Session::new();
            let val = StateValue::V1(v1::StateValue::Session(state));

            // --------------------
            // WHEN
            // StateValue::as_start() is called
            // --------------------
            let result = val.as_start();

            // --------------------
            // THEN
            // None is returned
            // --------------------
            let testval = match result {
                None => true,
                Some(_) => false,
            };

            assert!(testval);
        }
    }

    mod to_v1 {
        use protocol::{Start, StateValue};
        use protocol::v1::{Session, StateValue as V1StateValue};

        #[test]
        fn return_v1()
        {
            // --------------------------
            // GIVEN
            // a v1::StateValue::Session value
            // --------------------------
            let val = StateValue::V1(V1StateValue::Session(Session::new()));

            // --------------------
            // WHEN
            // StateValue::to_start() is called
            // --------------------
            let result = val.to_v1();

            // --------------------
            // THEN
            // the V1StateValue object is returned
            // --------------------
            let testval = match result {
                Some(s) => {
                    let state: V1StateValue = s;
                    state.is_session()
                }
                None => false,
            };

            assert!(testval);
        }

        #[test]
        fn return_none()
        {
            // --------------------------
            // GIVEN
            // a StateValue::Start value
            // --------------------------
            let val = StateValue::Start(Start::new());

            // --------------------
            // WHEN
            // StateValue::to_v1() is called
            // --------------------
            let result = val.to_v1();

            // --------------------
            // THEN
            // None is returned
            // --------------------
            let testval = match result {
                None => true,
                Some(_) => false,
            };

            assert!(testval);
        }
    }

    mod to_start {
        use protocol::{Start, StateValue};
        use protocol::v1;

        #[test]
        fn return_start()
        {
            // --------------------------
            // GIVEN
            // a StateValue::Start value
            // --------------------------
            let val = StateValue::Start(Start::new());

            // --------------------
            // WHEN
            // StateValue::to_start() is called
            // --------------------
            let result = val.to_start();

            // --------------------
            // THEN
            // the Start object is returned
            // --------------------
            let testval = match result {
                Some(s) => {
                    let _state: Start = s;
                    true
                }
                None => false,
            };

            assert!(testval);
        }

        #[test]
        fn return_none()
        {
            // --------------------------
            // GIVEN
            // a StateValue::V1(v1::StateValue::Session) value
            // --------------------------
            let state = v1::Session::new();
            let val = StateValue::V1(v1::StateValue::Session(state));

            // --------------------
            // WHEN
            // StateValue::to_start() is called
            // --------------------
            let result = val.to_start();

            // --------------------
            // THEN
            // None is returned
            // --------------------
            let testval = match result {
                None => true,
                Some(_) => false,
            };

            assert!(testval);
        }
    }
}


// ===========================================================================
//
// ===========================================================================
