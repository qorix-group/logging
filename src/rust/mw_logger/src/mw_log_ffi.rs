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

use core::ffi::c_char;
use std::ffi::CString;

/// Represents severity of a log message.
#[repr(u8)]
pub enum LogLevel {
    #[allow(dead_code)]
    Off = 0x00,
    Fatal = 0x01,
    Error = 0x02,
    Warn = 0x03,
    Info = 0x04,
    Debug = 0x05,
    Verbose = 0x06,
}

impl From<mw_log::Level> for LogLevel {
    fn from(mw_log_level: mw_log::Level) -> Self {
        match mw_log_level {
            mw_log::Level::Fatal => LogLevel::Fatal,
            mw_log::Level::Error => LogLevel::Error,
            mw_log::Level::Warn => LogLevel::Warn,
            mw_log::Level::Info => LogLevel::Info,
            mw_log::Level::Debug => LogLevel::Debug,
            mw_log::Level::Trace => LogLevel::Verbose,
        }
    }
}

impl From<LogLevel> for mw_log::Level {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Fatal => mw_log::Level::Fatal,
            LogLevel::Error => mw_log::Level::Error,
            LogLevel::Warn => mw_log::Level::Warn,
            LogLevel::Info => mw_log::Level::Info,
            LogLevel::Debug => mw_log::Level::Debug,
            LogLevel::Verbose => mw_log::Level::Trace,
            _ => panic!("Log level not supported"),
        }
    }
}

impl From<LogLevel> for mw_log::LevelFilter {
    fn from(level: LogLevel) -> mw_log::LevelFilter {
        match level {
            LogLevel::Off => mw_log::LevelFilter::Off,
            LogLevel::Fatal => mw_log::LevelFilter::Fatal,
            LogLevel::Error => mw_log::LevelFilter::Error,
            LogLevel::Warn => mw_log::LevelFilter::Warn,
            LogLevel::Info => mw_log::LevelFilter::Info,
            LogLevel::Debug => mw_log::LevelFilter::Debug,
            LogLevel::Verbose => mw_log::LevelFilter::Trace,
        }
    }
}

/// Opaque type representing `LogStream`.
#[repr(C)]
pub struct LogStreamPtr {
    _private: [u8; 0],
}

pub struct LogStream {
    ptr: *mut LogStreamPtr,
}

impl LogStream {
    pub fn new(ptr: *mut LogStreamPtr) -> Self {
        Self { ptr }
    }

    pub fn log_bool(&self, v: &bool) {
        unsafe { log_stream_log_bool(self.ptr, v as *const bool) }
    }

    pub fn log_f32(&self, v: &f32) {
        unsafe { log_stream_log_f32(self.ptr, v as *const f32) }
    }

    pub fn log_f64(&self, v: &f64) {
        unsafe { log_stream_log_f64(self.ptr, v as *const f64) }
    }

    pub fn log_string(&self, v: &str) {
        let v_cstr = CString::new(v).unwrap();
        let v_cchar = v_cstr.as_ptr() as *const c_char;
        let size = v.len();
        unsafe { log_stream_log_string(self.ptr, v_cchar, size) }
    }

    pub fn log_i8(&self, v: &i8) {
        unsafe { log_stream_log_i8(self.ptr, v as *const i8) }
    }

    pub fn log_i16(&self, v: &i16) {
        unsafe { log_stream_log_i16(self.ptr, v as *const i16) }
    }

    pub fn log_i32(&self, v: &i32) {
        unsafe { log_stream_log_i32(self.ptr, v as *const i32) }
    }

    pub fn log_i64(&self, v: &i64) {
        unsafe { log_stream_log_i64(self.ptr, v as *const i64) }
    }

    pub fn log_u8(&self, v: &u8) {
        unsafe { log_stream_log_u8(self.ptr, v as *const u8) }
    }

    pub fn log_u16(&self, v: &u16) {
        unsafe { log_stream_log_u16(self.ptr, v as *const u16) }
    }

    pub fn log_u32(&self, v: &u32) {
        unsafe { log_stream_log_u32(self.ptr, v as *const u32) }
    }

    pub fn log_u64(&self, v: &u64) {
        unsafe { log_stream_log_u64(self.ptr, v as *const u64) }
    }

    pub fn log_bin8(&self, v: &u8) {
        unsafe { log_stream_log_bin8(self.ptr, v as *const u8) }
    }

    pub fn log_bin16(&self, v: &u16) {
        unsafe { log_stream_log_bin16(self.ptr, v as *const u16) }
    }

    pub fn log_bin32(&self, v: &u32) {
        unsafe { log_stream_log_bin32(self.ptr, v as *const u32) }
    }

    pub fn log_bin64(&self, v: &u64) {
        unsafe { log_stream_log_bin64(self.ptr, v as *const u64) }
    }

    pub fn log_hex8(&self, v: &u8) {
        unsafe { log_stream_log_hex8(self.ptr, v as *const u8) }
    }

    pub fn log_hex16(&self, v: &u16) {
        unsafe { log_stream_log_hex16(self.ptr, v as *const u16) }
    }

    pub fn log_hex32(&self, v: &u32) {
        unsafe { log_stream_log_hex32(self.ptr, v as *const u32) }
    }

    pub fn log_hex64(&self, v: &u64) {
        unsafe { log_stream_log_hex64(self.ptr, v as *const u64) }
    }
}

impl Drop for LogStream {
    fn drop(&mut self) {
        unsafe { log_stream_destroy(self.ptr) }
    }
}

/// Opaque type representing `Logger`.
#[repr(C)]
pub struct LoggerPtr {
    _private: [u8; 0],
}

pub struct Logger {
    ptr: *mut LoggerPtr,
}

impl Logger {
    pub fn new(context: &str) -> Self {
        let context_cstr = CString::new(context).unwrap();
        let logger_ptr = unsafe { logger_create(context_cstr.as_ptr().cast::<c_char>()) };
        Self { ptr: logger_ptr }
    }

    pub fn log_level_enabled(&self, log_level: LogLevel) -> bool {
        unsafe { logger_log_level_enabled(self.ptr, log_level) }
    }

    pub fn log_level_current(&self) -> LogLevel {
        unsafe { logger_log_level_current(self.ptr) }
    }

    pub fn log_stream_create(&self, log_level: LogLevel) -> LogStream {
        let ptr = unsafe { logger_log_stream_create(self.ptr, log_level) };
        LogStream::new(ptr)
    }
}

unsafe extern "C" {
    fn logger_create(context: *const c_char) -> *mut LoggerPtr;
    fn logger_log_level_enabled(logger: *const LoggerPtr, log_level: LogLevel) -> bool;
    fn logger_log_level_current(logger: *const LoggerPtr) -> LogLevel;
    fn logger_log_stream_create(logger: *const LoggerPtr, log_level: LogLevel) -> *mut LogStreamPtr;
    fn log_stream_destroy(log_stream: *mut LogStreamPtr);
    fn log_stream_log_bool(log_stream: *mut LogStreamPtr, value: *const bool);
    fn log_stream_log_f32(log_stream: *mut LogStreamPtr, value: *const f32);
    fn log_stream_log_f64(log_stream: *mut LogStreamPtr, value: *const f64);
    fn log_stream_log_string(log_stream: *mut LogStreamPtr, value: *const c_char, size: usize);
    fn log_stream_log_i8(log_stream: *mut LogStreamPtr, value: *const i8);
    fn log_stream_log_i16(log_stream: *mut LogStreamPtr, value: *const i16);
    fn log_stream_log_i32(log_stream: *mut LogStreamPtr, value: *const i32);
    fn log_stream_log_i64(log_stream: *mut LogStreamPtr, value: *const i64);
    fn log_stream_log_u8(log_stream: *mut LogStreamPtr, value: *const u8);
    fn log_stream_log_u16(log_stream: *mut LogStreamPtr, value: *const u16);
    fn log_stream_log_u32(log_stream: *mut LogStreamPtr, value: *const u32);
    fn log_stream_log_u64(log_stream: *mut LogStreamPtr, value: *const u64);
    fn log_stream_log_bin8(log_stream: *mut LogStreamPtr, value: *const u8);
    fn log_stream_log_bin16(log_stream: *mut LogStreamPtr, value: *const u16);
    fn log_stream_log_bin32(log_stream: *mut LogStreamPtr, value: *const u32);
    fn log_stream_log_bin64(log_stream: *mut LogStreamPtr, value: *const u64);
    fn log_stream_log_hex8(log_stream: *mut LogStreamPtr, value: *const u8);
    fn log_stream_log_hex16(log_stream: *mut LogStreamPtr, value: *const u16);
    fn log_stream_log_hex32(log_stream: *mut LogStreamPtr, value: *const u32);
    fn log_stream_log_hex64(log_stream: *mut LogStreamPtr, value: *const u64);
}
