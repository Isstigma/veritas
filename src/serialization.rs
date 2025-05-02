use crate::kreide::helpers::fixpoint_to_raw;
use crate::kreide::native_types::{NativeArray, NativeObject, NativeString};
use crate::kreide::types::rpg::client::*;
use crate::kreide::types::rpg::gamecore::*;
use crate::kreide::types::{MMNDIEBMDNL, OLHMAHMMBNN};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::backtrace::Backtrace;
use std::{ffi, slice};
use std::ffi::c_void;

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

/// Serialize a raw pointer to `*const CharacterConfig` as its memory address (`u64`).
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


        if self.PreshowConditions.is_null() { state.serialize_field("PreshowConditions", "null")?; }
        else {state.serialize_field("PreshowConditions", &*self.PreshowConditions)?; }

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

            if self.CustomReadyConfigConditions.is_null() { state.serialize_field("CustomReadyConfigConditions", "null")?; }
            else {state.serialize_field("CustomReadyConfigConditions", &*self.CustomReadyConfigConditions)?; }

            state.serialize_field("_Slot", &serialize_pointer(&self._Slot))?;
            state.serialize_field("UsableCondTask", &serialize_pointer(&self.UsableCondTask))?;
            state.serialize_field("InsertCondTask", &serialize_pointer(&self.InsertCondTask))?;
            state.serialize_field("OverrideCameraConfig", &serialize_pointer(&self.OverrideCameraConfig))?;
            state.serialize_field("OverrideCameraConfigAdded", &serialize_pointer(&self.OverrideCameraConfigAdded))?;


            if self.ParentSkillData.is_null() { state.serialize_field("ParentSkillData", "null")?; }
            else {state.serialize_field("ParentSkillData", &*self.ParentSkillData)?;
            }

            if self._SkillProperties.is_null() { state.serialize_field("_SkillProperties", "null")?; }
            else {    state.serialize_field("_SkillProperties", &*self._SkillProperties)?;
            }

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


        if self.CameraConfigList.is_null() { state.serialize_field("CameraConfigList", "null")?; }
        else {state.serialize_field("CameraConfigList", &*self.CameraConfigList)?; }

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

            if self.SkillList.is_null() { state.serialize_field("SkillList", "null")?; }
            else {    state.serialize_field("SkillList", &*self.SkillList)?;
            }

            if self.AbilityList.is_null() { state.serialize_field("AbilityList", "null")?; }
            else {state.serialize_field("AbilityList", &*self.AbilityList)?;
            }

            if self.SkillAbilityList.is_null() { state.serialize_field("SkillAbilityList", "null")?; }
            else {state.serialize_field("SkillAbilityList", &*self.SkillAbilityList)?; }

            state.serialize_field("DynamicValues", &serialize_pointer(&self.DynamicValues))?;
            state.serialize_field("CustomValues", &serialize_pointer(&self.CustomValues))?;
            state.serialize_field("WeaponType", &self.WeaponType)?;
            state.serialize_field("ArmorType", &self.ArmorType)?;


        if self.SkillReadyTransits.is_null() { state.serialize_field("SkillReadyTransits", "null")?; }
        else {state.serialize_field("SkillReadyTransits", &*self.SkillReadyTransits)?; }

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


        if self.PhaseList.is_null() { state.serialize_field("PhaseList", "null")?; }
        else {state.serialize_field("PhaseList", &*self.PhaseList)?;
        }

            if self.OverrideWaveMonsterPerform.is_null() { state.serialize_field("OverrideWaveMonsterPerform", "null")?; }
            else {state.serialize_field("OverrideWaveMonsterPerform", &*self.OverrideWaveMonsterPerform)?;
            }

            if self.OverrideColliderCameraByName.is_null() { state.serialize_field("OverrideColliderCameraByName", "null")?; }
            else {state.serialize_field("OverrideColliderCameraByName", &*self.OverrideColliderCameraByName)?;
            }

            state.serialize_field("EntityColliderConfig", &serialize_pointer(&self.EntityColliderConfig))?;


            if self.EffectAdaptionList.is_null() { state.serialize_field("EffectAdaptionList", "null")?; }
            else {state.serialize_field("EffectAdaptionList", &*self.EffectAdaptionList)?;
            }

            if self.AttachPointEffectAdaptionList.is_null() { state.serialize_field("AttachPointEffectAdaptionList", "null")?; }
            else {state.serialize_field("AttachPointEffectAdaptionList", &*self.AttachPointEffectAdaptionList)?;
            }

            if self.FieldEffectAdaptionList.is_null() { state.serialize_field("FieldEffectAdaptionList", "null")?; }
            else {state.serialize_field("FieldEffectAdaptionList", &*self.FieldEffectAdaptionList)?; }
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

            if self._DummpyEntityList.is_null() { state.serialize_field("_DummpyEntityList", "null")?; }
            else {state.serialize_field("_DummpyEntityList", &*self._DummpyEntityList)?;}

            state.serialize_field("_RowData", &serialize_pointer(&self._RowData))?;
        

            if self._DynamicScaleAdaptConfigs.is_null() { state.serialize_field("_DynamicScaleAdaptConfigs", "null")?; }
            else {state.serialize_field("_DynamicScaleAdaptConfigs", &*self._DynamicScaleAdaptConfigs)?; }

            state.serialize_field("_DynamicScaleAdaptEffectPathRule", &serialize_pointer(&self._DynamicScaleAdaptEffectPathRule))?;
        

            if self._DynamicScaleAdaptTypes.is_null() { state.serialize_field("_DynamicScaleAdaptTypes", "null")?; }
            else {state.serialize_field("_DynamicScaleAdaptTypes", &*self._DynamicScaleAdaptTypes)?;}

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


            if self.OnAbilityPropertyChanged.is_null() { state.serialize_field("OnAbilityPropertyChanged", "null")?; }
            else {state.serialize_field("OnAbilityPropertyChanged", &*self.OnAbilityPropertyChanged)?;
            }

            if self._BuffLockStepSources.is_null() { state.serialize_field("_BuffLockStepSources", "null")?; }
            else {state.serialize_field("_BuffLockStepSources", &*self._BuffLockStepSources)?;
            }

            if self._ExtraMaxLayerConfig.is_null() { state.serialize_field("_ExtraMaxLayerConfig", "null")?; }
            else {state.serialize_field("_ExtraMaxLayerConfig", &*self._ExtraMaxLayerConfig)?;
            }

            if self._CharacterDataRef.is_null() { state.serialize_field("_CharacterDataRef", "null")?; }
            else {state.serialize_field("_CharacterDataRef", &*self._CharacterDataRef)?;
            }

            if self.AdditionalAbilityParamList.is_null() { state.serialize_field("AdditionalAbilityParamList", "null")?; }
            else {state.serialize_field("AdditionalAbilityParamList", &*self.AdditionalAbilityParamList)?;
            }

            if self._SelfExtrAbilityList.is_null() { state.serialize_field("_SelfExtrAbilityList", "null")?; }
            else {state.serialize_field("_SelfExtrAbilityList", &*self._SelfExtrAbilityList)?;
            }

            state.serialize_field("Weakness", &serialize_pointer(&self.Weakness))?;


            if self._AbilityPropertiesInitSnapshot.is_null() { state.serialize_field("_AbilityPropertiesInitSnapshot", "null")?; }
            else {state.serialize_field("_AbilityPropertiesInitSnapshot", &*self._AbilityPropertiesInitSnapshot)?;
            }

            if self.RegardAsAttackTypeMap.is_null() { state.serialize_field("RegardAsAttackTypeMap", "null")?; }
            else {state.serialize_field("RegardAsAttackTypeMap", &*self.RegardAsAttackTypeMap)?;
            }

            if self._KillerEntity.is_null() { state.serialize_field("_KillerEntity", "null")?; }
            else {state.serialize_field("_KillerEntity", &*self._KillerEntity)?;
            }

            if self._DebuffLockStepSources.is_null() { state.serialize_field("_DebuffLockStepSources", "null")?; }
            else {state.serialize_field("_DebuffLockStepSources", &*self._DebuffLockStepSources)?;
            }

            if self.RegardAsSkillTypeMap.is_null() { state.serialize_field("RegardAsSkillTypeMap", "null")?; }
            else {state.serialize_field("RegardAsSkillTypeMap", &*self.RegardAsSkillTypeMap)?;
            }

            if self.ProjectileTargetAttachPoint.is_null() { state.serialize_field("ProjectileTargetAttachPoint", "null")?; }
            else {state.serialize_field("ProjectileTargetAttachPoint", &*self.ProjectileTargetAttachPoint)?;
            }

            if self._DotModifierEventProcessors.is_null() { state.serialize_field("_DotModifierEventProcessors", "null")?; }
            else {state.serialize_field("_DotModifierEventProcessors", &*self._DotModifierEventProcessors)?;
            }

            state.serialize_field("_DmgChunk", &serialize_pointer(&self._DmgChunk))?;

            if self.CharmDamageTarget.is_null() { state.serialize_field("CharmDamageTarget", "null")?; }
            else { state.serialize_field("CharmDamageTarget", &*self.CharmDamageTarget)?; }

            state.serialize_field("_AbilityToSkillMapping", &serialize_pointer(&self._AbilityToSkillMapping))?;
            state.serialize_field("ModifierOverrideMapping", &serialize_pointer(&self.ModifierOverrideMapping))?;


            if self._EnergyPointEntries.is_null() { state.serialize_field("_EnergyPointEntries", "null")?; }
            else {state.serialize_field("_EnergyPointEntries", &*self._EnergyPointEntries)?; }

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

            if self._OnHitEffectMultipleOverride.is_null() { state.serialize_field("_OnHitEffectMultipleOverride", "null")?; }
            else {
                state.serialize_field("_OnHitEffectMultipleOverride", &*self._OnHitEffectMultipleOverride)?;
            }

            if self._DamageStoreList.is_null() { state.serialize_field("_DamageStoreList", "null")?; }
            else {
                state.serialize_field("_DamageStoreList", &*self._DamageStoreList)?;
            }

            if self._JsonConfigRef.is_null() { state.serialize_field("_JsonConfigRef", "null")?; }
            else {
                state.serialize_field("_JsonConfigRef", &*self._JsonConfigRef)?;
            }

            if self.DamageSplitData.is_null() { state.serialize_field("DamageSplitData", "null")?; }
            else {
                state.serialize_field("DamageSplitData", &*self.DamageSplitData)?;
            }

            if self._StancePreshowConfigs.is_null() { state.serialize_field("_StancePreshowConfigs", "null")?; }
            else {
                state.serialize_field("_StancePreshowConfigs", &*self._StancePreshowConfigs)?;
            }

            if self._EnableNegativeHPSourceList.is_null() { state.serialize_field("_EnableNegativeHPSourceList", "null")?; }
            else {
                state.serialize_field("_EnableNegativeHPSourceList", &*self._EnableNegativeHPSourceList)?;
            }

            if self._ModifierEventSourceMuteCounter.is_null() { state.serialize_field("_ModifierEventSourceMuteCounter", "null")?; }
            else {
                state.serialize_field("_ModifierEventSourceMuteCounter", &serialize_pointer(&self._ModifierEventSourceMuteCounter))?;
            }

            if self._LockHPList.is_null() { state.serialize_field("_LockHPList", "null")?; }
            else {
                state.serialize_field("_LockHPList", &*self._LockHPList)?;
            }

            if self._RedStanceInfoList.is_null() { state.serialize_field("_RedStanceInfoList", "null")?; }
            else {
                state.serialize_field("_RedStanceInfoList", &*self._RedStanceInfoList)?;
            }

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

            if self._OnHitEffectOverride.is_null() { state.serialize_field("_OnHitEffectOverride", "null")?; }
            else {
                state.serialize_field("_OnHitEffectOverride", &*self._OnHitEffectOverride)?;
            }

            if self.DamageDefender.is_null() { state.serialize_field("DamageDefender", "null")?; }
            else {
            state.serialize_field("DamageDefender", &*self.DamageDefender)?;}

            if self._AbilityProperties.is_null() { state.serialize_field("_AbilityProperties", "null")?; }
            else { state.serialize_field("_AbilityProperties", &*self._AbilityProperties)?; }

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

            if self._ModifierEventProcessors.is_null() { state.serialize_field("_ModifierEventProcessors", "null")?; }
            else { state.serialize_field("_ModifierEventProcessors", &*self._ModifierEventProcessors)?; }

            if self.OverflowStanceDamageAttacker__BackingField.is_null() { state.serialize_field("OverflowStanceDamageAttacker__BackingField", "null")?; }
            else { state.serialize_field("OverflowStanceDamageAttacker__BackingField", &*self.OverflowStanceDamageAttacker__BackingField)?; }

            state.serialize_field("_TransformRef", &serialize_pointer(&self._TransformRef))?;
            state.serialize_field("_StatusProbabilityDict", &serialize_pointer(&self._StatusProbabilityDict))?;

            if self.ResistModifierBehaviorFlags__BackingField.is_null() { state.serialize_field("ResistModifierBehaviorFlags__BackingField", "null")?; }
            else { state.serialize_field("ResistModifierBehaviorFlags__BackingField", &*self.ResistModifierBehaviorFlags__BackingField)?; }

            if self._DepartedParams.is_null() { state.serialize_field("_DepartedParams", "null")?; }
            else { state.serialize_field("_DepartedParams", &*self._DepartedParams)?; }

            state.serialize_field("_DelayModifyActionDelayQueue", &serialize_pointer(&self._DelayModifyActionDelayQueue))?;
            state.serialize_field("_LockShieldCounter", &serialize_pointer(&self._LockShieldCounter))?;
            state.serialize_field("_ModifierDelayParamList", &serialize_pointer(&self._ModifierDelayParamList))?;
            state.serialize_field("TotalDamageCurrentAttack", &fixpoint_to_raw(&self.TotalDamageCurrentAttack))?;
            state.serialize_field("BattleTag__BackingField", &self.BattleTag__BackingField)?;
            state.serialize_field("ForceKillFlag__BackingField", &self.ForceKillFlag__BackingField)?;
            state.serialize_field("ActionDelayChanged__BackingField", &self.ActionDelayChanged__BackingField)?;
            state.serialize_field("CharmDisableBPAdd", &self.CharmDisableBPAdd)?;
            state.serialize_field("bIsInCharmAction", &self.bIsInCharmAction)?;
            state.serialize_field("VisualFlagValue__BackingField", &self.VisualFlagValue__BackingField)?;
            state.serialize_field("_DeathVersion", &self._DeathVersion)?;
            state.serialize_field("TotalHitNum", &fixpoint_to_raw(&self.TotalHitNum))?;
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
            state.serialize_field("InheritSPRatio", &fixpoint_to_raw(&self.InheritSPRatio))?;
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
        let mut state = serializer.serialize_struct("SkillCharacterComponent", 40)?;
        unsafe {
            // Directly serialize `_parent_object`
            state.serialize_field("_parent_object", &self._parent_object)?;

            // Serialize custom fields with helpers for pointers and arrays
            if self._SkillDataList.is_null() { state.serialize_field("_SkillDataList", "null")?; }
            else {
                //log::info!("_SkillDataList");
                state.serialize_field("_SkillDataList", &*self._SkillDataList)?;
            }

            if self._CharacterDataRef.is_null() { state.serialize_field("_CharacterDataRef", "null")?; }
            else { state.serialize_field("_CharacterDataRef", &*self._CharacterDataRef)?; }

            if self._SkillTargetRedirectEntries.is_null() { state.serialize_field("_SkillTargetRedirectEntries", "null")?; }
            else {
                //log::info!("_SkillTargetRedirectEntries");
                state.serialize_field("_SkillTargetRedirectEntries", &*self._SkillTargetRedirectEntries)?;
            }

            if self._TBAbilityRef.is_null() { state.serialize_field("_TBAbilityRef", "null")?; }
            else { state.serialize_field("_TBAbilityRef", &*self._TBAbilityRef)?; }

            if self._SkillSlots.is_null() { state.serialize_field("_SkillSlots", "null")?; }
            else {
                //log::info!("_SkillSlots");
                state.serialize_field("_SkillSlots", &*self._SkillSlots)?;
            }

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

            if self.OnSkillSetup.is_null() { state.serialize_field("OnSkillSetup", "null")?; }
            else {
                //log::info!("OnSkillSetup");
                state.serialize_field("OnSkillSetup", &*self.OnSkillSetup)?;
            }

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
            else { state.serialize_field("_SkillTypeDisableCountArr", &*self._SkillTypeDisableCountArr)?; }

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
        else { unsafe { state.serialize_field("HasTakenPromotionRewardList__BackingField",
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
        else { unsafe { state.serialize_field("_SkinIDList", &*self._SkinIDList)?; } }

        state.serialize_field("SkillTreeData", &serialize_pointer(&self.SkillTreeData))?;
        state.serialize_field("SpecialRow__BackingField",
                              &serialize_pointer(&self.SpecialRow__BackingField))?;
        state.serialize_field("_AvatarRowData",
                              &serialize_pointer(&self._AvatarRowData))?;
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

        serializer.serialize_f64(fixpoint_to_raw(&self))
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
    T: Serialize + Clone
{
    ///Naming is actually incorrect - vector at the moment of writing is supposed to contain only the first item
    /// and 'bounds' contain something very questionable
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        if let "i32" | "u32" = std::any::type_name::<T>() {
            log::info!("serialize::NativeArray {} obj {} {}, bounds: {}, length {}, vector: {}, ptr {}",
                std::any::type_name::<T>(), self.obj.klass as u64, self.obj.monitor as u64,
                &(self.bounds as u64), &self.length, &(self.vector as u32), (self as *const NativeArray<T>) as u64);
            if self.bounds as u64 > 0x70000000000 && //0x70000000000 = 7696581394432u64
                (self.bounds as u64) < 8700100315968u64 {
                unsafe { log::info!("value at the ref u32 {} ", &*(self.bounds as *const u32 )); }
                unsafe { log::info!("value at the ref u64 {} ", &*(self.bounds as *const u64 )); }
            }
            if self.vector as u64 > 0x70000000000 && //0x70000000000 = 7696581394432u64
                (self.vector as u64) < 8700100315968u64 {
                unsafe { log::info!("value at the ref u32 {} ", &*(self.vector as *const u32 )); }
                unsafe { log::info!("value at the ref u64 {} ", &*(self.vector as *const u64 )); }
            }
            if self.bounds as u64 == 0 && (self.vector as u64) == 0 {
                unsafe { log::info!("value at the ref u32 {} ", &*((((self as *const NativeArray<T>) as u64)+0x8)as *const u64)); }
            }
        }
        let mut state = serializer.serialize_struct("NativeArray", (&self.length + 10) as usize)?;

        state.serialize_field("obj", &self.obj)?;
        state.serialize_field("length", &&self.length)?;

        state.serialize_field("bounds", &(self.bounds as usize))?;
        state.serialize_field("vector", &serialize_pointer(&(self.vector as *const c_void)))?;

        if(self.vector as u64) > 0 && (self.bounds as u64) > 0 {
            //from my understanding such thins shouldn't happen but if it does then I'd like to know where
            log::info!("serialize::NativeArray {} obj {} {}, bounds: {}, length {}, vector: {}, ptr {}",
                std::any::type_name::<T>(), self.obj.klass as u64, self.obj.monitor as u64,
                &(self.bounds as u64), &self.length, &(self.vector as u32), (self as *const NativeArray<T>) as u64);
        }

        if self.length > 0 {
            if (self.vector as u64) > 0 {
                unsafe {
                    let items = self.to_slice();
                    let mut index = 0;
                    //log::info!("retrieved items");
                    for item in items {
                        let field_name = format!("data_{}", &index);
                        //log::info!("item {}", *item as u64);
                        if (*item as u64) > 0x70000000000 && std::any::type_name::<T>() != "i32" //0x70000000000 = 7696581394432u64
                            && (*item as u64) < 8700100315968u64 //for some reason some NativeObjects have retarded addresses like 3175009970383523287 (mb collapsed 32bit values?) or 1900545 (latter was found in SkillCharacterComponent)
                        {
                            state.serialize_field(Box::leak(field_name.into_boxed_str()), &**item)?;
                        } else {
                            state.serialize_field(Box::leak(field_name.into_boxed_str()), &(*item as u64))?;
                        }
                        index += 1;
                    }
                }
            }
            else if (self.bounds as u64) > 0 {
                let memory_slice = unsafe{ slice::from_raw_parts(self.bounds as *const u64, (&self.length+4) as usize)};
                let vec = &memory_slice.to_vec()
                    //.into_iter().skip(4).collect::<Vec<u64>>()
                    ;

                let mut index = 0;
                // for value in vec {
                //     println!("Value: {}", value); // Borrowed reference to each element
                // }
                for i in vec {
                    let field_name = format!("data_{}", &index);

                    if i > &(5497690084096)
                        && Backtrace::capture().frames().len() < 50 //this workaround is needed for now
                        && std::any::type_name::<T>() != "i32" {
                        //log::info!("super-serializing {}", i);
                        let item = (*i) as usize as *const T;
                        unsafe {state.serialize_field(Box::leak(field_name.into_boxed_str()), &*item)?;}
                    }
                    else {
                        state.serialize_field(Box::leak(field_name.into_boxed_str()), i)?;
                    }
                    index += 1;
                }
            }
        }

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
        let mut state = serializer.serialize_struct("LineUpCharacter", 22)?;

        // 2. Serialize simple fields directly.
        state.serialize_field("native_object", &self.native_object)?;
        state.serialize_field("CharacterSP_Denominator", &fixpoint_to_raw(&self.CharacterSP_Denominator))?;
        state.serialize_field("SpecialAvatarID", &self.SpecialAvatarID)?;
        state.serialize_field("Index", &self.Index)?;
        state.serialize_field("CharacterSP_Numerator", &fixpoint_to_raw(&self.CharacterSP_Numerator))?;
        state.serialize_field("AssistUid", &self.AssistUid)?;
        state.serialize_field("CharacterAvatarType", &self.CharacterAvatarType)?;
        state.serialize_field("CharacterLevel", &self.CharacterLevel)?;
        state.serialize_field("WorldLevel", &self.WorldLevel)?;
        state.serialize_field("TotalPower", &self.TotalPower)?;
        state.serialize_field("CharacterRank", &self.CharacterRank)?;
        state.serialize_field("CharacterHPRatio", &fixpoint_to_raw(&self.CharacterHPRatio))?;
        state.serialize_field("CharacterPromotion", &self.CharacterPromotion)?;
        state.serialize_field("CharacterID", &self.CharacterID)?;
        state.serialize_field("SpiritLineupType", &self.SpiritLineupType)?;
        state.serialize_field("CharacterRowIndex", &self.CharacterRowIndex)?;

        // 3. Serialize fields with custom logic.

        unsafe {
            // SkillTreePointList: Serialize `NativeArray<NativeObject>` pointer.
            if self.SkillTreePointList.is_null() { state.serialize_field("SkillTreePointList", "null")?; }
            else {
                state.serialize_field("SkillTreePointList", &*self.SkillTreePointList)?;
            }

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
            state.serialize_field("BattleGridAvatarData", &serialize_pointer(&self.BattleGridAvatarData))?;

            // SpiritPassiveList: Serialize `NativeArray<u32>` pointer.
            if self.SpiritPassiveList.is_null() { state.serialize_field("SpiritPassiveList", "null")?; }
            else {
                state.serialize_field("SpiritPassiveList", &*self.SpiritPassiveList)?;
            }

            // ChangedSkillTreePointList: Serialize `NativeArray<NativeObject>` pointer.
            if self.ChangedSkillTreePointList.is_null() { state.serialize_field("ChangedSkillTreePointList", "null")?; }
            else {
                state.serialize_field("ChangedSkillTreePointList", &*self.ChangedSkillTreePointList)?;
            }
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

        // Serialize as a struct with two fields
        let mut state = serializer.serialize_struct("NativeObject", 2)?;

        // Serialize `klass` pointer as a hexadecimal string
        state.serialize_field("klass", &serialize_pointer(&self.klass))?;

        // Serialize `monitor` pointer as a hexadecimal string
        state.serialize_field("monitor", &serialize_pointer(&self.monitor))?;

        state.end()
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

        if self.OAAMONICNLE.is_null(){ state.serialize_field("OAAMONICNLE", "null") ?;}
        else{unsafe { state.serialize_field("OAAMONICNLE", &*self.OAAMONICNLE) ?;}}

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

        let mut state = serializer.serialize_struct("BattleLineupData", 13)?;

        unsafe {
            // Serialize fields
            state.serialize_field("native_object", &self.native_object)?;
    
            if self.ExtraTeam.is_null(){ state.serialize_field("ExtraTeam", "null") ?;}
            else { state.serialize_field("ExtraTeam", &*self.ExtraTeam)?; }

            if self.TeamBuffIDList.is_null(){ state.serialize_field("TeamBuffIDList", "null") ?;}
            else {
                state.serialize_field(
                    "TeamBuffIDList", &*self.TeamBuffIDList
                )?;
            }
            if self.MazeBuffAdded.is_null(){ state.serialize_field("MazeBuffAdded", "null") ?;}
            else {
                state.serialize_field(
                    "MazeBuffAdded",
                    &*self.MazeBuffAdded
                )?;
            }

            if self.SpecialAvatarLevelAreaConfigs.is_null(){ state.serialize_field("SpecialAvatarLevelAreaConfigs", "null") ?;}
            else {
                state.serialize_field(
                    "SpecialAvatarLevelAreaConfigs",
                    &serialize_pointer(&self.SpecialAvatarLevelAreaConfigs)
                )?;
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
            // Serialize simple u32 field directly
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
        log::info!("serialize::SpecialRelicData");
        state.serialize_field("native_object", &self.native_object)?;

        unsafe{
            if self.LGBJKGGCELB.is_null(){ state.serialize_field("LGBJKGGCELB", "null") ?;}
            else {
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
        log::info!("serialize::BattleRelicModule");

        // Serialize native_object
        state.serialize_field("native_object", &self.native_object)?;

        unsafe {
        // Safely serialize each pointer field
            state.serialize_field("AAEONBIGBBP", &serialize_pointer(&self.AAEONBIGBBP))?;
            state.serialize_field("BKCGOLIBNHC", &serialize_pointer(&self.BKCGOLIBNHC))?;

            if self.BattleRelicInfos.is_null() { state.serialize_field("BattleRelicInfos", "null")?; }
            else {
                state.serialize_field("BattleRelicInfos", &*self.BattleRelicInfos)?;
            }

            state.serialize_field("PMMGFOHHKPM", &serialize_pointer(&self.PMMGFOHHKPM))?;
            state.serialize_field("BIJMJNIMPOM", &serialize_pointer(&self.BIJMJNIMPOM))?;
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

impl Serialize for BattleRelicInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Define struct serialization with 5 fields
        let mut state = serializer.serialize_struct("BattleRelicInfo", 5)?;
        log::info!("serialize::BattleRelicInfo");

        // Serialize the native object
        state.serialize_field("native_object", &self.native_object)?;
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

impl Serialize for NCGNFPLFBOJ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Start serializing the struct with 4 fields
        let mut state = serializer.serialize_struct("NCGNFPLFBOJ", 4)?;
        log::info!("serialize::NCGNFPLFBOJ");

        // Serialize each field individually
        state.serialize_field("native_object", &self.native_object)?;
        state.serialize_field("NIKFINDKDKO", &self.NIKFINDKDKO)?;
        state.serialize_field("KBMCHLGDKEF", &self.KBMCHLGDKEF)?;
        state.serialize_field("KHADHNNCFLH", &self.KHADHNNCFLH)?;

        // End the serialization
        state.end()
    }
}