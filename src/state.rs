// state.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Externs
// ===========================================================================


// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

// Local imports

use protocol::BoxState;
#[cfg(windows)]
use protocol::SessionStore;
use settings::SettingsHandle;



// ===========================================================================
// SessionState
// ===========================================================================


pub struct SessionState {
    #[cfg(windows)]
    session_store: SessionStore,

    server_settings: SettingsHandle,
    state: BoxState,
}


impl SessionState {
    #[cfg(unix)]
    pub fn new(server_settings: SettingsHandle, state: BoxState)
        -> SessionState
    {
        SessionState {
            server_settings: server_settings,
            state: state,
        }
    }

    #[cfg(windows)]
    pub fn new(
        session_store: SessionStore, server_settings: SettingsHandle,
        state: BoxState
    ) -> SessionState
    {
        SessionState {
            session_store: session_store,
            server_settings: server_settings,
            state: state,
        }
    }

    #[cfg(windows)]
    pub fn session_store(&mut self) -> &mut SessionStore
    {
        &mut self.session_store
    }

    pub fn server_settings(&mut self) -> &mut SettingsHandle
    {
        &mut self.server_settings
    }

    pub fn handle(&mut self) -> SessionStateHandle
    {
        SessionStateHandle::new(self)
    }
}


pub struct SessionStateHandle<'sessionstate> {
    session_state: &'sessionstate mut SessionState,
}


impl<'sessionstate> SessionStateHandle<'sessionstate> {
    pub fn new(session_state: &'sessionstate mut SessionState) -> Self
    {
        SessionStateHandle { session_state: session_state }
    }

    #[cfg(windows)]
    pub fn session_store(&mut self) -> &mut SessionStore
    {
        self.session_state.session_store()
    }

    pub fn server_settings(&mut self) -> &mut SettingsHandle
    {
        self.session_state.server_settings()
    }
}


// ===========================================================================
//
// ===========================================================================
