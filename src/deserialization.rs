use serde::{Deserialize, Deserializer};
use crate::kreide::native_types::{NativeArray, NativeString};
use crate::kreide::types::rpg::client::{AvatarData, AvatarServantData};
use crate::kreide::types::rpg::gamecore::{CharacterConfig, CharacterDataComponent, GameEntity, SkillCharacterComponent, SkillData, TurnBasedAbilityComponent};

// impl<'de, T> Deserialize<'de> for NativeArray<T> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         use serde::de::{self, MapAccess, Visitor};
//         use std::fmt;
// 
//         struct NativeArrayVisitor<T> {
//             marker: std::marker::PhantomData<T>,
//         }
// 
//         impl<'de, T> Visitor<'de> for NativeArrayVisitor<T> {
//             type Value = NativeArray<T>;
// 
//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("struct NativeArray")
//             }
// 
//             fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
//             where
//                 V: MapAccess<'de>,
//             {
//                 let mut obj = None;
//                 let mut bounds = None;
//                 let mut max_length = None;
//                 let mut vector = None;
// 
//                 while let Some(key) = map.next_key()? {
//                     match key {
//                         "obj" => {
//                             if obj.is_some() {
//                                 return Err(de::Error::duplicate_field("obj"));
//                             }
//                             obj = Some(map.next_value()?);
//                         }
//                         "bounds" => {
//                             if bounds.is_some() {
//                                 return Err(de::Error::duplicate_field("bounds"));
//                             }
//                             bounds = Some(map.next_value::<u64>()? as *const std::ffi::c_void);
//                         }
//                         "max_length" => {
//                             if max_length.is_some() {
//                                 return Err(de::Error::duplicate_field("max_length"));
//                             }
//                             max_length = Some(map.next_value()?);
//                         }
//                         "vector" => {
//                             if vector.is_some() {
//                                 return Err(de::Error::duplicate_field("vector"));
//                             }
//                             vector = Some(map.next_value::<u64>()? as *const T);
//                         }
//                         _ => {
//                             return Err(de::Error::unknown_field(key, &["obj", "bounds", "max_length", "vector"]));
//                         }
//                     }
//                 }
// 
//                 let obj = obj.ok_or_else(|| de::Error::missing_field("obj"))?;
//                 let bounds = bounds.ok_or_else(|| de::Error::missing_field("bounds"))?;
//                 let max_length = max_length.ok_or_else(|| de::Error::missing_field("max_length"))?;
//                 let vector = vector.ok_or_else(|| de::Error::missing_field("vector"))?;
// 
//                 Ok(NativeArray {
//                     obj,
//                     bounds,
//                     length: max_length,
//                     vector,
//                 })
//             }
//         }
// 
//         const FIELDS: &'static [&'static str] = &["obj", "bounds", "max_length", "vector"];
//         deserializer.deserialize_struct("NativeArray", FIELDS, NativeArrayVisitor { marker: std::marker::PhantomData })
//     }
// }

// /// Deserialize a memory address (u64) back into a raw pointer to `NativeArray<T>`.

// pub fn deserialize_native_array_pointer<'de, D, T>(deserializer: D) -> Result<*const NativeArray<T>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let address = u64::deserialize(deserializer)?; // Deserialize the address as `u64`
//     Ok(address as *const NativeArray<T>) // Convert back to a raw pointer
// }
//
// /// Function to deserialize a memory address (u64) into a raw pointer (`*const GameEntity`).
// pub fn deserialize_game_entity_pointer<'de, D>(deserializer: D) -> Result<*const GameEntity, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     // Deserialize the memory address and cast it back to a raw pointer
//     let address = u64::deserialize(deserializer)?;
//     Ok(address as *const GameEntity)
// }
//
// /// Deserialize a `u64` address back into a raw pointer to `*const CharacterConfig`.
// pub fn deserialize_character_config_pointer<'de, D>(deserializer: D) -> Result<*const CharacterConfig, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let address = u64::deserialize(deserializer)?; // Deserialize the pointer address as `u64`
//     Ok(address as *const CharacterConfig) // Convert the address back into a raw pointer
// }
//
// /// Deserialize a `u64` address back into a raw pointer to `*const CharacterDataComponent`.
// pub fn deserialize_character_data_pointer<'de, D>(deserializer: D) -> Result<*const CharacterDataComponent, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let address = u64::deserialize(deserializer)?; // Deserialize the address as `u64`
//     Ok(address as *const CharacterDataComponent) // Convert the address back into a raw pointer
// }
//
// /// Deserialize a memory address (`u64`) back into a raw pointer of type `*const TurnBasedAbilityComponent`.
// pub fn deserialize_turnbased_ability_component_pointer<'de, D>(deserializer: D) -> Result<*const TurnBasedAbilityComponent, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let address = u64::deserialize(deserializer)?; // Deserialize the address as `u64`
//     Ok(address as *const TurnBasedAbilityComponent) // Convert the address back into a raw pointer
// }
//
// /// Deserialize a numeric memory address (`u64`) back into a raw pointer of type `*const SkillCharacterComponent`.
// pub fn deserialize_skill_character_component_pointer<'de, D>(deserializer: D) -> Result<*const SkillCharacterComponent, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let address = u64::deserialize(deserializer)?; // Deserialize the address (as `u64`).
//     Ok(address as *const SkillCharacterComponent) // Cast it back into a raw pointer.
// }
//
// /// Deserialize a numeric address (`u64`) back into a raw pointer `*const AvatarData`.
// pub fn deserialize_avatar_data_pointer<'de, D>(deserializer: D) -> Result<*const AvatarData, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let address = u64::deserialize(deserializer)?; // Deserialize the address from `u64`.
//     Ok(address as *const AvatarData) // Convert the address back into a raw pointer.
// }
//
// /// Deserialize a numeric memory address (`u64`) back into a raw pointer `*const AvatarServantData`.
// pub fn deserialize_avatar_servant_data_pointer<'de, D>(deserializer: D) -> Result<*const AvatarServantData, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let address = u64::deserialize(deserializer)?; // Deserialize the memory address from `u64`
//     Ok(address as *const AvatarServantData) // Convert the address back into a raw pointer
// }
//
// /// Deserialize a numeric address (`u64`) back into a raw pointer `*const SkillData`.
// pub fn deserialize_skill_data_pointer<'de, D>(deserializer: D) -> Result<*const SkillData, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let address = u64::deserialize(deserializer)?; // Deserialize the address (as `u64`).
//     Ok(address as *const SkillData) // Convert the address back into a raw pointer.
// }

// pub fn deserialize_pointer<'de, D>(deserializer: D) -> Result<*const std::ffi::c_void, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let addr = u64::deserialize(deserializer)?;
//     Ok(addr as *const std::ffi::c_void)
// }

// pub fn deserialize_native_string<'de, D>(deserializer: D) -> Result<*const NativeString, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let addr = u64::deserialize(deserializer)?;
//     Ok(addr as *const NativeString)
// }