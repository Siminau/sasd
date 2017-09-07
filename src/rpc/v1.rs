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
//
// ===========================================================================


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum RequestMethod {
    // Initiate client session by requesting an API version
    Version,

    // Request to attach to the agent session. Response will be a list with 2 items:
    // 1. Session token
    // 2. Absolute path to a temporary file that contains an auth token
    Attach,

    // The client will have to read the auth token contained in the file
    // referenced by the second result item of the attach response, and send
    // this request which has 2 arguments:
    // 1. Session token
    // 2. Auth token
    AuthAttach,

    // No arguments
    KeyList,

    // Single argument: map of attr=value pairs (both attr and value are
    // strings)
    CreateKey,

    // Single argument: map of attr=value pairs (both attr and value are
    // strings)
    DeleteKey,

    // Single argument: map of attr=value pairs (both attr and value are
    // strings)
    // The map must include a proto attribute whose value is the name of the
    // protocol module to use
    ProtocolStart,

    // Single argument: bytes
    ProtocolWrite,

    // Single argument: bytes
    ProtocolRead,

    // Single argument: map of attr=value pairs (attr is a string)
    ProtocolConfirm,

    // Single argument: map of attr=value pairs (attr is a string)
    ProtocolNeedKey,

    // Single argument: map of attr=value pairs (attr is a string)
    ProtocolNeedKeyDone,

    // No arguments
    ProtocolAuthInfo,
}


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum ResponseError {
    Nil,

    // API version is unsupported by the server
    VersionUnsupported,

    // Attach attempt failed (eg invalid auth)
    InvalidAttach,

    KeyExists,

    KeyNotFound,

    UnknownProtocol,

    InvalidProtocolMessage,

    ProtocolError,

    ProtocolNeedKey,

    ProtocolNeedConfirmation,

    InvalidProtocolAuth,
}


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum Notice {
    // No more requests will be made
    Done,
}


// ===========================================================================
//
// ===========================================================================
