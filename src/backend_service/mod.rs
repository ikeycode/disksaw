// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! This is the backend service, which will be used to perform the actual disk operations
//! in a privileged context.

use disks::BlockDevice;
use privileged_ipc::IpcServer;

use crate::api::{Request, Response};

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
            Request::Shutdown => {
                client.shutdown(std::net::Shutdown::Both)?;
                break;
            }
        }
    }
    Ok(())
}
