// src/test/mod.rs
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
// Modules
// ===========================================================================


mod os;
mod protocol;
mod settings;


// ===========================================================================
// Helpers
// ===========================================================================


struct Hello {
    hello: String,
    world: u64,
}


impl Hello {
    fn new() -> Hello {
        Hello
    }

    fn hidden(&self) -> bool {
        unimplemented!()
    }
}


mkwrapper!(Hello, hello: String, world: u64);


#[macro_export]
macro_rules! mkwrapper {
    (
        $(
            $x:ident, $( $y:ident ): $( $z:ty ),*
        )
    ) => {

        pub struct $xWrapper($x);

        impl $xWrapper {
            pub fn get_$y(&self) -> &$z
        }

    };
}

pub struct Wrapper(Hello);


impl Wrapper {
    pub fn get_hello(&self) -> &String {
        unimplemented!()
    }

    pub fn getmut_hello(&mut self) -> &mut String {
        unimplemented!()
    }

    pub fn set_hello(&mut self, val: String) {
        unimplemented!()
    }
}


// ===========================================================================
//
// ===========================================================================
