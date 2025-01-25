// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionMenu {
    Create,
    Delete,
    Resize,
    Format,
    Mount,
    Unmount,
    List,
    Quit,
}

pub fn enums_to_cliclack<T>(enums: &[T]) -> Vec<(&T, String, &'static str)>
where
    T: ToString,
{
    enums.iter().map(|x| (x, x.to_string(), "")).collect()
}

impl Display for PartitionMenu {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PartitionMenu::Create => write!(f, "Create Partition"),
            PartitionMenu::Delete => write!(f, "Delete Partition"),
            PartitionMenu::Resize => write!(f, "Resize Partition"),
            PartitionMenu::Format => write!(f, "Format Partition"),
            PartitionMenu::Mount => write!(f, "Mount Partition"),
            PartitionMenu::Unmount => write!(f, "Unmount Partition"),
            PartitionMenu::List => write!(f, "List Partitions"),
            PartitionMenu::Quit => write!(f, "Quit"),
        }
    }
}
