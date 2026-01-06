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

//! C++-based backend for `mw_log`.

#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::alloc_instead_of_core)]

mod mw_log_ffi;

use crate::mw_log_ffi::{LogStream, Logger};
use mw_log::fmt::{score_write, write, DisplayHint, Error, FormatSpec, Result as FmtResult, ScoreWrite};
use mw_log::{LevelFilter, Log, Metadata, Record};

/// C++-based writer implementation.
struct MwLogWriter {
    log_stream: LogStream,
}

impl MwLogWriter {
    pub fn new(log_stream: LogStream) -> Self {
        Self { log_stream }
    }
}

impl ScoreWrite for MwLogWriter {
    fn write_bool(&mut self, v: &bool, _spec: &FormatSpec) -> FmtResult {
        self.log_stream.log_bool(v);
        Ok(())
    }

    fn write_f32(&mut self, v: &f32, _spec: &FormatSpec) -> FmtResult {
        self.log_stream.log_f32(v);
        Ok(())
    }

    fn write_f64(&mut self, v: &f64, _spec: &FormatSpec) -> FmtResult {
        self.log_stream.log_f64(v);
        Ok(())
    }

    fn write_i8(&mut self, v: &i8, spec: &FormatSpec) -> FmtResult {
        match spec.get_display_hint() {
            DisplayHint::NoHint => self.log_stream.log_i8(v),
            DisplayHint::LowerHex | DisplayHint::UpperHex => {
                let v_u8 = unsafe { core::mem::transmute(v) };
                self.log_stream.log_hex8(v_u8);
            },
            DisplayHint::Binary => {
                let v_u8 = unsafe { core::mem::transmute(v) };
                self.log_stream.log_bin8(v_u8);
            },
            _ => return Err(Error),
        }
        Ok(())
    }

    fn write_i16(&mut self, v: &i16, spec: &FormatSpec) -> FmtResult {
        match spec.get_display_hint() {
            DisplayHint::NoHint => self.log_stream.log_i16(v),
            DisplayHint::LowerHex | DisplayHint::UpperHex => {
                let v_u16 = unsafe { core::mem::transmute(v) };
                self.log_stream.log_hex16(v_u16);
            },
            DisplayHint::Binary => {
                let v_u16 = unsafe { core::mem::transmute(v) };
                self.log_stream.log_bin16(v_u16);
            },
            _ => return Err(Error),
        }
        Ok(())
    }

    fn write_i32(&mut self, v: &i32, spec: &FormatSpec) -> FmtResult {
        match spec.get_display_hint() {
            DisplayHint::NoHint => self.log_stream.log_i32(v),
            DisplayHint::LowerHex | DisplayHint::UpperHex => {
                let v_u32 = unsafe { core::mem::transmute(v) };
                self.log_stream.log_hex32(v_u32);
            },
            DisplayHint::Binary => {
                let v_u32 = unsafe { core::mem::transmute(v) };
                self.log_stream.log_bin32(v_u32);
            },
            _ => return Err(Error),
        }
        Ok(())
    }

    fn write_i64(&mut self, v: &i64, spec: &FormatSpec) -> FmtResult {
        match spec.get_display_hint() {
            DisplayHint::NoHint => self.log_stream.log_i64(v),
            DisplayHint::LowerHex | DisplayHint::UpperHex => {
                let v_u64 = unsafe { core::mem::transmute(v) };
                self.log_stream.log_hex64(v_u64);
            },
            DisplayHint::Binary => {
                let v_u64 = unsafe { core::mem::transmute(v) };
                self.log_stream.log_bin64(v_u64);
            },
            _ => return Err(Error),
        }
        Ok(())
    }

    fn write_u8(&mut self, v: &u8, spec: &FormatSpec) -> FmtResult {
        match spec.get_display_hint() {
            DisplayHint::NoHint => self.log_stream.log_u8(v),
            DisplayHint::LowerHex | DisplayHint::UpperHex => self.log_stream.log_hex8(v),
            DisplayHint::Binary => self.log_stream.log_bin8(v),
            _ => return Err(Error),
        }
        Ok(())
    }

    fn write_u16(&mut self, v: &u16, spec: &FormatSpec) -> FmtResult {
        match spec.get_display_hint() {
            DisplayHint::NoHint => self.log_stream.log_u16(v),
            DisplayHint::LowerHex | DisplayHint::UpperHex => self.log_stream.log_hex16(v),
            DisplayHint::Binary => self.log_stream.log_bin16(v),
            _ => return Err(Error),
        }
        Ok(())
    }

    fn write_u32(&mut self, v: &u32, spec: &FormatSpec) -> FmtResult {
        match spec.get_display_hint() {
            DisplayHint::NoHint => self.log_stream.log_u32(v),
            DisplayHint::LowerHex | DisplayHint::UpperHex => self.log_stream.log_hex32(v),
            DisplayHint::Binary => self.log_stream.log_bin32(v),
            _ => return Err(Error),
        }
        Ok(())
    }

    fn write_u64(&mut self, v: &u64, spec: &FormatSpec) -> FmtResult {
        match spec.get_display_hint() {
            DisplayHint::NoHint => self.log_stream.log_u64(v),
            DisplayHint::LowerHex | DisplayHint::UpperHex => self.log_stream.log_hex64(v),
            DisplayHint::Binary => self.log_stream.log_bin64(v),
            _ => return Err(Error),
        }
        Ok(())
    }

    fn write_str(&mut self, v: &str, _spec: &FormatSpec) -> FmtResult {
        self.log_stream.log_string(v);
        Ok(())
    }
}

/// Builder for the `MwLogger`.
pub struct MwLoggerBuilder(MwLogger);

impl MwLoggerBuilder {
    /// Create builder with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set context for the `MwLogger`.
    pub fn context(mut self, context: &str) -> Self {
        self.0.context = context.to_string();
        self
    }

    /// Show module name in logs.
    pub fn show_module(mut self, show_module: bool) -> Self {
        self.0.show_module = show_module;
        self
    }

    /// Show file name in logs.
    pub fn show_file(mut self, show_file: bool) -> Self {
        self.0.show_file = show_file;
        self
    }

    /// Show line number in logs.
    pub fn show_line(mut self, show_line: bool) -> Self {
        self.0.show_line = show_line;
        self
    }

    /// Build the `MwLogger` with provided context and configuration.
    pub fn build(self) -> MwLogger {
        self.0
    }

    /// Build the `MwLogger` and set it as the default logger.
    pub fn set_as_default_logger(self) {
        let logger = self.build();
        mw_log::set_max_level(logger.log_level());
        if let Err(e) = mw_log::set_global_logger(Box::new(logger)) {
            panic!("unable to set logger: {e}");
        }
    }
}

impl Default for MwLoggerBuilder {
    fn default() -> Self {
        Self(MwLogger {
            context: "DFLT".to_string(),
            show_module: false,
            show_file: false,
            show_line: false,
        })
    }
}

/// C++-based logger implementation.
pub struct MwLogger {
    context: String,
    show_module: bool,
    show_file: bool,
    show_line: bool,
}

impl MwLogger {
    /// Get FFI logger object.
    fn logger(context: &str) -> Logger {
        // Logger objects are cached internally.
        Logger::new(context)
    }

    /// Current log level for default context.
    pub(crate) fn log_level(&self) -> LevelFilter {
        Self::logger(self.context()).log_level_current().into()
    }
}

impl Log for MwLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let logger = Self::logger(metadata.context());
        logger.log_level_enabled(metadata.level().into())
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

        // Create logger and stream.
        // Logger objects are cached internally.
        let logger = Self::logger(self.context());
        let log_stream = logger.log_stream_create(metadata.level().into());
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
        // Written data is flushed on writer drop.
    }

    fn flush(&self) {
        // No-op.
    }
}

// SAFETY: The underlying C++ logger is known to be thread-safe.
unsafe impl Send for MwLogger {}

// SAFETY: The underlying C++ logger is known to be thread-safe.
unsafe impl Sync for MwLogger {}
