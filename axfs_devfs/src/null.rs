use axfs_vfs::{VfsError, VfsNodeAttr, VfsNodeAttrX, VfsNodeOps, VfsNodePerm, VfsNodeRef, VfsNodeType, VfsResult};
/// A null device behaves like `/dev/null`.
///
/// Nothing can be read and all writes are discarded.
pub struct NullDev;

impl VfsNodeOps for NullDev {
    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        Ok(VfsNodeAttr::new(
            1,
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

    fn read_at(&self, _offset: u64, _buf: &mut [u8]) -> VfsResult<usize> {
        Ok(0)
    }

    fn write_at(&self, _offset: u64, buf: &[u8]) -> VfsResult<usize> {
        Ok(buf.len())
    }

    fn truncate(&self, _size: u64) -> VfsResult {
        Ok(())
    }

    axfs_vfs::impl_vfs_non_dir_default! {}
}
