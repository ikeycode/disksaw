// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use console::style;

mod emojis;
mod menu;
use emojis::*;
use menu::{enums_to_cliclack, PartitionMenu};
use partitioning::planner::format_size;

use crate::api::{self, client::Client};

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

fn render_device(device: &api::BlockDevice, path_width: usize) -> String {
    let model = device.model.as_deref().unwrap_or("Unknown");
    format!(
        "{:width$} {} {}",
        style(&device.path).bold(),
        style("⟶").dim(),
        style(format!("{} ({})", model, format_size(device.size))).yellow(),
        width = path_width
    )
}

pub fn run() -> color_eyre::Result<()> {
    print_intro()?;

    let our_exe = std::env::current_exe()?.to_string_lossy().to_string();

    // Create temporary client just for enumerating devices
    let mut client = Client::new_direct_with_path(&our_exe)?;

    let devices = client
        .get_block_devices()?
        .into_iter()
        .filter(|d| {
            if let api::BlockDeviceKind::Loopback { backing_file } = &d.kind {
                backing_file.is_some()
            } else {
                true
            }
        })
        .collect::<Vec<_>>();
    let path_width = devices.iter().map(|d| d.path.len()).max().unwrap_or(0);
    let listing = devices
        .iter()
        .enumerate()
        .map(|(i, d)| (i, render_device(d, path_width), ""))
        .collect::<Vec<_>>();

    let _ = cliclack::select("Select a disk from the list")
        .items(&listing)
        .filter_mode()
        .interact()?;

    // Terminate helper backend
    client.shutdown_backend()?;
    cliclack::clear_screen()?;

    loop {
        let p = *cliclack::select("What do you want to do")
            .items(&enums_to_cliclack(&[
                PartitionMenu::Create,
                PartitionMenu::Delete,
                PartitionMenu::Resize,
                PartitionMenu::Format,
                PartitionMenu::Mount,
                PartitionMenu::Unmount,
                PartitionMenu::Info,
                PartitionMenu::Quit,
            ]))
            .filter_mode()
            .interact()?;

        cliclack::clear_screen()?;

        match p {
            PartitionMenu::Quit => break,
            _ => {
                cliclack::log::error(format!("Unimplemented: {}", style(&p).bold().yellow()))?;
            }
        }
    }

    cliclack::outro(format!(
        "Exiting - No changes have been written {}",
        SPARKLES
    ))?;

    Ok(())
}
