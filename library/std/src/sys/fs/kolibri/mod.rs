use crate::ffi::{CString, OsString};
use crate::fmt;
use crate::fs::TryLockError;
use crate::hash::{Hash, Hasher};
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, SeekFrom};
use crate::path::{Path, PathBuf};
pub use crate::sys::fs::common::Dir;
use crate::sys::time::SystemTime;
use crate::sys::unsupported;

use core::fmt::Write;

pub struct File {
    path: CString,
    position: core::cell::Cell<u64>,
    options: OpenOptions,
}

pub struct FileAttr(crate::sys::pal::fs::NamelessDirectoryEntryInfo);

pub struct ReadDir(!);

pub struct DirEntry(!);

#[derive(Clone, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    create: bool
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {}

pub struct FilePermissions {
    is_readonly: bool
}

pub struct FileType {
    is_folder: bool,
}

#[derive(Debug)]
pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.0.size()
    }

    pub fn perm(&self) -> FilePermissions {
        FilePermissions { is_readonly: self.0.is_readonly() }
    }

    pub fn file_type(&self) -> FileType {
        FileType { is_folder: self.0.is_folder() }
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        let unix = crate::sys::time::UNIX_EPOCH;

        let dt = self.0.modified();

        let unix_duration = crate::time::Duration::from_secs(dt.to_unix());

        Ok(unix.checked_add_duration(&unix_duration).unwrap())
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        let unix = crate::sys::time::UNIX_EPOCH;

        let dt = self.0.accessed();

        let unix_duration = crate::time::Duration::from_secs(dt.to_unix());

        Ok(unix.checked_add_duration(&unix_duration).unwrap())
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        let unix = crate::sys::time::UNIX_EPOCH;

        let dt = self.0.created();

        let unix_duration = crate::time::Duration::from_secs(dt.to_unix());

        Ok(unix.checked_add_duration(&unix_duration).unwrap())
    }
}

impl Clone for FileAttr {
    fn clone(&self) -> FileAttr {
        FileAttr(self.0.clone())
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.is_readonly
    }

    pub fn set_readonly(&mut self, readonly: bool) {
        // ???
        self.is_readonly = readonly;
    }
}

impl Clone for FilePermissions {
    fn clone(&self) -> FilePermissions {
        Self {
            is_readonly: self.is_readonly
        }
    }
}

impl PartialEq for FilePermissions {
    fn eq(&self, other: &FilePermissions) -> bool {
        self.is_readonly == other.is_readonly
    }
}

impl Eq for FilePermissions {}

impl fmt::Debug for FilePermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilePermissions").field("is_readonly", &self.is_readonly).finish()
    }
}

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.is_folder
    }

    pub fn is_file(&self) -> bool {
        !self.is_folder
    }

    pub fn is_symlink(&self) -> bool {
        false
    }
}

impl Clone for FileType {
    fn clone(&self) -> FileType {
        Self {
            is_folder: self.is_folder
        }
    }
}

impl Copy for FileType {}

impl PartialEq for FileType {
    fn eq(&self, other: &FileType) -> bool {
        self.is_folder == other.is_folder
    }
}

impl Eq for FileType {}

impl Hash for FileType {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.is_folder.hash(h)
    }
}

impl fmt::Debug for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileType").field("type", if self.is_folder {
            &"directory"
        } else {
            &"file"
        }).finish()
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        self.0
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.0
    }

    pub fn file_name(&self) -> OsString {
        self.0
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        self.0
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        self.0
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            create: false
        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;
    }
    pub fn write(&mut self, write: bool) {
        self.write = write;
    }
    pub fn append(&mut self, _append: bool) {}
    pub fn truncate(&mut self, _truncate: bool) {}
    pub fn create(&mut self, create: bool) {
        self.create = create;
    }
    pub fn create_new(&mut self, _create_new: bool) {}
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        // If file doesn't exist and user doesn't want it be created, bail out.
        if !exists(path)? && !opts.create {
            return Err(io::ErrorKind::NotFound.into());
        }

        // If file doesn't exist and its stated to be created, do it.
        if !exists(path)? && opts.create {
            let blk = crate::sys::pal::fs::FSDataBlockBuilder::new()
                .stat()
                .path_from_path(path)
                .offset(0)
                .buffer(&[])
                .build()
                .unwrap();

            let (status, _) = crate::sys::pal::fs::fs_request(blk);

            match status {
                0 => (),
                1.. => return Err(io::Error::from_raw_os_error(status as _))
            };
        }
        
        Ok(File {
            // TODO: Handle `.unwrap()`
            path: crate::ffi::CString::new(path.to_str().unwrap()).unwrap(),
            position: core::cell::Cell::new(0),
            options: opts.clone()
        })
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        let mut info: crate::sys::pal::fs::NamelessDirectoryEntryInfo = unsafe { core::mem::zeroed() };

        let info_ptr: *mut u8 = core::ptr::addr_of_mut!(info).cast();

        let info_buf = unsafe { core::slice::from_raw_parts_mut(info_ptr, core::mem::size_of_val(&info)) };

        let blk = crate::sys::pal::fs::FSDataBlockBuilder::new()
            .stat()
            .path(self.path.clone())
            .offset(0)
            .buffer_mut(info_buf)
            .build()
            .unwrap();

        let (status, _) = crate::sys::pal::fs::fs_request(blk);

        if status == 0 {
            Ok(FileAttr(info))
        } else {
            Err(io::Error::from_raw_os_error(status as _))
        }
    }

    pub fn fsync(&self) -> io::Result<()> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: fsync").unwrap();

        unsupported()
    }

    pub fn datasync(&self) -> io::Result<()> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: datasync").unwrap();

        unsupported()
    }

    pub fn lock(&self) -> io::Result<()> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: lock").unwrap();

        unsupported()
    }

    pub fn lock_shared(&self) -> io::Result<()> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: lock_shared").unwrap();
        
        unsupported()
    }

    pub fn try_lock(&self) -> Result<(), TryLockError> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: try_lock").unwrap();
        
        todo!()
    }

    pub fn try_lock_shared(&self) -> Result<(), TryLockError> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: try_lock_shared").unwrap();
        
        todo!()
    }

    pub fn unlock(&self) -> io::Result<()> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: unlock").unwrap();

        unsupported()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: truncate").unwrap();

        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let blk = crate::sys::pal::fs::FSDataBlockBuilder::new()
            .read()
            .path(self.path.clone())
            .offset(self.position.get() as _)
            .buffer_mut(buf)
            .build()
            .unwrap();

        let (status, read_bytes) = crate::sys::pal::fs::fs_request(blk);

        if status == 6 || status == 0 {
            self.position.set(self.position.get() + read_bytes as u64);

            return Ok(read_bytes);
        } else {
            return Err(io::Error::from_raw_os_error(status as _));
        }
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: read_vectored").unwrap();
        
        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_, u8>) -> io::Result<()> {
        unsafe { 
            let ptr = cursor.as_mut().as_mut_ptr().cast::<u8>();
            let len = cursor.capacity();

            if len == 0 {
                return Ok(());
            }

            let slice = core::slice::from_raw_parts_mut(ptr, len);

            let result = self.read(slice);

            match result {
                Ok(value) => cursor.advance(value),
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(()),
                Err(e) => return Err(e)
            };
        }

        Ok(())
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        if !self.options.write {
            return Err(io::ErrorKind::PermissionDenied.into());
        }

        let blk = crate::sys::pal::fs::FSDataBlockBuilder::new()
            .create_file()  // or rewrite.
            .path(self.path.clone())
            .offset(self.position.get() as _)
            .buffer(buf)
            .build()
            .unwrap();

        let (status, written_bytes) = crate::sys::pal::fs::fs_request(blk);

        // TOOD: Refine logic.
        // Errors can happen if data is actually being written.
        // For example EOF is being thrown if buffer size is 32 and file size is actually 10.
        // In this case, 10 bytes are being written, BUT also the EOF (6) will be thrown.
        if status == 0 {
            self.position.set(self.position.get() + written_bytes as u64);

            return Ok(written_bytes);
        } else {
            return Err(io::Error::from_raw_os_error(status as _));
        }

    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: write_vectored").unwrap();

        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let mut current_pos = self.position.get();

        match pos {
            SeekFrom::Start(st) => {
                current_pos = st;
            },
            SeekFrom::End(_en) => {
                todo!("End position in `seek`!")
            },
            SeekFrom::Current(cu) => {
                current_pos = current_pos.saturating_add_signed(cu);
            },
        }

        self.position.set(current_pos);

        Ok(current_pos)
    }

    pub fn size(&self) -> Option<io::Result<u64>> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: size").unwrap();

        None
    }

    pub fn tell(&self) -> io::Result<u64> {
        Ok(self.position.get())
    }

    pub fn duplicate(&self) -> io::Result<File> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: duplicate").unwrap();

        unsupported()
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: set_permissions").unwrap();

        unsupported()
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        writeln!(crate::sys::pal::api::debugboard(), "unimplemented: set_times").unwrap();

        unsupported()
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File").field("path", &self.path).finish()
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::readdir").unwrap();

    unsupported()
}

pub fn unlink(_p: &Path) -> io::Result<()> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::unlink").unwrap();

    unsupported()
}

pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::rename").unwrap();

    unsupported()
}

pub fn set_perm(_p: &Path, _perm: FilePermissions) -> io::Result<()> {
    // match perm.0 {}
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::set_perm").unwrap();

    unsupported()
}

pub fn set_times(_p: &Path, _times: FileTimes) -> io::Result<()> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::set_times").unwrap();

    unsupported()
}

pub fn set_times_nofollow(_p: &Path, _times: FileTimes) -> io::Result<()> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::set_times_nofollow").unwrap();

    unsupported()
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::rmdir").unwrap();

    unsupported()
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::remove_dir_all").unwrap();

    unsupported()
}

pub fn exists(path: &Path) -> io::Result<bool> {
    let mut info: crate::sys::pal::fs::NamelessDirectoryEntryInfo = unsafe { core::mem::zeroed() };

    let info_ptr: *mut u8 = core::ptr::addr_of_mut!(info).cast();

    let info_buf = unsafe { core::slice::from_raw_parts_mut(info_ptr, core::mem::size_of_val(&info)) };

    let blk = crate::sys::pal::fs::FSDataBlockBuilder::new()
        .stat()
        .path_from_path(path)
        .offset(0)
        .buffer_mut(info_buf)
        .build()
        .unwrap();

    let (status, _) = crate::sys::pal::fs::fs_request(blk);

    match status {
        5 => Ok(false),
        0 => Ok(true),
        _ => Err(io::Error::from_raw_os_error(status as _))
    }
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::readlink").unwrap();

    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::symlink").unwrap();

    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::link").unwrap();

    unsupported()
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    // writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::stat").unwrap();

    // unsupported()

    File::open(p, &OpenOptions::new())?.file_attr()
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::lstat").unwrap();

    unsupported()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::canonicalize").unwrap();

    unsupported()
}

pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    writeln!(crate::sys::pal::api::debugboard(), "unimplemented: ::copy").unwrap();

    unsupported()
}
