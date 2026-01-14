//
// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// <https://www.apache.org/licenses/LICENSE-2.0>
//
// SPDX-License-Identifier: Apache-2.0
//

//! C++-based logger implementation

use crate::mw_log_ffi::{LogLevel, LogStream, Recorder, SlotHandlePtr};
use crate::mw_log_writer::MwLogWriter;
use mw_log::fmt::{score_write, write};
use mw_log::{Log, Metadata, Record};
use std::env::{set_var, var_os};
use std::path::PathBuf;
use std::sync::Once;

/// Perform layout check, panic on mismatch.
fn layout_check() {
    let slot_layout_rust = SlotHandlePtr::layout_rust();
    let slot_layout_cpp = SlotHandlePtr::layout_cpp();
    if slot_layout_rust != slot_layout_cpp {
        panic!("SlotHandle layout mismatch, Rust: {slot_layout_rust:?}, C++: {slot_layout_cpp:?}");
    }
}

/// Builder for the [`MwLogger`].
pub struct MwLoggerBuilder {
    context: String,
    show_module: bool,
    show_file: bool,
    show_line: bool,
    config_path: Option<PathBuf>,
}

impl MwLoggerBuilder {
    /// Create builder with default parameters.
    ///
    /// # Note
    ///
    /// This operation perform data layout check.
    /// This might cause panic if layout of FFI structures is mismatched.
    pub fn new() -> Self {
        // Perform layout check - only once.
        static LAYOUT_CHECK: Once = Once::new();
        LAYOUT_CHECK.call_once(layout_check);

        Self::default()
    }

    /// Set context for the [`MwLogger`].
    ///
    /// Only ASCII characters are allowed.
    /// Max 4 characters are used. Rest of the provided string will be trimmed.
    pub fn context(mut self, context: &str) -> Self {
        self.context = context.to_string();
        self
    }

    /// Show module name in logs.
    pub fn show_module(mut self, show_module: bool) -> Self {
        self.show_module = show_module;
        self
    }

    /// Show file name in logs.
    pub fn show_file(mut self, show_file: bool) -> Self {
        self.show_file = show_file;
        self
    }

    /// Show line number in logs.
    pub fn show_line(mut self, show_line: bool) -> Self {
        self.show_line = show_line;
        self
    }

    /// Set `MW_LOG_CONFIG_FILE` environment variable during [`Self::build`].
    ///
    /// Following conditions must be met:
    /// - Variable is set only during the first call to [`Self::set_as_default_logger`].
    /// - Variable is set only if not set externally.
    pub fn config(mut self, config_path: PathBuf) -> Self {
        self.config_path = Some(config_path);
        self
    }

    /// Build the [`MwLogger`] with provided context and configuration.
    pub fn build(self) -> MwLogger {
        let recorder = Recorder::new();
        MwLogger {
            context: self.context,
            show_module: self.show_module,
            show_file: self.show_file,
            show_line: self.show_line,
            recorder,
        }
    }

    /// Build the [`MwLogger`] and set it as the default logger.
    pub fn set_as_default_logger(self) {
        // Set `MW_LOG_CONFIG_FILE`.
        {
            const KEY: &str = "MW_LOG_CONFIG_FILE";

            // Set variable only if:
            // - environment variable is not set
            // - `config_path` is set and not empty
            if var_os(KEY).is_none() {
                if let Some(ref path) = self.config_path {
                    let path_os_str = path.as_os_str();
                    if !path_os_str.is_empty() {
                        unsafe { set_var(KEY, path_os_str) };
                    }
                }
            }
        }

        // Build logger and set as default.
        let context = self.context.clone();
        let logger = self.build();
        mw_log::set_max_level(logger.log_level(&context).into());
        if let Err(e) = mw_log::set_global_logger(Box::new(logger)) {
            panic!("unable to set logger: {e}");
        }
    }
}

impl Default for MwLoggerBuilder {
    fn default() -> Self {
        Self {
            context: "DFLT".to_string(),
            show_module: false,
            show_file: false,
            show_line: false,
            config_path: None,
        }
    }
}

/// C++-based logger implementation.
pub struct MwLogger {
    context: String,
    show_module: bool,
    show_file: bool,
    show_line: bool,
    recorder: Recorder,
}

impl MwLogger {
    /// Current log level for provided context.
    pub(crate) fn log_level(&self, context: &str) -> LogLevel {
        self.recorder.log_level(context)
    }
}

impl Log for MwLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.log_level(metadata.context()) >= metadata.level().into()
    }

    fn context(&self) -> &str {
        &self.context
    }

    fn log(&self, record: &Record) {
        // Finish early if not enabled for requested level.
        let metadata = record.metadata();
        if !self.enabled(metadata) {
            return;
        }

        // Create log stream.
        let context = metadata.context();
        let log_level = metadata.level().into();
        let log_stream = LogStream::new(&self.recorder, context, log_level);

        // Create writer.
        let mut writer = MwLogWriter::new(log_stream);

        // Write module, file and line.
        if self.show_module || self.show_file || self.show_line {
            let _ = score_write!(&mut writer, "[");
            if self.show_module {
                let _ = score_write!(&mut writer, "{}:", record.module_path());
            }
            if self.show_file {
                let _ = score_write!(&mut writer, "{}:", record.file());
            }
            if self.show_line {
                let _ = score_write!(&mut writer, "{}", record.line());
            }
            let _ = score_write!(&mut writer, "]");
        }

        // Write log data.
        let _ = write(&mut writer, *record.args());
        // Written data is flushed on log stream drop.
    }

    fn flush(&self) {
        // No-op.
    }
}

// SAFETY: The underlying C++ logger is known to be thread-safe.
unsafe impl Send for MwLogger {}

// SAFETY: The underlying C++ logger is known to be thread-safe.
unsafe impl Sync for MwLogger {}
