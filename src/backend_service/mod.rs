// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! This is the backend service, which will be used to perform the actual disk operations
//! in a privileged context.

use std::fs;

use disks::BlockDevice;
use privileged_ipc::IpcServer;

use crate::api::{Request, Response};

fn get_superblock(path: &str) -> color_eyre::Result<superblock::Superblock> {
    let mut reader = fs::File::open(path)?;
    let sb = superblock::Superblock::from_reader(&mut reader)?;
    Ok(sb)
}

pub fn run() -> color_eyre::Result<()> {
    privileged_ipc::service_init()?;

    let server: IpcServer<Response, Request> = IpcServer::new()?;
    let mut client = server.accept()?;
    let incoming = client.incoming()?;

    for request in incoming {
        match request? {
            Request::GetBlockDevices => {
                let devices = BlockDevice::discover().unwrap_or_default();
                let mapped = devices.iter().map(Into::into).collect();
                client.send(&Response::BlockDevices(mapped))?;
            }
            Request::GetSuperblock(device) => {
                let superblock = get_superblock(&device);
                if let Ok(sb) = superblock {
                    client.send(&Response::Superblock(sb.into()))?;
                } else {
                    client.send(&Response::Error("Failed to get superblock".to_string()))?;
                }
            }
            Request::Shutdown => {
                client.shutdown(std::net::Shutdown::Both)?;
                break;
            }
        }
    }
    Ok(())
}
