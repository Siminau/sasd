// src/test/protocol/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================


mod state;
mod statevalue;
mod start;
mod v1;


// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

use std::fs::remove_dir_all;
use std::path::PathBuf;

// Third-party imports

use rmpv::Value;
use siminau_rpc::error::RpcErrorKind;
use siminau_rpc::message::{Message, MessageType};
use tempdir::TempDir;

// Local imports

use error::{SasdErrorKind, SasdResult};
use protocol::{Info, Protocol, Request, Response, State, StateValue};

#[cfg(windows)]
use protocol::SessionStore;

use rpc;
use settings::{SettingsBuilder, SettingsHandle, new_settings_handle};

#[cfg(windows)]
use settings::WindowsSection;

#[cfg(windows)]
use settings::test::helper::new_settings;

use state::{SessionState, SessionStateHandle};


// ===========================================================================
// Helpers
// ===========================================================================


struct Test;


impl State for Test {
    fn dispatch(&mut self, _state: &mut SessionStateHandle, _msg: Message)
        -> SasdResult<(Option<StateValue>, Option<Message>)>
    {
        Ok((None, None))
    }
}


#[cfg(unix)]
fn dummy_settings() -> SasdResult<SettingsHandle>
{
    let tempdir = TempDir::new("sasd").unwrap();
    let dirpath = tempdir.into_path().into_os_string().into_string().unwrap();

    // Build settings
    let config = SettingsBuilder::new()
        .port(1234)?
        .unix()
        .socket_dir(dirpath)?
        .unix_done()?
        .build()?;
    Ok(new_settings_handle(config))
}


#[cfg(windows)]
fn dummy_settings() -> SasdResult<SettingsHandle>
{
    let tempdir = TempDir::new("sasd").unwrap();
    let dirpath = tempdir.into_path().into_os_string().into_string().unwrap();

    let config = SettingsBuilder::new()
        .port(1234)?
        .windows()
        .token_data_dir(dirpath)?
        .windows_done()?
        .build()?;
    Ok(new_settings_handle(config))
}


#[cfg(unix)]
pub fn dummy_session_state(state: StateValue) -> SessionState
{
    let settings = dummy_settings().unwrap();
    SessionState::new(settings, state)
}


#[cfg(windows)]
pub fn dummy_session_state_nofs(state: StateValue) -> SessionState
{
    let auth_token = "world".to_owned();
    let settings =
        new_settings(
            1234,
            None,
            WindowsSection { token_data_dir: PathBuf::from("/does/not/exist") },
        );
    let settings_handle = new_settings_handle(settings);
    let session_store = SessionStore {
        auth_token: auth_token,
        auth_file: None,
    };
    SessionState::new(session_store, settings_handle, state)
}

#[cfg(windows)]
pub fn dummy_session_state(state: StateValue) -> SessionState
{
    let settings = dummy_settings().unwrap();
    let store = SessionStore::default();
    SessionState::new(store, settings, state)
}


#[cfg(unix)]
pub fn cleanup_settings(mut state: SessionState)
{
    let dirpath = {
        let config = state.server_settings().read().expect(
            "failed to read server \
             settings",
        );
        PathBuf::from(&config.unix().socket_dir)
    };
    remove_dir_all(dirpath).unwrap();
}


#[cfg(windows)]
pub fn cleanup_settings(mut state: SessionState)
{
    // Drop the open auth file
    use std::mem;
    {
        let store = state.session_store();
        mem::replace(&mut store.auth_file, None);
    }
    let dirpath = {
        let config = state.server_settings().read().expect(
            "failed to read server \
             settings",
        );
        PathBuf::from(&config.windows().token_data_dir)
    };
    remove_dir_all(dirpath).unwrap();
}


// ===========================================================================
//
// ===========================================================================
