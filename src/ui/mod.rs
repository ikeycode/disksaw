// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use console::style;

mod emojis;
mod menu;
use emojis::*;
use menu::{enums_to_cliclack, PartitionMenu};
use partitioning::planner::format_size;
use tabled::{settings::Style, Table, Tabled};

use crate::api::{self, client::Client};

/// ASCII art logo loaded from ascii.txt file
static ASCII_LOGO: &str = include_str!("ascii.txt");

/// Prints the intro message with styling and warning
fn print_intro() -> color_eyre::Result<()> {
    cliclack::intro(format!(
        "{name_disk}{name_saw} {DISK}{SAW} - {version}",
        name_disk = style("disk").cyan().bold(),
        name_saw = style("saw").yellow().bold(),
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

/// Formats a block device for display, showing path, model and size
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

/// Struct representing a partition for display purposes
#[derive(Debug, Tabled)]
struct DisplayPartition {
    #[tabled(rename = "💽 Name")]
    name: String,
    #[tabled(rename = "Starting sector")]
    start: u64,
    #[tabled(rename = "Ending sector")]
    end: u64,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "📂 Filesystem")]
    filesystem: String,
    #[tabled(skip)]
    path: String,
}

impl From<&api::Partition> for DisplayPartition {
    fn from(val: &api::Partition) -> Self {
        DisplayPartition {
            name: format!("{}", style(format!("/dev/{}", val.name)).cyan()),
            start: val.start,
            end: val.end,
            size: style(format_size(val.size * 512)).yellow().to_string(),
            path: val.path.clone(),
            filesystem: style("unknown").dim().to_string(),
        }
    }
}

/// Prints a formatted table of partitions for a given block device
fn print_partitions(client: &mut Client, device: &api::BlockDevice) -> color_eyre::Result<()> {
    let mut partitions: Vec<DisplayPartition> =
        device.partitions.iter().map(Into::into).collect::<Vec<_>>();
    for part in partitions.iter_mut() {
        if let Ok(sb) = client.get_superblock(&part.path) {
            part.filesystem = style(sb.filesystem).cyan().bold().to_string();
        }
    }

    let mut table = Table::new(partitions);
    table.with(Style::modern_rounded());
    cliclack::note(format!("{}Partitions", emojis::CHART), table)?;
    Ok(())
}

/// Main entry point for the disk partitioning interface
pub fn run() -> color_eyre::Result<()> {
    print_intro()?;

    let our_exe = std::env::current_exe()?.to_string_lossy().to_string();
    let mut client = Client::new_privileged_with_path(&our_exe)?;

    // Get list of valid block devices, filtering out loopback devices without backing files
    let mut devices = client
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

    let n = cliclack::select("Select a disk from the list")
        .items(&listing)
        .filter_mode()
        .interact()?;
    let device = devices.remove(n);

    print_partitions(&mut client, &device)?;

    // Main interaction loop
    loop {
        let p = *cliclack::select("What do you want to do")
            .items(&enums_to_cliclack(&[
                PartitionMenu::Create,
                PartitionMenu::Delete,
                PartitionMenu::Resize,
                PartitionMenu::Format,
                PartitionMenu::Mount,
                PartitionMenu::Unmount,
                PartitionMenu::List,
                PartitionMenu::Quit,
            ]))
            .filter_mode()
            .interact()?;

        // prevent unnecessary redraws
        if matches!(p, PartitionMenu::Quit) {
            break;
        }

        match p {
            PartitionMenu::List => {
                print_partitions(&mut client, &device)?;
            }
            _ => {
                cliclack::log::error(format!("Unimplemented: {}", style(&p).bold().yellow()))?;
            }
        }
    }

    cliclack::outro(format!(
        "Exiting - No changes have been written {}",
        SPARKLES
    ))?;

    // Terminate helper backend
    client.shutdown_backend()?;

    Ok(())
}
