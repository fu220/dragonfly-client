/*
 *     Copyright 2024 The Dragonfly Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use clap::{Arg, Command};
use std::path::PathBuf;

pub mod dfcache;
pub mod dfdaemon;
pub mod dfget;
pub mod dfinit;

/// SERVICE_NAME is the name of the service.
pub const SERVICE_NAME: &str = "dragonfly";

/// NAME is the name of the package.
pub const NAME: &str = "client";

/// CARGO_PKG_VERSION is the version of the cargo package.
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// CARGO_PKG_RUSTC_VERSION is the minimum Rust version supported by the package, not the current Rust version.
pub const CARGO_PKG_RUSTC_VERSION: &str = env!("CARGO_PKG_RUST_VERSION");

/// BUILD_PLATFORM is the platform of the build.
pub const BUILD_PLATFORM: &str = env!("BUILD_PLATFORM");

// BUILD_TIMESTAMP is the timestamp of the build.
pub const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

/// GIT_COMMIT_SHORT_HASH is the short git commit hash of the package.
pub const GIT_COMMIT_SHORT_HASH: &str = {
    match option_env!("GIT_COMMIT_SHORT_HASH") {
        Some(hash) => hash,
        None => "unknown",
    }
};

/// GIT_COMMIT_DATE is the git commit date of the package.
pub const GIT_COMMIT_DATE: &str = {
    match option_env!("GIT_COMMIT_DATE") {
        Some(hash) => hash,
        None => "unknown",
    }
};

/// default_root_dir is the default root directory for client.
pub fn default_root_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    return PathBuf::from("/var/run/dragonfly/");

    #[cfg(target_os = "macos")]
    return home::home_dir().unwrap().join(".dragonfly");
}

/// default_config_dir is the default config directory for client.
pub fn default_config_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    return PathBuf::from("/etc/dragonfly/");

    #[cfg(target_os = "macos")]
    return home::home_dir().unwrap().join(".dragonfly").join("config");
}

/// default_log_dir is the default log directory for client.
pub fn default_log_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    return PathBuf::from("/var/log/dragonfly/");

    #[cfg(target_os = "macos")]
    return home::home_dir().unwrap().join(".dragonfly").join("logs");
}

/// default_storage_dir is the default storage directory for client.
pub fn default_storage_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    return PathBuf::from("/var/lib/dragonfly/");

    #[cfg(target_os = "macos")]
    return home::home_dir().unwrap().join(".dragonfly").join("storage");
}

/// default_lock_dir is the default lock directory for client.
pub fn default_lock_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    return PathBuf::from("/var/lock/dragonfly/");

    #[cfg(target_os = "macos")]
    return home::home_dir().unwrap().join(".dragonfly");
}

/// default_plugin_dir is the default plugin directory for client.
pub fn default_plugin_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    return PathBuf::from("/usr/local/lib/dragonfly/plugins/");

    #[cfg(target_os = "macos")]
    return home::home_dir().unwrap().join(".dragonfly").join("plugins");
}

/// default_cache_dir is the default cache directory for client.
pub fn default_cache_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    return PathBuf::from("/var/cache/dragonfly/");

    #[cfg(target_os = "macos")]
    return home::home_dir().unwrap().join(".dragonfly").join("cache");
}

/// VersionValueParser is a custom value parser for the version flag.
#[derive(Debug, Clone)]
pub struct VersionValueParser;

/// Implement the TypedValueParser trait for VersionValueParser.
impl clap::builder::TypedValueParser for VersionValueParser {
    type Value = bool;

    fn parse_ref(
        &self,
        cmd: &Command,
        _arg: Option<&Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        if value == std::ffi::OsStr::new("true") {
            println!(
                "{} {} ({}, {})",
                cmd.get_name(),
                cmd.get_version().unwrap_or("unknown"),
                GIT_COMMIT_SHORT_HASH,
                GIT_COMMIT_DATE,
            );

            std::process::exit(0);
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{builder::TypedValueParser, Command};
    use std::ffi::OsStr;

    #[test]
    fn version_value_parser_references_non_real_values() {
        let parser = VersionValueParser;
        let cmd = Command::new("test_app");
        let value = OsStr::new("false");
        let result = parser.parse_ref(&cmd, None, value);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
