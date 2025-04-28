use std::{fmt::Display, slice, string::FromUtf16Error};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NativeObject {
    #[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
    pub klass: *const std::ffi::c_void,
    #[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
    pub monitor: *const std::ffi::c_void, // *const MonitorData
}

impl Default for NativeObject {
    fn default() -> Self {
        Self {
            klass: std::ptr::null(),
            monitor: std::ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
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
    pub max_length: u32,
    // This is the first item of some pointer
    vector: *const T,
}

impl<T> Default for NativeArray<T> {
    fn default() -> Self {
        Self {
            obj: NativeObject::default(),
            bounds: std::ptr::null(),
            max_length: 0,
            vector: std::ptr::null(),
        }
    }
}

impl<T> Serialize for NativeArray<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("NativeArray", 4)?;
        state.serialize_field("obj", &self.obj)?;
        state.serialize_field("bounds", &(self.bounds as u64))?;
        state.serialize_field("max_length", &self.max_length)?;
        state.serialize_field("vector", &(self.vector as u64))?;
        state.end()
    }
}

impl<'de, T> Deserialize<'de> for NativeArray<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct NativeArrayVisitor<T> {
            marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for NativeArrayVisitor<T> {
            type Value = NativeArray<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct NativeArray")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut obj = None;
                let mut bounds = None;
                let mut max_length = None;
                let mut vector = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "obj" => {
                            if obj.is_some() {
                                return Err(de::Error::duplicate_field("obj"));
                            }
                            obj = Some(map.next_value()?);
                        }
                        "bounds" => {
                            if bounds.is_some() {
                                return Err(de::Error::duplicate_field("bounds"));
                            }
                            bounds = Some(map.next_value::<u64>()? as *const std::ffi::c_void);
                        }
                        "max_length" => {
                            if max_length.is_some() {
                                return Err(de::Error::duplicate_field("max_length"));
                            }
                            max_length = Some(map.next_value()?);
                        }
                        "vector" => {
                            if vector.is_some() {
                                return Err(de::Error::duplicate_field("vector"));
                            }
                            vector = Some(map.next_value::<u64>()? as *const T);
                        }
                        _ => {
                            return Err(de::Error::unknown_field(key, &["obj", "bounds", "max_length", "vector"]));
                        }
                    }
                }

                let obj = obj.ok_or_else(|| de::Error::missing_field("obj"))?;
                let bounds = bounds.ok_or_else(|| de::Error::missing_field("bounds"))?;
                let max_length = max_length.ok_or_else(|| de::Error::missing_field("max_length"))?;
                let vector = vector.ok_or_else(|| de::Error::missing_field("vector"))?;

                Ok(NativeArray {
                    obj,
                    bounds,
                    max_length,
                    vector,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["obj", "bounds", "max_length", "vector"];
        deserializer.deserialize_struct("NativeArray", FIELDS, NativeArrayVisitor { marker: std::marker::PhantomData })
    }
}

impl<T> NativeArray<T> {
    pub fn to_slice(&self) -> &[*const T] {
        unsafe {
            let ptr = &self.vector;
            slice::from_raw_parts(ptr, self.max_length as usize)
        }
    }
}

pub fn serialize_pointer<S>(ptr: &*const std::ffi::c_void, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u64(*ptr as u64)
}

pub fn deserialize_pointer<'de, D>(deserializer: D) -> Result<*const std::ffi::c_void, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let addr = u64::deserialize(deserializer)?;
    Ok(addr as *const std::ffi::c_void)
}

pub fn serialize_native_string<S>(ptr: &*const NativeString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u64(*ptr as u64)
}

pub fn deserialize_native_string<'de, D>(deserializer: D) -> Result<*const NativeString, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let addr = u64::deserialize(deserializer)?;
    Ok(addr as *const NativeString)
}

/// Serialize a raw pointer to `NativeArray<T>` as its memory address (u64).
pub fn serialize_native_array_pointer<S, T>(ptr: &*const NativeArray<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let address = *ptr as u64; // Convert the pointer to an address
    serializer.serialize_u64(address) // Serialize the address as `u64`
}

/// Deserialize a memory address (u64) back into a raw pointer to `NativeArray<T>`.

pub fn deserialize_native_array_pointer<'de, D, T>(deserializer: D) -> Result<*const NativeArray<T>, D::Error>
where
    D: Deserializer<'de>,
{
    let address = u64::deserialize(deserializer)?; // Deserialize the address as `u64`
    Ok(address as *const NativeArray<T>) // Convert back to a raw pointer
}