use crate::ffi::CString;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
pub enum FSSubFunction {
    ReadFile = 0,
    ReadDir = 1,
    RewriteFile = 2,
    WriteFile = 3,
    SetFileSize = 4,
    Stat = 5,
    SetAttributes = 6,
    LaunchProgram = 7,
    DeleteFileDir = 8,
    CreateDirectory = 9
}

#[repr(C)]
pub struct FSOffsetWithFlags {
    offset: u32,
    flags: u32
}

#[repr(C)]
pub union FSOffset {
    offset: u64,
    offset_with_flags: core::mem::ManuallyDrop<FSOffsetWithFlags>
}

#[repr(C, packed(1))]
pub struct FSDataBlock<'buf> {
    subfunction: FSSubFunction,
    offset: FSOffset,

    phantom_buffer: core::marker::PhantomData<&'buf ()>,
    buffer_size: u32,
    buffer_addr: u32,
    zero: u8,
    filepath: *const core::ffi::c_char
}

const _: &[()] = &[
    assert!(core::mem::offset_of!(FSDataBlock<'_>, subfunction) == 0),
    assert!(core::mem::offset_of!(FSDataBlock<'_>, offset) == 4),
    assert!(core::mem::offset_of!(FSDataBlock<'_>, buffer_size) == 12),
    assert!(core::mem::offset_of!(FSDataBlock<'_>, buffer_addr) == 16),
    assert!(core::mem::offset_of!(FSDataBlock<'_>, zero) == 20),
    assert!(core::mem::offset_of!(FSDataBlock<'_>, filepath) == 21),
];

impl Drop for FSDataBlock<'_> {
    fn drop(&mut self) {
        drop(unsafe { CString::from_raw(self.filepath as _) });
    }
}

enum FSDataBlockBuilderBuffer<'buf> {
    Immutable(&'buf [u8]),
    Mutable(&'buf mut [u8]),
}

impl FSDataBlockBuilderBuffer<'_> {
    pub fn len(&self) -> usize {
        match self {
            Self::Immutable(buf) => buf.len(),
            Self::Mutable(buf) => buf.len(),
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        match self {
            Self::Immutable(buf) => buf.as_ptr(),
            Self::Mutable(buf) => buf.as_ptr(),
        }
    }
}

#[derive(Default)]
pub struct FSDataBlockBuilder<'buf> {
    subfunction: Option<FSSubFunction>,
    offset: Option<FSOffset>,
    buffer: Option<FSDataBlockBuilderBuffer<'buf>>,
    filepath: Option<CString>
}

impl<'buf> FSDataBlockBuilder<'buf> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(mut self) -> Self {
        self.subfunction = Some(FSSubFunction::ReadFile);

        self
    }

    pub fn write(mut self) -> Self {
        self.subfunction = Some(FSSubFunction::WriteFile);

        self
    }

    pub fn create_file(mut self) -> Self {
        self.subfunction = Some(FSSubFunction::RewriteFile);

        self
    }

    pub fn set_file_size(mut self) -> Self {
        self.subfunction = Some(FSSubFunction::SetFileSize);

        self
    }

    pub fn create_directory(mut self) -> Self {
        self.subfunction = Some(FSSubFunction::CreateDirectory);

        self
    }

    pub fn stat(mut self) -> Self {
        self.subfunction = Some(FSSubFunction::Stat);

        self
    }

    pub fn path(mut self, path: CString) -> Self {
        self.filepath = Some(path);

        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(FSOffset { offset: offset.to_be() });

        self
    }

    pub fn offset_flags(mut self, offset: u32, flags: u32) -> Self {
        self.offset = Some(FSOffset {
            offset_with_flags: core::mem::ManuallyDrop::new(FSOffsetWithFlags {
                offset,
                flags
            })
        });

        self
    }

    pub fn buffer(mut self, buf: &'buf [u8]) -> Self {
        self.buffer = Some(FSDataBlockBuilderBuffer::Immutable(buf));
        
        self
    }

    pub fn buffer_mut(mut self, buf: &'buf mut [u8]) -> Self {
        self.buffer = Some(FSDataBlockBuilderBuffer::Mutable(buf));
        
        self
    }

    // TODO: Return Result<_, BlockBuilderError>
    pub fn build(self) -> Option<FSDataBlock<'buf>> {
        let mut new_path = self.filepath.and_then(|x| x.into_string().ok()).unwrap();

        // Mark this as UTF-8 path.
        new_path.insert_str(0, "\x03");

        let buffer_length = if self.subfunction? == FSSubFunction::Stat {
            0
        } else {
            self.buffer.as_ref()?.len()
        };
        
        Some(FSDataBlock {
            subfunction: self.subfunction?,
            offset: self.offset?,

            phantom_buffer: core::marker::PhantomData,
            buffer_size: buffer_length as _,
            buffer_addr: self.buffer?.as_ptr().addr() as _,
            zero: 0,
            filepath: CString::new(new_path).unwrap().into_raw()
        })
    }
}

#[repr(C, packed(1))]
pub struct DirectoryEntryInfoTime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    _reserved: u8,
}

impl DirectoryEntryInfoTime {
    pub fn to_system_time(&self) -> super::time::Time {
        super::time::Time {
            second: self.seconds,
            minute: self.minutes,
            hour: self.hours,
        }
    }
}

#[repr(C, packed(1))]
pub struct DirectoryEntryInfoDate {
    pub day: u8,
    pub month: u8,
    pub year: u16
}

impl DirectoryEntryInfoDate {
    pub fn to_system_date(&self) -> super::time::Date {
        super::time::Date {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

#[repr(C, packed(1))]
pub struct DirectoryEntryInfo {
    // TODO: Use `bitflags` crate here?
    attributes: u32,
    encoding: u8,
    
    _rsv: [u8; 3],

    creation_time: DirectoryEntryInfoTime,
    creation_date: DirectoryEntryInfoDate,

    access_time: DirectoryEntryInfoTime,
    access_date: DirectoryEntryInfoDate,

    modify_time: DirectoryEntryInfoTime,
    modify_date: DirectoryEntryInfoDate,

    filesize: u64,

    // Here comes the nul-terminated name.
    // As said in `https://wiki.kolibrios.org/wiki/SysFn70/ru#%D0%9F%D0%BE%D0%B4%D1%84%D1%83%D0%BD%D0%BA%D1%86%D0%B8%D1%8F_5_-_%D0%BF%D0%BE%D0%BB%D1%83%D1%87%D0%B5%D0%BD%D0%B8%D0%B5_%D0%B8%D0%BD%D1%84%D0%BE%D1%80%D0%BC%D0%B0%D1%86%D0%B8%D0%B8_%D0%BE_%D1%84%D0%B0%D0%B9%D0%BB%D0%B5/%D0%BF%D0%B0%D0%BF%D0%BA%D0%B5.`
    // ... this structre is dynamically-sized (with CP866 name = 304 bytes, 560 bytes otherwise.)
    // This is not cool, so set this field to max possible size.

    name_raw: [core::ffi::c_char; 520],  // 560 - sizeof previous fields
}

impl Clone for DirectoryEntryInfo {
    fn clone(&self) -> Self {
        unsafe {
            let mut empty: Self = core::mem::zeroed();

            core::ptr::copy_nonoverlapping(self as *const Self, &mut empty as &mut Self, core::mem::size_of::<Self>());

            empty
        }
    }
}

impl DirectoryEntryInfo {
    #[inline]
    pub fn is_readonly(&self) -> bool {
        (self.attributes & 1) != 0
    }

    #[inline]
    pub fn is_hidden(&self) -> bool {
        (self.attributes & 2) != 0
    }

    #[inline]
    pub fn is_system(&self) -> bool {
        (self.attributes & 4) != 0
    }

    #[inline]
    pub fn is_volumelabel(&self) -> bool {
        (self.attributes & 8) != 0
    }

    #[inline]
    pub fn is_folder(&self) -> bool {
        (self.attributes & 0x10) != 0
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.filesize
    }

    #[inline]
    pub fn created(&self) -> super::time::DateTime {
        super::time::DateTime { date: self.creation_date.to_system_date(), time: self.creation_time.to_system_time() }
    }
    
    #[inline]
    pub fn accessed(&self) -> super::time::DateTime {
        super::time::DateTime { date: self.access_date.to_system_date(), time: self.access_time.to_system_time() }
    }

    #[inline]
    pub fn modified(&self) -> super::time::DateTime {
        super::time::DateTime { date: self.modify_date.to_system_date(), time: self.modify_time.to_system_time() }
    }
}

impl DirectoryEntryInfo {
    pub fn structure_size(&self) -> usize {
        // 0 = default
        // 1 = cp866
        // 2 = UTF-16LE
        // 3 = UTF-8

        if self.encoding == 1 {
            304
        } else {
            560
        }
    }
}

pub fn fs_request(block: FSDataBlock<'_>) -> (usize, usize) {
    unsafe {
        super::syscall::syscall2_all(70, (&block as *const FSDataBlock<'_>).addr())
    }
}