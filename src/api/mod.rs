// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

pub mod client;

/// Encapsulation of client-initiated requests
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    GetBlockDevices,
    Shutdown,
}

/// Encapsulation of server-initiated responses
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    BlockDevices(Vec<String>),
    Error(String),
}
