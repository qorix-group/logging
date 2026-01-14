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

use core::alloc::Layout;
use core::cmp::min;
use core::ffi::c_char;

/// Represents severity of a log message.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

/// Name of the context.
/// Max 4 bytes containing ASCII characters.
struct Context {
    data: [c_char; 4],
    size: usize,
}

impl Context {
    pub fn from(context: &str) -> Self {
        // Disallow non-ASCII strings.
        // ASCII characters are single byte in UTF-8.
        if !context.is_ascii() {
            panic!("Provided context contains non-ASCII characters: {context}");
        }

        // Get number of characters.
        let size = min(context.len(), 4);

        // Copy data into array.
        let mut data: [c_char; 4] = [0; 4];
        unsafe {
            core::ptr::copy_nonoverlapping(context.as_ptr(), data.as_mut_ptr() as *mut u8, size);
        }

        Self { data, size }
    }
}

/// Opaque type representing `Recorder`.
#[repr(C)]
struct RecorderPtr {
    _private: [u8; 0],
}

/// Recorder instance.
pub(crate) struct Recorder {
    inner: *mut RecorderPtr,
}

impl Recorder {
    pub fn new() -> Self {
        let inner = unsafe { recorder_get() };
        Self { inner }
    }

    pub fn log_level(&self, context: &str) -> LogLevel {
        let context = Context::from(context);
        unsafe { recorder_log_level(self.inner, context.data.as_ptr(), context.size) }
    }
}

/// Opaque type representing `SlotHandle`.
#[cfg(feature = "x86_64_linux")]
#[repr(C, align(8))]
pub(crate) struct SlotHandlePtr {
    _private: [u8; 24],
}

impl SlotHandlePtr {
    pub fn layout_rust() -> Layout {
        Layout::new::<Self>()
    }

    pub fn layout_cpp() -> Layout {
        let size = unsafe { slot_handle_size() };
        let align = unsafe { slot_handle_alignment() };
        Layout::from_size_align(size, align).expect("Invalid SlotHandle layout, size: {size}, alignment: {align}")
    }
}

/// Single log message stream.
pub struct LogStream<'a> {
    recorder: &'a Recorder,
    slot: Option<SlotHandlePtr>,
}

impl<'a> LogStream<'a> {
    pub fn new(recorder: &'a Recorder, context: &str, log_level: LogLevel) -> Self {
        // Create context object.
        let context = Context::from(context);

        // Start record.
        // `SlotHandle` is allocated on stack.
        let mut slot_buffer = SlotHandlePtr { _private: [0; 24] };
        let slot_result = unsafe {
            recorder_start(
                recorder.inner,
                context.data.as_ptr(),
                context.size,
                log_level,
                &mut slot_buffer as *mut SlotHandlePtr,
            )
        };

        // Store buffer only if acquired.
        let slot = if !slot_result.is_null() {
            Some(slot_buffer)
        } else {
            None
        };

        Self { recorder, slot }
    }

    pub fn log_bool(&mut self, v: &bool) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_bool(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const bool);
            }
        }
    }

    pub fn log_f32(&mut self, v: &f32) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_f32(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const f32);
            }
        }
    }

    pub fn log_f64(&mut self, v: &f64) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_f64(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const f64);
            }
        }
    }

    pub fn log_string(&mut self, v: &str) {
        // Disallow non-ASCII strings.
        if !v.is_ascii() {
            panic!("Provided string contains non-ASCII characters: {v}");
        }

        // Get string as pointer and size.
        // ASCII characters are single byte in UTF-8.
        let v_ptr = v.as_ptr() as *const c_char;
        let v_size = v.len();

        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_string(self.recorder.inner, slot as *mut SlotHandlePtr, v_ptr, v_size);
            }
        }
    }

    pub fn log_i8(&mut self, v: &i8) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_i8(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const i8);
            }
        }
    }

    pub fn log_i16(&mut self, v: &i16) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_i16(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const i16);
            }
        }
    }

    pub fn log_i32(&mut self, v: &i32) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_i32(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const i32);
            }
        }
    }

    pub fn log_i64(&mut self, v: &i64) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_i64(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const i64);
            }
        }
    }

    pub fn log_u8(&mut self, v: &u8) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_u8(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u8);
            }
        }
    }

    pub fn log_u16(&mut self, v: &u16) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_u16(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u16);
            }
        }
    }

    pub fn log_u32(&mut self, v: &u32) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_u32(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u32);
            }
        }
    }

    pub fn log_u64(&mut self, v: &u64) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_u64(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u64);
            }
        }
    }

    pub fn log_bin8(&mut self, v: &u8) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_bin8(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u8);
            }
        }
    }

    pub fn log_bin16(&mut self, v: &u16) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_bin16(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u16);
            }
        }
    }

    pub fn log_bin32(&mut self, v: &u32) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_bin32(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u32);
            }
        }
    }

    pub fn log_bin64(&mut self, v: &u64) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_bin64(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u64);
            }
        }
    }

    pub fn log_hex8(&mut self, v: &u8) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_hex8(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u8);
            }
        }
    }

    pub fn log_hex16(&mut self, v: &u16) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_hex16(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u16);
            }
        }
    }

    pub fn log_hex32(&mut self, v: &u32) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_hex32(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u32);
            }
        }
    }

    pub fn log_hex64(&mut self, v: &u64) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe {
                log_hex64(self.recorder.inner, slot as *mut SlotHandlePtr, v as *const u64);
            }
        }
    }
}

impl Drop for LogStream<'_> {
    fn drop(&mut self) {
        if let Some(slot) = self.slot.as_mut() {
            unsafe { recorder_stop(self.recorder.inner, slot) }
        }
    }
}

unsafe extern "C" {
    fn recorder_get() -> *mut RecorderPtr;
    fn recorder_start(
        recorder: *mut RecorderPtr,
        context: *const c_char,
        context_size: usize,
        log_level: LogLevel,
        slot: *mut SlotHandlePtr,
    ) -> *mut SlotHandlePtr;
    fn recorder_stop(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr);
    fn recorder_log_level(recorder: *const RecorderPtr, context: *const c_char, context_size: usize) -> LogLevel;
    fn log_bool(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const bool);
    fn log_f32(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const f32);
    fn log_f64(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const f64);
    fn log_string(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const c_char, size: usize);
    fn log_i8(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const i8);
    fn log_i16(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const i16);
    fn log_i32(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const i32);
    fn log_i64(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const i64);
    fn log_u8(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u8);
    fn log_u16(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u16);
    fn log_u32(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u32);
    fn log_u64(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u64);
    fn log_bin8(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u8);
    fn log_bin16(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u16);
    fn log_bin32(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u32);
    fn log_bin64(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u64);
    fn log_hex8(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u8);
    fn log_hex16(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u16);
    fn log_hex32(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u32);
    fn log_hex64(recorder: *mut RecorderPtr, slot: *mut SlotHandlePtr, value: *const u64);
    fn slot_handle_size() -> usize;
    fn slot_handle_alignment() -> usize;
}
