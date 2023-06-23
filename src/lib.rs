// Licensed to the Software Freedom Conservancy (SFC) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The SFC licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::files::compose_cache_folder;
use std::fs;

use reqwest::{Client, Proxy};

use std::error::Error;

use std::time::Duration;

use crate::logger::Logger;

pub mod config;

pub mod files;

pub mod logger;

pub const REQUEST_TIMEOUT_SEC: u64 = 180; // The timeout is applied from when the request starts connecting until the response body has finished
pub const STABLE: &str = "stable";
pub const BETA: &str = "beta";
pub const DEV: &str = "dev";
pub const CANARY: &str = "canary";
pub const NIGHTLY: &str = "nightly";
pub const WMIC_COMMAND: &str = r#"wmic datafile where name='{}' get Version /value"#;
pub const WMIC_COMMAND_ENV: &str =
    r#"set PFILES=%{}{}%&& wmic datafile where name='!PFILES:\=\\!{}' get Version /value"#;
pub const WMIC_COMMAND_OS: &str = r#"wmic os get osarchitecture"#;
pub const REG_QUERY: &str = r#"REG QUERY {} /v version"#;
pub const REG_QUERY_FIND: &str = r#"REG QUERY {} /f {}"#;
pub const PLIST_COMMAND: &str =
    r#"/usr/libexec/PlistBuddy -c "print :CFBundleShortVersionString" {}/Contents/Info.plist"#;
pub const DASH_VERSION: &str = "{} -v";
pub const DASH_DASH_VERSION: &str = "{} --version";
pub const ENV_PROGRAM_FILES: &str = "PROGRAMFILES";
pub const ENV_PROGRAM_FILES_X86: &str = "PROGRAMFILES(X86)";
pub const ENV_LOCALAPPDATA: &str = "LOCALAPPDATA";
pub const REMOVE_X86: &str = ": (x86)=";
pub const ARCH_X86: &str = "x86";
pub const ARCH_AMD64: &str = "amd64";
pub const ARCH_ARM64: &str = "arm64";
pub const ENV_PROCESSOR_ARCHITECTURE: &str = "PROCESSOR_ARCHITECTURE";
pub const WHERE_COMMAND: &str = "where {}";
pub const WHICH_COMMAND: &str = "which {}";
pub const TTL_BROWSERS_SEC: u64 = 0;
pub const TTL_DRIVERS_SEC: u64 = 86400;
pub const UNAME_COMMAND: &str = "uname -{}";
pub const CRLF: &str = "\r\n";
pub const LF: &str = "\n";

pub fn clear_cache(log: &Logger) {
    let cache_path = compose_cache_folder();
    if cache_path.exists() {
        log.debug(format!("Clearing cache at: {}", cache_path.display()));
        fs::remove_dir_all(&cache_path).unwrap_or_else(|err| {
            log.warn(format!(
                "The cache {} cannot be cleared: {}",
                cache_path.display(),
                err
            ))
        });
    }
}

pub fn create_http_client(timeout: u64, proxy: &str) -> Result<Client, Box<dyn Error>> {
    let mut client_builder = Client::builder()
        .danger_accept_invalid_certs(true)
        .use_rustls_tls()
        .timeout(Duration::from_secs(timeout));
    if !proxy.is_empty() {
        client_builder = client_builder.proxy(Proxy::all(proxy)?);
    }
    Ok(client_builder.build().unwrap_or_default())
}

pub fn format_one_arg(string: &str, arg1: &str) -> String {
    string.replacen("{}", arg1, 1)
}

pub fn format_two_args(string: &str, arg1: &str, arg2: &str) -> String {
    string.replacen("{}", arg1, 1).replacen("{}", arg2, 1)
}

pub fn format_three_args(string: &str, arg1: &str, arg2: &str, arg3: &str) -> String {
    string
        .replacen("{}", arg1, 1)
        .replacen("{}", arg2, 1)
        .replacen("{}", arg3, 1)
}

// ----------------------------------------------------------
// Private functions
// ----------------------------------------------------------

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix(CRLF)
        .or_else(|| input.strip_suffix(LF))
        .unwrap_or(input)
}
