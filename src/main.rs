// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use console::{style, Emoji};
use pretty_env_logger::formatted_builder;

// Pretty logo :3
static ASCII_LOGO: &str = include_str!("ascii.txt");

fn print_intro() -> color_eyre::Result<()> {
    static SAW: Emoji<'_, '_> = Emoji("🪚 ", "");
    static DISK: Emoji<'_, '_> = Emoji("💾 ", "");
    static LIGHTNING: Emoji<'_, '_> = Emoji("⚡️ ", "");

    cliclack::intro(format!(
        "{name_saw}{name_disk} - {version} {SAW}{DISK}",
        name_saw = style("saw").yellow().bold(),
        name_disk = style("disk").bold(),
        version = style(env!("CARGO_PKG_VERSION")).dim(),
    ))?;
    cliclack::log::remark(format!(
        "{header}{line1}{line2} {LIGHTNING}",
        header = style("WARNING:").bold().red(),
        line1 = style(" This tool is experimental and may cause data loss.").bold(),
        line2 = style(" Use at your own risk.").bold().yellow(),
    ))?;
    cliclack::log::remark(ASCII_LOGO)?;
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    static SPARKLES: Emoji<'_, '_> = Emoji("✨ ", "");

    // Initialize color_eyre for better error messages + error reports
    color_eyre::config::HookBuilder::default()
        .issue_url("https://github.com/ikeycode/sawdisk/issues/new")
        .add_issue_metadata("version", env!("CARGO_PKG_VERSION"))
        .issue_filter(|_| true)
        .install()
        .unwrap();

    // Initialize the logger
    formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    print_intro()?;

    cliclack::outro(format!(
        "Exiting - No changes have been written {}",
        SPARKLES
    ))?;
    Ok(())
}
