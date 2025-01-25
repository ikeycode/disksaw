// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! This is the backend service, which will be used to perform the actual disk operations
//! in a privileged context.

pub fn run() -> color_eyre::Result<()> {
    privileged_ipc::service_init()?;
    Ok(())
}
