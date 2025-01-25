// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::env;

use pretty_env_logger::formatted_builder;
use sawdisk::{backend_service, ui};

fn main() -> color_eyre::Result<()> {
    // Check if --backend-service is in args
    let is_backend = env::args().any(|arg| arg == "--backend-service");

    // Initialize color_eyre for better error messages + error reports
    color_eyre::config::HookBuilder::default()
        .issue_url("https://github.com/ikeycode/sawdisk/issues/new")
        .add_issue_metadata("version", env!("CARGO_PKG_VERSION"))
        .add_issue_metadata("backend", is_backend.to_string())
        .display_env_section(true)
        .display_location_section(true)
        .add_default_filters()
        .capture_span_trace_by_default(true)
        .issue_filter(|_| true)
        .install()
        .unwrap();

    // Initialize the logger
    formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    if is_backend {
        Ok(backend_service::run()?)
    } else {
        Ok(ui::run()?)
    }
}
