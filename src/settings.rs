// src/settings.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

use std::mem;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::RwLock;

// Third-party imports

// use config::*;

// Local imports

use error::{SasdErrorKind, SasdResult};


// ===========================================================================
// Config Helpers
// ===========================================================================


pub type SettingsHandle = Rc<RwLock<Settings>>;


pub fn new_settings_handle(settings: Settings) -> SettingsHandle
{
    Rc::new(RwLock::new(settings))
}


// ===========================================================================
// SettingsConfig
// ===========================================================================


#[derive(Debug, Deserialize)]
pub struct WindowsConfig {
    token_data_dir: String,
}


#[derive(Debug, Deserialize)]
pub struct UnixConfig {
    socket_dir: String,
}


#[derive(Debug, Deserialize)]
pub struct SettingsConfig {
    port: u16,
    unix: Option<UnixConfig>,
    windows: Option<WindowsConfig>,
}


// ===========================================================================
// SettingsBuilder
// ===========================================================================


pub struct UnixBuilder {
    _builder: SettingsBuilder,

    // Settings
    socket_dir: Option<PathBuf>,
}


impl UnixBuilder {
    fn new(builder: SettingsBuilder) -> Self
    {
        UnixBuilder {
            _builder: builder,
            socket_dir: None,
        }
    }

    pub fn socket_dir(mut self, dir: String) -> SasdResult<Self>
    {
        self.socket_dir = Some(self._builder.validate_path(dir)?);
        Ok(self)
    }

    pub fn done(self) -> SasdResult<SettingsBuilder>
    {
        if cfg!(unix) && self.socket_dir.is_none() {
            bail!(SasdErrorKind::SettingsError(
                "Missing socket directory".to_owned(),
            ))
        }
        let mut builder = self._builder;
        let unix = UnixSection { socket_dir: self.socket_dir.unwrap() };
        builder.unix = Some(unix);
        Ok(builder)
    }
}


pub struct WindowsBuilder {
    _builder: SettingsBuilder,
    token_data_dir: Option<PathBuf>,
}


impl WindowsBuilder {
    fn new(builder: SettingsBuilder) -> Self
    {
        WindowsBuilder {
            _builder: builder,
            token_data_dir: None,
        }
    }

    pub fn token_data_dir(mut self, dir: String) -> SasdResult<Self>
    {
        self.token_data_dir = Some(self._builder.validate_path(dir)?);
        Ok(self)
    }

    pub fn done(self) -> SasdResult<SettingsBuilder>
    {
        if cfg!(windows) && self.token_data_dir.is_none() {
            bail!(SasdErrorKind::SettingsError(
                "Missing token data directory".to_owned(),
            ))
        }
        let mut builder = self._builder;
        let windows =
            WindowsSection { token_data_dir: self.token_data_dir.unwrap() };
        builder.windows = Some(windows);
        Ok(builder)
    }
}


pub struct SettingsBuilder {
    port: Option<u16>,
    unix: Option<UnixSection>,
    windows: Option<WindowsSection>,
}


impl SettingsBuilder {
    pub fn new() -> Self
    {
        SettingsBuilder {
            port: None,
            unix: None,
            windows: None,
        }
    }

    fn from_unix_config(self, config: &mut SettingsConfig) -> SasdResult<Self>
    {
        let unix_config = mem::replace(&mut config.unix, None);
        match unix_config {
            Some(c) => self.unix().socket_dir(c.socket_dir)?.done(),
            None => {
                if cfg!(unix) {
                    bail!(SasdErrorKind::SettingsError(
                        "Missing unix configuration".to_owned(),
                    ))
                }
                Ok(self)
            }
        }
    }

    fn from_windows_config(self, config: &mut SettingsConfig)
        -> SasdResult<Self>
    {
        let windows_config = mem::replace(&mut config.windows, None);
        match windows_config {
            Some(c) => self.windows().token_data_dir(c.token_data_dir)?.done(),
            None => {
                if cfg!(windows) {
                    bail!(SasdErrorKind::SettingsError(
                        "Missing windows configuration".to_owned(),
                    ))
                }
                Ok(self)
            }
        }
    }

    pub fn from_config(mut config: SettingsConfig) -> SasdResult<Settings>
    {
        let builder = SettingsBuilder::new();
        let builder = builder.port(config.port)?;
        let builder = builder.from_unix_config(&mut config)?;
        let builder = builder.from_windows_config(&mut config)?;
        let ret = builder.build();
        Ok(ret)
    }

    pub fn unix(self) -> UnixBuilder
    {
        UnixBuilder::new(self)
    }

    pub fn windows(self) -> WindowsBuilder
    {
        WindowsBuilder::new(self)
    }

    pub fn port(mut self, port: u16) -> SasdResult<Self>
    {
        if port < 1024 {
            let errmsg =
                format!("port: value must not be less than 1024, got {}", port);
            bail!(SasdErrorKind::SettingsError(errmsg))
        }
        self.port = Some(port);
        Ok(self)
    }

    #[cfg(unix)]
    pub fn build(self) -> Settings
    {
        Settings {
            port: self.port.unwrap(),
            unix: self.unix.unwrap(),
            windows: self.windows,
        }
    }

    #[cfg(windows)]
    pub fn build(self) -> Settings
    {
        Settings {
            port: self.port.unwrap(),
            unix: self.unix,
            windows: self.windows.unwrap(),
        }
    }

    fn validate_path(&self, path: String) -> SasdResult<PathBuf>
    {
        let p = PathBuf::from(path);
        let errmsg = if !p.exists() {
            Some(format!("path does not exist: {}", p.display()))
        } else if !p.is_dir() {
            Some(format!("path is not a directory: {}", p.display()))
        } else {
            None
        };

        match errmsg {
            Some(msg) => bail!(SasdErrorKind::SettingsError(msg)),
            None => Ok(p),
        }
    }
}


// ===========================================================================
// Settings
// ===========================================================================


#[derive(Debug)]
pub struct WindowsSection {
    pub token_data_dir: PathBuf,
}


#[derive(Debug)]
pub struct UnixSection {
    pub socket_dir: PathBuf,
}


#[cfg(unix)]
#[derive(Debug)]
pub struct Settings {
    pub port: u16,
    unix: UnixSection,
    windows: Option<WindowsSection>,
}


#[cfg(windows)]
#[derive(Debug)]
pub struct Settings {
    pub port: u16,
    unix: Option<UnixSection>,
    windows: WindowsSection,
}


impl Settings {
    #[cfg(unix)]
    pub fn unix(&self) -> &UnixSection
    {
        &self.unix
    }

    #[cfg(windows)]
    pub fn unix(&self) -> Option<&UnixSection>
    {
        match self.unix {
            Some(ref u) => Some(u),
            None => None,
        }
    }

    #[cfg(unix)]
    pub fn windows(&self) -> Option<&WindowsSection>
    {
        match self.windows {
            Some(ref w) => Some(w),
            None => None,
        }
    }

    #[cfg(windows)]
    pub fn windows(&self) -> &WindowsSection
    {
        &self.windows
    }
}


// ===========================================================================
// Scratch
// ===========================================================================


#[cfg(test)]
mod scratch {
    // port (String)
    //
    // [windows]
    // token_data_dir (String)
    //
    // [unix]
    // socket_dir (String)
    //
    //

    use super::*;
    use config::*;
    use std::path::Path;

    #[test]
    fn test_tmp()
    {
        let path = Path::new("files/test.toml");
        let mut config = Config::new();
        config.merge(File::from(path)).unwrap();
        let s: SettingsConfig = config.try_into().unwrap();

        let settings = SettingsBuilder::from_config(s).unwrap();
        println!("{:?}", settings);
        println!("{}", settings.windows().token_data_dir.display());
    }
}


// ===========================================================================
//
// ===========================================================================
