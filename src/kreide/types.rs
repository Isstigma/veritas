use crate::kreide::gamecore::*;
use crate::kreide::native_types::*;
use serde::{Deserialize, Serialize};
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NOPBAAAGGLA {
    pub native_object: NativeObject,
    //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
    pub HKFGOHGKOGK: *const c_void,                    // 0x10
    //#[serde(skip_deserializing, serialize_with = "serialize_turnbased_ability_component_pointer")]
    pub JKCOIOLCMEP: *const TurnBasedAbilityComponent, // 0x18
    //#[serde(skip_deserializing, serialize_with = "serialize_native_array_pointer")]
    pub FKKDFMPMJHG: *const NativeArray<NativeObject>, // 0x20
    //#[serde(skip_deserializing, serialize_with = "serialize_native_array_pointer")]
    pub JODAJBNCCNP: *const NativeArray<NativeObject>, // 0x28
    //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
    pub PBHCGDFPEED: *const c_void,                    // 0x30
    //#[serde(skip_deserializing, serialize_with = "serialize_native_array_pointer")]
    pub MDEHKOOKJCK: *const NativeArray<NativeObject>, // 0x38
    //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
    pub LGGEDDMACDF: *const NativeString,              // 0x40
    //#[serde(with = "serde_arrays")]
    pub AAHMMHBHMFN: [u8; 0x90],                       // 0x48
    //#[serde(skip_deserializing, serialize_with = "serialize_turnbased_ability_component_pointer")]
    pub KNDJNKNHFFG: *const TurnBasedAbilityComponent, // 0xd8
    //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
    pub BEAJGANIDLJ: *const c_void,                    // 0xe0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub NAGMKEABGEE: FixPoint,                         // 0xe8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub KLMAGCLFBAO: FixPoint,                         // 0xf0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub PDCMJAMPJNL: FixPoint,                         // 0xf8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub FFCGIMAMDPP: FixPoint,                         // 0x100
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub EFAAJEAENFF: FixPoint,                         // 0x108
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub JCPEINMPKAM: FixPoint,                         // 0x110
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GAALBDHLFOG: FixPoint,                         // 0x118
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub PJNEJPNBNMP: FixPoint,                         // 0x120
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GLPLDJKMOBE: FixPoint,                         // 0x128
    //#[serde(with = "serde_arrays")]
    pub FFFOLNDHIEH: [u8; 0x48],                       // 0x130
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub CMNBOEIDAOD: FixPoint,                         // 0x178
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub MGFECPHDPHB: FixPoint,                         // 0x180
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub JFKEEOMKMLI: FixPoint,                         // 0x188
    //#[serde(with = "serde_arrays")]
    pub HHEIPBOKCOH: [u8; 0x40],                       // 0x190
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub PNGJIDMHIOE: FixPoint,                         // 0x1d0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub PJPKDAKBEJI: FixPoint,                         // 0x1d8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub PAIGBKBOKDI: FixPoint,                         // 0x1e0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub COIDNPMCCFG: FixPoint,                         // 0x1e8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub OHBMMFAFMDP: FixPoint,                         // 0x1f0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub JFMADBFKBDK: FixPoint,                         // 0x1f8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub MHEBPGAHFCB: FixPoint,                         // 0x200
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub EPJEDLOBFPG: FixPoint,                         // 0x208
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub DGFBMAPFPNH: FixPoint,                         // 0x210
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub KOEGLFLGADD: FixPoint,                         // 0x218
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub JNFPCNAKNOP: FixPoint,                         // 0x220
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub PGOHAIPOCNK: FixPoint,                         // 0x228
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub MLKFKKACBCE: FixPoint,                         // 0x230
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub CGMHNNNOKAI: FixPoint,                         // 0x238
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub NEPGNKOMAAA: FixPoint,                         // 0x240
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub EFFODBPOOCN: FixPoint,                         // 0x248
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub ABIPIIBIIBE: FixPoint,                         // 0x250
    pub BBDANLEJCIA: bool,                             // 0x258
    pub HEMFDDDJOGK: bool,                             // 0x259
    pub DPEJKHJPLAC: bool,                             // 0x25a
    pub JICCOEHBPJJ: bool,                             // 0x25b
    pub APDDLHNGGIM: AttackType,                       // 0x25c
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub KODEDHBLGGH: FixPoint,                         // 0x260
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GLGFEKEMMJJ: FixPoint,                         // 0x268
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub CAILJEGIDKL: FixPoint,                         // 0x270
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub NHHNLMOBEGH: FixPoint,                         // 0x278
    pub COKMLMJPKLH: u32,                              // 0x280
    pub CAANBNCPACE: bool,                             // 0x284
    pub FNBALMGFGDM: bool,                             // 0x285
    pub HKNLHAMMIIM: bool,                             // 0x286
    pub GFFCEBJGABG: bool,                             // 0x287
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GCNOMMHFPOG: FixPoint,                         // 0x288
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub EBDJIHNKAOC: FixPoint,                         // 0x290
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub AHOCGHANMCE: FixPoint,                         // 0x298
    pub EGINKGPDNPK: bool,                             // 0x2a0
    pub AHPFPMEGEKG: bool,                             // 0x2a1
    pub EKBHFCODKFO: bool,                             // 0x2a2
    pub MNAPDDFFHJF: bool,                             // 0x2a3
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub KDJBABPDHEG: FixPoint,                         // 0x2a8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub HCGBHCPHDKJ: FixPoint,                         // 0x2b0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub DKOIGIHEBCD: FixPoint,                         // 0x2b8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub FNDCNMHMCIC: FixPoint,                         // 0x2c0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub CCLFKIPGJOG: FixPoint,                         // 0x2c8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub OEPAPFDLMML: FixPoint,                         // 0x2d0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub JHOHCEFOJNB: FixPoint,                         // 0x2d8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub MKNDMBOCCBO: FixPoint,                         // 0x2e0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub DJHDAOOEJOF: FixPoint,                         // 0x2e8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub MNGPDEOEHPE: FixPoint,                         // 0x2f0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GJNAGCJONAO: FixPoint,                         // 0x2f8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GCFCCDPIACO: FixPoint,                         // 0x300
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub DBNKBGKCMKH: FixPoint,                         // 0x308
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub DINCHAHPEAC: FixPoint,                         // 0x310
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub FOLCDHNIMMI: FixPoint,                         // 0x318
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub JEHMOKDJDDE: FixPoint,                         // 0x320
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GIHPOCDLJOA: FixPoint,                         // 0x328
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub FLMEBELNIKK: FixPoint,                         // 0x330
    pub CFBOJBAJCEA: i32,                              // 0x338
    pub IICNDPJGCFA: i32,                              // 0x33c
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub EBDJHPNOALL: FixPoint,                         // 0x340
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub HJAEPANAFLN: FixPoint,                         // 0x348
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub CINNHMENLIJ: FixPoint,                         // 0x350
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub NCOHIAPKAED: FixPoint,                         // 0x358
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub PGGOANFBJON: FixPoint,                         // 0x360
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GNMAKKBFOCH: FixPoint,                         // 0x368
    //#[serde(with = "serde_arrays")]
    pub BDLFBDLDEND: [u8; 0x48],                       // 0x370
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub DBBDIMCJIKE: FixPoint,                         // 0x3b8
    //#[serde(with = "serde_arrays")]
    pub ANHNDBECCJD: [u8; 0x40],                       // 0x3c0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub BKIFAEKCIHN: FixPoint,                         // 0x400
    pub KMIKODLPNGL: i32,                              // 0x408
    pub JGHJIGOCPNP: i32,                              // 0x40c
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub BGBOFNMKDNJ: FixPoint,                         // 0x410
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub DPPDEDGCLJJ: FixPoint,                         // 0x418
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GOHOJAIMDNM: FixPoint,                         // 0x420
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub DJCAFPFKOGP: FixPoint,                         // 0x428
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GMBACFCLEGD: FixPoint,                         // 0x430
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub LJGPDLDGCEO: FixPoint,                         // 0x438
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub DCEBGGFOFAO: FixPoint,                         // 0x440
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GHBPOPKEGLE: FixPoint,                         // 0x448
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub DEOICHHPAIF: FixPoint,                         // 0x450
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub BLFCEOMPDKK: FixPoint,                         // 0x458
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub HNJBAFCNNDD: FixPoint,                         // 0x460
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub BBNMJNPDOCP: FixPoint,                         // 0x468
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub JIINJMJGCOH: FixPoint,                         // 0x470
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub ILNAKPIOOAK: FixPoint,                         // 0x478
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub POLANGDKOKH: FixPoint,                         // 0x480
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub AMAJNHHAJIM: FixPoint,                         // 0x488
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub FMMBMJKNAHI: FixPoint,                         // 0x490
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub MJMDGNPPILN: FixPoint,                         // 0x498
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub ODBPMMGBKGA: FixPoint,                         // 0x4a0
    //#[serde(with = "serde_arrays")]
    pub KOCOLHHLFLD: [u8; 0x40],                       // 0x4a8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub ELGMFJLGCPH: FixPoint,                         // 0x4e8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub MAKENPDPHDN: FixPoint,                         // 0x4f0
    pub OJGNIBKADHK: u32,                              // 0x4f8
    pub AHHEDGLMDMG: i32,                              // 0x4fc
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub MKIMEBNOEGI: FixPoint,                         // 0x500
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub IAAJMHADJDG: FixPoint,                         // 0x508
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub GBENLNNEIJM: FixPoint,                         // 0x510
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub PJLPGAGKIDE: FixPoint,                         // 0x518
    //#[serde(with = "serde_arrays")]
    pub ACDFHOGEMCC: [u8; 0x40],                       // 0x520
    //#[serde(with = "serde_arrays")]
    pub MKMILJKLJON: [u8; 0x58],                       // 0x560
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub ENFFBMJBEDP: FixPoint,                         // 0x5b8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub FGIPOLJPICM: FixPoint,                         // 0x5c0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub KPELFJICFDH: FixPoint,                         // 0x5c8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub BDGDFKGOLPJ: FixPoint,                         // 0x5d0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub BEGDMOGLLGM: FixPoint,                         // 0x5d8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub BJAEJMLMJCL: FixPoint,                         // 0x5e0
    pub IJJHMGEHMHB: bool,                             // 0x5e8
    pub KDCHAHHPPGD: bool,                             // 0x5e9
    pub EJJMIFKCFHP: bool,                             // 0x5ea
    pub KBKGNDFAKGD: bool,                             // 0x5eb
    pub GCGEEFLGCIG: i32,                              // 0x5ec
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub APDLLHIMMEM: FixPoint,                         // 0x5f0
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub HMMMDOHLFEP: FixPoint,                         // 0x5f8
    //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
    pub ALOGNJIBIPG: FixPoint,                         // 0x600
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OLHMAHMMBNN {
    pub JBHFMCDFPPL: *const c_void,                    // 0x0
    pub FKHHOBBFMEH: *const NativeString,              // 0x8
    pub BAICECGKLBG: *const c_void,                    // 0x10
    pub OAAMONICNLE: *const NativeArray<NativeObject>, // 0x18
    pub MOIPJLBAODO: i32,                              // 0x20
    pub NMJEMHAMIHD: i32,                              // 0x24
    pub AHNHNPOCNDJ: bool,                             // 0x28
    pub OBNPIDPHFDE: bool,                             // 0x29
    pub EKFIDPFOILC: bool,                             // 0x2a
    pub NMKBJGEONOJ: bool,                             // 0x2b
    pub EDIDAHIELAG: *const c_void,                    // 0x30
    pub OKHBBILFBND: [u8; 0x2],                        // 0x38
    pub LDJAAEOOOLC: [u8; 0x2],                        // 0x3a
    pub MHFEBJINMBP: bool,                             // 0x3c
    pub AJENNABILJC: bool,                             // 0x3d
    pub GJIMBAPCJLF: bool,                             // 0x3e
    pub ODNBNHFLMCD: *const c_void,                    // 0x40
    pub FGJEHAKCLNL: *const c_void,                    // 0x48
    pub KGKBLOJMDPH: bool,                             // 0x50
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MMNDIEBMDNL {
    pub native_object: NativeObject,
    //#[serde(skip_deserializing, serialize_with = "serialize_turnbased_ability_component_pointer")]
    pub FIMNOPAAFEP: *const TurnBasedAbilityComponent, // 0x10
    //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
    pub MKMMNLODHDD: *const c_void,                    // 0x18
    //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
    pub GNBEIGMFGIP: *const c_void,                    // 0x20
    //#[serde(skip_deserializing, serialize_with = "serialize_skill_character_component_pointer")]
    pub HECCDOHIAFD: *const SkillCharacterComponent,   // 0x28
    pub HMCDHMFHABF: OLHMAHMMBNN,                      // 0x30
    pub OOIFIGDBNBO: i32,                              // 0x88
    pub DADCNHAIOMI: i32,                              // 0x8c
    pub NMJEMHAMIHD: i32,                              // 0x90
}
pub mod rpg {
    pub mod client {
        use crate::kreide::types::*;
        use std::ffi::c_void;
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AvatarData {
            pub native_object: NativeObject,
            //#[serde(skip_deserializing, serialize_with = "serialize_native_array_pointer")]
            pub HasTakenPromotionRewardList__BackingField: *const NativeArray<u32>, // 0x10
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub Row__BackingField: *const c_void,                                   // 0x18
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ExtraPropertyAddition: *const c_void,                              // 0x20
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub _AvatarName: *const NativeString,                                   // 0x28
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub PromotedBeforeData__BackingField: *const c_void,                    // 0x30
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _TrialEquipment: *const c_void,                                     // 0x38
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub GrowUpBeforeData__BackingField: *const c_void,                      // 0x40
            //#[serde(skip_deserializing, serialize_with = "serialize_avatar_servant_data_pointer")]
            pub ServantData__BackingField: *const AvatarServantData,                // 0x48
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub CombatPowerData__BackingField: *const c_void,                       // 0x50
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub AvatarPropertyData__BackingField: *const c_void,                    // 0x58
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub UltraSkillConfig__BackingField: *const c_void,                      // 0x60
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub LevelUpedBeforeData__BackingField: *const c_void,                   // 0x68
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _SkillDataMap: *const c_void,                                       // 0x70
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub RelicsData__BackingField: *const c_void,                            // 0x78
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _SkinIDList: *const NativeArray<u32>,                               // 0x80
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub SkillTreeData: *const c_void,                                       // 0x88
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub SpecialRow__BackingField: *const c_void,                            // 0x90
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _AvatarRowData: *const c_void,                                      // 0x98
            pub FirstMetTimeStamp: u64,                                             // 0xa0
            pub Promotion__BackingField: u32,                                       // 0xa8
            pub Level__BackingField: u32,                                           // 0xac
            pub _AdventurePlayerID: u32,                                            // 0xb0
            pub IsMarked__BackingField: bool,                                       // 0xb4
            pub IsDisplayOnly__BackingField: bool,                                  // 0xb5
            pub IsNew__BackingField: bool,                                          // 0xb6
            pub AvatarType__BackingField: i32,                                      // 0xb8
            pub DressedSkinID__BackingField: u32,                                   // 0xbc
            pub EquipmentUID__BackingField: u32,                                    // 0xc0
            pub RealID__BackingField: u32,                                          // 0xc4
            pub Rank__BackingField: u32,                                            // 0xc8
            pub SpecialAvatarID__BackingField: u32,                                 // 0xcc
            pub CurrentExp__BackingField: u32,                                      // 0xd0
            pub _BaseID: u32,                                                       // 0xd4
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct ModuleManager {
            pub native_object: NativeObject,
            pub MapConnectivityModule: *const c_void, // 0x10
            pub BattleCollegeModule: *const c_void,   // 0x18
            pub FantasticStoryActivityModule: *const c_void, // 0x20
            pub RogueMagicModule: *const c_void,      // 0x28
            pub FindChestModule: *const c_void,       // 0x30
            pub TutorialSupportModule: *const c_void, // 0x38
            pub MaterialSubmissionModule: *const c_void, // 0x40
            pub ActivityPlayerReturnModule: *const c_void, // 0x48
            pub ChatModule: *const c_void,            // 0x50
            pub ActivityModule: *const c_void,        // 0x58
            pub AntiAddictionModule: *const c_void,   // 0x60
            pub MultipleDropModule: *const c_void,    // 0x68
            pub TravelBrochureModule: *const c_void,  // 0x70
            pub WolfBroShootingModule: *const c_void, // 0x78
            pub ItemComposeModule: *const c_void,     // 0x80
            pub MuseumModule: *const c_void,          // 0x88
            pub ChimeraModule: *const c_void,         // 0x90
            pub LuaDataModule: *const c_void,         // 0x98
            pub SystemOpenModule: *const c_void,      // 0xa0
            pub _ModuleInitRequestList: *const NativeArray<NativeObject>, // 0xa8
            pub MultiPlayerActivityModule: *const c_void, // 0xb0
            pub MapRotationModule: *const c_void,     // 0xb8
            pub ActivityPhotoExhibitionModule: *const c_void, // 0xc0
            pub ActivityTrackPhotoModule: *const c_void, // 0xc8
            pub ActivityClockParkModule: *const c_void, // 0xd0
            pub MissionChronicleModule: *const c_void, // 0xd8
            pub ActivityStrongChallengeModule: *const c_void, // 0xe0
            pub PetModule: *const c_void,             // 0xe8
            pub ActivityAetherDivideModule: *const c_void, // 0xf0
            pub ShopModule: *const c_void,            // 0xf8
            pub SwitchHandModule: *const c_void,      // 0x100
            pub BoxingClubModule: *const c_void,      // 0x108
            pub RogueModule: *const c_void,           // 0x110
            pub MessageModule: *const c_void,         // 0x118
            pub QuestModule: *const c_void,           // 0x120
            pub TransferModule: *const c_void,        // 0x128
            pub RogueArcadeModule: *const c_void,     // 0x130
            pub FarmModule: *const c_void,            // 0x138
            pub DifficultyAdjustModule: *const c_void, // 0x140
            pub ArchiveModule: *const c_void,         // 0x148
            pub MusicAlbumModule: *const c_void,      // 0x150
            pub AchievementModule: *const c_void,     // 0x158
            pub FightActivityModule: *const c_void,   // 0x160
            pub EraFlipperModule: *const c_void,      // 0x168
            pub TrainModule: *const c_void,           // 0x170
            pub RelicModule: *const c_void,           // 0x178
            pub AvatarModule: *const c_void,          // 0x180
            pub DialogueModule: *const c_void,        // 0x188
            pub FormationMoveModule: *const c_void,   // 0x190
            pub EvolveBuildModule: *const c_void,     // 0x198
            pub TrainPartyModule: *const c_void,      // 0x1a0
            pub MissionModule: *const c_void,         // 0x1a8
            pub FloorConnectivityModule: *const c_void, // 0x1b0
            pub GrowthModule: *const c_void,          // 0x1b8
            pub HandbookModule: *const c_void,        // 0x1c0
            pub ShareModule: *const c_void,           // 0x1c8
            pub ToastQueueModule: *const c_void,      // 0x1d0
            pub PhotoGraphModule: *const c_void,      // 0x1d8
            pub OperationModule: *const c_void,       // 0x1e0
            pub BigMapModule: *const c_void,          // 0x1e8
            pub MarbleModule: *const c_void,          // 0x1f0
            pub GameStateServiceModule: *const c_void, // 0x1f8
            pub ActivityBenefitModule: *const c_void, // 0x200
            pub PamSkinModule: *const c_void,         // 0x208
            pub PerformanceRecallModule: *const c_void, // 0x210
            pub TeamModule: *const c_void,            // 0x218
            pub EntityTimeRewindModule: *const c_void, // 0x220
            pub AetherDivideModule: *const c_void,    // 0x228
            pub PamModule: *const c_void,             // 0x230
            pub DrinkMakerModule: *const c_void,      // 0x238
            pub FightFestModule: *const c_void,       // 0x240
            pub HeartDialModule: *const c_void,       // 0x248
            pub RechargeShopModule: *const c_void,    // 0x250
            pub BattleTipsModule: *const c_void,      // 0x258
            pub BattleModule: *const c_void,          // 0x260
            pub CatchGhostModule: *const c_void,      // 0x268
            pub MapPropOverrideConditionModule: *const c_void, // 0x270
            pub PingPongModule: *const c_void,        // 0x278
            pub OfferingModule: *const c_void,        // 0x280
            pub AnniversaryAvatarDeliverModule: *const c_void, // 0x288
            pub NavMapModule: *const c_void,          // 0x290
            pub NovelModule: *const c_void,           // 0x298
            pub PunkLordModule: *const c_void,        // 0x2a0
            pub ActivityMusicRhythmModule: *const c_void, // 0x2a8
            pub ActivityGuessTheSilhouetteModule: *const c_void, // 0x2b0
            pub ServerPrefsModule: *const c_void,     // 0x2b8
            pub MultiplayerGameModule: *const c_void, // 0x2c0
            pub TarotBookModule: *const c_void,       // 0x2c8
            pub RollShopModule: *const c_void,        // 0x2d0
            pub AlleyModule: *const c_void,           // 0x2d8
            pub MissionTimelineModule: *const c_void, // 0x2e0
            pub ActivityParkourModule: *const c_void, // 0x2e8
            pub ExpeditionModule: *const c_void,      // 0x2f0
            pub MultiPathAvatarModule: *const c_void, // 0x2f8
            pub PlayerModule: *const c_void,          // 0x300
            pub GamePlayLockModule: *const c_void,    // 0x308
            pub BattlePassModule: *const c_void,      // 0x310
            pub RaidModule: *const c_void,            // 0x318
            pub PayModule: *const c_void,             // 0x320
            pub ColonyCollectionPuzzleModule: *const c_void, // 0x328
            pub WhiteListInteractUploadModule: *const c_void, // 0x330
            pub MovieRacingModule: *const c_void,     // 0x338
            pub SpaceZooModule: *const c_void,        // 0x340
            pub RoleTrialModule: *const c_void,       // 0x348
            pub CumulativeConsumptionModule: *const c_void, // 0x350
            pub modules: *const NativeArray<NativeObject>, // 0x358
            pub FriendModule: *const c_void,          // 0x360
            pub AnniversaryCollectionModule: *const c_void, // 0x368
            pub ActivityFeverTimeModule: *const c_void, // 0x370
            pub PersonalizeModule: *const c_void,     // 0x378
            pub StarFightModule: *const c_void,       // 0x380
            pub TextJoinModule: *const c_void,        // 0x388
            pub LoadingTipsModule: *const c_void,     // 0x390
            pub ActivitySwordTrainingModule: *const c_void, // 0x398
            pub ChessRogueModule: *const c_void,      // 0x3a0
            pub PlanetFesModule: *const c_void,       // 0x3a8
            pub RogueTournModule: *const c_void,      // 0x3b0
            pub ChallengeModule: *const c_void,       // 0x3b8
            pub FiveDimModule: *const c_void,         // 0x3c0
            pub HeliobusModule: *const c_void,        // 0x3c8
            pub RogueHandbookModule: *const c_void,   // 0x3d0
            pub MonopolyModule: *const c_void,        // 0x3d8
            pub LobbyModule: *const c_void,           // 0x3e0
            pub ScheduleModule: *const c_void,        // 0x3e8
            pub ActivityQuestTimeLimitModule: *const c_void, // 0x3f0
            pub RogueAdventureModule: *const c_void,  // 0x3f8
            pub TitanAtlasModule: *const c_void,      // 0x400
            pub EntityScoreModule: *const c_void,     // 0x408
            pub ActivitySummonModule: *const c_void,  // 0x410
            pub SilverWolfModule: *const c_void,      // 0x418
            pub AdventureModule: *const c_void,       // 0x420
            pub FeatureSwitchModule: *const c_void,   // 0x428
            pub TreasureDungeonModule: *const c_void, // 0x430
            pub MultiFloorConflictModule: *const c_void, // 0x438
            pub StoryTokenModule: *const c_void,      // 0x440
            pub InventoryModule: *const c_void,       // 0x448
            pub MatchThreeModule: *const c_void,      // 0x450
            pub CompanionMissionActivityModule: *const c_void, // 0x458
            pub GachaModule: *const c_void,           // 0x460
            pub RecommendModule: *const c_void,       // 0x468
            pub EarlyAccessModule: *const c_void,     // 0x470
            pub ActivityTelevisionModule: *const c_void, // 0x478
            pub LoginModule: *const c_void,           // 0x480
            pub GridFightModule: *const c_void,       // 0x488
            pub RaidCollectionModule: *const c_void,  // 0x490
            pub TalkModule: *const c_void,            // 0x498
            pub BattleEventModule: *const c_void,     // 0x4a0
            pub WorldShop4ThModule: *const c_void,    // 0x4a8
            pub StoryLineModule: *const c_void,       // 0x4b0
            pub isInited: bool,                       // 0x4b8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TextID {
            pub hash: i32,   // 0x0
            pub hash64: u64, // 0x8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AvatarServantData {
            pub native_object: NativeObject,
            //#[serde(skip_deserializing, serialize_with = "serialize_avatar_data_pointer")]
            pub _AvatarData: *const AvatarData, // 0x10
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _Row: *const c_void,            // 0x18
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _SkillDataMap: *const c_void,   // 0x20
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _Json: *const c_void,           // 0x28
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ServantRowData: *const c_void, // 0x30
        }
    }
    pub mod gamecore {
        use crate::kreide::types::*;
        use std::ffi::c_void;
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TurnBasedAbilityComponent {
            pub _parent_object: GameComponentBase,                // 0x0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub CharmDamageAttackProperty: *const c_void,         // 0x18
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub AbilityComponentRef__BackingField: *const c_void, // 0x20
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub DisableActionStateByTask__BackingField: *const c_void, // 0x28
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub OnAbilityPropertyChanged: *const NativeArray<NativeObject>, // 0x30
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _BuffLockStepSources: *const NativeArray<NativeObject>, // 0x38
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _ExtraMaxLayerConfig: *const NativeArray<NativeObject>, // 0x40
            //#[serde(skip_deserializing, serialize_with = "serialize_character_data_pointer")]
            pub _CharacterDataRef: *const CharacterDataComponent, // 0x48
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub AdditionalAbilityParamList: *const NativeArray<NativeObject>, // 0x50
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _SelfExtrAbilityList: *const NativeArray<NativeString>, // 0x58
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub Weakness: *const c_void,                          // 0x60
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _AbilityPropertiesInitSnapshot: *const NativeArray<FixPoint>, // 0x68
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub RegardAsAttackTypeMap: *const NativeArray<NativeObject>, // 0x70
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub _KillerEntity: *const GameEntity,                 // 0x78
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _DebuffLockStepSources: *const NativeArray<NativeObject>, // 0x80
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub RegardAsSkillTypeMap: *const NativeArray<NativeObject>, // 0x88
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub ProjectileTargetAttachPoint: *const NativeString, // 0x90
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _DotModifierEventProcessors: *const NativeArray<NativeObject>, // 0x98
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _DmgChunk: *const c_void,                         // 0xa0
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub CharmDamageTarget: *const GameEntity,             // 0xa8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _AbilityToSkillMapping: *const c_void,            // 0xb0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub ModifierOverrideMapping: *const c_void,           // 0xb8
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _EnergyPointEntries: *const NativeArray<NativeObject>, // 0xc0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub AddModifierBindValueMapping: *const c_void,       // 0xc8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub CustomDataRef__BackingField: *const c_void,       // 0xd0
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub LastStanceBreakEntity__BackingField: *const GameEntity, // 0xd8
            //#[serde(skip_deserializing, serialize_with = "serialize_turnbased_ability_component_pointer")]
            pub _SyncPropertySource: *const TurnBasedAbilityComponent, // 0xe0
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _OnHitEffectMultipleOverride: *const NativeArray<NativeObject>, // 0xe8
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _DamageStoreList: *const NativeArray<NativeObject>, // 0xf0
            //#[serde(skip_deserializing, serialize_with = "serialize_character_config_pointer")]
            pub _JsonConfigRef: *const CharacterConfig,           // 0xf8
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub DamageSplitData: *const NativeArray<NativeObject>, // 0x100
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _StancePreshowConfigs: *const NativeArray<NativeObject>, // 0x108
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _EnableNegativeHPSourceList: *const NativeArray<NativeObject>, // 0x110
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ModifierEventSourceMuteCounter: *const c_void,   // 0x118
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _LockHPList: *const NativeArray<NativeObject>,    // 0x120
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _RedStanceInfoList: *const NativeArray<NativeObject>, // 0x128
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub CharmSkillName: *const NativeString,              // 0x130
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub KillerSkill__BackingField: *const c_void,         // 0x138
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _DamagedEntityListInAttack: *const NativeArray<GameEntity>, // 0x140
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _OnHitEffectOverride: *const NativeArray<NativeObject>, // 0x148
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub DamageDefender: *const GameEntity,                // 0x150
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _AbilityProperties: *const NativeArray<NativeObject>, // 0x158
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _RedStanceInfo: *const c_void,                    // 0x160
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _DefaultStanceInfo: *const c_void,                // 0x168
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _SyncPropertyMask: *const c_void,                 // 0x170
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ModifierRecordList: *const c_void,               // 0x178
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _StatusChanceResistanceDict: *const c_void,       // 0x180
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _DamagedAllEntityIDListInAttack: *const c_void,   // 0x188
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ExtraStanceInfo: *const c_void,                  // 0x190
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub _DamageAttacker: *const GameEntity,               // 0x198
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub LockActionDelayChange: *const c_void,             // 0x1a0
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _ModifierEventProcessors: *const NativeArray<NativeObject>, // 0x1a8
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub OverflowStanceDamageAttacker__BackingField: *const GameEntity, // 0x1b0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _TransformRef: *const c_void,                     // 0x1b8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _StatusProbabilityDict: *const c_void,            // 0x1c0
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub ResistModifierBehaviorFlags__BackingField: *const NativeArray<NativeObject>, // 0x1c8
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _DepartedParams: *const NativeArray<NativeObject>, // 0x1d0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _DelayModifyActionDelayQueue: *const c_void,       // 0x1d8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _LockShieldCounter: *const c_void,                 // 0x1e0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ModifierDelayParamList: *const c_void,            // 0x1e8
            //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
            pub TotalDamageCurrentAttack: FixPoint,                // 0x1f0
            pub BattleTag__BackingField: i32,                      // 0x1f8
            pub ForceKillFlag__BackingField: bool,                 // 0x1fc
            pub ActionDelayChanged__BackingField: [u8; 0x2],       // 0x1fd
            pub CharmDisableBPAdd: bool,                           // 0x1ff
            pub bIsInCharmAction: bool,                            // 0x200
            pub VisualFlagValue__BackingField: i32,                // 0x204
            pub _DeathVersion: u32,                                // 0x208
            //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
            pub TotalHitNum: FixPoint,                             // 0x210
            pub DeathSource__BackingField: i32,                    // 0x218
            pub IsTriggeringStanceCountDown__BackingField: bool,   // 0x21c
            pub HasRevived: bool,                                  // 0x21d
            pub UseSpecialSP__BackingField: bool,                  // 0x21e
            pub IsSharedDamageDataTarget: bool,                    // 0x21f
            pub LastStanceDamageType__BackingField: i32,           // 0x220
            pub _ModifierUIOperationIncr: i32,                     // 0x224
            pub IsInAttack: bool,                                  // 0x228
            pub MuteTriggerDeath__BackingField: bool,              // 0x229
            pub _HighestPriorityOnHitEffect: i32,                  // 0x22c
            pub CurrentAttackType__BackingField: AttackType,       // 0x230
            pub ProjectileHitCount: i32,                           // 0x234
            pub CharmDamageCount: i32,                             // 0x238
            pub _ModifierDelayAddCount: i32,                       // 0x23c
            pub _DebuffLockStep: i32,                              // 0x240
            pub StanceType: i32,                                   // 0x244
            //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
            pub InheritSPRatio: FixPoint,                          // 0x248
            pub InsertAbilityCount: i32,                           // 0x250
            pub SpeedVisualFlagValue__BackingField: i32,           // 0x254
            pub _CurrentAttackPhase: i32,                          // 0x258
            pub PropertyEnumBoundary__BackingField: i32,           // 0x25c
            pub ForbidVisualFlagValue__BackingField: i32,          // 0x260
            pub StanceState__BackingField: i32,                    // 0x264
            pub _BreakExtendEventUnsettled: bool,                  // 0x268
            pub TriggerBreakExtendLogic: bool,                     // 0x269
            pub MuteAllTriggerDeath__BackingField: bool,           // 0x26a
            pub CharmDisableSPAdd: bool,                           // 0x26b
            pub PropertyChangeFlag__BackingField: bool,            // 0x26c
            pub LockSelfActionDelay: bool,                         // 0x26d
            pub _IsProcessingModifierDelayParam: bool,             // 0x26e
            pub LastBreakStanceDamageType__BackingField: i32,      // 0x270
            pub _ResetStanceVersion: u32,                          // 0x274
            //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
            pub ActionDelayDistance__BackingField: FixPoint,       // 0x278
            pub IsTriggeredBlockDamage: bool,                      // 0x280
            pub BlockModifySp__BackingField: bool,                 // 0x281
            pub IsSnapshot__BackingField: bool,                    // 0x282
            pub _IsBehaviorFlagVisualDirty: bool,                  // 0x283
            pub _BuffLockStep: i32,                                // 0x284
            //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
            pub OverflowStanceDamage__BackingField: FixPoint,      // 0x288
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct EntityManager {
            pub native_object: NativeObject,
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _EntityUniqueNameDict: *const NativeArray<NativeObject>, // 0x10
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ProcessEntityTeamChangeDelg: *const c_void,             // 0x18
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub PerformanceGORoot__BackingField: *const c_void,          // 0x20
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub DataViewUISelectFadeOutEntity__BackingField: *const GameEntity, // 0x28
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub LevelEntity__BackingField: *const GameEntity,            // 0x30
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub DataViewUISelectFadeInFollowEntities__BackingField: *const c_void, // 0x38
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _OwnerWorldRef: *const c_void,                           // 0x40
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub PlayerGORoot__BackingField: *const c_void,               // 0x48
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _GroupEntityIDToEntityDict: *const c_void,               // 0x50
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _PauseEntityTimeSlowIndexDic: *const NativeArray<NativeObject>, // 0x58
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub EntityGORoot__BackingField: *const c_void,               // 0x60
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub DataViewUISelectFadeInEntity__BackingField: *const GameEntity, // 0x68
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub DataViewUISelectSummonerOfUncreatedServant__BackingField: *const GameEntity, // 0x70
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub GroupGORoot__BackingField: *const c_void,                // 0x78
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub DataViewUISelectFadeOutSummonerEntity__BackingField: *const GameEntity, // 0x80
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ServerEntityIDToEntityDict: *const c_void,              // 0x88
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _AllTeamEntityList: *const NativeArray<GameEntity>,      // 0x90
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _SnapshotEntityMap: *const c_void,                       // 0x98
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub DataViewUILeaveSummonerOfUncreatedServant__BackingField: *const GameEntity, // 0xa0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub LittleGameGORoot__BackingField: *const c_void,           // 0xa8
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _AllTeamEntity: *const NativeArray<GameEntity>,          // 0xb0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _AllEntityDictionary: *const c_void,                     // 0xb8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _UniqueNamedEntityDictionary: *const c_void,             // 0xc0
            pub _UseUniqueSnapshot: bool,                                // 0xc8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleEventDataComponent {
            pub _parent_object: CharacterDataComponent,          // 0x0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _EnergyBarState: *const c_void,                  // 0x90
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub CreateParams__BackingField: *const c_void,       // 0x98
            //#[serde(skip_deserializing, serialize_with = "serialize_turnbased_ability_component_pointer")]
            pub _TBAbilityRef: *const TurnBasedAbilityComponent, // 0xa0
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub SourceCaster__BackingField: *const GameEntity,   // 0xa8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub BattleEventConfig__BackingField: *const c_void,  // 0xb0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _BattleEventRowData: *const c_void,              // 0xb8
            pub BattleEventTotalDamageType: TeamType,            // 0xc0
            pub WarningChallengeTurnLeft: u32,                   // 0xc4
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub enum TeamType {
            TeamUnknow = 0,
            TeamLight = 1,
            TeamDark = 2,
            TeamNeutral = 3,
            TeamNPC = 4,
            Count = 5,
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TurnBasedGameMode {
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
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub enum AttackType {
            Unknown = 0,
            Normal = 1,
            BPSkill = 2,
            Ultra = 3,
            QTE = 4,
            DOT = 5,
            Pursued = 6,
            Maze = 7,
            MazeNormal = 8,
            Insert = 9,
            ElementDamage = 10,
            Level = 11,
            Servant = 12,
            TrueDamage = 13,
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct CharacterDataComponent {
            pub _parent_object: GameComponentBase,                   // 0x0
            //#[serde(skip_deserializing, serialize_with = "serialize_character_config_pointer")]
            pub JsonConfig__BackingField: *const CharacterConfig,    // 0x18
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _CharacterUICustomValueDict: *const c_void,          // 0x20
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub Summoner: *const GameEntity,                         // 0x28
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _DummpyEntityList: *const NativeArray<NativeObject>, // 0x30
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _RowData: *const c_void,                             // 0x38
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _DynamicScaleAdaptConfigs: *const NativeArray<NativeObject>, // 0x40
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _DynamicScaleAdaptEffectPathRule: *const c_void,     // 0x48
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _DynamicScaleAdaptTypes: *const NativeArray<NativeObject>, // 0x50
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub HideDisplayInfoSkillNames: *const c_void,            // 0x58
            pub LastActTurnCount__BackingField: u32,                 // 0x60
            pub GridFightTag__BackingField: i32,                     // 0x64
            pub EnhancedState: i32,                                  // 0x68
            pub SpawnTurnCount: u32,                                 // 0x6c
            pub CreateReason: i32,                                   // 0x70
            pub DisableHeadLookAtActionEntityOverride: [u8; 0x2],    // 0x74
            pub IsBodyPart: bool,                                    // 0x76
            pub IsVisibleInViewMode__BackingField: bool,             // 0x77
            pub _SaveModelWhenDeadOverride: [u8; 0x2],               // 0x78
            pub DisableRootYawMapping__BackingField: bool,           // 0x7a
            pub TriggerLimbo: bool,                                  // 0x7b
            pub LocalOffsetAsMoveTarget__BackingField: [u8; 0xc],    // 0x7c
            pub CharacterID__BackingField: u32,                      // 0x88
            pub LineupIndex: i32,                                    // 0x8c
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct CharacterConfig {
            pub _parent_object: NativeObject,                          // 0x0
            pub SomatoType: i32,                                       // 0x10
            pub CharacterBodySize: i32,                                // 0x14
            pub CharacterHUDOffset: [u8; 0xc],                         // 0x18
            pub BuffPanelOffset: [u8; 0xc],                            // 0x24
            pub HitBoxOffset: [u8; 0xc],                               // 0x30
            pub TargetSelectGroup: i32,                                // 0x3c
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub CameraConfigList: *const NativeArray<NativeObject>,    // 0x40
            pub HitBoxType: i32,                                       // 0x48
            pub HitBoxWidth: f32,                                      // 0x4c
            pub HitBoxLength: f32,                                     // 0x50
            pub HitBoxHeight: f32,                                     // 0x54
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub HitBoxAttachPoint: *const NativeString,                // 0x58
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub Resilience: *const c_void,                             // 0x60
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub Location: *const c_void,                               // 0x68
            pub VisualRadius: f32,                                     // 0x70
            pub LookAtIKEnableRadius: f32,                             // 0x74
            pub AutoFlipModel: bool,                                   // 0x78
            pub SaveModelWhenDead: bool,                               // 0x79
            pub DeadPerform: bool,                                     // 0x7a
            pub PreloadUltraSkill: bool,                               // 0x7b
            pub IsSpecialVisualCharacter: i32,                         // 0x7c
            pub HideInTimeline: bool,                                  // 0x80
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub AnimEventConfigList: *const NativeArray<NativeString>, // 0x88
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub SkillList: *const NativeArray<NativeObject>,           // 0x90
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub AbilityList: *const NativeArray<NativeString>,         // 0x98
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub SkillAbilityList: *const NativeArray<NativeObject>,    // 0xa0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub DynamicValues: *const c_void,                          // 0xa8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub CustomValues: *const c_void,                           // 0xb0
            pub WeaponType: i32,                                       // 0xb8
            pub ArmorType: i32,                                        // 0xbc
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub SkillReadyTransits: *const NativeArray<NativeObject>,  // 0xc0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub PhaseAnimConfig: *const c_void,                        // 0xc8
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub AnimZoneConfigPath: *const NativeString,               // 0xd0
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub InitAnimStateName: *const NativeString,                // 0xd8
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub WhitelistSkillStateForInterrupt: *const NativeArray<NativeString>, // 0xe0
            pub ModifierPerformTimeFactor: f32,                        // 0xe8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub AsAidAttackTask: *const c_void,                        // 0xf0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub AsAidDefenderTask: *const c_void,                      // 0xf8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub AsAidProtectorTask: *const c_void,                     // 0x100
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub DisableAnimEventLayers: *const NativeArray<NativeString>, // 0x108
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub OnHitEditFootIKModeMap: *const c_void,                 // 0x110
            pub RepeatOccurAnimWhenBeHitNormalizedTime: f32,           // 0x118
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub CameraNamedDynamicOffset: *const NativeString,         // 0x120
            pub IgnoreDynamicOffsetBySelf: bool,                       // 0x128
            pub OverrideHeightForCameraOffset: f32,                    // 0x12c
            pub MonsterIgnoreGlobalDymanicOffset: bool,                // 0x130
            pub MaxMonsterPhase: u32,                                  // 0x134
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub PhaseList: *const NativeArray<NativeObject>,           // 0x138
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub OverrideWaveMonsterPerform: *const NativeString,       // 0x140
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub OverrideColliderCameraByName: *const NativeString,     // 0x148
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub EntityColliderConfig: *const c_void,                   // 0x150
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub EffectAdaptionList: *const NativeArray<NativeObject>,  // 0x158
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub AttachPointEffectAdaptionList: *const NativeArray<NativeObject>, // 0x160
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub FieldEffectAdaptionList: *const NativeArray<NativeObject>, // 0x168
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub EffectAttachPointRedirect: *const c_void,              // 0x170
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub MonsterConfig: *const c_void,                          // 0x178
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub ResidentEffectKey: *const NativeString,                // 0x180
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub ResidentPossessionKey: *const NativeString,            // 0x188
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub EmotionCharacterID: *const NativeString,               // 0x190
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub GraphEmotionAsset: *const NativeString,                // 0x198
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub AITagList: *const c_void,                              // 0x1a0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub GlobalAIFactorGroups: *const c_void,                   // 0x1a8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub ReplaceEmoConfig: *const c_void,                       // 0x1b0
            pub WillUnstage: bool,                                     // 0x1b8
            pub ViewModeSortPriority: u32,                             // 0x1bc
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub ReplaceAnimtorControllerPath: *const NativeString,     // 0x1c0
            pub AlwaysCutOnSkillTargetTeamChange: bool,                // 0x1c8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AvatarSkillRowData {
            pub native_object: NativeObject,
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _Row: *const c_void,              // 0x10
            //#[serde(with = "serde_arrays")]
            pub _OverrideData: [u8; 0xe8],        // 0x18
            //#[serde(skip_deserializing, serialize_with = "serialize_pointer")]
            pub _Config: *const c_void,           // 0x100
            //#[serde(with = "serde_arrays")]
            pub _DefaultOverrideData: [u8; 0xe0], // 0x108
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy, Serialize)]
        pub enum EntityType {
            None = 0,
            Avatar = 1,
            Monster = 2,
            LocalPlayer = 3,
            NPC = 4,
            NPCMonster = 5,
            StoryCharacter = 6,
            Prop = 7,
            Mission = 8,
            LevelEntity = 9,
            Neutral = 10,
            AtmoNpc = 11,
            BattleEvent = 12,
            TutorialEntity = 13,
            Team = 14,
            Partner = 15,
            LevelGraph = 16,
            Snapshot = 17,
            TeamFormation = 18,
            Model = 19,
            UICamera = 20,
            District = 21,
            GlobalShield = 22,
            CustomData = 23,
            Simple = 24,
            PuzzleGameObjectProp = 25,
            PerformanceLevelGraph = 26,
            Group = 27,
            ChessCharacter = 28,
            ChessTerrain = 29,
            SummonUnit = 30,
            LittleGameInstance = 31,
            Servant = 32,
            PreviewShow = 33,
            LittleGameContainer = 34,
            LittleGameViewProxy = 35,
            GridFightBackend = 36,
            DummyEntity = 37,
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct FixPoint {
            pub m_rawValue: i64, // 0x0
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct GameEntity {
            pub native_object: NativeObject,
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub HoyoTagContainer: *const c_void,         // 0x10
            pub _CurTickListRef: [u8; 0x10],             // 0x18
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _LateUpdateComponentList: *const c_void, // 0x28
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ComponentList: *const c_void,           // 0x30
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _OwnerWorldRef: *const c_void,           // 0x38
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub Name__BackingField: *const NativeString, // 0x40
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _UnityGO: *const c_void,                 // 0x48
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub TagComponentContainer: *const c_void,    // 0x50
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub TickLodTemplate: *const NativeString,    // 0x58
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub _UnstageReasonKey: *const NativeString,  // 0x60
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _DestroyWaitList: *const c_void,         // 0x68
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub DisposeCallback: *const c_void,          // 0x70
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub WorldTimeScaleAdpator: *const c_void,    // 0x78
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ComponentArrayRef: *const c_void,       // 0x80
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _TickLodProxy: *const c_void,            // 0x88
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _ComponentArray: *const NativeArray<GameComponentBase>, // 0x90
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _TickComponentList: *const c_void,       // 0x98
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub OnStageStateChange: *const c_void,       // 0xa0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub TimeScaleStack: *const c_void,           // 0xa8
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub NameForGameCore__BackingField: *const NativeString, // 0xb0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub OnTeamChange: *const c_void,             // 0xb8
            pub Visible__BackingField: bool,             // 0xc0
            pub IsStoryMode__BackingField: bool,         // 0xc1
            pub HasDisposed: bool,                       // 0xc2
            pub IsFakeAvatar__BackingField: bool,        // 0xc3
            pub _TickDelayFrameCount: u32,               // 0xc4
            pub LastTickTime__BackingField: f32,         // 0xc8
            pub CampID__BackingField: i32,               // 0xcc
            pub _ShouldLateUpdate: bool,                 // 0xd0
            pub Disposing: bool,                         // 0xd1
            pub _Tickable: bool,                         // 0xd2
            pub _IsRegisterEnviroChara: bool,            // 0xd3
            pub _AliveState: i32,                        // 0xd4
            pub IsLoaded__BackingField: bool,            // 0xd8
            pub IsHero__BackingField: bool,              // 0xd9
            pub KillImmediately: bool,                   // 0xda
            pub _IsOnStage: bool,                        // 0xdb
            pub _ServerEntityID: u32,                    // 0xdc
            pub _GroupID: u32,                           // 0xe0
            pub _GroupEntityID: u32,                     // 0xe4
            pub LastTickBucket__BackingField: i32,       // 0xe8
            pub LastTickFrame__BackingField: u64,        // 0xf0
            pub _EntityType: EntityType,                 // 0xf8
            pub RuntimeID__BackingField: u32,            // 0xfc
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _ForceTickLodLowestReason: *const c_void, // 0x100
            pub TickLodBoundSize__BackingField: f32,     // 0x108
            pub ObjectFeature__BackingField: i32,        // 0x10c
            pub ForceIgnoreTickLodBistSet: u32,          // 0x110
            pub _Team: TeamType,                         // 0x114
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct SkillData {
            pub native_object: NativeObject,
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub PreshowConditions: *const NativeArray<NativeObject>, // 0x10
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub OverrideTargetInfo: *const c_void,                   // 0x18
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub RowData: *const c_void,                              // 0x20
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub DefaultTargetInfo: *const c_void,                    // 0x28
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub VisibleCondTask: *const c_void,                      // 0x30
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub AllChildSkillDatas: *const NativeArray<SkillData>,   // 0x38
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub Config: *const c_void,                               // 0x40
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub OverrideAnimState: *const NativeString,              // 0x48
            //#[serde(skip_deserializing, serialize_with = "serialize_skill_character_component_pointer")]
            pub SkillCom: *const SkillCharacterComponent,            // 0x50
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub CustomReadyConfigConditions: *const NativeArray<NativeObject>, // 0x58
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _Slot: *const c_void,                                // 0x60
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub UsableCondTask: *const c_void,                       // 0x68
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub InsertCondTask: *const c_void,                       // 0x70
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub OverrideCameraConfig: *const c_void,                 // 0x78
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub OverrideCameraConfigAdded: *const c_void,            // 0x80
            //#[serde(skip_deserializing, serialize_with = "serialize_skill_data_pointer")]
            pub ParentSkillData: *const SkillData,                   // 0x88
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _SkillProperties: *const NativeArray<NativeObject>,  // 0x90
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub SkillTriggerKey: *const NativeString,                // 0x98
            pub CommonActiveSkillID: u32,                            // 0xa0
            pub LeftCastTimes: i32,                                  // 0xa4
            pub AttackDamageTypePreshowAttach: i32,                  // 0xa8
            pub ChildIndex: i32,                                     // 0xac
            pub MaxCastTimes: i32,                                   // 0xb0
            pub CurrentCoolDown: i32,                                // 0xb4
            pub DefaultCoolDown: i32,                                // 0xb8
            pub SkillConfigID: u32,                                  // 0xbc
            pub SkillIndex: i32,                                     // 0xc0
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct LineUpCharacter {
            pub native_object: NativeObject,
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub SkillTreePointList: *const NativeArray<NativeObject>, // 0x10
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub BattleEquipmentList: *const NativeArray<NativeObject>, // 0x18
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub BattleRelicItemModule: *const c_void,                 // 0x20
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub BattleGridAvatarData: *const c_void,                  // 0x28
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub SpiritPassiveList: *const NativeArray<u32>,           // 0x30
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub ChangedSkillTreePointList: *const NativeArray<NativeObject>, // 0x38
            //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
            pub CharacterSP_Denominator: FixPoint,                    // 0x40
            pub SpecialAvatarID: u32,                                 // 0x48
            pub Index: u32,                                           // 0x4c
            //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
            pub CharacterSP_Numerator: FixPoint,                      // 0x50
            pub AssistUid: u32,                                       // 0x58
            pub CharacterAvatarType: i32,                             // 0x5c
            pub CharacterLevel: u32,                                  // 0x60
            pub WorldLevel: u32,                                      // 0x64
            pub TotalPower: u32,                                      // 0x68
            pub CharacterRank: u32,                                   // 0x6c
            //#[serde(serialize_with = "serialize_fixpoint_pointer", skip_deserializing)]
            pub CharacterHPRatio: FixPoint,                           // 0x70
            pub CharacterPromotion: u32,                              // 0x78
            pub CharacterID: u32,                                     // 0x7c
            pub SpiritLineupType: i32,                                // 0x80
            pub CharacterRowIndex: u32,                               // 0x84
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleEventSkillRowData {
            pub native_object: NativeObject,
            //#[serde(with = "serde_arrays")]
            pub _DefaultOverrideData: [u8; 0xe8], // 0x10
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _Row: *const c_void,              // 0xf8
            //#[serde(with = "serde_arrays")]
            pub _OverrideData: [u8; 0xe8],        // 0x100
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _Config: *const c_void,           // 0x1e8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct ServantSkillRowData {
            pub native_object: NativeObject,
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _Row: *const c_void,              // 0x10
            //#[serde(with = "serde_arrays")]
            pub _OverrideData: [u8; 0xe8],        // 0x18
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _Config: *const c_void,           // 0x100
            //#[serde(with = "serde_arrays")]
            pub _DefaultOverrideData: [u8; 0xe0], // 0x108
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AbilityConfig {
            pub _parent_object: NativeObject,                       // 0x0
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub Name: *const NativeString,                          // 0x10
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub TargetInfo: *const c_void,                          // 0x18
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub OnAdd: *const NativeArray<NativeObject>,            // 0x20
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub OnRemove: *const NativeArray<NativeObject>,         // 0x28
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub OnStart: *const NativeArray<NativeObject>,          // 0x30
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub DynamicValues: *const c_void,                       // 0x38
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub TaskListTemplate: *const NativeArray<NativeObject>, // 0x40
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _TaskListTemplatesMap: *const c_void,               // 0x48
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct SkillCharacterComponent {
            pub _parent_object: GameComponentBase,                // 0x0
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _SkillDataList: *const NativeArray<SkillData>,    // 0x18
            //#[serde(skip_deserializing, serialize_with = "serialize_character_data_pointer")]
            pub _CharacterDataRef: *const CharacterDataComponent, // 0x20
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _SkillTargetRedirectEntries: *const NativeArray<NativeObject>, // 0x28
            //#[serde(skip_deserializing, serialize_with = "serialize_turnbased_ability_component_pointer")]
            pub _TBAbilityRef: *const TurnBasedAbilityComponent,  // 0x30
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub _SkillSlots: *const NativeArray<NativeObject>,    // 0x38
            //#[serde(skip_deserializing, serialize_with = "serialize_character_config_pointer")]
            pub _JsonConfigRef: *const CharacterConfig,           // 0x40
            //#[serde(with = "serde_arrays")]
            pub _recordAbilityInfo: [u8; 0x30],                   // 0x48
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub CurrentSkillTargetList__BackingField: *const NativeArray<GameEntity>, // 0x78
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub TaskContext__BackingField: *const c_void,         // 0x80
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub CurrentAimAtTargetList: *const NativeArray<GameEntity>, // 0x88
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub CurrentSkillSubTargetList__BackingField: *const NativeArray<GameEntity>, // 0x90
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub CurrentAimAtMainTargetList: *const NativeArray<GameEntity>, // 0x98
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub OnSkillSetup: *const NativeArray<NativeObject>,   // 0xa0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _SkillTypeDisableSlots: *const c_void,            // 0xa8
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub CurrentSkillTargetDamageHP: *const c_void,        // 0xb0
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub CurrentAimAtSubTargetList: *const NativeArray<GameEntity>, // 0xb8
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub SkillActualAttacker__BackingField: *const GameEntity, // 0xc0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub CurrentSkillTargetCharacterId: *const c_void,     // 0xc8
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub SkillPointEntity__BackingField: *const GameEntity, // 0xd0
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub AutoUseUltraParams: *const c_void,                // 0xd8
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
                ///todo: should there be some other serializer for value types? This array likely contains not refs to ints but values themselves
            pub _SkillTypeDisableCountArr: *const NativeArray<i32>, // 0xe0
            pub CurrentSkillKilledCount: i32,                     // 0xe8
            pub CharmAction: bool,                                // 0xec
            pub _AutoStandbyOnCurSkillFinish: bool,               // 0xed
            pub CurrentSkillBreakStance: bool,                    // 0xee
            pub SelfWaitActiveSkillIndex: i32,                    // 0xf0
            pub _SelfSkillPerformState: i32,                      // 0xf4
            pub _RecordSkillExtraUseParam: i32,                   // 0xf8
            pub CurrentSkillHasTriggerEffect: bool,               // 0xfc
            pub _hasRecordSkill: bool,                            // 0xfd
            pub _RedirectTargetIDIncr: i32,                       // 0x100
            pub _isPassive: bool,                                 // 0x104
            pub IsNoBpCost__BackingField: bool,                   // 0x105
            pub _hasOpInSkill: bool,                              // 0x106
            pub CurrentSkillKillAllOrBoss: bool,                  // 0x107
            pub _TargetPerformTimeCounter: f32,                   // 0x108
            pub _CurrentSkillIndex: i32,                          // 0x10c
            pub _CurrentSkillExtraUseParam: i32,                  // 0x110
            pub _OpIndexInSkill: i32,                             // 0x114
            pub _actionSkillIndex: i32,                           // 0x118
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct GameComponentBase {
            pub native_object: NativeObject,
            //#[serde(skip_deserializing, serialize_with = "serialize_game_entity_pointer")]
            pub _OwnerRef: *const GameEntity, // 0x10
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleLineupData {
            pub native_object: NativeObject,
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub ExtraTeam: *const NativeArray<LineUpCharacter>, // 0x10
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub TeamBuffIDList: *const NativeArray<u32>,        // 0x18
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub MazeBuffAdded: *const NativeArray<NativeObject>, // 0x20
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub SpecialAvatarLevelAreaConfigs: *const c_void,   // 0x28
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub _TemplateVariables: *const c_void,              // 0x30
            //#[serde(skip_deserializing, serialize_with= "serialize_native_array_pointer")]
            pub LightTeam: *const NativeArray<LineUpCharacter>, // 0x38
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub Context: *const c_void,                         // 0x40
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub BattleExtraPropertyAdditionDict__BackingField: *const c_void, // 0x48
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub AdditionalTemplateVariables: *const c_void,     // 0x50
            //#[serde(serialize_with = "serialize_pointer", deserialize_with = "deserialize_pointer")]
            pub DeferCreateTrialPlayerDic: *const c_void,       // 0x58
            //#[serde(skip_deserializing, serialize_with = "serialize_native_string")]
            pub _LevelPath: *const NativeString,                // 0x60
            pub WorldLevel: u32,                                // 0x68
        }
    }
}
