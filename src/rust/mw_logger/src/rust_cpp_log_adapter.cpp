/********************************************************************************
 * Copyright (c) 2025 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * SPDX-License-Identifier: Apache-2.0
 ********************************************************************************/

#include "score/mw/log/logger.h"

extern "C" {
using namespace score::mw::log;

Logger* logger_create(const char* context) { return &CreateLogger(context); }

bool logger_log_level_enabled(const Logger* logger, LogLevel log_level) {
    return logger->IsLogEnabled(log_level);
}

LogLevel logger_log_level_current(const Logger* logger) {
    auto first{static_cast<uint8_t>(LogLevel::kOff)};
    auto last{static_cast<uint8_t>(LogLevel::kVerbose)};
    // Reversed order - `kOff` always seem to report true.
    for (uint8_t i = last; i > first; --i) {
        auto current = static_cast<LogLevel>(i);
        if (logger->IsLogEnabled(current)) {
            return current;
        }
    }

    // Fall-back.
    return LogLevel::kOff;
}

LogStream* logger_log_stream_create(const Logger* logger, LogLevel log_level) {
    auto log_stream{logger->WithLevel(log_level)};
    return new LogStream{std::move(log_stream)};
}

void log_stream_destroy(LogStream* log_stream) { delete log_stream; }

void log_stream_log_bool(LogStream* log_stream, const bool* value) { *log_stream << *value; }

void log_stream_log_f32(LogStream* log_stream, const float* value) { *log_stream << *value; }

void log_stream_log_f64(LogStream* log_stream, const double* value) { *log_stream << *value; }

void log_stream_log_string(LogStream* log_stream, const char* value, size_t size) {
    *log_stream << LogString(value, size);
}

void log_stream_log_i8(LogStream* log_stream, const int8_t* value) { *log_stream << *value; }

void log_stream_log_i16(LogStream* log_stream, const int16_t* value) { *log_stream << *value; }

void log_stream_log_i32(LogStream* log_stream, const int32_t* value) { *log_stream << *value; }

void log_stream_log_i64(LogStream* log_stream, const int64_t* value) { *log_stream << *value; }

void log_stream_log_u8(LogStream* log_stream, const uint8_t* value) { *log_stream << *value; }

void log_stream_log_u16(LogStream* log_stream, const uint16_t* value) { *log_stream << *value; }

void log_stream_log_u32(LogStream* log_stream, const uint32_t* value) { *log_stream << *value; }

void log_stream_log_u64(LogStream* log_stream, const uint64_t* value) { *log_stream << *value; }

void log_stream_log_bin8(LogStream* log_stream, const uint8_t* value) {
    *log_stream << LogBin8{*value};
}

void log_stream_log_bin16(LogStream* log_stream, const uint16_t* value) {
    *log_stream << LogBin16{*value};
}

void log_stream_log_bin32(LogStream* log_stream, const uint32_t* value) {
    *log_stream << LogBin32{*value};
}

void log_stream_log_bin64(LogStream* log_stream, const uint64_t* value) {
    *log_stream << LogBin64{*value};
}

void log_stream_log_hex8(LogStream* log_stream, const uint8_t* value) {
    *log_stream << LogHex8{*value};
}

void log_stream_log_hex16(LogStream* log_stream, const uint16_t* value) {
    *log_stream << LogHex16{*value};
}

void log_stream_log_hex32(LogStream* log_stream, const uint32_t* value) {
    *log_stream << LogHex32{*value};
}

void log_stream_log_hex64(LogStream* log_stream, const uint64_t* value) {
    *log_stream << LogHex64{*value};
}
}
