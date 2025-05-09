//! Device filesystem used by [ArceOS](https://github.com/arceos-org/arceos).
//!
//! The implementation is based on [`axfs_vfs`].

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::collections::BTreeMap;
use core::num::NonZeroU64;
use spin::RwLock;

mod dir;
mod null;
mod zero;

#[cfg(test)]
mod tests;

pub use self::dir::DirNode;
pub use self::null::NullDev;
pub use self::zero::ZeroDev;

use alloc::sync::Arc;
use axfs_vfs::{VfsNodeRef, VfsOps, VfsResult};
use spin::once::Once;

fn make_dev(major: u32, minor: u32) -> u64 {
    ((major as u64) << 32) | (minor as u64)
}

/// A device filesystem that implements [`axfs_vfs::VfsOps`].
pub struct DeviceFileSystem {
    parent: Once<VfsNodeRef>,
    root: Arc<DirNode>,
    dev_map: RwLock<BTreeMap<u64, VfsNodeRef>>,
}

impl DeviceFileSystem {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            parent: Once::new(),
            root: DirNode::new(None),
            dev_map: RwLock::new(BTreeMap::new()),
        }
    }

    /// Create a subdirectory at the root directory.
    pub fn mkdir(&self, name: &'static str) -> Arc<DirNode> {
        self.root.mkdir(name)
    }

    /// Add a node to the root directory.
    ///
    /// The node must implement [`axfs_vfs::VfsNodeOps`], and be wrapped in [`Arc`].
    pub fn add(&self, name: &'static str, node: VfsNodeRef) {
        self.root.add(name, node);
    }

    pub fn register_device(&self, major: u32, minor: u32, node: VfsNodeRef) {
        let dev_id = make_dev(major, minor);
        self.dev_map.write().insert(dev_id, node);
    }

    pub fn get_device(&self, major: u32, minor: u32) -> Option<VfsNodeRef> {
        let dev_id = make_dev(major, minor);
        self.dev_map.read().get(&dev_id).cloned()
    }
}

impl VfsOps for DeviceFileSystem {
    fn mount(&self, _path: &str, mount_point: VfsNodeRef) -> VfsResult {
        if let Some(parent) = mount_point.parent() {
            self.root.set_parent(Some(self.parent.call_once(|| parent)));
        } else {
            self.root.set_parent(None);
        }
        Ok(())
    }

    fn root_dir(&self) -> VfsNodeRef {
        self.root.clone()
    }
}

impl Default for DeviceFileSystem {
    fn default() -> Self {
        Self::new()
    }
}
