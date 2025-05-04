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

///Bounds is a pointer to the collection if the initial type is in System.Collections.Generic (only List?)
/// While Vector is a first item of the collection if the initial type is object[]
/// How it looks like in case of smth like int[] is a question for now
/// NativeArray should be split into NativeArray<> and NativeCollection<> to check this cases properly
/// mb the third type will be needed for uint[] alikes?
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct NativeArray<T> {
    pub obj: NativeObject,
    //there are 4 more pointers at the address below, resembling members of the initial C# Collection<>
    //type-object pointer (presumably the type of list items), 2 zeros (who knows what they stand for)
    // and an actual max_length of the collection
    //so should bounds be used in one kind of collection and vector is for the other stuff?
    pub bounds: *const std::ffi::c_void, // 0x10
    pub length: u32, // 0x14
    // This is the first item of some pointer
    pub(crate) vector: *const T, // 0x1c
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