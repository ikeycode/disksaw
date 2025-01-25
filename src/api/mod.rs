// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

pub mod client;

/// The kind of block device
#[derive(Debug, Serialize, Deserialize)]
pub enum BlockDeviceKind {
    Disk,
    Loopback { backing_file: Option<String> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDevice {
    pub path: String,
    pub size: u64,
    pub sectors: u64,
    pub kind: BlockDeviceKind,
    pub model: Option<String>,
}

impl From<&disks::BlockDevice> for BlockDevice {
    fn from(val: &disks::BlockDevice) -> Self {
        match val {
            disks::BlockDevice::Disk(disk) => BlockDevice {
                path: disk.device_path().to_string_lossy().to_string(),
                size: disk.size(),
                sectors: disk.sectors(),
                kind: BlockDeviceKind::Disk,
                model: disk.model().map(String::from),
            },
            disks::BlockDevice::Loopback(loopback) => BlockDevice {
                path: loopback.device_path().to_string_lossy().to_string(),
                size: loopback.disk().map_or(0, |d| d.size()),
                sectors: loopback.disk().map_or(0, |d| d.sectors()),
                kind: BlockDeviceKind::Loopback {
                    backing_file: loopback
                        .file_path()
                        .map(|p| p.to_string_lossy().to_string()),
                },
                model: None,
            },
        }
    }
}

/// Encapsulation of client-initiated requests
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    GetBlockDevices,
    Shutdown,
}

/// Encapsulation of server-initiated responses
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    BlockDevices(Vec<BlockDevice>),
    Error(String),
}
