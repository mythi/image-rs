// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[cfg(feature = "snapshot-unionfs")]
pub mod occlum;
#[cfg(feature = "snapshot-overlayfs")]
pub mod overlay;

/// Snapshot types.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotType {
    Unknown,
    #[cfg(feature = "snapshot-overlayfs")]
    Overlay,
    #[cfg(feature = "snapshot-unionfs")]
    OcclumUnionfs,
}

impl Default for SnapshotType {
    fn default() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "snapshot-overlayfs")] {
                Self::Overlay
            } else {
                Self::Unknown
            }
        }
    }
}

impl std::fmt::Display for SnapshotType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Unknown => "unknown",
            #[cfg(feature = "snapshot-overlayfs")]
            Self::Overlay => "overlay",
            #[cfg(feature = "snapshot-unionfs")]
            Self::OcclumUnionfs => "occlum_unionfs",
        };

        write!(f, "{out}")
    }
}

/// A MountPoint contains the info to represents a mount point.
#[derive(Clone, Debug, Deserialize)]
pub struct MountPoint {
    /// The filesystem type of mount point.
    pub r#type: String,

    /// The mount destination path.
    pub mount_path: PathBuf,

    /// The work dir generated by snapshot.
    pub work_dir: PathBuf,
}

/// Trait to mount/umount image snapshots.
pub trait Snapshotter: Send + Sync {
    // mount the OCI image layers to destination mount path.
    fn mount(&mut self, layer_path: &[&str], mount_path: &Path) -> Result<MountPoint>;

    // unmount the mount_point and cleanup snapshot work dir.
    fn unmount(&self, mount_point: &MountPoint) -> Result<()>;
}
