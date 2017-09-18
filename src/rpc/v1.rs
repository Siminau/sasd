// src/message.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

use siminau_rpc::error::{RpcErrorKind, RpcResult};
use siminau_rpc::message::CodeConvert;

// Local imports


// ===========================================================================
// Session
// ===========================================================================


#[cfg(target_family = "windows")]
#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum SessionMethod {
    // Request to attach to the agent session. Response will be a list with 2 items:
    // 1. Session token
    // 2. Absolute path to a temporary file that contains an auth token
    Attach = 4,

    // The client will have to read the auth token contained in the file
    // referenced by the second result item of the attach response, and send
    // this request which has 2 arguments:
    // 1. Session token
    // 2. Auth token
    AuthAttach = 5,

    // No arguments
    KeyList = 6,

    // Single argument: map of attr=value pairs (both attr and value are
    // strings)
    CreateKey = 7,

    // Single argument: map of attr=value pairs (both attr and value are
    // strings)
    DeleteKey = 8,
}


#[cfg(target_family = "unix")]
#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum SessionMethod {
    // No arguments
    KeyList = 6,

    // Single argument: map of attr=value pairs (both attr and value are
    // strings)
    CreateKey = 7,

    // Single argument: map of attr=value pairs (both attr and value are
    // strings)
    DeleteKey = 8,
}


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum SessionError {
    Nil = 0,

    // Attach attempt failed (eg invalid auth)
    InvalidAttach = 9,

    KeyExists = 10,

    KeyNotFound = 11,
}


// ===========================================================================
// Protocol
// ===========================================================================


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum ProtocolMethod {
    // Single argument: map of attr=value pairs (both attr and value are
    // strings)
    // The map must include a proto attribute whose value is the name of the
    // protocol module to use
    ProtocolStart = 12,

    // Single argument: bytes
    ProtocolWrite = 13,

    // Single argument: bytes
    ProtocolRead = 14,

    // Single argument: map of attr=value pairs (attr is a string)
    ProtocolConfirm = 15,

    // Single argument: map of attr=value pairs (attr is a string)
    ProtocolNeedKey = 16,

    // Single argument: map of attr=value pairs (attr is a string)
    ProtocolNeedKeyDone = 17,

    // No arguments
    ProtocolAuthInfo = 18,
}


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum ProtocolError {
    Nil = 0,

    UnknownProtocol = 19,

    InvalidProtocolMessage = 20,

    ProtocolError = 21,

    ProtocolNeedKey = 22,

    ProtocolNeedConfirmation = 23,

    InvalidProtocolAuth = 24,
}


// ===========================================================================
//
// ===========================================================================
