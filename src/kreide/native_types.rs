use std::{fmt::Display, slice, string::FromUtf16Error};
use std::ffi::c_void;

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
    //so should bounds be used in one kind of collection and vector is for array?
    pub bounds: *const std::ffi::c_void, // 0x10
    pub length: u32, // 0x14
    // This is the first item of some pointer
    pub(crate) vector: *const T, // 0x1c
}

///Array of value types, has slightly different layout????
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct NativeValueArray<T> {
    pub obj: *const c_void,
    pub zero1: u64,
    pub zero2: u64,
    pub length: u64, // 0x18
    // This is the first item of some pointer
    pub vector: T, // 0x20
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
    pub fn to_slice(&self) -> &[*const T] { //works for underlying [] and not List<>
        unsafe {
            let ptr = &self.vector;
            slice::from_raw_parts(ptr, self.length as usize)
        }
    }
}

impl<T> NativeValueArray<T> {
    pub fn to_slice(&self) -> &[T] { //works for underlying val[]
        unsafe {
            let ptr = &self.vector;
            slice::from_raw_parts(ptr, self.length as usize)
        }
    }
}

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct NativeDictionary<K, V> {
    pub obj: NativeObject,
    pub buckets: *const NativeArray<i32>, // 0x10 int[] buckets, I don't need that
    pub entries: *const NativeArray<NativeDictionaryEntry<K, V>>, // 0x18 Dictionary.Entry<TKey,TValue>[]
    pub count: i32, // 0x1c
    // pub version: i32,
    // pub free_list: i32,
    // pub free_count: i32,
    // pub comparer: *const c_void,
    // pub keys: *const c_void,
    // pub values: *const c_void,
    // pub key_comparer: *const c_void,
    // pub value_comparer: *const c_void,
    // pub syncroot: *const c_void,
    // pub syncroot_is_shared: bool,
    // pub is_readonly: bool,
    // pub is_thread_safe: bool,
    // pub is_fixed_size: bool,
}

//automatic memory alignment of structures sucks
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct NativeDictionaryEntry<K, V> {
    //pub obj: NativeObject,
    pub hash_code: i32, // Lower 31 bits of hash code, -1 if unused
    pub next: i32, // index of next entry, -1 if last
    pub key: *const K,
    pub value: *const V,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeDictionaryValueEntry<K, V> {
    //pub obj: NativeObject,
    pub hash_code: i32, // Lower 31 bits of hash code, -1 if unused
    pub next: i32, // index of next entry, -1 if last
    pub key: K,
    pub value: V,
}