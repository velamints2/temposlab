use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use ostd::{
    mm::{FallibleVmRead, FallibleVmWrite, Frame, FrameAllocOptions, PAGE_SIZE, VmIo, VmReader, VmWriter},
    sync::{Mutex, RwMutex},
};

use crate::error::{Errno, Error, Result};
use crate::fs::{Inode, InodeMeta, InodeType};

pub struct RamInode {
    inner: Inner,
    metadata: InodeMeta,
}

struct RamFile {
    data: Vec<Frame<()>>,
    size: usize,
}

enum Inner {
    File(Mutex<RamFile>),
    Directory(RwMutex<BTreeMap<String, Arc<RamInode>>>),
}

impl RamInode {
    fn new_file() -> Arc<Self> {
        Arc::new(RamInode {
            inner: Inner::File(Mutex::new(RamFile {
                data: Vec::new(),
                size: 0,
            })),
            metadata: InodeMeta {
                size: 0,
                atime: core::time::Duration::new(0, 0),
                mtime: core::time::Duration::new(0, 0),
                ctime: core::time::Duration::new(0, 0),
            },
        })
    }

    fn new_directory() -> Arc<Self> {
        Arc::new(RamInode {
            inner: Inner::Directory(RwMutex::new(BTreeMap::new())),
            metadata: InodeMeta {
                size: 0,
                atime: core::time::Duration::new(0, 0),
                mtime: core::time::Duration::new(0, 0),
                ctime: core::time::Duration::new(0, 0),
            },
        })
    }
}

impl Inode for RamInode {
    fn read_at(&self, offset: usize, mut writer: ostd::mm::VmWriter) -> Result<usize> {
        let Inner::File(file) = &self.inner else {
            return Err(Error::new(Errno::EISDIR));
        };

        let file = file.lock();
        if offset >= file.size {
            return Ok(0);
        }

        let mut current_offset = offset;
        let mut bytes_read = 0;
        let total_to_read = core::cmp::min(file.size - offset, writer.avail());

        while bytes_read < total_to_read {
            let page_idx = current_offset / PAGE_SIZE;
            let page_offset = current_offset % PAGE_SIZE;
            let frame = &file.data[page_idx];
            
            let remaining_in_page = PAGE_SIZE - page_offset;
            let to_read = core::cmp::min(remaining_in_page, total_to_read - bytes_read);

            frame.read(page_offset, &mut writer.split_at(to_read)).unwrap();

            bytes_read += to_read;
            current_offset += to_read;
        }

        Ok(bytes_read)
    }

    fn write_at(&self, offset: usize, mut reader: ostd::mm::VmReader) -> Result<usize> {
        let Inner::File(file) = &self.inner else {
            return Err(Error::new(Errno::EISDIR));
        };

        let mut file = file.lock();
        let write_end = offset + reader.remain();
        
        // Ensure enough frames are allocated
        let needed_frames = (write_end + PAGE_SIZE - 1) / PAGE_SIZE;
        while file.data.len() < needed_frames {
            let frame = FrameAllocOptions::new()
                .alloc_frame()
                .map_err(|_| Error::new(Errno::ENOMEM))?;
            file.data.push(frame);
        }

        let mut current_offset = offset;
        let mut bytes_written = 0;
        let total_to_write = reader.remain();

        while bytes_written < total_to_write {
            let page_idx = current_offset / PAGE_SIZE;
            let page_offset = current_offset % PAGE_SIZE;
            let frame = &mut file.data[page_idx];

            let remaining_in_page = PAGE_SIZE - page_offset;
            let to_write = core::cmp::min(remaining_in_page, total_to_write - bytes_written);

            frame.write(page_offset, &mut reader.split_at(to_write)).unwrap();

            bytes_written += to_write;
            current_offset += to_write;
        }

        if current_offset > file.size {
            file.size = current_offset;
        }

        Ok(bytes_written)
    }

    fn size(&self) -> usize {
        match &self.inner {
            Inner::File(file) => file.lock().size,
            Inner::Directory(_) => 12,
        }
    }

    fn metadata(&self) -> &InodeMeta {
        &self.metadata
    }

    fn lookup(&self, name: &str) -> Result<Arc<dyn Inode>> {
        let Inner::Directory(ref entries) = self.inner else {
            return Err(Error::new(Errno::ENOTDIR));
        };

        let entries = entries.read();
        let inode = entries.get(name).ok_or(Error::new(Errno::ENOENT))?;

        Ok(inode.clone())
    }

    fn create(&self, name: &str, type_: InodeType) -> Result<Arc<dyn Inode>> {
        let Inner::Directory(ref entries) = self.inner else {
            return Err(Error::new(Errno::ENOTDIR));
        };

        let inode = match type_ {
            InodeType::File => RamInode::new_file(),
            InodeType::Directory => RamInode::new_directory(),
            InodeType::SymbolLink => todo!(),
        };

        entries.write().insert(name.to_string(), inode.clone());

        Ok(inode)
    }

    fn read_link(&self) -> Result<String> {
        todo!()
    }

    fn write_link(&self, _target: &str) -> Result<()> {
        todo!()
    }

    fn typ(&self) -> InodeType {
        match &self.inner {
            Inner::Directory(_) => InodeType::Directory,
            Inner::File(_) => InodeType::File,
        }
    }
}

pub struct RamFS {
    root: Arc<RamInode>,
}

impl RamFS {
    pub fn new() -> Self {
        RamFS {
            root: RamInode::new_directory(),
        }
    }
}

impl crate::fs::FileSystem for RamFS {
    fn name(&self) -> &str {
        "ramfs"
    }

    fn root_inode(&self) -> Arc<dyn Inode> {
        self.root.clone()
    }
}
