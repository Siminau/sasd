// error.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

use config::ConfigError;
use siminau_rpc::error as rpcerror;

// Local imports

// ===========================================================================
// Errors
// ===========================================================================

error_chain! {
    types {
        SasdError, SasdErrorKind, SasdResultExt, SasdResult;
    }

    links {
        Net(rpcerror::RpcError, rpcerror::RpcErrorKind);
    }

    foreign_links {
        Config(ConfigError);
    }

    errors {
        UnexpectedMessage
        InvalidMessage
        SettingsError(msg: String) {
            description("settings validation failure")
            display("Settings validation error: {}", msg)
        }
    }
}


// ===========================================================================
//
// ===========================================================================
