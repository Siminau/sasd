// src/test/settings/windowsbuilder.rs
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


// ===========================================================================
// Modules
// ===========================================================================


// See also test in settings module
#[cfg(unix)]
mod new {
    use settings::SettingsBuilder;
    use std::env;

    // Default value of token_data_dir is None
    #[test]
    fn token_data_dir_default_value()
    {
        // ----------------------------
        // WHEN
        // WindowsBuilder is instantiated
        // ----------------------------
        let mut settings = SettingsBuilder::new();
        let curdir = env::current_dir().unwrap().to_str().unwrap().to_owned();

        settings = settings
            // Port is mandatory
            .port(1234).unwrap()

            // This is needed to prevent error message
            .unix()
                .socket_dir(curdir).unwrap()
            .unix_done().unwrap()

            // The windows() call instantiates WindowsBuilder
            .windows().windows_done().unwrap();

        // ------------------------------------------------
        // THEN
        // the private token_data_dir field is set to None
        // ------------------------------------------------
        // This is proven by building the config and confirming that the
        // windows section is None
        let config = settings.build().unwrap();
        assert!(config.windows().is_none());
    }
}


mod token_data_dir {
    use error::SasdErrorKind;
    use quickcheck::TestResult;
    use settings::SettingsBuilder;
    use std::env;

    #[test]
    fn valid_path_return_windows()
    {
        // -------------------------------
        // GIVEN
        // a SettingsBuilder instance and
        // the current directory
        // -------------------------------
        let settings = SettingsBuilder::new();
        let curdir = env::current_dir().unwrap().to_str().unwrap().to_owned();

        // ------------------------------------------------------------------
        // WHEN
        // windows token_data_dir is set with the current directory
        // ------------------------------------------------------------------

        let result = settings
            // The windows() call instantiates UnixBuilder
            .windows().token_data_dir(curdir);

        // ---------------------------------------------
        // THEN
        // the created WindowsBuilder instance is returned
        // ---------------------------------------------
        let val = match result {
            Ok(_) => true,
            Err(_) => false,
        };
        assert!(val)
    }

    #[cfg(unix)]
    #[test]
    fn valid_path_build_settings()
    {
        // -------------------------------
        // GIVEN
        // a SettingsBuilder instance and
        // the current directory
        // -------------------------------
        let settings = SettingsBuilder::new();
        let expected = env::current_dir().unwrap();
        let curdir = expected.clone().to_str().unwrap().to_owned();

        // ------------------------------------------------------------------
        // WHEN
        // windows token_data_dir is set with the current directory and
        // the settings builder is built
        // ------------------------------------------------------------------

        let settings = settings
            // Port is mandatory
            .port(1234).unwrap()

            // This is needed to prevent error message
            .unix()
                .socket_dir(curdir.clone()).unwrap()
            .unix_done().unwrap()

            // The windows() call instantiates WindowsBuilder
            .windows()
                .token_data_dir(curdir).unwrap()
            .windows_done().unwrap()
            .build().unwrap();

        // ---------------------------------------------
        // THEN
        // a windows section exists
        // ---------------------------------------------
        let val = match settings.windows() {
            Some(u) => u.token_data_dir == expected,
            None => false,
        };
        assert!(val)
    }

    #[cfg(windows)]
    #[test]
    fn valid_path_build_settings()
    {
        // -------------------------------
        // GIVEN
        // a SettingsBuilder instance and
        // the current directory
        // -------------------------------
        let settings = SettingsBuilder::new();
        let expected = env::current_dir().unwrap();
        let curdir_string = expected.clone().to_str().unwrap().to_owned();

        // ------------------------------------------------------------------
        // WHEN
        // windows token_data_dir is set with the current directory and
        // the settings builder is built
        // ------------------------------------------------------------------

        let settings = settings
            // Port is mandatory
            .port(1234).unwrap()

            // The windows() call instantiates WindowsBuilder
            .windows()
                .token_data_dir(curdir_string.clone()).unwrap()
            .windows_done().unwrap()

            .build().unwrap();

        // ---------------------------------------------
        // THEN
        // a windows section exists and
        // the stored token_data_dir value is as expected
        // ---------------------------------------------
        assert!(settings.windows().token_data_dir == expected)
    }

    #[test]
    fn bad_path()
    {
        // -------------------------------
        // GIVEN
        // a SettingsBuilder instance and
        // a non-existant path string
        // -------------------------------
        let settings = SettingsBuilder::new();
        let badpath = "/does/not/exist";

        // ------------------------------------------------------------------
        // WHEN
        // unix socket_dir is set with non-existant directory path
        // ------------------------------------------------------------------

        let result = settings
            // The windows() call instantiates WindowsBuilder
            .windows().token_data_dir(String::from(badpath));

        // --------------------
        // THEN
        // An error is raised
        // --------------------
        let val = match result {
            Err(e) => {
                match e.kind() {
                    &SasdErrorKind::SettingsError(ref msg) => {
                        msg == &format!("path does not exist: {}", badpath)
                    }
                    _ => false,
                }
            }
            _ => false,
        };
        assert!(val)
    }

    quickcheck! {
        fn bad_path_random_string(badpath: String) -> TestResult {
            if badpath == String::from(".") || badpath == String::from("..") {
                return TestResult::discard()
            }

            // -------------------------------
            // GIVEN
            // a SettingsBuilder instance and
            // a non-existant path string
            // -------------------------------
            let settings = SettingsBuilder::new();

            // ------------------------------------------------------------------
            // WHEN
            // unix socket_dir is set with non-existant directory path
            // ------------------------------------------------------------------

            let result = settings
                // The windows() call instantiates WindowsBuilder
                .windows().token_data_dir(badpath.clone());

            // --------------------
            // THEN
            // An error is raised
            // --------------------
            let val = match result {
                Err(e) => {
                    match e.kind() {
                        &SasdErrorKind::SettingsError(ref msg) => {
                            msg == &format!("path does not exist: {}", badpath)
                        }
                        _ => false,
                    }
                }
                _ => false,
            };
            TestResult::from_bool(val)
        }
    }
}


mod windows_done {
    use settings::SettingsBuilder;

    #[cfg(windows)]
    use error::SasdErrorKind;

    #[cfg(unix)]
    use std::env;

    #[cfg(windows)]
    #[test]
    fn token_data_dir_required_on_windows()
    {
        let expected = "Missing token data directory";

        // ----------------------------
        // WHEN
        // WindowsBuilder is instantiated and
        // windows() is called and
        // then windows_done() is called
        // ----------------------------
        let settings = SettingsBuilder::new();

        let result = settings
            // The unix() call instantiates UnixBuilder
            .windows().windows_done();

        // --------------------------------------------------------------------
        // THEN
        // an error is raised indicating that the token_data_dir value must be
        // provided
        // --------------------------------------------------------------------
        let val = match result {
            Err(e) => {
                match e.kind() {
                    &SasdErrorKind::SettingsError(ref msg) => {
                        msg == &expected.to_owned()
                    }
                    _ => false,
                }
            }
            _ => false,
        };
        assert!(val)
    }

    #[cfg(unix)]
    #[test]
    fn unix_can_have_no_windows()
    {
        // ----------------------------
        // WHEN
        // WindowsBuilder is instantiated and
        // windows() is called and
        // then windows_done() is called and
        // then settings are built
        // ----------------------------
        let settings = SettingsBuilder::new();
        let curdir = env::current_dir().unwrap();
        let curdir_string = curdir.clone().to_str().unwrap().to_owned();

        let result = settings
            // Port is mandatory
            .port(1234).unwrap()

            // The windows() call instantiates WindowsBuilder
            .windows().windows_done().unwrap()

            // This is needed to prevent error message
            .unix()
                .socket_dir(curdir_string).unwrap()
            .unix_done().unwrap()
            .build().unwrap();

        // ------------------------------------------
        // THEN
        // The settings does not have a windows section
        // ------------------------------------------
        assert!(result.windows().is_none())
    }
}

// ===========================================================================
//
// ===========================================================================
