// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::ops::{Deref, DerefMut};

use color_eyre::eyre::bail;
use privileged_ipc::{DirectExecutor, IpcClient, PkexecExecutor};

use super::{BlockDevice, Request, Response};

pub struct Client(pub IpcClient<Request, Response>);

impl Deref for Client {
    type Target = IpcClient<Request, Response>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Client {
    /// Create a new client that will directly execute the backend service.
    pub fn new_direct_with_path(executable: &str) -> color_eyre::Result<Self> {
        let client = IpcClient::new::<DirectExecutor>(executable, &["--backend-service"])?;
        Ok(Self(client))
    }

    /// Create a new client that will use pkexec to execute the backend service.
    pub fn new_privileged_with_path(executable: &str) -> color_eyre::Result<Self> {
        let client = IpcClient::new::<PkexecExecutor>(executable, &["--backend-service"])?;
        Ok(Self(client))
    }

    /// Get a list of block devices.
    pub fn get_block_devices(&mut self) -> color_eyre::Result<Vec<BlockDevice>> {
        self.send(&Request::GetBlockDevices)?;
        if let Some(response) = self.incoming()?.next() {
            let response = response?;
            match response {
                Response::BlockDevices(devices) => Ok(devices),
                Response::Error(e) => bail!(e),
            }
        } else {
            bail!("No response from backend service")
        }
    }

    /// Shutdown the backend service gracefully.
    pub fn shutdown_backend(mut self) -> color_eyre::Result<()> {
        self.send(&Request::Shutdown)?;
        self.0.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
}
