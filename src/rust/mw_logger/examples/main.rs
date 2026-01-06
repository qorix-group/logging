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

use mw_log::{debug, error, info, trace, warn, Log};
use mw_logger::MwLoggerBuilder;
use std::path::PathBuf;

fn main() {
    // Setup for example using config file
    let path = PathBuf::from(std::env::current_dir().unwrap())
        .join(file!())
        .parent()
        .unwrap()
        .join("config")
        .join("logging.json");

    unsafe { std::env::set_var("MW_LOG_CONFIG_FILE", path.as_os_str()) };

    // Just initialize and set as default logger
    MwLoggerBuilder::new()
        .show_module(false)
        .show_file(true)
        .show_line(false)
        .set_as_default_logger();

    trace!("This is a trace log - hidden");
    debug!("This is a debug log - hidden");
    error!("This is an error log");
    info!("This is an info log");
    warn!("This is a warn log");

    let x1 = 123.4;
    let x2 = 111;
    let x3 = true;
    let x4 = -0x3Fi8;
    error!(
        "This is an error log with numeric values: {} {} {} {:x}",
        x1, x2, x3, x4,
    );

    // Using logger instance with context
    let logger = MwLoggerBuilder::new()
        .context("ALFA")
        .show_module(false)
        .show_file(true)
        .show_line(false)
        .build();

    trace!(
        logger: logger,
        "This is a trace log - hidden"
    );
    debug!(logger: logger, "This is a debug log - hidden");
    error!(logger: logger, "This is an error log");
    info!(logger: logger, "This is an info log");
    warn!(logger: logger, "This is a warn log");
}
