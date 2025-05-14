use crate::kreide::helpers::{fixpoint_to_raw, get_avatar_data_from_id, round_to_places};
use crate::kreide::native_types::{NativeArray, NativeDictionary, NativeDictionaryEntry, NativeDictionaryValueEntry, NativeObject, NativeString, NativeValueArray};
use crate::kreide::types::rpg::client::*;
use crate::kreide::types::rpg::gamecore::*;
use crate::kreide::types::{MMNDIEBMDNL, NOPBAAAGGLA, OLHMAHMMBNN};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::backtrace::Backtrace;
use std::{ffi, slice};
use std::ffi::c_void;
use uuid::Uuid;
use crate::kreide::functions::rpg::gamecore::SkillCharacterComponent_GetAllAllowSkillIdxList;
/*
/// Function to serialize a raw pointer (`*const GameEntity`) as its memory address.
// pub fn serialize_game_entity_pointer<S>(ptr: &*const GameEntity, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     // Convert the raw pointer to a `u64` representing its memory address
//     if *ptr as u64 != 0
//     {
//         unsafe {
//             let deref = **ptr;
//
//             serialize_game_entity(deref, serializer)
//
//             // let address = *ptr as u64;
//             // serializer.serialize_u64(address)
//         }
//     }
//     else {
//         serializer.serialize_u64(0)
//     }
// }

/// Serialize a raw popub(crate)pub(crate)inter to `*const CharacterConfig` as its memory address (`u64`).
// pub fn serialize_character_config_pointer<S>(ptr: &*const CharacterConfig, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     if *ptr as u64 != 0
//     {
//         unsafe {
//             let deref = **ptr;
//
//             serialize_character_config(deref, serializer)
//
//             // let address = *ptr as u64;
//             // serializer.serialize_u64(address)
//         }
//     }
//     else {
//         serializer.serialize_u64(0)
//     }
// }

/// Serialize a raw pointer to `*const CharacterDataComponent` as its memory address (`u64`).
// pub fn serialize_character_data_pointer<S>(ptr: &*const CharacterDataComponent, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     if *ptr as u64 != 0
//     {
//         unsafe {
//             let deref = **ptr;
//
//             serialize_character_data_component(deref, serializer)
//
//             // let address = *ptr as u64;
//             // serializer.serialize_u64(address)
//         }
//     }
//     else {
//         serializer.serialize_u64(0)
//     }
// }

/// Serialize a raw pointer to `*const TurnBasedAbilityComponent` as its memory address (`u64`).
// pub fn serialize_turnbased_ability_component_pointer<S>(ptr: &*const TurnBasedAbilityComponent, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     if *ptr as u64 != 0
//     {
//         unsafe {
//             let deref = **ptr;
//
//             serialize_turn_based_ability_component(deref, serializer)
//
//             // let address = *ptr as u64;
//             // serializer.serialize_u64(address)
//         }
//     }
//     else {
//         serializer.serialize_u64(0)
//     }
// }
//
/// Serialize a raw pointer to `*const SkillCharacterComponent` as its memory address (`u64`).
// pub fn serialize_skill_character_component_pointer<S>(ptr: &*const SkillCharacterComponent, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     if *ptr as u64 != 0
//     {
//         unsafe {
//             let deref = **ptr;
//
//             serialize_skill_character_component(deref, serializer)
//
//             // let address = *ptr as u64;
//             // serializer.serialize_u64(address)
//         }
//     }
//     else {
//         serializer.serialize_u64(0)
//     }
// }
//
/// Serialize a raw pointer to `*const AvatarServantData` as its memory address (`u64`).
// pub fn serialize_avatar_servant_data_pointer<S>(ptr: &*const AvatarServantData, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     if *ptr as u64 != 0
//     {
//         unsafe {
//             let deref = **ptr;
//
//             serialize_avatar_servant_data(deref, serializer)
//
//             // let address = *ptr as u64;
//             // serializer.serialize_u64(address)
//         }
//     }
//     else {
//         serializer.serialize_u64(0)
//     }
// }

/// Serialize a raw pointer to `*const AvatarData` as its memory address (`u64`).
// pub fn serialize_avatar_data_pointer<S>(ptr: &*const AvatarData, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     if *ptr as u64 != 0
//     {
//         unsafe {
//             let deref = **ptr;
//
//             serialize_avatar_data(deref, serializer)
//
//             // let address = *ptr as u64;
//             // serializer.serialize_u64(address)
//         }
//     }
//     else {
//         serializer.serialize_u64(0)
//     }
// }

/// Serialize a raw pointer to `*const SkillData` as its memory address (`u64`).
// pub fn serialize_skill_data_pointer<S>(ptr: &*const SkillData, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     if *ptr as u64 != 0
//     {
//         unsafe {
//             let deref = **ptr;
//
//             serialize_skill_data(deref, serializer)
//
//             // let address = *ptr as u64;
//             // serializer.serialize_u64(address)
//         }
//     }
//     else {
//         serializer.serialize_u64(0)
//     }
// }

*/

impl Serialize for GameEntity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::GameEntity");
        let mut state = serializer.serialize_struct("GameEntity", 45)?; // Number of fields explicitly serialized

        // Serialize each field explicitly
        unsafe{
            // Serialize pointers using a custom helper method
            state.serialize_field("HoyoTagContainer", &serialize_pointer(&self.HoyoTagContainer))?;
            state.serialize_field("_LateUpdateComponentList", &serialize_pointer(&self._LateUpdateComponentList))?;
            state.serialize_field("_ComponentList", &serialize_pointer(&self._ComponentList))?;
            state.serialize_field("_OwnerWorldRef", &serialize_pointer(&self._OwnerWorldRef))?;


        if self.Name__BackingField.is_null() { state.serialize_field("Name__BackingField", "null")?; }
        else {state.serialize_field("Name__BackingField", &*self.Name__BackingField)?; }

            state.serialize_field("_UnityGO", &serialize_pointer(&self._UnityGO))?;
            state.serialize_field("TagComponentContainer", &serialize_pointer(&self.TagComponentContainer))?;


        if self.TickLodTemplate.is_null() { state.serialize_field("TickLodTemplate", "null")?; }
        else {state.serialize_field("TickLodTemplate", &*self.TickLodTemplate)?;
        }

        if self._UnstageReasonKey.is_null() { state.serialize_field("_UnstageReasonKey", "null")?; }
        else {state.serialize_field("_UnstageReasonKey", &*self._UnstageReasonKey)?;}

            state.serialize_field("_DestroyWaitList", &serialize_pointer(&self._DestroyWaitList))?;
            state.serialize_field("DisposeCallback", &serialize_pointer(&self.DisposeCallback))?;
            state.serialize_field("WorldTimeScaleAdpator", &serialize_pointer(&self.WorldTimeScaleAdpator))?;
            state.serialize_field("_ComponentArrayRef", &serialize_pointer(&self._ComponentArrayRef))?;
            state.serialize_field("_TickLodProxy", &serialize_pointer(&self._TickLodProxy))?;



    if self._ComponentArray.is_null() { state.serialize_field("_ComponentArray", "null")?; }
    else {state.serialize_field("_ComponentArray", &*self._ComponentArray)?; }

            state.serialize_field("_TickComponentList", &serialize_pointer(&self._TickComponentList))?;
            state.serialize_field("OnStageStateChange", &serialize_pointer(&self.OnStageStateChange))?;
            state.serialize_field("TimeScaleStack", &serialize_pointer(&self.TimeScaleStack))?;



    if self.NameForGameCore__BackingField.is_null() { state.serialize_field("NameForGameCore__BackingField", "null")?; }
    else {state.serialize_field("NameForGameCore__BackingField", &*self.NameForGameCore__BackingField)?; }

            state.serialize_field("OnTeamChange", &serialize_pointer(&self.OnTeamChange))?;

            // Serialize primitive types directly
            state.serialize_field("Visible__BackingField", &self.Visible__BackingField)?;
            state.serialize_field("IsStoryMode__BackingField", &self.IsStoryMode__BackingField)?;
            state.serialize_field("HasDisposed", &self.HasDisposed)?;
            state.serialize_field("IsFakeAvatar__BackingField", &self.IsFakeAvatar__BackingField)?;
            state.serialize_field("_TickDelayFrameCount", &self._TickDelayFrameCount)?;
            state.serialize_field("LastTickTime__BackingField", &self.LastTickTime__BackingField)?;
            state.serialize_field("CampID__BackingField", &self.CampID__BackingField)?;
            state.serialize_field("_ShouldLateUpdate", &self._ShouldLateUpdate)?;
            state.serialize_field("Disposing", &self.Disposing)?;
            state.serialize_field("_Tickable", &self._Tickable)?;
            state.serialize_field("_IsRegisterEnviroChara", &self._IsRegisterEnviroChara)?;
            state.serialize_field("_AliveState", &self._AliveState)?;
            state.serialize_field("IsLoaded__BackingField", &self.IsLoaded__BackingField)?;
            state.serialize_field("IsHero__BackingField", &self.IsHero__BackingField)?;
            state.serialize_field("KillImmediately", &self.KillImmediately)?;
            state.serialize_field("_IsOnStage", &self._IsOnStage)?;
            state.serialize_field("_Server&entityID", &self._ServerEntityID)?;
            state.serialize_field("_GroupID", &self._GroupID)?;
            state.serialize_field("_Group&entityID", &self._GroupEntityID)?;
            state.serialize_field("LastTickBucket__BackingField", &self.LastTickBucket__BackingField)?;
            state.serialize_field("LastTickFrame__BackingField", &self.LastTickFrame__BackingField)?;
            state.serialize_field("_&entityType", &self._EntityType)?;
            state.serialize_field("RuntimeID__BackingField", &self.RuntimeID__BackingField)?;
            state.serialize_field("_ForceTickLodLowestReason", &serialize_pointer(&self._ForceTickLodLowestReason))?;
            state.serialize_field("TickLodBoundSize__BackingField", &self.TickLodBoundSize__BackingField)?;
            state.serialize_field("ObjectFeature__BackingField", &self.ObjectFeature__BackingField)?;
            state.serialize_field("ForceIgnoreTickLodBistSet", &self.ForceIgnoreTickLodBistSet)?;
            state.serialize_field("_Team", &self._Team)?;
            state.serialize_field("native_object", &self.native_object)?;
        }
        // Complete the structure serialization
        state.end()
    }
}

impl Serialize for SkillData{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        //log::info!("serialize::SkillData");

        let mut state = serializer.serialize_struct("SkillData", 24)?;

        // Serialize native_object
        state.serialize_field("native_object", &self.native_object)?;

        // Serialize pointers using provided serialize_* functions
        unsafe {

            state.serialize_field("PreshowConditions", &serialize_pointer(&self.PreshowConditions))?;

            state.serialize_field("OverrideTargetInfo", &serialize_pointer(&self.OverrideTargetInfo))?;
            state.serialize_field("RowData", &serialize_pointer(&self.RowData))?;
            state.serialize_field("DefaultTargetInfo", &serialize_pointer(&self.DefaultTargetInfo))?;
            state.serialize_field("VisibleCondTask", &serialize_pointer(&self.VisibleCondTask))?;



            if self.AllChildSkillDatas.is_null() { state.serialize_field("AllChildSkillDatas", "null")?; }
            else {state.serialize_field("AllChildSkillDatas", &*self.AllChildSkillDatas)?;

            }

            if self.Config.is_null() { state.serialize_field("Config", "null")?; }
            else {state.serialize_field("Config", &serialize_pointer(&self.Config))?;
            }

            if self.OverrideAnimState.is_null() { state.serialize_field("OverrideAnimState", "null")?; }
            else {state.serialize_field("OverrideAnimState", &*self.OverrideAnimState)?;

            }

            if self.SkillCom.is_null() { state.serialize_field("SkillCom", "null")?; }
            else {state.serialize_field("SkillCom", &*self.SkillCom)?;
            }

            state.serialize_field("CustomReadyConfigConditions", &serialize_pointer(&self.CustomReadyConfigConditions))?;

            state.serialize_field("_Slot", &serialize_pointer(&self._Slot))?;
            state.serialize_field("UsableCondTask", &serialize_pointer(&self.UsableCondTask))?;
            state.serialize_field("InsertCondTask", &serialize_pointer(&self.InsertCondTask))?;
            state.serialize_field("OverrideCameraConfig", &serialize_pointer(&self.OverrideCameraConfig))?;
            state.serialize_field("OverrideCameraConfigAdded", &serialize_pointer(&self.OverrideCameraConfigAdded))?;

            if self.ParentSkillData.is_null() { state.serialize_field("ParentSkillData", "null")?; }
            else {state.serialize_field("ParentSkillData", &*self.ParentSkillData)?;
            }

            state.serialize_field("_SkillProperties", &serialize_pointer(&self._SkillProperties))?;

            if self.SkillTriggerKey.is_null() { state.serialize_field("SkillTriggerKey", "null")?; }
            else {state.serialize_field("SkillTriggerKey", &*self.SkillTriggerKey)?; }

            // Serialize primitive fields
            state.serialize_field("CommonActiveSkillID", &self.CommonActiveSkillID)?;
            state.serialize_field("LeftCastTimes", &self.LeftCastTimes)?;
            state.serialize_field("AttackDamageTypePreshowAttach", &self.AttackDamageTypePreshowAttach)?;
            state.serialize_field("ChildIndex", &self.ChildIndex)?;
            state.serialize_field("MaxCastTimes", &self.MaxCastTimes)?;
            state.serialize_field("CurrentCoolDown", &self.CurrentCoolDown)?;
            state.serialize_field("DefaultCoolDown", &self.DefaultCoolDown)?;
            state.serialize_field("SkillConfigID", &self.SkillConfigID)?;
            state.serialize_field("SkillIndex", &self.SkillIndex)?;
        }
        //log::info!("serialize::SkillData end");
        state.end()
    }
}
impl Serialize for CharacterConfig{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::CharacterConfig");

        // Count the fields to be serialized
        // Note: You'd count only fields that are actually serialized using serde attributes or manual implementation.
        let mut state = serializer.serialize_struct("CharacterConfig", 63)?;
        unsafe {
        // Serialize fields (native Rust types or custom)
        state.serialize_field("_parent_object", &self._parent_object)?;
        state.serialize_field("SomatoType", &self.SomatoType)?;
        state.serialize_field("CharacterBodySize", &self.CharacterBodySize)?;
        state.serialize_field("CharacterHUDOffset", &self.CharacterHUDOffset)?;
        state.serialize_field("BuffPanelOffset", &self.BuffPanelOffset)?;
        state.serialize_field("HitBoxOffset", &self.HitBoxOffset)?;
        state.serialize_field("TargetSelectGroup", &self.TargetSelectGroup)?;

        // Serialize fields using custom serialization methods

        state.serialize_field("CameraConfigList", &serialize_pointer(&self.CameraConfigList))?;

        state.serialize_field("HitBoxType", &self.HitBoxType)?;
        state.serialize_field("HitBoxWidth", &self.HitBoxWidth)?;
        state.serialize_field("HitBoxLength", &self.HitBoxLength)?;
        state.serialize_field("HitBoxHeight", &self.HitBoxHeight)?;

        if self.HitBoxAttachPoint.is_null() { state.serialize_field("HitBoxAttachPoint", "null")?; }
        else {state.serialize_field("HitBoxAttachPoint", &*self.HitBoxAttachPoint)?; }

        state.serialize_field("Resilience", &serialize_pointer(&self.Resilience))?;
        state.serialize_field("Location", &serialize_pointer(&self.Location))?;
        state.serialize_field("VisualRadius", &self.VisualRadius)?;
        state.serialize_field("LookAtIKEnableRadius", &self.LookAtIKEnableRadius)?;
        state.serialize_field("AutoFlipModel", &self.AutoFlipModel)?;
        state.serialize_field("SaveModelWhenDead", &self.SaveModelWhenDead)?;
        state.serialize_field("DeadPerform", &self.DeadPerform)?;
        state.serialize_field("PreloadUltraSkill", &self.PreloadUltraSkill)?;
        state.serialize_field("IsSpecialVisualCharacter", &self.IsSpecialVisualCharacter)?;
        state.serialize_field("HideInTimeline", &self.HideInTimeline)?;

        if self.AnimEventConfigList.is_null() { state.serialize_field("AnimEventConfigList", "null")?; }
        else {state.serialize_field("AnimEventConfigList", &*self.AnimEventConfigList)?;
        }

        state.serialize_field("SkillList", &serialize_pointer(&self.SkillList))?;

        if self.AbilityList.is_null() { state.serialize_field("AbilityList", "null")?; }
        else {state.serialize_field("AbilityList", &*self.AbilityList)?;
        }

        state.serialize_field("SkillAbilityList", &serialize_pointer(&self.SkillAbilityList))?;

        state.serialize_field("DynamicValues", &serialize_pointer(&self.DynamicValues))?;
        state.serialize_field("CustomValues", &serialize_pointer(&self.CustomValues))?;
        state.serialize_field("WeaponType", &self.WeaponType)?;
        state.serialize_field("ArmorType", &self.ArmorType)?;

        state.serialize_field("SkillReadyTransits", &serialize_pointer(&self.SkillReadyTransits))?;

        state.serialize_field("PhaseAnimConfig", &serialize_pointer(&self.PhaseAnimConfig))?;

        if self.AnimZoneConfigPath.is_null() { state.serialize_field("AnimZoneConfigPath", "null")?; }
        else {state.serialize_field("AnimZoneConfigPath", &*self.AnimZoneConfigPath)?;
        }

        if self.InitAnimStateName.is_null() { state.serialize_field("InitAnimStateName", "null")?; }
        else {state.serialize_field("InitAnimStateName", &*self.InitAnimStateName)?;
        }

            if self.WhitelistSkillStateForInterrupt.is_null() { state.serialize_field("WhitelistSkillStateForInterrupt", "null")?; }
            else {   state.serialize_field("WhitelistSkillStateForInterrupt", &*self.WhitelistSkillStateForInterrupt)?; }

            state.serialize_field("ModifierPerformTimeFactor", &self.ModifierPerformTimeFactor)?;
            state.serialize_field("AsAidAttackTask", &serialize_pointer(&self.AsAidAttackTask))?;
            state.serialize_field("AsAidDefenderTask", &serialize_pointer(&self.AsAidDefenderTask))?;
            state.serialize_field("AsAidProtectorTask", &serialize_pointer(&self.AsAidProtectorTask))?;



        if self.DisableAnimEventLayers.is_null() { state.serialize_field("DisableAnimEventLayers", "null")?; }
        else {state.serialize_field("DisableAnimEventLayers", &*self.DisableAnimEventLayers)?; }

            state.serialize_field("OnHitEditFootIKModeMap", &serialize_pointer(&self.OnHitEditFootIKModeMap))?;
            state.serialize_field("RepeatOccurAnimWhenBeHitNormalizedTime", &self.RepeatOccurAnimWhenBeHitNormalizedTime)?;


        if self.CameraNamedDynamicOffset.is_null() { state.serialize_field("CameraNamedDynamicOffset", "null")?; }
        else {state.serialize_field("CameraNamedDynamicOffset", &*self.CameraNamedDynamicOffset)?;  }

        state.serialize_field("IgnoreDynamicOffsetBySelf", &self.IgnoreDynamicOffsetBySelf)?;
        state.serialize_field("OverrideHeightForCameraOffset", &self.OverrideHeightForCameraOffset)?;
        state.serialize_field("MonsterIgnoreGlobalDymanicOffset", &self.MonsterIgnoreGlobalDymanicOffset)?;
        state.serialize_field("MaxMonsterPhase", &self.MaxMonsterPhase)?;

        state.serialize_field("PhaseList", &serialize_pointer(&self.PhaseList))?;

        if self.OverrideWaveMonsterPerform.is_null() { state.serialize_field("OverrideWaveMonsterPerform", "null")?; }
        else {state.serialize_field("OverrideWaveMonsterPerform", &*self.OverrideWaveMonsterPerform)?;
        }

        if self.OverrideColliderCameraByName.is_null() { state.serialize_field("OverrideColliderCameraByName", "null")?; }
        else {state.serialize_field("OverrideColliderCameraByName", &*self.OverrideColliderCameraByName)?;
        }

        state.serialize_field("EntityColliderConfig", &serialize_pointer(&self.EntityColliderConfig))?;

        state.serialize_field("EffectAdaptionList", &serialize_pointer(&self.EffectAdaptionList))?;

        if self.AttachPointEffectAdaptionList.is_null() { state.serialize_field("AttachPointEffectAdaptionList", "null")?; }
        else {state.serialize_field("AttachPointEffectAdaptionList", &serialize_pointer(&self.AttachPointEffectAdaptionList))?;
        }

        state.serialize_field("FieldEffectAdaptionList", &serialize_pointer(&self.FieldEffectAdaptionList))?;
        state.serialize_field("EffectAttachPointRedirect", &serialize_pointer(&self.EffectAttachPointRedirect))?;
        state.serialize_field("MonsterConfig", &serialize_pointer(&self.MonsterConfig))?;


        if self.ResidentEffectKey.is_null() { state.serialize_field("ResidentEffectKey", "null")?; }
        else {state.serialize_field("ResidentEffectKey", &*self.ResidentEffectKey)?;
        }

        if self.ResidentPossessionKey.is_null() { state.serialize_field("ResidentPossessionKey", "null")?; }
        else {    state.serialize_field("ResidentPossessionKey", &*self.ResidentPossessionKey)?;
        }

        if self.EmotionCharacterID.is_null() { state.serialize_field("EmotionCharacterID", "null")?; }
        else {state.serialize_field("EmotionCharacterID", &*self.EmotionCharacterID)?;
        }

        if self.GraphEmotionAsset.is_null() { state.serialize_field("GraphEmotionAsset", "null")?; }
        else {state.serialize_field("GraphEmotionAsset", &*self.GraphEmotionAsset)?; }

        state.serialize_field("AITagList", &serialize_pointer(&self.AITagList))?;
        state.serialize_field("GlobalAIFactorGroups", &serialize_pointer(&self.GlobalAIFactorGroups))?;
        state.serialize_field("ReplaceEmoConfig", &serialize_pointer(&self.ReplaceEmoConfig))?;
        state.serialize_field("WillUnstage", &self.WillUnstage)?;
        state.serialize_field("ViewModeSortPriority", &self.ViewModeSortPriority)?;

        if self.ReplaceAnimtorControllerPath.is_null() { state.serialize_field("ReplaceAnimtorControllerPath", "null")?; }
        else {state.serialize_field("ReplaceAnimtorControllerPath", &*self.ReplaceAnimtorControllerPath)?; }

            state.serialize_field("AlwaysCutOnSkillTargetTeamChange", &self.AlwaysCutOnSkillTargetTeamChange)?;
        }
        // End serialization
        state.end()
    }
}

impl Serialize for CharacterDataComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>

    where
        S: Serializer,
    {
        //log::info!("serialize::CharacterDataComponent");

        // Initialize a serializer for the struct
        let mut state = serializer.serialize_struct("CharacterDataComponent", 24)?;
        unsafe {
            // Serialize the `_parent_object` field directly
            state.serialize_field("_parent_object", &self._parent_object)?;

            // Serialize fields that require custom serialization

            if self.JsonConfig__BackingField.is_null() { state.serialize_field("JsonConfig__BackingField", "null")?; }
            else {   state.serialize_field("JsonConfig__BackingField", &*self.JsonConfig__BackingField)?;
            }
            state.serialize_field("_CharacterUICustomValueDict", &serialize_pointer(&self._CharacterUICustomValueDict))?;
        

            if self.Summoner.is_null() { state.serialize_field("Summoner", "null")?; }
            else {state.serialize_field("Summoner", &*self.Summoner)?;
            }

            state.serialize_field("_DummpyEntityList", &serialize_pointer(&self._DummpyEntityList))?;

            state.serialize_field("_RowData", &serialize_pointer(&self._RowData))?;
        
            state.serialize_field("_DynamicScaleAdaptConfigs", &serialize_pointer(&self._DynamicScaleAdaptConfigs))?;

            state.serialize_field("_DynamicScaleAdaptEffectPathRule", &serialize_pointer(&self._DynamicScaleAdaptEffectPathRule))?;
        
            state.serialize_field("_DynamicScaleAdaptTypes", &serialize_pointer(&self._DynamicScaleAdaptTypes))?;

            state.serialize_field("HideDisplayInfoSkillNames", &serialize_pointer(&self.HideDisplayInfoSkillNames))?;

            // Serialize simple fields directly
            state.serialize_field("LastActTurnCount__BackingField", &self.LastActTurnCount__BackingField)?;
            state.serialize_field("GridFightTag__BackingField", &self.GridFightTag__BackingField)?;
            state.serialize_field("EnhancedState", &self.EnhancedState)?;
            state.serialize_field("SpawnTurnCount", &self.SpawnTurnCount)?;
            state.serialize_field("CreateReason", &self.CreateReason)?;
            state.serialize_field("DisableHeadLookAtActionEntityOverride", &self.DisableHeadLookAtActionEntityOverride)?;
            state.serialize_field("IsBodyPart", &self.IsBodyPart)?;
            state.serialize_field("IsVisibleInViewMode__BackingField", &self.IsVisibleInViewMode__BackingField)?;
            state.serialize_field("_SaveModelWhenDeadOverride", &self._SaveModelWhenDeadOverride)?;
            state.serialize_field("DisableRootYawMapping__BackingField", &self.DisableRootYawMapping__BackingField)?;
            state.serialize_field("TriggerLimbo", &self.TriggerLimbo)?;
            state.serialize_field("LocalOffsetAsMoveTarget__BackingField", &self.LocalOffsetAsMoveTarget__BackingField)?;
            state.serialize_field("CharacterID__BackingField", &self.CharacterID__BackingField)?;
            state.serialize_field("LineupIndex", &self.LineupIndex)?;
        }
        // Finalize serialization
        state.end()
    }
}

impl Serialize for TurnBasedAbilityComponent{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::TurnBasedAbilityComponent");

        // Initialize the serializer for this struct
        let mut state = serializer.serialize_struct("TurnBasedAbilityComponent", 50)?;

        // Serialize the `_parent_object` field directly
        state.serialize_field("_parent_object", &self._parent_object)?;
        unsafe {
        // Serialize fields with custom logic (e.g., pointers, arrays)
            state.serialize_field("CharmDamageAttackProperty", &serialize_pointer(&self.CharmDamageAttackProperty))?;
            state.serialize_field("AbilityComponentRef__BackingField", &serialize_pointer(&self.AbilityComponentRef__BackingField))?;
            state.serialize_field("DisableActionStateByTask__BackingField", &serialize_pointer(&self.DisableActionStateByTask__BackingField))?;

            state.serialize_field("OnAbilityPropertyChanged", &serialize_pointer(&self.OnAbilityPropertyChanged))?;

            state.serialize_field("_BuffLockStepSources", &serialize_pointer(&self._BuffLockStepSources))?;


            state.serialize_field("_ExtraMaxLayerConfig", &serialize_pointer(&self._ExtraMaxLayerConfig))?;


            if self._CharacterDataRef.is_null() { state.serialize_field("_CharacterDataRef", "null")?; }
            else {state.serialize_field("_CharacterDataRef", &*self._CharacterDataRef)?;
            }

            state.serialize_field("AdditionalAbilityParamList", &serialize_pointer(&self.AdditionalAbilityParamList))?;

            if self._SelfExtrAbilityList.is_null() { state.serialize_field("_SelfExtrAbilityList", "null")?; }
            else {state.serialize_field("_SelfExtrAbilityList", &*self._SelfExtrAbilityList)?;
            }

            state.serialize_field("Weakness", &serialize_pointer(&self.Weakness))?;


            if self._AbilityPropertiesInitSnapshot.is_null() { state.serialize_field("_AbilityPropertiesInitSnapshot", "null")?; }
            else {state.serialize_field("_AbilityPropertiesInitSnapshot", &*self._AbilityPropertiesInitSnapshot)?;
            }

            state.serialize_field("RegardAsAttackTypeMap", &serialize_pointer(&self.RegardAsAttackTypeMap))?;

            if self._KillerEntity.is_null() { state.serialize_field("_KillerEntity", "null")?; }
            else {state.serialize_field("_KillerEntity", &*self._KillerEntity)?;
            }

            state.serialize_field("_DebuffLockStepSources", &serialize_pointer(&self._DebuffLockStepSources))?;


            state.serialize_field("RegardAsSkillTypeMap", &serialize_pointer(&self.RegardAsSkillTypeMap))?;

            if self.ProjectileTargetAttachPoint.is_null() { state.serialize_field("ProjectileTargetAttachPoint", "null")?; }
            else {state.serialize_field("ProjectileTargetAttachPoint", &*self.ProjectileTargetAttachPoint)?;
            }

            state.serialize_field("_DotModifierEventProcessors", &serialize_pointer(&self._DotModifierEventProcessors))?;


            state.serialize_field("_DmgChunk", &serialize_pointer(&self._DmgChunk))?;

            if self.CharmDamageTarget.is_null() { state.serialize_field("CharmDamageTarget", "null")?; }
            else { state.serialize_field("CharmDamageTarget", &*self.CharmDamageTarget)?; }

            state.serialize_field("_AbilityToSkillMapping", &serialize_pointer(&self._AbilityToSkillMapping))?;
            state.serialize_field("ModifierOverrideMapping", &serialize_pointer(&self.ModifierOverrideMapping))?;

            state.serialize_field("_EnergyPointEntries", &serialize_pointer(&self._EnergyPointEntries))?;

            state.serialize_field("AddModifierBindValueMapping", &serialize_pointer(&self.AddModifierBindValueMapping))?;
            state.serialize_field("CustomDataRef__BackingField", &serialize_pointer(&self.CustomDataRef__BackingField))?;


            if self.LastStanceBreakEntity__BackingField.is_null() { state.serialize_field("LastStanceBreakEntity__BackingField", "null")?; }
            else {
                state.serialize_field("LastStanceBreakEntity__BackingField", &*self.LastStanceBreakEntity__BackingField)?;
            }

            if self._SyncPropertySource.is_null() { state.serialize_field("_SyncPropertySource", "null")?; }
            else {
                state.serialize_field("_SyncPropertySource", &*self._SyncPropertySource)?;
            }

            state.serialize_field("_OnHitEffectMultipleOverride", &serialize_pointer(&self._OnHitEffectMultipleOverride))?;

            state.serialize_field("_DamageStoreList", &serialize_pointer(&self._DamageStoreList))?;

            if self._JsonConfigRef.is_null() { state.serialize_field("_JsonConfigRef", "null")?; }
            else {
                state.serialize_field("_JsonConfigRef", &*self._JsonConfigRef)?;
            }

            state.serialize_field("DamageSplitData",  &serialize_pointer(&self.DamageSplitData))?;


            state.serialize_field("_StancePreshowConfigs",  &serialize_pointer(&self._StancePreshowConfigs))?;

            state.serialize_field("_EnableNegativeHPSourceList",  &serialize_pointer(&self._EnableNegativeHPSourceList))?;

            if self._ModifierEventSourceMuteCounter.is_null() { state.serialize_field("_ModifierEventSourceMuteCounter", "null")?; }
            else {
                state.serialize_field("_ModifierEventSourceMuteCounter", &serialize_pointer(&self._ModifierEventSourceMuteCounter))?;
            }

            state.serialize_field("_LockHPList",  &serialize_pointer(&self._LockHPList))?;

            state.serialize_field("_RedStanceInfoList",  &serialize_pointer(&self._RedStanceInfoList))?;

            if self.CharmSkillName.is_null() { state.serialize_field("CharmSkillName", "null")?; }
            else {
                state.serialize_field("CharmSkillName", &*self.CharmSkillName)?;
            }

            if self.KillerSkill__BackingField.is_null() { state.serialize_field("KillerSkill__BackingField", "null")?; }
            else {
                state.serialize_field("KillerSkill__BackingField", &serialize_pointer(&self.KillerSkill__BackingField))?;
            }

            if self._DamagedEntityListInAttack.is_null() { state.serialize_field("_DamagedEntityListInAttack", "null")?; }
            else {
                state.serialize_field("_DamagedEntityListInAttack", &*self._DamagedEntityListInAttack)?;
            }

            state.serialize_field("_OnHitEffectOverride",  &serialize_pointer(&self._OnHitEffectOverride))?;

            if self.DamageDefender.is_null() { state.serialize_field("DamageDefender", "null")?; }
            else {
            state.serialize_field("DamageDefender", &*self.DamageDefender)?;}

            state.serialize_field("_AbilityProperties",  &serialize_pointer(&self._AbilityProperties))?;

            state.serialize_field("_RedStanceInfo", &serialize_pointer(&self._RedStanceInfo))?;
            state.serialize_field("_DefaultStanceInfo", &serialize_pointer(&self._DefaultStanceInfo))?;
            state.serialize_field("_SyncPropertyMask", &serialize_pointer(&self._SyncPropertyMask))?;
            state.serialize_field("_ModifierRecordList", &serialize_pointer(&self._ModifierRecordList))?;
            state.serialize_field("_StatusChanceResistanceDict", &serialize_pointer(&self._StatusChanceResistanceDict))?;
            state.serialize_field("_DamagedAllEntityIDListInAttack", &serialize_pointer(&self._DamagedAllEntityIDListInAttack))?;
            state.serialize_field("_ExtraStanceInfo", &serialize_pointer(&self._ExtraStanceInfo))?;

            if self._DamageAttacker.is_null() { state.serialize_field("_DamageAttacker", "null")?; }
            else { state.serialize_field("_DamageAttacker", &*self._DamageAttacker)?; }

            state.serialize_field("LockActionDelayChange", &serialize_pointer(&self.LockActionDelayChange))?;

            state.serialize_field("_ModifierEventProcessors",  &serialize_pointer(&self._ModifierEventProcessors))?;

            if self.OverflowStanceDamageAttacker__BackingField.is_null() { state.serialize_field("OverflowStanceDamageAttacker__BackingField", "null")?; }
            else { state.serialize_field("OverflowStanceDamageAttacker__BackingField", &*self.OverflowStanceDamageAttacker__BackingField)?; }

            state.serialize_field("_TransformRef", &serialize_pointer(&self._TransformRef))?;
            state.serialize_field("_StatusProbabilityDict", &serialize_pointer(&self._StatusProbabilityDict))?;

            state.serialize_field("ResistModifierBehaviorFlags__BackingField",  &serialize_pointer(&self.ResistModifierBehaviorFlags__BackingField))?;

            state.serialize_field("_DepartedParams",  &serialize_pointer(&self._DepartedParams))?;

            state.serialize_field("_DelayModifyActionDelayQueue", &serialize_pointer(&self._DelayModifyActionDelayQueue))?;
            state.serialize_field("_LockShieldCounter", &serialize_pointer(&self._LockShieldCounter))?;
            state.serialize_field("_ModifierDelayParamList", &serialize_pointer(&self._ModifierDelayParamList))?;
            state.serialize_field("TotalDamageCurrentAttack", &self.TotalDamageCurrentAttack)?;
            state.serialize_field("BattleTag__BackingField", &self.BattleTag__BackingField)?;
            state.serialize_field("ForceKillFlag__BackingField", &self.ForceKillFlag__BackingField)?;
            state.serialize_field("ActionDelayChanged__BackingField", &self.ActionDelayChanged__BackingField)?;
            state.serialize_field("CharmDisableBPAdd", &self.CharmDisableBPAdd)?;
            state.serialize_field("bIsInCharmAction", &self.bIsInCharmAction)?;
            state.serialize_field("VisualFlagValue__BackingField", &self.VisualFlagValue__BackingField)?;
            state.serialize_field("_DeathVersion", &self._DeathVersion)?;
            state.serialize_field("TotalHitNum", &self.TotalHitNum)?;
            state.serialize_field("DeathSource__BackingField", &self.DeathSource__BackingField)?;
            state.serialize_field("IsTriggeringStanceCountDown__BackingField", &self.IsTriggeringStanceCountDown__BackingField)?;
            state.serialize_field("HasRevived", &self.HasRevived)?;
            state.serialize_field("UseSpecialSP__BackingField", &self.UseSpecialSP__BackingField)?;
            state.serialize_field("IsSharedDamageDataTarget", &self.IsSharedDamageDataTarget)?;
            state.serialize_field("LastStanceDamageType__BackingField", &self.LastStanceDamageType__BackingField)?;
            state.serialize_field("_ModifierUIOperationIncr", &self._ModifierUIOperationIncr)?;
            state.serialize_field("IsInAttack", &self.IsInAttack)?;
            state.serialize_field("MuteTriggerDeath__BackingField", &self.MuteTriggerDeath__BackingField)?;
            state.serialize_field("_HighestPriorityOnHitEffect", &self._HighestPriorityOnHitEffect)?;
            state.serialize_field("CurrentAttackType__BackingField", &self.CurrentAttackType__BackingField)?;
            state.serialize_field("ProjectileHitCount", &self.ProjectileHitCount)?;
            state.serialize_field("CharmDamageCount", &self.CharmDamageCount)?;
            state.serialize_field("_ModifierDelayAddCount", &self._ModifierDelayAddCount)?;
            state.serialize_field("_DebuffLockStep", &self._DebuffLockStep)?;
            state.serialize_field("StanceType", &self.StanceType)?;
            state.serialize_field("InheritSPRatio", &self.InheritSPRatio)?;
            state.serialize_field("InsertAbilityCount", &self.InsertAbilityCount)?;
            state.serialize_field("SpeedVisualFlagValue__BackingField", &self.SpeedVisualFlagValue__BackingField)?;
            state.serialize_field("_CurrentAttackPhase", &self._CurrentAttackPhase)?;
            state.serialize_field("PropertyEnumBoundary__BackingField", &self.PropertyEnumBoundary__BackingField)?;
            state.serialize_field("ForbidVisualFlagValue__BackingField", &self.ForbidVisualFlagValue__BackingField)?;
            state.serialize_field("StanceState__BackingField", &self.StanceState__BackingField)?;
            state.serialize_field("_BreakExtendEventUnsettled", &self._BreakExtendEventUnsettled)?;
            state.serialize_field("TriggerBreakExtendLogic", &self.TriggerBreakExtendLogic)?;
            state.serialize_field("MuteAllTriggerDeath__BackingField", &self.MuteAllTriggerDeath__BackingField)?;
            state.serialize_field("CharmDisableSPAdd", &self.CharmDisableSPAdd)?;
            state.serialize_field("PropertyChangeFlag__BackingField", &self.PropertyChangeFlag__BackingField)?;
            state.serialize_field("LockSelfActionDelay", &self.LockSelfActionDelay)?;
            state.serialize_field("_IsProcessingModifierDelayParam", &self._IsProcessingModifierDelayParam)?;
        }
        state.end()
    }
}
impl Serialize for SkillCharacterComponent{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        //log::info!("serialize::SkillCharacterComponent");

        // Initialize serializer for this struct
        let mut state = serializer.serialize_struct("SkillCharacterComponent", 41)?;
        unsafe {
            // Directly serialize `_parent_object`
            state.serialize_field("_parent_object", &self._parent_object)?;

            // let get_all_allow_skill_idx_list = SkillCharacterComponent_GetAllAllowSkillIdxList(self);
            // if get_all_allow_skill_idx_list.is_null() { state.serialize_field("GetAllAllowSkillIdxList", "null")?; }
            // else { state.serialize_field("GetAllAllowSkillIdxList", &*get_all_allow_skill_idx_list)?; }

            // Serialize custom fields with helpers for pointers and arrays
            if self._SkillDataList.is_null() { state.serialize_field("_SkillDataList", "null")?; }
            else {
                //log::info!("_SkillDataList");
                state.serialize_field("_SkillDataList", &*self._SkillDataList)?;
            }

            if self._CharacterDataRef.is_null() { state.serialize_field("_CharacterDataRef", "null")?; }
            else { state.serialize_field("_CharacterDataRef", &*self._CharacterDataRef)?; }

            //log::info!("_SkillTargetRedirectEntries");
            state.serialize_field("_SkillTargetRedirectEntries",  &serialize_pointer(&self._SkillTargetRedirectEntries))?;

            if self._TBAbilityRef.is_null() { state.serialize_field("_TBAbilityRef", "null")?; }
            else { state.serialize_field("_TBAbilityRef", &*self._TBAbilityRef)?; }

            //log::info!("_SkillSlots");
            state.serialize_field("_SkillSlots",  &serialize_pointer(&self._SkillSlots))?;

            if self._JsonConfigRef.is_null() { state.serialize_field("_JsonConfigRef", "null")?; }
            else { state.serialize_field("_JsonConfigRef", &*self._JsonConfigRef)?; }

            let mut index: i32 = 0;
            for chunk in (&self._recordAbilityInfo).chunks(32) {
                let temp = format!("{}_{}", "_recordAbilityInfo", index);
                state.serialize_field(Box::leak(temp.into_boxed_str()), &chunk)?;
                index += 1;
            }

            if self.CurrentSkillTargetList__BackingField.is_null() { state.serialize_field("CurrentSkillTargetList__BackingField", "null")?; }
            else { state.serialize_field("CurrentSkillTargetList__BackingField", &*self.CurrentSkillTargetList__BackingField)?; }

            state.serialize_field("TaskContext__BackingField", &serialize_pointer(&self.TaskContext__BackingField))?;

            if self.CurrentAimAtTargetList.is_null() { state.serialize_field("CurrentAimAtTargetList", "null")?; }
            else { state.serialize_field("CurrentAimAtTargetList", &*self.CurrentAimAtTargetList)?; }

            if self.CurrentSkillSubTargetList__BackingField.is_null() { state.serialize_field("CurrentSkillSubTargetList__BackingField", "null")?; }
            else { state.serialize_field("CurrentSkillSubTargetList__BackingField", &*self.CurrentSkillSubTargetList__BackingField)?; }

            if self.CurrentAimAtMainTargetList.is_null() { state.serialize_field("CurrentAimAtMainTargetList", "null")?; }
            else { state.serialize_field("CurrentAimAtMainTargetList", &*self.CurrentAimAtMainTargetList)?; }


            //log::info!("OnSkillSetup");
            state.serialize_field("OnSkillSetup",  &serialize_pointer(&self.OnSkillSetup))?;

            state.serialize_field("_SkillTypeDisableSlots", &serialize_pointer(&self._SkillTypeDisableSlots))?;
            state.serialize_field("CurrentSkillTargetDamageHP", &serialize_pointer(&self.CurrentSkillTargetDamageHP))?;

            if self.CurrentAimAtSubTargetList.is_null() { state.serialize_field("CurrentAimAtSubTargetList", "null")?; }
            else { state.serialize_field("CurrentAimAtSubTargetList", &*self.CurrentAimAtSubTargetList)?; }

            if self.SkillActualAttacker__BackingField.is_null() { state.serialize_field("SkillActualAttacker__BackingField", "null")?; }
            else { state.serialize_field("SkillActualAttacker__BackingField", &*self.SkillActualAttacker__BackingField)?; }

            state.serialize_field("CurrentSkillTargetCharacterId", &serialize_pointer(&self.CurrentSkillTargetCharacterId))?;

            if self.SkillPointEntity__BackingField.is_null() { state.serialize_field("SkillPointEntity__BackingField", "null")?; }
            else { state.serialize_field("SkillPointEntity__BackingField", &*self.SkillPointEntity__BackingField)?; }

            state.serialize_field("AutoUseUltraParams", &serialize_pointer(&self.AutoUseUltraParams))?;

            if self._SkillTypeDisableCountArr.is_null() { state.serialize_field("_SkillTypeDisableCountArr", "null")?; }
            else {
                //log::info!("SkillCharacterComponent::_SkillTypeDisableCountArr");
                state.serialize_field("_SkillTypeDisableCountArr", &*self._SkillTypeDisableCountArr)?;
            }

            // Serialize standard simple fields normally
            state.serialize_field("CurrentSkillKilledCount", &self.CurrentSkillKilledCount)?;
            state.serialize_field("CharmAction", &self.CharmAction)?;
            state.serialize_field("_AutoStandbyOnCurSkillFinish", &self._AutoStandbyOnCurSkillFinish)?;
            state.serialize_field("CurrentSkillBreakStance", &self.CurrentSkillBreakStance)?;
            state.serialize_field("SelfWaitActiveSkillIndex", &self.SelfWaitActiveSkillIndex)?;
            state.serialize_field("_SelfSkillPerformState", &self._SelfSkillPerformState)?;
            state.serialize_field("_RecordSkillExtraUseParam", &self._RecordSkillExtraUseParam)?;
            state.serialize_field("CurrentSkillHasTriggerEffect", &self.CurrentSkillHasTriggerEffect)?;
            state.serialize_field("_hasRecordSkill", &self._hasRecordSkill)?;
            state.serialize_field("_RedirectTargetIDIncr", &self._RedirectTargetIDIncr)?;
            state.serialize_field("_isPassive", &self._isPassive)?;
            state.serialize_field("IsNoBpCost__BackingField", &self.IsNoBpCost__BackingField)?;
            state.serialize_field("_hasOpInSkill", &self._hasOpInSkill)?;
            state.serialize_field("CurrentSkillKillAllOrBoss", &self.CurrentSkillKillAllOrBoss)?;
            state.serialize_field("_TargetPerformTimeCounter", &self._TargetPerformTimeCounter)?;
            state.serialize_field("_CurrentSkillIndex", &self._CurrentSkillIndex)?;
            state.serialize_field("_CurrentSkillExtraUseParam", &self._CurrentSkillExtraUseParam)?;
            state.serialize_field("_OpIndexInSkill", &self._OpIndexInSkill)?;
            state.serialize_field("_actionSkillIndex", &self._actionSkillIndex)?;
        }
        // Finalize serialization
        state.end()
    }
}

impl Serialize for AvatarServantData{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::AvatarServantData");

        // Begin serializing as a struct, specifying the number of fields
        let mut state = serializer.serialize_struct("AvatarServantData", 6)?;

        // Serialize each field
        state.serialize_field("native_object", &self.native_object)?;

        // Use the provided custom serialization functions for pointers
        if self._AvatarData.is_null() { state.serialize_field("_AvatarData", "null")?; }
        else { unsafe { state.serialize_field("_AvatarData", &*self._AvatarData)? }; }
        state.serialize_field("_Row", &serialize_pointer(&self._Row))?;
        state.serialize_field("_SkillDataMap", &serialize_pointer(&self._SkillDataMap))?;
        state.serialize_field("_Json", &serialize_pointer(&self._Json))?;
        state.serialize_field("_ServantRowData", &serialize_pointer(&self._ServantRowData))?;

        // End serialization
        state.end()
    }
}

impl Serialize for AvatarData{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::AvatarData");

        let mut state = serializer.serialize_struct("AvatarData", 36)?;

        // Serialize each field
        state.serialize_field("native_object", &self.native_object)?;

        if self.HasTakenPromotionRewardList__BackingField.is_null() { state.serialize_field("HasTakenPromotionRewardList__BackingField", "null")?; }
        else { unsafe {
            //log::info!("serialize::AvatarData::HasTakenPromotionRewardList__BackingField");

            state.serialize_field("HasTakenPromotionRewardList__BackingField",
                                              &*self.HasTakenPromotionRewardList__BackingField)?; } }

        state.serialize_field("Row__BackingField", &serialize_pointer(&self.Row__BackingField))?;
        state.serialize_field("_ExtraPropertyAddition", &serialize_pointer(&self._ExtraPropertyAddition))?;

        if self._AvatarName.is_null() { state.serialize_field("_AvatarName", "null")?; }
        else { unsafe { state.serialize_field("_AvatarName", &*self._AvatarName)?; } }

        state.serialize_field("PromotedBeforeData__BackingField", &serialize_pointer(&self.PromotedBeforeData__BackingField))?;
        state.serialize_field("_TrialEquipment", &serialize_pointer(&self._TrialEquipment))?;
        state.serialize_field("GrowUpBeforeData__BackingField", &serialize_pointer(&self.GrowUpBeforeData__BackingField))?;

        if self.ServantData__BackingField.is_null() { state.serialize_field("ServantData__BackingField", "null")?; }
        else { unsafe {state.serialize_field("ServantData__BackingField", &*self.ServantData__BackingField)?;} }

        state.serialize_field("CombatPowerData__BackingField",
                              &serialize_pointer(&self.CombatPowerData__BackingField))?;
        state.serialize_field("AvatarPropertyData__BackingField",
                              &serialize_pointer(&self.AvatarPropertyData__BackingField))?;
        state.serialize_field("UltraSkillConfig__BackingField",
                              &serialize_pointer(&self.UltraSkillConfig__BackingField))?;
        state.serialize_field("LevelUpedBeforeData__BackingField",
                              &serialize_pointer(&self.LevelUpedBeforeData__BackingField))?;
        state.serialize_field("_SkillDataMap",
                              &serialize_pointer(&self._SkillDataMap))?;
        state.serialize_field("RelicsData__BackingField",
                              &serialize_pointer(&self.RelicsData__BackingField))?;

        if self._SkinIDList.is_null() { state.serialize_field("_SkinIDList", "null")?; }
        else { unsafe {
            //log::info!("serialize::AvatarData::_SkinIDList");
            state.serialize_field("_SkinIDList", &*self._SkinIDList)?; } }

        state.serialize_field("SkillTreeData", &serialize_pointer(&self.SkillTreeData))?;
        state.serialize_field("SpecialRow__BackingField", &serialize_pointer(&self.SpecialRow__BackingField))?;
        state.serialize_field("_AvatarRowData", &serialize_pointer(&self._AvatarRowData))?;
        state.serialize_field("FirstMetTimeStamp", &self.FirstMetTimeStamp)?;
        state.serialize_field("Promotion__BackingField", &self.Promotion__BackingField)?;
        state.serialize_field("Level__BackingField", &self.Level__BackingField)?;
        state.serialize_field("_AdventurePlayerID", &self._AdventurePlayerID)?;
        state.serialize_field("IsMarked__BackingField", &self.IsMarked__BackingField)?;
        state.serialize_field("IsDisplayOnly__BackingField", &self.IsDisplayOnly__BackingField)?;
        state.serialize_field("IsNew__BackingField", &self.IsNew__BackingField)?;
        state.serialize_field("AvatarType__BackingField", &self.AvatarType__BackingField)?;
        state.serialize_field("DressedSkinID__BackingField", &self.DressedSkinID__BackingField)?;
        state.serialize_field("EquipmentUID__BackingField", &self.EquipmentUID__BackingField)?;
        state.serialize_field("RealID__BackingField", &self.RealID__BackingField)?;
        state.serialize_field("Rank__BackingField", &self.Rank__BackingField)?;
        state.serialize_field("SpecialAvatarID__BackingField", &self.SpecialAvatarID__BackingField)?;
        state.serialize_field("CurrentExp__BackingField", &self.CurrentExp__BackingField)?;
        state.serialize_field("_BaseID", &self._BaseID)?;

        // End serialization
        state.end()
    }
}
pub fn serialize_pointer(ptr: &*const ffi::c_void) -> String
{
    format!("{}", *ptr as u64)
}


// pub  fn serialize_native_string<S>(ptr: &*const NativeString, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     if *ptr as u64 != 0
//     {
//         unsafe {
//             match (**ptr).to_string() {
//                 Ok(s) => serializer.serialize_str(s.as_str()),
//                 Err(e) => serializer.serialize_str(e.to_string().as_str()), // Propagate or handle the error
//             }
//         }
//     }
//     else {
//         serializer.serialize_str("")
//     }
// }

impl Serialize for FixPoint{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::FixPoint");
        // if self.m_rawValue != 0
        // {
            serializer.serialize_f64(
//                round_to_places(
                    fixpoint_to_raw(&self),
                //3
            )
  //          )
    //    }
      //  else { serializer.serialize_f64(0.0) }
    }
}

//
// impl Serialize for NativeArray<u32>
// where
// {
//     ///Naming is actually incorrect - vector at the moment of writing is supposed to contain only the first item
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         Ok(())
//     }
// }

impl<T> Serialize for NativeArray<T>
where
    T: Serialize + Clone + std::fmt::Debug
{
    ///Naming is actually incorrect - vector at the moment of writing is supposed to contain only the first item
    /// and 'bounds' contain something very questionable
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        if self.length == 0{
            return serializer.serialize_str("[]");
        }

        let ser_id = Uuid::new_v4();
        //veritas::kreide::native_types::NativeObject
        //log::info!("{}", std::any::type_name::<T>());

        // log::info!("{} serialize::NativeArray val {} obj {} {}, bounds: {}, length {}, vector: {} {}, ptr {}",
        //             ser_id, std::any::type_name::<T>(), self.obj.klass as u64, self.obj.monitor as u64,
        //             &(self.bounds as u64), &self.length, &(self.vector as u32), self.vector as u64, (self as *const NativeArray<T>) as u64);

        if "i32" == std::any::type_name::<T>() || "u32" == std::any::type_name::<T>() {
            if self.length > 0 &&
                (self.bounds as u64) > 0  /*|| (self.vector as u32) > 0*/ {
                log::info!("{} serialize::NativeArray val {} obj {} {}, bounds: {}, length {}, vector: {} {}, ptr {}",
                    ser_id, std::any::type_name::<T>(), self.obj.klass as u64, self.obj.monitor as u64,
                    &(self.bounds as u64), &self.length, &(self.vector as u32), self.vector as u64, (self as *const NativeArray<T>) as u64);
            }
            unsafe {
                if self.bounds as u64 > 0x70000000000 && //0x70000000000 = 7696581394432u64
                    (self.bounds as u64) < 8700100315968u64 &&
                    self.length > 0 &&
                    "u32" == std::any::type_name::<T>() {
                    // log::info!("{} bounds value u64 {} ", ser_id, self.bounds as u64);
                    // log::info!("{} bounds value u32 {} ", ser_id, self.bounds as u32);
                    // log::info!("{} bounds value at the ref u32 +0 {} ", ser_id, *((self.bounds as *const u32).add(0)));
                    // log::info!("{} bounds value at the ref u32 +4 {} ", ser_id, *((self.bounds as *const u64).add(1)));
                    // log::info!("{} bounds value at the ref u32 +8 {} ", ser_id, *((self.bounds as *const u64).add(2)));
                    // log::info!("{} bounds value at the ref u32 +12 {} ", ser_id, *((self.bounds as *const u64).add(3)));
                    // log::info!("{} bounds value at the ref u32 +16 {} ", ser_id, *((self.bounds as *const u64).add(4)));
                    // log::info!("{} bounds value at the ref u32 +20 {} ", ser_id, *((self.bounds as *const u64).add(5)));
                    //log::info!("{} bounds value at the ref u64 {} ", ser_id, &*(self.bounds as *const u64 ));
                }
                if self.bounds as u64 > 0x70000000000 && //0x70000000000 = 7696581394432u64
                    (self.bounds as u64) < 8700100315968u64 &&
                    self.length > 0 &&
                    "i32" == std::any::type_name::<T>() {
                    // log::info!("{} bounds value u64 {} ", ser_id, self.bounds as u64);
                    //log::info!("{} bounds value u32 {} ", ser_id, self.bounds as u32);
                    // log::info!("{} bounds value at the ref i32 +0 {} ", ser_id, *((self.bounds as *const i64).add(0)));
                    // log::info!("{} bounds value at the ref i32 +4 {} ", ser_id, *((self.bounds as *const i64).add(1)));
                    // log::info!("{} bounds value at the ref i32 +8 {} ", ser_id, *((self.bounds as *const i64).add(2)));
                    // log::info!("{} bounds value at the ref i32 +12 {} ", ser_id, *((self.bounds as *const i64).add(3)));
                    // log::info!("{} bounds value at the ref i32 +16 {} ", ser_id, *((self.bounds as *const i64).add(4)));
                    // log::info!("{} bounds value at the ref i32 +20 {} ", ser_id, *((self.bounds as *const i64).add(5)));
                    //log::info!("{} bounds value at the ref u64 {} ", ser_id, &*(self.bounds as *const u64 ));
                }
                if self.vector as u64 > 0x70000000000 && //0x70000000000 = 7696581394432u64
                    (self.vector as u64) < 8700100315968u64 {
                    // log::info!("{} vector value at the ref u32 {} ", ser_id, &*(self.vector as *const u32 ));
                    // log::info!("{} vector value at the ref u64 {} ", ser_id, &*(self.vector as *const u64 ));
                }
                // if self.bounds as u64 == 0 && (self.vector as u64) == 0
                // && ( *((((self as *const NativeArray<T>) as u64)+0x1c)as *const u64) != 0
                // || *((((self as *const NativeArray<T>) as u64)+0x8)as *const u64) != 0 ) {
                //     //unsafe { log::info!("value at the ref u32 {} ", &*((((self as *const NativeArray<T>) as u64)+0x8)as *const u64)); }
                //     log::info!("{} value at the ref u32 0x1c {} ", ser_id, &*((((self as *const NativeArray<T>) as u64)+0x1c)as *const u64));
                //     log::info!("{} value at the ref u32 0x8 {} ", ser_id, &*((((self as *const NativeArray<T>) as u64)+0x8)as *const u64));
                // }
            }
        }
        let mut state = serializer.serialize_struct("NativeArray", (self.length + 10) as usize)?;

        //state.serialize_field("obj", &self.obj)?;
        state.serialize_field("length", &self.length)?;

        state.serialize_field("bounds", &(self.bounds as usize))?;
        state.serialize_field("vector", &serialize_pointer(&(self.vector as *const c_void)))?;

        //checking just in case
        if(self.vector as u64) > 0 && (self.bounds as u64) > 0 {
            //from my understanding such thins shouldn't happen but if it does then I'd like to know where
            log::warn!("{} serialize::NativeArray weird {} obj {} {}, bounds: {}, length {}, vector: {}, ptr {}",
                ser_id, std::any::type_name::<T>(), self.obj.klass as u64, self.obj.monitor as u64,
                &(self.bounds as u64), &self.length, &(self.vector as u32), (self as *const NativeArray<T>) as u64);
        }

        if self.length > 0 {
            if (self.vector as u64) > 0 {
                unsafe {
                    /*
                    //
                    // if "i32" == std::any::type_name::<T>() {
                    //     log::info!("{} {} v retrieved items {:?}", ser_id, std::any::type_name::<T>(),
                    //         items.iter().map(|x| *x as i32).collect::<Vec<i32>>());
                    // }
                    // else if "u32" == std::any::type_name::<T>() {
                    //     log::info!("{} {} v retrieved items {:?}", ser_id, std::any::type_name::<T>(),
                    //         items.iter().map(|x| *x as u32).collect::<Vec<u32>>());
                    // }
                    */
                    let mut index = 0;

                    if std::any::type_name::<T>().starts_with("veritas::kreide::native_types::NativeDictionary")
                        || std::any::type_name::<T>() == "veritas::kreide::types::rpg::gamecore::BattleRelicInfo" {

                        let val_self: &NativeValueArray<T> = &*(self as *const NativeArray<T> as *const NativeValueArray<T>);

                        let items = val_self.to_slice();
                        for item in items {
                            let field_name = format!("i_{}", &index);

                            /*
                            // let self_ptr = ((self as *const NativeArray<T>) as u64);
                            // let int_ptr = self_ptr as *const u32;
                            // log::info!("int_ptr: {:p} {}", int_ptr, ser_id);
                            // if !int_ptr.is_null(){
                            //     // Safety: Ensure the pointer is valid for at least `n` bytes.
                            //     let mem_slice =std::slice::from_raw_parts(int_ptr, 64);
                            //     log::info!("Memory u32 slice content: {:?} {}", mem_slice, ser_id);
                            // }
                            */

                            if std::any::type_name::<T>() == "veritas::kreide::native_types::NativeDictionaryEntry<u32, u32>" ||
                                std::any::type_name::<T>() == "veritas::kreide::native_types::NativeDictionaryValueEntry<u32, u32>" ||
                                std::any::type_name::<T>() == "veritas::kreide::types::rpg::gamecore::BattleRelicInfo_struct" {
                                let item_dict_entry = &*(&item as *const _ as *const NativeDictionaryValueEntry<u32, u32>);

                                //log::info!("{} val array route trying to deserialize value from vector {} {:X} {} {:p}", ser_id, item as *const _ as u128, size_of::<T>(),std::any::type_name::<T>(), item);

                                state.serialize_field(Box::leak(field_name.into_boxed_str()), item_dict_entry)?;
                            }
                            else {
                                //log::info!("{} val array route trying to deserialize value from vector {} {}", ser_id, size_of::<T>(),std::any::type_name::<T>());
                                state.serialize_field(Box::leak(field_name.into_boxed_str()), &item)?;
                            }
                            index += 1;
                        }
                    }
                    else{
                        let _self2xcv: NativeArray<T> = NativeArray {
                            length: self.length + 1,
                            ..*self
                        };

                        let items = match std::any::type_name::<T>(){
                            "i32"  => _self2xcv.to_slice(),
                            "u32" => _self2xcv.to_slice(),
                            _ => self.to_slice()
                        };

                        for item in items {
                            let field_name = format!("i_{}", &index);
                            //log::info!("item {}", *item as u64);
                            if (*item as u64) > 0x70000000000 //0x70000000000 = 7696581394432u64
                                && std::any::type_name::<T>() != "i32" //at the moment of writing it is the only 2 value types used in collections
                                && std::any::type_name::<T>() != "u32"
                                && (*item as u64) < 8700100315968u64 { //for some reason some NativeObjects have retarded addresses
                                //like 3175009970383523287 (mb 2 collapsed 32bit values?) or 1900545 (latter was found in SkillCharacterComponent, looks like a real id btw)
                                state.serialize_field(Box::leak(field_name.into_boxed_str()), &**item)?;
                            } else {
                                if std::any::type_name::<T>() == "u32" {
                                    log::info!("{} deserializing value from val vector u32 {}", ser_id, *item as u32);
                                    state.serialize_field(Box::leak(field_name.into_boxed_str()), &(*item as u32))?;
                                } else if std::any::type_name::<T>() == "i32" {
                                    let i32item = *item as i32;
                                    log::info!("{} writing value from val vector i32 {}", ser_id, &i32item);
                                    state.serialize_field(Box::leak(field_name.into_boxed_str()), &i32item)?;
                                } else if is_value_type_under65b(std::any::type_name::<T>()) {
                                    state.serialize_field(Box::leak(field_name.into_boxed_str()), &(*item as u64))?;

                                } else if (*item as u64) > 0 {
                                    if std::any::type_name::<T>() == "veritas::kreide::types::rpg::gamecore::BattleRelicInfo" {
                                        let self_ptr = ((*item) as u64);
                                        let int_ptr = self_ptr as *const u32;
                                        log::info!("int_ptr: {:p} {}", int_ptr, ser_id);
                                        if !int_ptr.is_null(){
                                            // Safety: Ensure the pointer is valid for at least `n` bytes.
                                            let mem_slice =std::slice::from_raw_parts(int_ptr, 64);
                                            log::info!("BattleRelicInfo Memory u32 slice content: {:?} {}", mem_slice, ser_id);
                                        }
                                    }

                                    log::info!("{} serialize::NativeArray val {} obj {} {}, bounds: {}, length {}, vector: {} {}, ptr {}",
                                        ser_id, std::any::type_name::<T>(), self.obj.klass as u64, self.obj.monitor as u64,
                                        &(self.bounds as u64), &self.length, &(self.vector as u32), self.vector as u64, (self as *const NativeArray<T>) as u64);

                                    log::info!("{} unusual value from vector {}", ser_id, *item as u64); //we expect ref here and got smth weird
                                    state.serialize_field(Box::leak(field_name.into_boxed_str()), &(*item as u64))?;
                                }
                                else {
                                    // log::warn!("Dead end in finding the correct deser case {} serialize::NativeArray val {} obj {} {}, bounds: {}, length {}, vector: {} {}, ptr {}, item {}",
                                    //     ser_id, std::any::type_name::<T>(), self.obj.klass as u64, self.obj.monitor as u64,
                                    //     &(self.bounds as u64), &self.length, &(self.vector as u32), self.vector as u64, (self as *const NativeArray<T>) as u64, *item as u64);
                                    state.serialize_field(Box::leak(field_name.into_boxed_str()), "null")?;
                                }

                            }
                            index += 1;
                        }
                    }
                }
            }
            else if (self.bounds as u64) > 0 {
                let memory_slice = unsafe{ slice::from_raw_parts(self.bounds as *const u64, (&self.length+4) as usize)};
                let vec = &memory_slice.to_vec()
                    //.into_iter().skip(4).collect::<Vec<u64>>()
                    ;
                if "i32" == std::any::type_name::<T>() {
                    // log::info!("{} {} b retrieved items i32 {:?}", ser_id, std::any::type_name::<T>(),
                    //         vec.iter().map(|x| *x as i32).collect::<Vec<i32>>());
                }
                if "u32" == std::any::type_name::<T>() {
                    // log::info!("{} {} b retrieved items u32 {:?}", ser_id, std::any::type_name::<T>(),
                    //         vec.iter().map(|x| *x as u32).collect::<Vec<u32>>());
                }

                let mut index = 0;
                // for value in vec {
                //     println!("Value: {}", value); // Borrowed reference to each element
                // }
                for i in vec {
                    let field_name = format!("i_{}", &index);

                    if i > &(5497690084096)
                        && Backtrace::capture().frames().len() < 50 //this workaround is needed for now
                        && std::any::type_name::<T>() != "i32" {
                        //log::info!("super-serializing {}", i);
                        let item = (*i) as usize as *const T;
                        unsafe {state.serialize_field(Box::leak(field_name.into_boxed_str()), &*item)?;}
                    }
                    else {
                        //if "i32" == std::any::type_name::<T>() || "u32" == std::any::type_name::<T>()
                        // {
                        //     if std::any::type_name::<T>() == "u32" && *i != 0 && index > 3 {
                        //         log::info!("{} value from val bounds u32 {}", ser_id, *i as u32);
                        //     }
                        //     if std::any::type_name::<T>() == "i32" && *i != 0 && index > 3 {
                        //         log::info!("{} value from val bounds i32 {}", ser_id, *i as i32);
                        //     }
                        // }
                        // if Backtrace::capture().frames().len() < 50 && index > 3 {
                        //log::info!("{} unusual value from bounds {} {}", ser_id, *i, std::any::type_name::<T>());
                        // we expect ref here and got smth weird
                        // }
                        state.serialize_field(Box::leak(field_name.into_boxed_str()), i)?;
                    }
                    index += 1;
                }
            }
        }

        state.end()
    }
}

impl<T> Serialize for NativeValueArray<T>
where
    T: Serialize + Clone + std::fmt::Debug
{
    ///Naming is actually incorrect - vector at the moment of writing is supposed to contain only the first item
    /// and 'bounds' contain something very questionable
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        if self.length == 0{
            return serializer.serialize_str("[]");
        }

        let ser_id = Uuid::new_v4();

        let mut state = serializer.serialize_struct("NativeArray", (self.length + 10) as usize)?;

        if self.length > 0 {
                let items =  self.to_slice();
                let mut index = 0;

                for item in items {
                    let field_name = format!("i_{}", &index);
                    //log::info!("item {}", *item as u64);

                    //log::info!("{} unusual value from vector {:?} {:X} {}", ser_id, item, size_of::<T>(), std::any::type_name::<T>());

                    log::info!("{} serialize::NativeArray val {:?} size {} obj {:?}, length {:?}, vector: {:?}, ptr {:?}",
                                ser_id, std::any::type_name::<T>(), size_of::<T>(), self.obj,
                        &self.length, &self.vector, self as *const NativeValueArray<T>);

                    state.serialize_field(Box::leak(field_name.into_boxed_str()), &item)?;
                    index += 1;
            }
        }

        state.end()
    }
}

pub fn get_dictionary_entry_size<K, V>(dictionary: &NativeDictionary<K, V>) -> usize
{
    if (std::any::type_name::<V>() == "i32" || std::any::type_name::<V>() == "u32") &&
        (std::any::type_name::<K>() == "i32" || std::any::type_name::<K>() == "u32")
    {
        0x10
    }
    else {
        0x18 //what if its more, like big structs or smth
    }
}

impl<K, V> Serialize for NativeDictionary<K, V>
where
    K: Serialize + Clone + std::fmt::Debug,
    V: Serialize + Clone + std::fmt::Debug
{
    ///Naming is actually incorrect - vector at the moment of writing is supposed to contain only the first item
    /// and 'bounds' contain something very questionable
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        if self.count == 0 {
            return serializer.serialize_str("[]");
        }

        let ser_id = Uuid::new_v4();

        //log::info!("serialize::BattleRelicModule {:p}", self);

        log::info!("NativeDictionary<{},{}> obj {} count {}, buckets: {}, entries {}, ptr {} sizes: dict {:X} k {:X} v {:X} entrysize {:X} val {:X} {:p} {}",
            std::any::type_name::<K>(), std::any::type_name::<V>(), (&self.obj as *const NativeObject) as u64, &self.count,
            self.buckets as u64, self.entries as u64,
            (self as *const NativeDictionary<K, V>) as u64,
            size_of::<NativeDictionary<K,V>>(), size_of::<K>(), size_of::<V>(),
            size_of::<NativeDictionaryEntry<K,V>>(),
            size_of::<NativeDictionaryValueEntry<K,V>>(),
            self, ser_id
        );

        unsafe {
            /*
            // let self_ptr = ((self as *const NativeDictionary<K, V>) as u64);
            // let int_ptr = self_ptr as *const u32;
            // if !int_ptr.is_null(){
            //     // Safety: Ensure the pointer is valid for at least `n` bytes.
            //     let mem_slice =std::slice::from_raw_parts(int_ptr, 64);
            //     log::info!("Memory u32 slice content: {:?} {}", mem_slice, ser_id);
            // }
            // let long_ptr = self_ptr as *const u64;
            // if !long_ptr.is_null(){
            //     // Safety: Ensure the pointer is valid for at least `n` bytes.
            //     let mem_slice =std::slice::from_raw_parts(long_ptr, 32);
            //     log::info!("Memory u64 slice content: {:?} {}", mem_slice, ser_id);
            // }

            // if self.buckets as u64 > 0x70000000000 && (self.buckets as u64) < 8700100315968u64 {
            // //trying to make sure buckets are at 0x10
            //     let mem_slice32 = std::slice::from_raw_parts(self.buckets as *const u32, (self.count*8) as usize);
            //     log::info!("Memory u32 at buckets ptr slice content: {:?} {}", mem_slice32, ser_id);;
            //
            //     let mem_slice64 = std::slice::from_raw_parts(self.buckets as *const u64, (self.count*4)as usize);
            //     log::info!("Memory u64 at buckets ptr slice content: {:?} {}", mem_slice64, ser_id);
            // }
             */
            // if self.entries as u64 > 0x70000000000 && (self.entries as u64) < 8700100315968u64 {
            //     let mem_slice32 =std::slice::from_raw_parts(self.entries as *const u32, ((self.count*8)+4) as usize);
            //     log::info!("Memory u32 at entries ptr slice content: {:?} {}", mem_slice32, ser_id);;
            //
            //     let mem_slice64 =std::slice::from_raw_parts(self.entries as *const u64, ((self.count*4)+4) as usize);
            //     log::info!("Memory u64 at entries ptr slice content: {:?} {}", mem_slice64, ser_id);
            // }
        }

        if self.count > 1000
        {
            log::warn!("huge dictionary count {} {}", &self.count, ser_id);
            return serializer.serialize_str("[]");
        }

        let mut state = serializer.serialize_struct("Dictionary", ((self.count*2) + 10) as usize)?;
        state.serialize_field("obj", &self.obj)?;
        state.serialize_field("count", &&self.count)?;

        unsafe {
            // if self.buckets.is_null() {
            //     state.serialize_field("buckets", "[]")?;
            // }
            // else{
            //     state.serialize_field("buckets", &*self.buckets)?;
            // }
            if self.entries.is_null() {
                state.serialize_field("entries", "[]")?;
            }
            else {
                let entries_mut = self.entries as *mut NativeArray<NativeDictionaryEntry<K, V>>;
                if self.count < (&*(self.entries)).length as i32
                {
                    log::info!("dict size and underlying array size mismatch {} {}", self.count, (&*(self.entries)).length);
                    (*entries_mut).length = self.count as u32;
                }

                if size_of::<NativeDictionaryValueEntry<K,V>>() == 16 ||
                    (is_value_type_under65b(std::any::type_name::<K>()) && is_value_type_under65b(std::any::type_name::<V>()))
                {
                    state.serialize_field("entries", &*(entries_mut as *const NativeArray<NativeDictionaryValueEntry<K, V>>))?;
                }
                else {
                    state.serialize_field("entries", &*entries_mut)?;
                }
            }
        }

        state.end()
    }
}

//checks if the type is the value one and no bigger than 64 bits
pub fn is_value_type_under65b(t: &str) -> bool
{
    t == "i32" || t == "u32" || t == "f32" || t == "f64" || t == "bool"
        || t == std::any::type_name::<FixPoint>()
        || t == std::any::type_name::<AbilityProperty>()
        || t == std::any::type_name::<NCGNFPLFBOJ_struct>()
        || t == std::any::type_name::<BattleRelicInfo_struct>()
}

impl<K,V> Serialize for NativeDictionaryEntry<K,V>
where
    K: Serialize + Clone + std::fmt::Debug,
    V: Serialize + Clone + std::fmt::Debug
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        log::info!("serializing NativeDictionaryEntry");

        let mut state = serializer.serialize_struct("NativeDictionaryEntry", 4)?;

        // Serialize hash_code
        //state.serialize_field("hash_code", &self.hash_code)?;

        // Serialize key by dereferencing the pointer
        if !is_value_type_under65b(std::any::type_name::<K>()) {
            if !self.key.is_null() {
                let key = unsafe { &*self.key }; // Use unsafe block to access raw pointer
                state.serialize_field("key", key)?;
            } else {
                state.serialize_field("key", &None::<K>)?;
            }
        }
        else {
            log::info!("NativeDictionaryEntry key {:?}", self.key);

            state.serialize_field("key",
                                      &format!("{:?}", self.key)
                )?;
        }
        // Serialize value by dereferencing the pointer

        if !is_value_type_under65b(std::any::type_name::<V>()) {
            if !self.value.is_null() {
                let value = unsafe { &*self.value };
                state.serialize_field("value", value)?;
            } else {
                state.serialize_field("value", &None::<V>)?;
            }
        }
        else {
            log::info!("NativeDictionaryEntry value {:?}", self.value);
            state.serialize_field("value",
                                  &format!("{:?}", self.value)
            )?;
        }

        // Serialize next by treating it as a pointer (e.g., serialize as an address or None if null)
        // This avoids recursively serializing the entire list structure.
        //state.serialize_field("next", &self.next)?;

        state.end()
    }
}
/* initial implementation of serialization (quite incorrect)
// impl<T> Serialize for NativeArray<T>
// where
//     T: Serialize + Clone
// {
//     ///Naming is actually incorrect - vector at the moment of writing is supposed to contain only the first item
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer,
//     {
//         //log::info!("serialize::NativeArray");
//
//         // Start serializing the struct with 4 fields (adjust the number for your fields)
//         let mut state = serializer.serialize_struct("NativeArray", 35)?;
//
//         // Serialize the `obj` field (NativeObject is Serializable)
//         state.serialize_field("obj", &self.obj)?;
//         // Serialize the `max_length` field
//         state.serialize_field("length", &&self.length)?;
//
//         // Serialize the `bounds` raw pointer as its numeric value (cast to `usize`)
//         state.serialize_field("bounds", &(self.bounds as usize))?;
//         //log::info!("{}", serde_json::to_value(Backtrace::capture().to_string()).unwrap());
//
//         if (*&self.bounds) as u64 != 0 && self.length != 0
//         {
//             /*// let t_size = size_of::<T>();
//             // let mem_size = t_size as u32 * &self.length;
//
//             // state.serialize_field("t_size", &t_size)?;
//             // state.serialize_field("mem_size", &mem_size)?;
//             // let x = [5, 6, 7];
//             // let raw_pointer = x.as_ptr();
//             // log::info!("dumping of a NativeArray started");
//             // log::info!("entity.max_length: {}", entity.max_length);
//             // log::info!("entity.bounds: {}", (*&entity.bounds) as u64);
//             // log::info!("t_size: {}", t_size);
//             // log::info!("mem_size: {}", mem_size);
//             //log::info!("type name: {}", std::any::type_name::<T>());
//             // log::info!("bounds size: {}", size_of_val(&(entity.bounds)));
//
//             //log::info!("bounds size as T: {}", size_of_val(&(entity.bounds as *const T)));
//             */
//             let memory_slice = unsafe{ slice::from_raw_parts(self.bounds as *const u64, (&self.length+4) as usize)};
//             // log::info!("ln877");
//             let vec = &memory_slice.to_vec()
//                 //.into_iter()
//                 //.skip(4)
//                 //.collect::<Vec<u64>>()
//                 ;
//
//             // log::info!("ln879");
//             // match serde_json::to_string(vec) {
//             //     Ok(serialized) => log::info!("vec serialized: {}", serialized),
//             //     Err(err) => log::error!("Failed to serialize vec: {}", err),
//             // }
//
//             let mut index = 0;
//             // for value in vec {
//             //     println!("Value: {}", value); // Borrowed reference to each element
//             // }
//             for i in vec {
//                 let field_name = format!("data_{}", &index);
//
//                 if i > &(5497690084096)
//                     && Backtrace::capture().frames().len() < 50
//                     && std::any::type_name::<T>() != "i32" {
//                     //log::info!("super-serializing {}", i);
//                     let item = (*i) as usize as *const T;
//                     unsafe {state.serialize_field(Box::leak(field_name.into_boxed_str()), &*item)?;}
//                 }
//                 else {
//                     state.serialize_field(Box::leak(field_name.into_boxed_str()), i)?;
//                 }
//                 index += 1;
//             }
//
//             //state.serialize_field("data", &vec)?;
//         }
//
//         //log::info!("successfully serialized a NativeArray item {}", std::any::type_name::<T>());
//
//         // Finalize serialization
//         state.end()
//     }
// }
*/

impl Serialize for BattleEquipmentData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BattleEquipmentData", 5)?;
        state.serialize_field("native_object", &self.native_object)?;
        state.serialize_field("CNIHAOIEFPI", &self.CNIHAOIEFPI)?;
        state.serialize_field("FIAKPENJJMN", &self.FIAKPENJJMN)?;
        state.serialize_field("LightConeId", &self.LightConeId)?;
        state.serialize_field("ILBPLOKBBEJ", &self.ILBPLOKBBEJ)?;
        state.end()
    }
}

impl Serialize for LineUpCharacter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::LineUpCharacter");

        // 1. Initialize the struct serializer.
        let mut state = serializer.serialize_struct("LineUpCharacter", 23)?;

        // 2. Serialize simple fields directly.
        state.serialize_field("native_object", &self.native_object)?;
        state.serialize_field("CharacterSP_Denominator", &self.CharacterSP_Denominator)?;
        state.serialize_field("SpecialAvatarID", &self.SpecialAvatarID)?;
        state.serialize_field("Index", &self.Index)?;
        state.serialize_field("CharacterSP_Numerator", &self.CharacterSP_Numerator)?;
        state.serialize_field("AssistUid", &self.AssistUid)?;
        state.serialize_field("CharacterAvatarType", &self.CharacterAvatarType)?;
        state.serialize_field("CharacterLevel", &self.CharacterLevel)?;
        state.serialize_field("WorldLevel", &self.WorldLevel)?;
        state.serialize_field("TotalPower", &self.TotalPower)?;
        state.serialize_field("CharacterRank", &self.CharacterRank)?;
        state.serialize_field("CharacterHPRatio", &self.CharacterHPRatio)?;
        state.serialize_field("CharacterPromotion", &self.CharacterPromotion)?;
        state.serialize_field("CharacterID", &self.CharacterID)?;
        state.serialize_field("SpiritLineupType", &self.SpiritLineupType)?;
        state.serialize_field("CharacterRowIndex", &self.CharacterRowIndex)?;
        unsafe { state.serialize_field("AvatarData", &*get_avatar_data_from_id(self.CharacterID))?; }

        // 3. Serialize fields with custom logic.

        unsafe {
            // SkillTreePointList: Serialize `NativeArray<NativeObject>` pointer.
            state.serialize_field("SkillTreePointList",  &serialize_pointer(&self.SkillTreePointList))?;

            // BattleEquipmentList: Serialize `NativeArray<NativeObject>` pointer.
            if self.BattleEquipmentList.is_null() { state.serialize_field("BattleEquipmentList", "null")?; }
            else {
                state.serialize_field("BattleEquipmentList", &*self.BattleEquipmentList)?;
            }

            // BattleRelicItemModule: Serialize `*const c_void` as a pointer address or None if null.
            if self.BattleRelicItemModule.is_null() { state.serialize_field("BattleRelicItemModule", "null")?; }
            else {
                state.serialize_field("BattleRelicItemModule", &*self.BattleRelicItemModule)?;
            }
            // BattleGridAvatarData: Serialize `*const c_void` as a pointer address or None if null.
            // log::info!("LineupCharacter::BattleGridAvatarData");
            if self.BattleGridAvatarData.is_null() { state.serialize_field("BattleGridAvatarData", "null")?; }
            else {
                state.serialize_field("BattleGridAvatarData", &*self.BattleGridAvatarData)?;
            }
            //log::info!("LineupCharacter::BattleGridAvatarData done");

            // SpiritPassiveList: Serialize `NativeArray<u32>` pointer.
            if self.SpiritPassiveList.is_null() { state.serialize_field("SpiritPassiveList", "null")?; }
            else {
                state.serialize_field("SpiritPassiveList", &*self.SpiritPassiveList)?;
            }

            // ChangedSkillTreePointList: Serialize `NativeArray<NativeObject>` pointer.
                state.serialize_field("ChangedSkillTreePointList",  &serialize_pointer(&self.ChangedSkillTreePointList))?;
        }
        // 4. Finish serialization.
        state.end()
    }
}


impl Serialize for NativeString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        //log::info!("serialize::NativeString");

        serializer.serialize_str(&self.to_string().unwrap_or_default())
    }
}

impl Serialize for NativeObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::NativeObject {} {}", self.klass as u64, self.monitor as u64);
        serializer.serialize_str(&format!("{}", (self as *const Self) as u64))
        // Serialize as a struct with two fields
        // let mut state = serializer.serialize_struct("NativeObject", 2)?;
        //
        // // Serialize `klass` pointer as a hexadecimal string
        // state.serialize_field("klass", &serialize_pointer(&self.klass))?;
        //
        // // Serialize `monitor` pointer as a hexadecimal string
        // state.serialize_field("monitor", &serialize_pointer(&self.monitor))?;
        //
        // state.end()
    }
}

// MMNDIEBMDNL implementation
impl Serialize for MMNDIEBMDNL {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::MMNDIEBMDNL");

        let mut state = serializer.serialize_struct("MMNDIEBMDNL", 8)?;

        // Serialize the nested NativeObject
        state.serialize_field("native_object", &self.native_object)?;

        // Serialize FIMNOPAAFEP (pointer to TurnBasedAbilityComponent) as hex or "null"
        if self.FIMNOPAAFEP.is_null() { state.serialize_field("FIMNOPAAFEP", "null")?; }
        else { unsafe { state.serialize_field("FIMNOPAAFEP", &*self.FIMNOPAAFEP)? }; }

        // Serialize MKMMNLODHDD (pointer to c_void) as hex or "null"
        state.serialize_field("MKMMNLODHDD", &serialize_pointer(&self.MKMMNLODHDD))?;

        // Serialize GNBEIGMFGIP (pointer to c_void) as hex or "null"
        state.serialize_field("GNBEIGMFGIP",&serialize_pointer(&self.GNBEIGMFGIP))?;

        // Serialize HECCDOHIAFD (pointer to SkillCharacterComponent) as hex or "null"
        if self.HECCDOHIAFD.is_null() { state.serialize_field("HECCDOHIAFD", "null")?; }
        else { unsafe { state.serialize_field("HECCDOHIAFD", &*self.HECCDOHIAFD)? };}

        // Serialize HMCDHMFHABF (nested OLHMAHMMBNN type)
        state.serialize_field("HMCDHMFHABF", &self.HMCDHMFHABF)?;

        // Serialize primitive fields
        state.serialize_field("OOIFIGDBNBO", &self.OOIFIGDBNBO)?;
        state.serialize_field("DADCNHAIOMI", &self.DADCNHAIOMI)?;
        state.serialize_field("NMJEMHAMIHD", &self.NMJEMHAMIHD)?;

        state.end()
    }
}

// Implement Serialize for OLHMAHMMBNN
impl Serialize for OLHMAHMMBNN {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::OLHMAHMMBNN");

        let mut state = serializer.serialize_struct("OLHMAHMMBNN", 16)?;

        // Serialize raw pointers as hexadecimal or "null"
        state.serialize_field("JBHFMCDFPPL", &serialize_pointer(&self.JBHFMCDFPPL))?;

        if self.FKHHOBBFMEH.is_null(){
            state.serialize_field("FKHHOBBFMEH", "null")?;
        }
        else {
            unsafe { state.serialize_field("FKHHOBBFMEH", &*self.FKHHOBBFMEH)?; }
        }

        state.serialize_field("BAICECGKLBG", &serialize_pointer(&self.BAICECGKLBG))?;

        state.serialize_field("OAAMONICNLE",  &serialize_pointer(&self.OAAMONICNLE))?;

        // Serialize primitive types
        state.serialize_field("MOIPJLBAODO", &self.MOIPJLBAODO)?;
        state.serialize_field("NMJEMHAMIHD", &self.NMJEMHAMIHD)?;
        state.serialize_field("AHNHNPOCNDJ", &self.AHNHNPOCNDJ)?;
        state.serialize_field("OBNPIDPHFDE", &self.OBNPIDPHFDE)?;
        state.serialize_field("EKFIDPFOILC", &self.EKFIDPFOILC)?;
        state.serialize_field("NMKBJGEONOJ", &self.NMKBJGEONOJ)?;

        // Serialize raw pointers
        state.serialize_field("EDIDAHIELAG", &serialize_pointer(&self.EDIDAHIELAG))?;

        // Serialize arrays
        state.serialize_field("OKHBBILFBND", &self.OKHBBILFBND)?;
        state.serialize_field("LDJAAEOOOLC", &self.LDJAAEOOOLC)?;

        // Serialize booleans
        state.serialize_field("MHFEBJINMBP", &self.MHFEBJINMBP)?;
        state.serialize_field("AJENNABILJC", &self.AJENNABILJC)?;
        state.serialize_field("GJIMBAPCJLF", &self.GJIMBAPCJLF)?;

        // Serialize more raw pointers
        state.serialize_field("ODNBNHFLMCD", &serialize_pointer(&self.ODNBNHFLMCD))?;
        state.serialize_field("FGJEHAKCLNL", &serialize_pointer(&self.FGJEHAKCLNL))?;

        // Serialize the final boolean field
        state.serialize_field("KGKBLOJMDPH", &self.KGKBLOJMDPH)?;

        state.end()
    }
}

impl Serialize for GameComponentBase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::GameComponentBase");

        let mut state = serializer.serialize_struct("GameComponentBase", 2)?;

        // Serialize the `native_object` field
        state.serialize_field("native_object", &self.native_object)?;

        // Serialize the `_OwnerRef` pointer
        //To dodge the circular reference, we serialize the reference as string
        state.serialize_field("_OwnerRef", &format!("{}", self._OwnerRef as u64))?;

        state.end()
    }
}

impl Serialize for TurnBasedGameMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::TurnBasedGameMode");
        let mut state = serializer.serialize_struct("TurnBasedGameMode", 10)?;

        // Example: Serializing specific fields
        // For `native_object`, directly serialize it (assuming it implements Serialize)
        state.serialize_field("native_object", &self.native_object)?;

        // For `_SwordTrainingMgr`, serialize the address as a hexadecimal string
        state.serialize_field("_SwordTrainingMgr", &format!("{:?}", self._SwordTrainingMgr))?;

        // For `_ActionDelayLinkMgr`, skip serialization (example choice)
        // state.skip_field("_ActionDelayLinkMgr")?; // Alternative if skipping logic implemented

        // Serialize `_ActionDelayChangeStamp` as a normal array
        state.serialize_field("_ActionDelayChangeStamp", &self._ActionDelayChangeStamp)?;

        // Example of serializing a `bool` directly
        state.serialize_field("WinFlag", &self.WinFlag)?;

        // Example of a `u32` field
        state.serialize_field("CurrentWaveIndexInStage__BackingField", &self.CurrentWaveIndexInStage__BackingField)?;

        // Add other fields you want to serialize similarly...
//TODO improve Serialization
        /*

                    pub native_object: NativeObject,
            pub _SwordTrainingMgr: *const c_void, // 0x10
            pub CurrentMVPEntity__BackingField: *const GameEntity, // 0x18
            pub BattleAIPublicKnowledge__BackingField: *const c_void, // 0x20
            pub CurrentWaveMainMonsterIDPool__BackingField: *const NativeArray<u32>, // 0x28
            pub _ActionDelayLinkMgr: *const c_void, // 0x30
            pub _PhaseModifierList: *const NativeArray<NativeObject>, // 0x38
            pub _LinkTeammateList: *const NativeArray<GameEntity>, // 0x40
            pub _ActionEntityList: *const NativeArray<GameEntity>, // 0x48
            pub StageBattleEventMgr__BackingField: *const c_void, // 0x50
            pub LastKillTargetList__BackingField: *const NativeArray<GameEntity>, // 0x58
            pub _ActionDelayChangeStamp: [u8; 0x18], // 0x60
            pub _ModifierPerformCamerContext: *const c_void, // 0x78
            pub LastSummonMonsterList: *const NativeArray<GameEntity>, // 0x80
            pub _ImmediateActionEntities: *const c_void, // 0x88
            pub _ActionEntityListSnapshot: *const NativeArray<GameEntity>, // 0x90
            pub BattleCounter: *const c_void,     // 0x98
            pub _LimboEntitiesSkipSettlement: *const NativeArray<NativeObject>, // 0xa0
            pub LastZombie__BackingField: *const GameEntity, // 0xa8
            pub _AllOffTeamCharacters: *const NativeArray<GameEntity>, // 0xb0
            pub _CurModifierPerformSeq: *const c_void, // 0xb8
            pub _ReplayData: *const c_void,       // 0xc0
            pub _RelationGroupMgr: *const c_void, // 0xc8
            pub _CurrentTurnTargetEntity: *const GameEntity, // 0xd0
            pub _CommonSkillPoolNames: *const c_void, // 0xd8
            pub _CachedDynamicSkillTargetSelection: *const GameEntity, // 0xe0
            pub BattleEventInitedData__BackingField: *const c_void, // 0xe8
            pub _OverrieWaveMonsterPerformDatas: *const NativeArray<NativeObject>, // 0xf0
            pub TurnActionDelayCostChangeSource__BackingField: *const GameEntity, // 0xf8
            pub _SkillAddBuffPerformList: *const NativeArray<NativeObject>, // 0x100
            pub _TurnStateFSM: *const c_void,     // 0x108
            pub ActionBarMgr__BackingField: *const c_void, // 0x110
            pub MonsterWaveTextInfo: *const c_void, // 0x118
            pub _LimboRevivableEntities: *const c_void, // 0x120
            pub LastTurnSnapshot: *const c_void,  // 0x128
            pub AssistantAvatarEntity__BackingField: *const GameEntity, // 0x130
            pub _InsertAbilityList: *const NativeArray<MMNDIEBMDNL>, // 0x138
            pub _VersusBarMgr: *const c_void,     // 0x140
            pub _CurrentActionDelayModifyGroup: *const NativeArray<GameEntity>, // 0x148
            pub _EntityCustomUnselectableDatas: *const NativeArray<NativeObject>, // 0x150
            pub _UnselectableEntities: *const NativeArray<GameEntity>, // 0x158
            pub PrepareAbility__BackingField: *const c_void, // 0x160
            pub BattleChangeAvatarManager__BackingField: *const c_void, // 0x168
            pub DamageQueue__BackingField: *const c_void, // 0x170
            pub _LastBreakMonster: *const GameEntity, // 0x178
            pub _performParam: *const c_void,     // 0x180
            pub _AvatarChangeParam: *const c_void, // 0x188
            pub ThisTurnAnimEvents: *const c_void, // 0x190
            pub _AttackingEntityList: *const c_void, // 0x198
            pub _AllTeamCharacters: *const NativeArray<GameEntity>, // 0x1a0
            pub _RogueInBattleData: *const c_void, // 0x1a8
            pub _EventProcessor: *const c_void,   // 0x1b0
            pub CurrentTurnOwnerEntity__BackingField: *const GameEntity, // 0x1b8
            pub LastKillCaster__BackingField: *const GameEntity, // 0x1c0
            pub _WaitingAbilityList: *const NativeArray<NativeObject>, // 0x1c8
            pub OwnerBattleInstanceRef__BackingField: *const c_void, // 0x1d0
            pub _InsertUltraSkillParamsQueue: *const NativeArray<NativeObject>, // 0x1d8
            pub SkillUsageLog__BackingField: *const c_void, // 0x1e0
            pub _EvolveBuildGearMgr: *const c_void, // 0x1e8
            pub _LimboEntities: *const NativeArray<NativeObject>, // 0x1f0
            pub _MainMonster: *const GameEntity,  // 0x1f8
            pub GridFightMananger__BackingField: *const c_void, // 0x200
            pub TimeGameStart: *const c_void,     // 0x208
            pub _allowQuitStates: *const NativeArray<NativeObject>, // 0x210
            pub _CurrentSkillCharacter: *const SkillCharacterComponent, // 0x218
            pub _LimboEntitiesWaitAbilityFinish: *const NativeArray<NativeObject>, // 0x220
            pub _ActionDelayOrderTrigger: *const c_void, // 0x228
            pub _EntityModifierPerforms: *const c_void, // 0x230
            pub _CurrentTurnActionEntity: *const GameEntity, // 0x238
            pub PerformDelayExecuteList: *const NativeArray<NativeObject>, // 0x240
            pub LastKillSkill__BackingField: *const c_void, // 0x248
            pub _SomatoModifierPerforms: *const NativeArray<NativeObject>, // 0x250
            pub _LevelLockedFeatureSet: *const c_void, // 0x258
            pub _AidDetail: *const c_void,        // 0x260
            pub _HoldFrameForCapture: u32,        // 0x268
            pub _DarkTeamTurnCount: u32,          // 0x26c
            pub CurrentWaveIndexInStage__BackingField: u32, // 0x270
            pub _ModifierEndingPerformRemainedTime: f32, // 0x274
            pub StanceCountDownSPChangeValue__BackingField: f32, // 0x278
            pub _PrevTickModeState: i32,          // 0x27c
            pub BattleResultState__BackingField: *const c_void, // 0x280
            pub _DamageCounter: u32,              // 0x288
            pub ChallengeTurnLimit__BackingField: u32, // 0x28c
            pub MuteLastKillTriggered: bool,      // 0x290
            pub IsActionOrder1UsedTBSkill__BackingField: bool, // 0x291
            pub ApplyUIOperateOnDisableActionFlagChange: bool, // 0x292
            pub BattleFinishReason: i32,          // 0x294
            pub CurrentWaveStageID__BackingField: u32, // 0x298
            pub SkipTurnOwnerActionFlag__BackingField: bool, // 0x29c
            pub _IsCreatingNewWave: bool,         // 0x29d
            pub WinFlag: bool,                    // 0x29e
            pub _OverrideAILocked: bool,          // 0x29f
            pub ThisTurnAnimEventCount: i32,      // 0x2a0
            pub WaveMonsterMaxCount__BackingField: i32, // 0x2a4
            pub CertainlyWinInAdvance__BackingField: bool, // 0x2a8
            pub ForbidAI: bool,                   // 0x2a9
            pub TurnOwnerPrepareAbilityUsed__BackingField: bool, // 0x2aa
            pub _IsReplayBeingSaved: bool,        // 0x2ab
            pub _NextModifierIndex: i32,          // 0x2ac
            pub UseSkillOneMoreDefaultSkill: i32, // 0x2b0
            pub _CurrentTurnTeam: TeamType,       // 0x2b4
            pub _ModifierPerformTimeTotal: f32,   // 0x2b8
            pub SkipDeathHandle__BackingField: bool, // 0x2bc
            pub _GamePauseFlag: bool,             // 0x2bd
            pub LocalWinFlag__BackingField: [u8; 0x2], // 0x2be
            pub IsTeamFormationExpansion__BackingField: bool, // 0x2c0
            pub _ModifierPerformTimerTotal: f32,  // 0x2c4
            pub _WaveMonsterCurrentCount: i32,    // 0x2c8
            pub _DeathVersion: u32,               // 0x2cc
            pub IsManualRetryExitBattle: bool,    // 0x2d0
            pub PrepareAbilityFinish__BackingField: bool, // 0x2d1
            pub CertainlyLoseInAdvance__BackingField: bool, // 0x2d2
            pub _ActionEntityListInited: bool,    // 0x2d3
            pub PauseState__BackingField: i32,    // 0x2d4
            pub TurnActionDelayCostRatio__BackingField: FixPoint, // 0x2d8
            pub _TurnCounter: u32,                // 0x2e0
            pub _SkillExecutionEventState: i32,   // 0x2e4
            pub ShowCutinUIState__BackingField: i32, // 0x2e8
            pub PropagateBeingAttackTeam__BackingField: TeamType, // 0x2ec
            pub _RecordOperationByLG: *const c_void, // 0x2f0
            pub _NextAbilityIndex: i32,           // 0x2f8
            pub _OperationCounter: u32,           // 0x2fc
            pub ChallengeTurnLimitType__BackingField: i32, // 0x300
            pub IsUseSkillOneMore: bool,          // 0x304
            pub _IsModifierPerformCameraSet: bool, // 0x305
            pub LastKillFinish__BackingField: bool, // 0x306
            pub BattleResultAsWin: bool,          // 0x307
            pub _ModifierPerformTimeScale: f32,   // 0x308
            pub ClearUltraSkillQueue__BackingField: bool, // 0x30c
            pub CurrentInsertSkillSkipActionFlag: bool, // 0x30d
            pub _CurrentTurnActionEntitySkipActionFlag: bool, // 0x30e
            pub IsLastKillTriggered: bool,        // 0x30f
            pub AutoInsertUltraSkill: bool,       // 0x310
            pub ApplyUIOperateOnSkillDisableChange: bool, // 0x311
            pub ClearUltraSkillEffect: bool,      // 0x312
            pub _RequireMakeLimboEntitiesDie: bool, // 0x313
            pub RealTimeCounter__BackingField: f32, // 0x314
            pub _CachedDynamicSkillInput: i32,    // 0x318
            pub _HitPerformMinTimer: f32,         // 0x31c
            pub _ChallengeTurnAcc: u32,           // 0x320
            pub _LightTeamTurnCount: u32,         // 0x324
            pub ElapsedActionDelay__BackingField: FixPoint, // 0x328
            pub SkipCameraDitherByLastKill: bool, // 0x330
            pub _AutoBattle: bool,                // 0x331
            pub TurnOwnerActionPhaseEnd__BackingField: bool, // 0x332
            pub _IsUseLinkSkill: bool,            // 0x333
            pub IsManualExitBattle: bool,         // 0x334
            pub _LastReplayAutoBattle: bool,      // 0x335
            pub TurnEndKeep: bool,                // 0x336
            pub _HoldFrameForCaptureFlag: bool,   // 0x337
            pub CurrentModeState__BackingField: i32, // 0x338
            pub AddOpCountOnInsertUltraWaitOrder: bool, // 0x33c
            pub PendingMonsterToWave__BackingField: bool, // 0x33d


         */

        state.end()
    }
}

impl Serialize for BattleLineupData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("serialize::BattleLineupData");

        let self_ptr = ((self as *const BattleLineupData) as u64);
        let int_ptr = self_ptr as *const u32;
        unsafe {
            if !int_ptr.is_null() {
                // Safety: Ensure the pointer is valid for at least `n` bytes.
                let mem_slice = std::slice::from_raw_parts(int_ptr, 6);
                log::info!("BattleLineupData Memory u32 slice content: {:?}", mem_slice);
            }
        }

        let mut state = serializer.serialize_struct("BattleLineupData", 13)?;

        unsafe {
            // Serialize fields
            state.serialize_field("native_object", &self.native_object)?;
    
            if self.ExtraTeam.is_null(){ state.serialize_field("ExtraTeam", "null") ?;}
            else { state.serialize_field("ExtraTeam", &*self.ExtraTeam)?; }

            if self.TeamBuffIDList.is_null(){ state.serialize_field("TeamBuffIDList", "null") ?;}
            else {
                //log::info!("BattleLineupData::TeamBuffIDList");
                state.serialize_field(
                    "TeamBuffIDList", &*self.TeamBuffIDList
                )?;
            }
            if self.MazeBuffAdded.is_null(){ state.serialize_field("MazeBuffAdded", "null") ?;}
            else {
                log::info!("BattleLineupData::MazeBuffAdded");
                state.serialize_field("MazeBuffAdded", &*self.MazeBuffAdded)?;
                log::info!("BattleLineupData::MazeBuffAdded done");
            }

            if self.SpecialAvatarLevelAreaConfigs.is_null(){ state.serialize_field("SpecialAvatarLevelAreaConfigs", "null") ?;}
            else {
                log::info!("BattleLineupData::SpecialAvatarLevelAreaConfigs");
                state.serialize_field("SpecialAvatarLevelAreaConfigs", &*self.SpecialAvatarLevelAreaConfigs)?;
            }

            if self._TemplateVariables.is_null(){ state.serialize_field("_TemplateVariables", "null") ?;}
            else {
                state.serialize_field(
                    "_TemplateVariables",
                    &serialize_pointer(&self._TemplateVariables)
                )?;
            }

            if self.LightTeam.is_null(){ state.serialize_field("LightTeam", "null") ?;}
            else {
                state.serialize_field(
                    "LightTeam",
                    &*self.LightTeam
                )?;
            }

            if self.Context.is_null(){ state.serialize_field("Context", "null") ?;}
            else {
                state.serialize_field(
                    "Context",
                    &serialize_pointer(&self.Context),
                )?;
            }

            if self.BattleExtraPropertyAdditionDict__BackingField.is_null(){ state.serialize_field("BattleExtraPropertyAdditionDict__BackingField", "null") ?;}
            else {
                state.serialize_field(
                    "BattleExtraPropertyAdditionDict__BackingField",
                    &serialize_pointer(&self.BattleExtraPropertyAdditionDict__BackingField),
                )?;
            }

            if self.AdditionalTemplateVariables.is_null(){ state.serialize_field("AdditionalTemplateVariables", "null") ?;}
            else {
                state.serialize_field(
                    "AdditionalTemplateVariables",
                    &serialize_pointer(&self.AdditionalTemplateVariables)
                )?;
            }

            if self.DeferCreateTrialPlayerDic.is_null(){ state.serialize_field("DeferCreateTrialPlayerDic", "null") ?;}
            else {
                state.serialize_field(
                    "DeferCreateTrialPlayerDic",
                    &serialize_pointer(&self.DeferCreateTrialPlayerDic)
                )?;
            }

            if self._LevelPath.is_null(){ state.serialize_field("_LevelPath", "null") ?;}
            else {
                state.serialize_field(
                    "_LevelPath",
                    &*self._LevelPath
                )?;
            }

            // state.serialize_field("s_TeamBoostSkillNumber",
            //                       &*((((self as *const BattleLineupData) as u64) + 0xb6a0) as *const i32))?;
            // state.serialize_field("s_TeamDefaultCharacterCount",
            //                       &*((((self as *const BattleLineupData) as u64) + 0xb6a4) as *const i32))?;
            // state.serialize_field("s_IsSkipBattlePerformance",
            //                       &*((((self as *const BattleLineupData) as u64) + 0xb6a8) as *const bool))?;
            // state.serialize_field("s_IsMonsterDontLoad",
            //                       &*((((self as *const BattleLineupData) as u64) + 0xb6a9) as *const bool))?;
            // state.serialize_field("s_IsPlayerDontLoad",
            //                       &*((((self as *const BattleLineupData) as u64) + 0xb6aa) as *const bool))?;

            /*public static int s_TeamBoostSkillNumber; // 0xb6a0
            public static int s_TeamDefaultCharacterCount; // 0xb6a4
            public static bool s_IsSkipBattlePerformance; // 0xb6a8
            public static bool s_IsMonsterDontLoad; // 0xb6a9
            public static bool s_IsPlayerDontLoad; // 0xb6aa
            */

            state.serialize_field("WorldLevel", &self.WorldLevel)?;
        }
        state.end()
    }
}

impl Serialize for SpecialRelicData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Handle struct serialization with 4 fields
        let mut state = serializer.serialize_struct("SpecialRelicData", 4)?;
        //log::info!("serialize::SpecialRelicData");
        state.serialize_field("native_object", &self.native_object)?;

        unsafe{
            if self.LGBJKGGCELB.is_null(){ state.serialize_field("LGBJKGGCELB", "null")?;}
            else {
                //log::info!("SpecialRelicData::LGBJKGGCELB");
                state.serialize_field("LGBJKGGCELB", &*self.LGBJKGGCELB)?;
            }
        }
        state.serialize_field("POFMKDABEHD", &self.POFMKDABEHD)?;
        state.serialize_field("JGJCDMJIMNN", &self.JGJCDMJIMNN)?;

        state.end()
    }
}

impl Serialize for BattleRelicModule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize 7 fields in the struct
        let mut state = serializer.serialize_struct("BattleRelicModule", 7)?;

        /* logging
        let ser_id = Uuid::new_v4();
        log::info!("serialize::BattleRelicModule {:p} {}", self, ser_id);

        let self_ptr = ((self as *const BattleRelicModule) as u64);
        let int_ptr = self_ptr as *const u32;
        unsafe {
            if !int_ptr.is_null() {
                // Safety: Ensure the pointer is valid for at least `n` bytes.
                let mem_slice = std::slice::from_raw_parts(int_ptr, 64);
                log::info!("Memory u32 slice content: {:?} {}", mem_slice, ser_id);
            }
        }
        let long_ptr = self_ptr as *const u64;
        unsafe {
            if !long_ptr.is_null() {
                // Safety: Ensure the pointer is valid for at least `n` bytes.
                let mem_slice = std::slice::from_raw_parts(long_ptr, 32);
                log::info!("Memory u64 slice content: {:?} {}", mem_slice, ser_id);
            }
        }
        */

        // Serialize native_object
        state.serialize_field("native_object", &self.native_object)?;

        unsafe {
        // Safely serialize each pointer field
            state.serialize_field("AAEONBIGBBP", &serialize_pointer(&self.AAEONBIGBBP))?;

            if self.BKCGOLIBNHC.is_null(){ state.serialize_field("BKCGOLIBNH", "null") ?;}
            unsafe {
                log::info!("BattleRelicModule::BKCGOLIBNHC {:p}", self.BKCGOLIBNHC);
                state.serialize_field("BKCGOLIBNHC", &*self.BKCGOLIBNHC)?;
            }

            if self.BattleRelicInfos.is_null() { state.serialize_field("BattleRelicInfos", "null")?; }
            else {
                state.serialize_field("BattleRelicInfos", &*self.BattleRelicInfos)?;
            }

            if self.PMMGFOHHKPM.is_null() { state.serialize_field("PMMGFOHHKPM", "null")?; }
            else {
                log::info!("BattleRelicModule::PMMGFOHHKPM {:p}", self.PMMGFOHHKPM);
                state.serialize_field("PMMGFOHHKPM", &*self.PMMGFOHHKPM)?;
            }
            if self.BIJMJNIMPOM.is_null() { state.serialize_field("BIJMJNIMPO", "null")?; }
            else { state.serialize_field("BIJMJNIMPOM", &*self.BIJMJNIMPOM)?; }
        }
        // Serialize SpecialRelicData if not null
        let special_relic_data = unsafe {
            if !self.SpecialRelicData.is_null() {
                Some(*self.SpecialRelicData)
            } else {
                None
            }
        };
        state.serialize_field("SpecialRelicData", &special_relic_data)?;

        state.end()
    }
}

impl Serialize for BattleRelicInfo_struct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Define struct serialization with 5 fields
        let mut state = serializer.serialize_struct("BattleRelicInfo", 5)?;
        //log::info!("serialize::BattleRelicInfo");

        // Serialize the native object
        //state.serialize_field("native_object", &self.native_object)?;
        state.serialize_field("IGIDDGDHAGI", &self.IGIDDGDHAGI)?;
        state.serialize_field("LightConeId", &self.LightConeId)?;
        state.serialize_field("FFPKKKEBDHL", &self.FFPKKKEBDHL)?;

        // Serialize the pointer to NativeArray<NCGNFPLFBOJ>

        if self.BNDGBHLOJHN.is_null() { state.serialize_field("BNDGBHLOJHN", "null")?; }
        else {
            unsafe { state.serialize_field("BNDGBHLOJHN", &*self.BNDGBHLOJHN)?; }
        }

        state.end()
    }
}

impl Serialize for NCGNFPLFBOJ_struct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Start serializing the struct with 4 fields
        let mut state = serializer.serialize_struct("NCGNFPLFBOJ", 4)?;
        //log::info!("serialize::NCGNFPLFBOJ");

        // Serialize each field individually
        //state.serialize_field("native_object", &self.native_object)?;
        state.serialize_field("NIKFINDKDKO", &self.NIKFINDKDKO)?;
        state.serialize_field("KBMCHLGDKEF", &self.KBMCHLGDKEF)?;
        state.serialize_field("KHADHNNCFLH", &self.KHADHNNCFLH)?;

        // End the serialization
        state.end()
    }
}

impl Serialize for FDPIKJAAKAH {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FDPIKJAAKAH", 3)?;

        state.serialize_field("BLIHJMNHCEA",&serialize_pointer(&self.BLIHJMNHCEA))?; // Serialize the pointer as a string
        state.serialize_field("LCEKDADJBDE", &self.LCEKDADJBDE)?;
        state.serialize_field("FLJPKFJAJFP", &self.FLJPKFJAJFP)?;
        state.end()
    }
}
impl Serialize for MazeBuffData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("MazeBuffData", 6)?;

        // Serialize target_index_list
        if !self.target_index_list.is_null() {
            unsafe { state.serialize_field("target_index_list", &*self.target_index_list)?; }
        } else {
            state.serialize_field("target_index_list", &None::<Vec<u32>>)?;
        }

        // Serialize extra_param_map
        if self.extra_param_map.is_null() { state.serialize_field("extra_param_map", "null")?; }
        else { unsafe {
            log::info!("MazeBuffData::extra_param_map");
            state.serialize_field("extra_param_map", &*self.extra_param_map)?; }
        }

        // Serialize other fields
        state.serialize_field("active_wave_flags", &self.active_wave_flags)?;
        state.serialize_field("owner_character_index", &self.owner_character_index)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("level", &self.level)?;

        state.end()
    }
}

impl<K, V> Serialize for NativeDictionaryValueEntry<K, V>
where
    K: Serialize + std::fmt::Debug,
    V: Serialize + std::fmt::Debug,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //log::info!("{} {} {}", std::any::type_name::<K>(), std::any::type_name::<V>(), size_of::<NativeDictionaryValueEntry<K,V>>());
        //log::info!("NativeDictionaryValueEntry pointer: {:p} {:?} {:?} {} {} {}", self, self.key, self.value, size_of::<NativeDictionaryValueEntry<K,V>>(), std::any::type_name::<K>(), size_of::<V>(),);

        // let self_ptr = ((self as *const NativeDictionaryValueEntry<K, V>) as u64);
        // let int_ptr = self_ptr as *const u32;
        // unsafe {
        //     if !int_ptr.is_null() {
        //         // Safety: Ensure the pointer is valid for at least `n` bytes.
        //         let mem_slice = std::slice::from_raw_parts(int_ptr, size_of::<NativeDictionaryValueEntry<K, V>>()/4);
        //         log::info!("Memory u32 slice content: {:?}", mem_slice);
        //     }
        // }

        let mut state = serializer.serialize_struct("NativeDictionaryValueEntry", 4)?;
        //state.serialize_field("hash_code", &self.hash_code)?; //state.serialize_field("next", &self.next)?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("value", &self.value)?;

        state.end()
    }
}

impl Serialize for NOPBAAAGGLA {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("NOPBAAAGGLA", 3)?;

        //TODO serialize more

        // Example of serializing a native object (assuming NativeObject implements Serialize)
        state.serialize_field("native_object", &self.native_object)?;

        // Example of converting pointers to numbers (addresses or null)
        state.serialize_field("HKFGOHGKOGK", &serialize_pointer(&self.HKFGOHGKOGK))?;

        if self.JKCOIOLCMEP.is_null() { state.serialize_field("JKCOIOLCMEP", "null")?; }
        {
            unsafe {
                state.serialize_field("JKCOIOLCMEP", &*self.JKCOIOLCMEP)?;
            }
        }

        // Example of serializing arrays (turn to Vec<u8>)
        state.serialize_field("AAHMMHBHMFN", &self.AAHMMHBHMFN.to_vec())?;
        state.serialize_field("FFFOLNDHIEH", &self.FFFOLNDHIEH.to_vec())?;

        // Serialize FixPoint fields using Debug or other custom logic
        state.serialize_field("NAGMKEABGEE", &self.NAGMKEABGEE)?;
        state.serialize_field("KLMAGCLFBAO", &self.KLMAGCLFBAO)?;
        state.serialize_field("JFKEEOMKMLI", &self.JFKEEOMKMLI)?;


        // Serialize other fields as normal
        state.serialize_field("COKMLMJPKLH", &self.COKMLMJPKLH)?;
        state.serialize_field("BBDANLEJCIA", &self.BBDANLEJCIA)?;
        state.serialize_field("HEMFDDDJOGK", &self.HEMFDDDJOGK)?;

        // Finish serialization
        state.end()
    }
}