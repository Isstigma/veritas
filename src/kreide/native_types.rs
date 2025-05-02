use std::{fmt::Display, slice, string::FromUtf16Error};

impl Default for NativeObject {
    fn default() -> Self {
        Self {
            klass: std::ptr::null(),
            monitor: std::ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NativeString {
    pub obj: NativeObject,
    pub m_stringLength: u32,
    pub m_firstChar: u16,
}

impl NativeString {
    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        unsafe {
            let ptr = &self.m_firstChar;
            let array = std::slice::from_raw_parts(ptr, self.m_stringLength as usize);
            String::from_utf16(&array)
        }
    }
}

impl Display for NativeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_string() {
            Ok(string) => write!(f, "{}", string),
            Err(e) => write!(f, "{}", e),
        }
    }
}

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct NativeArray<T> {
    pub obj: NativeObject,
    pub bounds: *const std::ffi::c_void,
    pub length: u32,
    // This is the first item of some pointer
    //there are 4 more pointers at the address below, resembling members of the initial C# Collection<>
    //type-object pointer (presumably the type of list items), 2 zeros (who knows what they stand for)
    // and an actual max_length of the collection
    pub(crate) vector: *const T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeObject {
    //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
    pub klass: *const std::ffi::c_void,
    //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
    pub monitor: *const std::ffi::c_void, // *const MonitorData
}

impl<T> Default for NativeArray<T> {
    fn default() -> Self {
        Self {
            obj: NativeObject::default(),
            bounds: std::ptr::null(),
            length: 0,
            vector: std::ptr::null(),
        }
    }
}

impl<T> NativeArray<T> {
    pub fn to_slice(&self) -> &[*const T] {
        unsafe {
            let ptr = &self.vector;
            slice::from_raw_parts(ptr, self.length as usize)
        }
    }
}