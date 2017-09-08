// error.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

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
        Net(rpcerror::RpcError, rpcerror::RpcErrorKind)
    }
}


// ===========================================================================
//
// ===========================================================================
