use crate::pnp::bindings;

use super::config::{ConfigParseError, InputConfiguration, parse_config};

/// Primary location of the configuration file.
///
/// The `luma/plugins` directory always exists when PokeReader is installed as
/// `/luma/plugins/default.3gx`, so no directory has to be created.
pub const PRIMARY_CONFIG_PATH: &str = "sdmc:/luma/plugins/PokeReader-input.cfg";

/// Used when the primary location is not writable.
pub const FALLBACK_CONFIG_PATH: &str = "sdmc:/PokeReader-input.cfg";

pub const CONFIG_PATHS: [&str; 2] = [PRIMARY_CONFIG_PATH, FALLBACK_CONFIG_PATH];

const MAX_CONFIG_BYTES: usize = 2048;

static mut BUFFER: [u8; MAX_CONFIG_BYTES] = [0; MAX_CONFIG_BYTES];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadStatus {
    /// A configuration file was read successfully.
    Loaded,
    /// No configuration file exists; the legacy defaults are used.
    Missing,
    /// The file exists but could not be parsed; the defaults are used.
    Invalid,
    /// The file was written by a newer version and is left untouched.
    UnsupportedVersion,
}

impl LoadStatus {
    pub const fn message(self) -> &'static str {
        match self {
            LoadStatus::Loaded => "Config loaded",
            LoadStatus::Missing => "Default config",
            LoadStatus::Invalid => "Config invalid",
            LoadStatus::UnsupportedVersion => "Config too new",
        }
    }

    /// True when saving would overwrite something we do not understand.
    pub const fn blocks_saving(self) -> bool {
        matches!(self, LoadStatus::UnsupportedVersion)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LoadedConfiguration {
    pub config: InputConfiguration,
    pub status: LoadStatus,
    pub path: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveResult {
    Saved(&'static str),
    Failed,
    Blocked,
}

impl SaveResult {
    pub const fn message(self) -> &'static str {
        match self {
            SaveResult::Saved(_) => "Saved",
            SaveResult::Failed => "Save failed",
            SaveResult::Blocked => "Config too new",
        }
    }

    pub const fn is_ok(self) -> bool {
        matches!(self, SaveResult::Saved(_))
    }
}

enum ReadResult {
    Missing,
    Invalid,
    Text(&'static str),
}

fn read_config(path: &str) -> ReadResult {
    let bytes_read = unsafe {
        bindings::host_config_read(
            path.as_ptr() as u32,
            path.len() as u32,
            BUFFER.as_mut_ptr() as u32,
            MAX_CONFIG_BYTES as u32,
        )
    };

    if bytes_read < 0 {
        return ReadResult::Missing;
    }

    let bytes_read = bytes_read as usize;
    if bytes_read > MAX_CONFIG_BYTES {
        return ReadResult::Invalid;
    }

    // SAFETY: the buffer is only written by `host_config_read`, which returns
    // the number of bytes written, and PokeReader runs single threaded.
    let bytes = unsafe { &BUFFER[..bytes_read] };

    match core::str::from_utf8(bytes) {
        Ok(text) => ReadResult::Text(text),
        Err(_) => ReadResult::Invalid,
    }
}

/// Reads the configuration from the SD card.
///
/// Missing or unreadable files fall back to the legacy defaults.
pub fn load() -> LoadedConfiguration {
    let mut found_invalid = false;

    for path in CONFIG_PATHS {
        match read_config(path) {
            ReadResult::Missing => continue,
            ReadResult::Invalid => {
                found_invalid = true;
                continue;
            }
            ReadResult::Text(text) => match parse_config(text) {
                Ok(parsed) => {
                    return LoadedConfiguration {
                        config: parsed.config,
                        status: LoadStatus::Loaded,
                        path: Some(path),
                    };
                }
                Err(ConfigParseError::UnsupportedVersion(_)) => {
                    return LoadedConfiguration {
                        config: InputConfiguration::legacy(),
                        status: LoadStatus::UnsupportedVersion,
                        path: Some(path),
                    };
                }
            },
        }
    }

    LoadedConfiguration {
        config: InputConfiguration::legacy(),
        status: if found_invalid {
            LoadStatus::Invalid
        } else {
            LoadStatus::Missing
        },
        path: None,
    }
}

/// Writes the configuration to the SD card.
///
/// The preferred path (the one that was read from) is tried first, then the
/// remaining known locations.
pub fn save(config: &InputConfiguration, preferred: Option<&'static str>) -> SaveResult {
    let text = config.to_config_string();

    let mut paths = CONFIG_PATHS;
    if preferred == Some(FALLBACK_CONFIG_PATH) {
        paths = [FALLBACK_CONFIG_PATH, PRIMARY_CONFIG_PATH];
    }

    for path in paths {
        let result = unsafe {
            bindings::host_config_write(
                path.as_ptr() as u32,
                path.len() as u32,
                text.as_ptr() as u32,
                text.len() as u32,
            )
        };

        if result == 0 {
            return SaveResult::Saved(path);
        }
    }

    SaveResult::Failed
}
