// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use console::style;

mod emojis;
use emojis::*;

use crate::api::client::Client;

// Pretty logo :3
static ASCII_LOGO: &str = include_str!("ascii.txt");

fn print_intro() -> color_eyre::Result<()> {
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

pub fn run() -> color_eyre::Result<()> {
    print_intro()?;

    let our_exe = std::env::current_exe()?.to_string_lossy().to_string();

    // Create temporary client just for enumerating devices
    let mut client = Client::new_direct_with_path(&our_exe)?;

    let devices = client.get_block_devices()?;
    cliclack::log::remark(format!(
        "Found the following block devices: {devices}",
        devices = style(devices.join(", ")).bold()
    ))?;

    // Terminate helper backend
    client.shutdown_backend()?;

    cliclack::outro(format!(
        "Exiting - No changes have been written {}",
        SPARKLES
    ))?;

    Ok(())
}
