# *******************************************************************************
# Copyright (c) 2025 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache License Version 2.0 which is available at
# https://www.apache.org/licenses/LICENSE-2.0
#
# SPDX-License-Identifier: Apache-2.0
# *******************************************************************************

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

config_setting(
    name = "x86_64-linux",
    constraint_values = [
        "@platforms//cpu:x86_64",
        "@platforms//os:linux",
    ],
)

cc_library(
    name = "librust_cpp_log_adapter",
    srcs = ["src/rust_cpp_log_adapter.cpp"],
    # C++/Rust interface objects must share layout.
    defines = select({
        ":x86_64-linux": ["x86_64_linux"],
        "//conditions:default": [],
    }),
    visibility = ["//visibility:private"],
    deps = [
        "//score/mw/log/detail/common:recorder_factory",
        "@score_baselibs//score/mw/log:frontend",
    ],
)

rust_library(
    name = "mw_logger",
    srcs = glob(["src/**/*.rs"]),
    # C++/Rust interface objects must share layout.
    crate_features = select({
        ":x86_64-linux": ["x86_64_linux"],
        "//conditions:default": [],
    }),
    edition = "2021",
    visibility = ["//visibility:public"],
    deps = [
        ":librust_cpp_log_adapter",
        "@score_baselibs_rust//src/log/mw_log",
    ],
)

rust_binary(
    name = "example",
    srcs = ["examples/main.rs"],
    data = [
        "examples/config/logging.json",
    ],
    edition = "2021",
    rustc_flags = select({
        "@platforms//os:qnx": [
            "-Clink-arg=-lc++",
            "-Clink-arg=-lm",
        ],
        "//conditions:default": [
            "-Clink-arg=-lstdc++",
            "-Clink-arg=-lm",
            "-Clink-arg=-lc",
        ],
    }),
    visibility = ["//visibility:public"],
    deps = [
        ":mw_logger",
        "@score_baselibs_rust//src/log/mw_log",
    ],
)
