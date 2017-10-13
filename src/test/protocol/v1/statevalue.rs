// src/test/protocol/v1/statevalue.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

// Local imports

// ===========================================================================
// Test is_* methods
// ===========================================================================


mod is_session {
    use protocol::StateValue;
    use protocol::v1::{Session, StateValue as V1StateValue};

    #[cfg(windows)]
    use protocol::v1::InitSession;

    #[test]
    fn return_true()
    {
        // --------------------
        // GIVEN
        // a v1::StateValue::Session value
        // --------------------
        let val = StateValue::V1(V1StateValue::Session(Session::new()));

        // --------------------
        // WHEN
        // StateValue::is_session() is called
        // --------------------
        let result = val.as_v1().unwrap().is_session();

        // --------------------
        // THEN
        // true is returned
        // --------------------
        assert!(result);
    }

    // TODO: Windows only for now since unix only has a single variant for
    // v1::StateValue as of this writing.
    #[cfg(windows)]
    #[test]
    fn return_false()
    {
        // --------------------
        // GIVEN
        // a v1::StateValue::InitSession value
        // --------------------
        let val = StateValue::V1(V1StateValue::InitSession(InitSession::new()));

        // --------------------
        // WHEN
        // StateValue::is_session() is called
        // --------------------
        let result = val.as_v1().unwrap().is_session();

        // --------------------
        // THEN
        // false is returned
        // --------------------
        assert!(!result);
    }
}


#[cfg(windows)]
mod is_initsession {
    use protocol::StateValue;
    use protocol::v1::{Session, StateValue as V1StateValue};

    #[cfg(windows)]
    use protocol::v1::InitSession;

    #[test]
    fn return_true()
    {
        // --------------------
        // GIVEN
        // a v1::StateValue::InitSession value
        // --------------------
        let val = StateValue::V1(V1StateValue::InitSession(InitSession::new()));

        // --------------------
        // WHEN
        // StateValue::is_initsession() is called
        // --------------------
        let result = val.as_v1().unwrap().is_initsession();

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
        // a v1::StateValue::Session value
        // --------------------
        let val = StateValue::V1(V1StateValue::Session(Session::new()));

        // --------------------
        // WHEN
        // StateValue::is_initsession() is called
        // --------------------
        let result = val.as_v1().unwrap().is_initsession();

        // --------------------
        // THEN
        // false is returned
        // --------------------
        assert!(!result);
    }
}


#[cfg(windows)]
mod is_authsession {
    use protocol::StateValue;
    use protocol::v1::{Session, StateValue as V1StateValue};

    #[cfg(windows)]
    use protocol::v1::AuthSession;

    #[test]
    fn return_true()
    {
        // --------------------
        // GIVEN
        // a v1::StateValue::AuthSession value
        // --------------------
        let val = StateValue::V1(V1StateValue::AuthSession(AuthSession::new()));

        // --------------------
        // WHEN
        // StateValue::is_authsession() is called
        // --------------------
        let result = val.as_v1().unwrap().is_authsession();

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
        // a v1::StateValue::Session value
        // --------------------
        let val = StateValue::V1(V1StateValue::Session(Session::new()));

        // --------------------
        // WHEN
        // StateValue::is_authsession() is called
        // --------------------
        let result = val.as_v1().unwrap().is_authsession();

        // --------------------
        // THEN
        // false is returned
        // --------------------
        assert!(!result);
    }
}


// ===========================================================================
// Test as_* methods
// ===========================================================================


mod as_session {
    use protocol::StateValue;
    use protocol::v1::{Session, StateValue as V1StateValue};

    #[cfg(windows)]
    use protocol::v1::InitSession;

    #[test]
    fn return_ref()
    {
        // --------------------
        // GIVEN
        // a v1::StateValue::Session value
        // --------------------
        let val = StateValue::V1(V1StateValue::Session(Session::new()));
        let sessref = match val.as_v1().unwrap() {
            &V1StateValue::Session(ref s) => s,
            _ => unreachable!(),
        };

        // --------------------
        // WHEN
        // v1::StateValue::as_session() is called
        // --------------------
        let result = val.as_v1().unwrap().as_session();

        // --------------------
        // THEN
        // a reference to the session state is returned
        // --------------------
        let testval = match result {
            Some(s) => s as *const _ == sessref as *const _,
            None => false,
        };
        assert!(testval);
    }

    #[cfg(windows)]
    #[test]
    fn return_none()
    {
        // --------------------
        // GIVEN
        // a v1::StateValue::InitSession value
        // --------------------
        let val = StateValue::V1(V1StateValue::InitSession(InitSession::new()));

        // --------------------
        // WHEN
        // v1::StateValue::as_session() is called
        // --------------------
        let result = val.as_v1().unwrap().as_session();

        // --------------------
        // THEN
        // None is returned
        // --------------------
        let testval = match result {
            None => true,
            _ => false,
        };

        assert!(testval);
    }
}


#[cfg(windows)]
mod as_initsession {
    use protocol::StateValue;
    use protocol::v1::{Session, StateValue as V1StateValue};

    #[cfg(windows)]
    use protocol::v1::InitSession;

    #[test]
    fn return_ref()
    {
        // --------------------
        // GIVEN
        // a v1::StateValue::InitSession value
        // --------------------
        let val = StateValue::V1(V1StateValue::InitSession(InitSession::new()));
        let initsessref = match val.as_v1().unwrap() {
            &V1StateValue::InitSession(ref s) => s,
            _ => unreachable!(),
        };

        // --------------------
        // WHEN
        // StateValue::as_initsession() is called
        // --------------------
        let result = val.as_v1().unwrap().as_initsession();

        // --------------------
        // THEN
        // a reference to the held InitSession state is returned
        // --------------------
        let testval = match result {
            Some(s) => s as *const _ == initsessref as *const _,
            None => false,
        };
        assert!(testval);
    }

    #[test]
    fn return_none()
    {
        // --------------------
        // GIVEN
        // a v1::StateValue::Session value
        // --------------------
        let val = StateValue::V1(V1StateValue::Session(Session::new()));

        // --------------------
        // WHEN
        // StateValue::is_initsession() is called
        // --------------------
        let result = val.as_v1().unwrap().as_initsession();

        // --------------------
        // THEN
        // false is returned
        // --------------------
        let testval = match result {
            None => true,
            _ => false,
        };
        assert!(testval);
    }
}


#[cfg(windows)]
mod as_authsession {
    use protocol::StateValue;
    use protocol::v1::{Session, StateValue as V1StateValue};

    #[cfg(windows)]
    use protocol::v1::AuthSession;

    #[test]
    fn return_ref()
    {
        // --------------------
        // GIVEN
        // a v1::StateValue::AuthSession value
        // --------------------
        let val = StateValue::V1(V1StateValue::AuthSession(AuthSession::new()));
        let authsessref = match val.as_v1().unwrap() {
            &V1StateValue::AuthSession(ref s) => s,
            _ => unreachable!(),
        };

        // --------------------
        // WHEN
        // StateValue::as_authsession() is called
        // --------------------
        let result = val.as_v1().unwrap().as_authsession();

        // --------------------
        // THEN
        // a reference to the held AuthSession state is returned
        // --------------------
        let testval = match result {
            Some(s) => s as *const _ == authsessref as *const _,
            None => false,
        };
        assert!(testval);
    }

    #[test]
    fn return_none()
    {
        // --------------------
        // GIVEN
        // a v1::StateValue::Session value
        // --------------------
        let val = StateValue::V1(V1StateValue::Session(Session::new()));

        // --------------------
        // WHEN
        // StateValue::is_authsession() is called
        // --------------------
        let result = val.as_v1().unwrap().as_authsession();

        // --------------------
        // THEN
        // false is returned
        // --------------------
        let testval = match result {
            None => true,
            _ => false,
        };
        assert!(testval);
    }
}


// ===========================================================================
// Test to_* methods
// ===========================================================================


mod to_session {
    use protocol::StateValue;
    use protocol::v1::{Session, StateValue as V1StateValue};

    #[cfg(windows)]
    use protocol::v1::InitSession;

    #[test]
    fn return_session()
    {
        // --------------------------
        // GIVEN
        // a v1::StateValue::Session value
        // --------------------------
        let val = StateValue::V1(V1StateValue::Session(Session::new()));

        // --------------------
        // WHEN
        // v1::StateValue::to_session() is called
        // --------------------
        let result = val.to_v1().unwrap().to_session();

        // --------------------
        // THEN
        // the Session object is returned
        // --------------------
        let testval = match result {
            Some(s) => {
                let _state: Session = s;
                true
            }
            None => false,
        };

        assert!(testval);
    }

    // TODO: windows only until unix has more variants
    #[cfg(windows)]
    #[test]
    fn return_none()
    {
        // --------------------------
        // GIVEN
        // a StateValue::V1(v1::StateValue::InitSession) value
        // --------------------------
        let val = StateValue::V1(V1StateValue::InitSession(InitSession::new()));

        // --------------------
        // WHEN
        // v1::StateValue::to_session() is called
        // --------------------
        let result = val.to_v1().unwrap().to_session();

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

#[cfg(windows)]
mod to_initsession {
    use protocol::StateValue;
    use protocol::v1::{InitSession, Session, StateValue as V1StateValue};

    #[test]
    fn return_initsession()
    {
        // --------------------------
        // GIVEN
        // a v1::StateValue::InitSession value
        // --------------------------
        let val = StateValue::V1(V1StateValue::InitSession(InitSession::new()));

        // --------------------
        // WHEN
        // v1::StateValue::to_initsession() is called
        // --------------------
        let result = val.to_v1().unwrap().to_initsession();

        // --------------------
        // THEN
        // the Session object is returned
        // --------------------
        let testval = match result {
            Some(s) => {
                let _state: InitSession = s;
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
        let val = StateValue::V1(V1StateValue::Session(Session::new()));

        // --------------------
        // WHEN
        // v1::StateValue::to_initsession() is called
        // --------------------
        let result = val.to_v1().unwrap().to_initsession();

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


#[cfg(windows)]
mod to_authsession {
    use protocol::StateValue;
    use protocol::v1::{AuthSession, Session, StateValue as V1StateValue};

    #[test]
    fn return_authsession()
    {
        // --------------------------
        // GIVEN
        // a v1::StateValue::AuthSession value
        // --------------------------
        let val = StateValue::V1(V1StateValue::AuthSession(AuthSession::new()));

        // --------------------
        // WHEN
        // v1::StateValue::to_authsession() is called
        // --------------------
        let result = val.to_v1().unwrap().to_authsession();

        // --------------------
        // THEN
        // the Session object is returned
        // --------------------
        let testval = match result {
            Some(s) => {
                let _state: AuthSession = s;
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
        let val = StateValue::V1(V1StateValue::Session(Session::new()));

        // --------------------
        // WHEN
        // v1::StateValue::to_authsession() is called
        // --------------------
        let result = val.to_v1().unwrap().to_authsession();

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


// ===========================================================================
//
// ===========================================================================
