use axfs_vfs::{VfsNodeAttr, VfsNodeAttrX, VfsNodeOps, VfsNodePerm, VfsNodeType, VfsResult};

/// A zero device behaves like `/dev/zero`.
///
/// It always returns a chunk of `\0` bytes when read, and all writes are discarded.
pub struct ZeroDev;

impl VfsNodeOps for ZeroDev {
    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        Ok(VfsNodeAttr::new(
            0,
            VfsNodePerm::default_file(),
            VfsNodeType::CharDevice,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ))
    }

    fn get_attr_x(&self) -> VfsResult<VfsNodeAttrX> {
        Ok(VfsNodeAttrX::new(
            0,0,0,0,0,0,
            VfsNodePerm::default_file(),
            VfsNodeType::CharDevice,
            0,0,
            0,0,
            0,0,0,0,
            0,0,0, 0,
            0,0,0,0,
        ))
    }
    fn read_at(&self, _offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        buf.fill(0);
        Ok(buf.len())
    }

    fn write_at(&self, _offset: u64, buf: &[u8]) -> VfsResult<usize> {
        Ok(buf.len())
    }

    fn truncate(&self, _size: u64) -> VfsResult {
        Ok(())
    }

    axfs_vfs::impl_vfs_non_dir_default! {}
}
