#![allow(improper_ctypes_definitions)]
use super::*;

//Gets the article owner boma
pub unsafe fn get_owner_boma(weapon: &mut L2CAgentBase) -> *mut BattleObjectModuleAccessor {
    return &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
}

//Gets Article Boma
pub unsafe fn get_article_boma(boma: *mut BattleObjectModuleAccessor, article_type: skyline::libc::c_int) -> *mut BattleObjectModuleAccessor {
    let article = ArticleModule::get_article(boma, article_type);
    let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    return sv_battle_object::module_accessor(object_id);
}

extern "C" {
    #[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
	pub fn dead_range(lua_state: u64) -> Vector4f;

    #[link_name = "_ZN3app8lua_bind39ArticleModule__get_article_from_no_implEPNS_26BattleObjectModuleAccessorEii"]
    pub fn get_article_from_no(boma: *mut smash::app::BattleObjectModuleAccessor, article_kind: i32, article_id: i32) -> *mut smash::app::Article;
}

//Collision Log
#[repr(C)]
pub struct CollisionLog {
    pub _next: *mut CollisionLog,
    pub _end: *mut CollisionLog,
    pub location: Vector3f,
    pub _padding_0: u32,
    pub _padding_1: u32,
    pub opponent_battle_object_id: u32,
    pub _padding_2: [u8;7],
    pub _collision_kind: u8,
    pub _receiver_part_id: u8,
    pub _collider_part_id: u8,
    pub _receiver_id: u8,
    pub _collider_id: u8,
    pub _padding_3: [u8;10]
}

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

//Used to get the pointer for a vtable function within a specific module.
pub unsafe fn get_module_vtable_func(boma: *mut BattleObjectModuleAccessor, module_offset: usize, func_offset: u64) -> u64 {
    let module = (boma as *mut u64).add(module_offset/0x8);
    let vtable = *module as *const u64;
    *((*vtable + func_offset) as *const u64)
}

#[skyline::from_offset(0x3ac560)]
pub unsafe extern "C" fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

//Gets the necessary grab animation for throws
pub unsafe extern "C" fn grabbed_anim_selector(fighter: &mut L2CFighterCommon, anim_name: &str, set_frame: f32, mot_rate: f32) {
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if capture_id as i32 != *BATTLE_OBJECT_ID_INVALID {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        let motion_share = WorkModule::get_param_int(capture_boma, hash40("param_motion"), hash40("motion_share"));
        let mut motion = hash40(anim_name);
        if motion_share != *FIGHTER_MOTION_SHARE_TYPE_TARO {
            if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
                motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_GIRL);
            }
        }
        else {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_DX);
        }
        MotionModule::change_motion(capture_boma, Hash40::new_raw(motion), set_frame, mot_rate, false, 0.0, false, false);
    }
}

//Credited to WuBoyTH

//Shield Data Resource, shieldboxes
#[repr(C)]
pub struct ShieldDataResource {
    pub offset: Vector3,
    pub offset2: Vector3,
    pub size: f32,
    pub x24: u32,
    pub joint: Hash40,
    pub shape: u8,
    pub shield_type: u8,
}

impl ShieldDataResource {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        size: f32,
        joint: Hash40,
        shape: u8,
        shield_type: u8
    ) -> Self {
        ShieldDataResource {
            offset: Vector3{vec: [x, y, z]},
            offset2: Vector3{vec: [x2, y2, z2]},
            size: size,
            x24: 0,
            joint: joint,
            shape: shape,
            shield_type: shield_type
        }
    }
}

//Shield Data, reflectorboxes

#[repr(C)]
pub struct ShieldData {
    pub offset: Vector3,
    pub offset2: Vector3,
    pub size: f32,
    pub x24: u32,
    pub joint: Hash40,
    pub shape: u8,
    pub shield_type: u8,
    pub x32: u16,
    pub x34: u32,
    pub x38: u64,
    pub status: i32,
    pub x44: u32,
    pub x48: u64,
}

impl ShieldData {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        size: f32,
        joint: Hash40,
        shape: u8,
        shield_type: u8
    ) -> Self {
        ShieldData {
            offset: Vector3{vec: [x, y, z]},
            offset2: Vector3{vec: [x2, y2, z2]},
            size: size,
            x24: 0,
            joint: joint,
            shape: shape,
            shield_type: shield_type,
            x32: 0,
            x34: 0,
            x38: 0,
            status: 0,
            x44: 0,
            x48: 0,
        }
    }
}

//Shield Datas, shieldboxes
#[repr(C)]
pub struct ShieldDatas {
    pub datas: [ShieldDataResource; 8]
}

impl ShieldDatas {
    pub fn new() -> ShieldDatas {
        ShieldDatas { datas: [
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0)
        ] }
    }
    pub fn add(mut self, shield_data: ShieldDataResource, index: usize) -> ShieldDatas {
        self.datas[index] = shield_data;
        self
    }
}

//Shield Datas 2, reflectors
#[repr(C)]
pub struct ShieldDatas2 {
    pub datas: [ShieldData; 8]
}

impl ShieldDatas2 {
    pub fn new() -> ShieldDatas2 {
        ShieldDatas2 { datas: [
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0)
        ] }
    }
    pub fn add(mut self, shield_data: ShieldData, index: usize) -> ShieldDatas2 {
        self.datas[index] = shield_data;
        self
    }
}

//Shield Group Resource, shieldboxes
#[repr(C)]
pub struct ShieldGroupResource {
    pub shield_datas: *const ShieldDatas,
    pub count: u64,
    pub front: u8,
    pub hop: bool,
    pub turn: bool,
    pub no_hop: bool
}

impl ShieldGroupResource {
    pub fn new(
        shield_datas: *const ShieldDatas,
        count: u64,
        front: u8,
        hop: bool,
        turn: bool,
        no_hop: bool
    ) -> Self {
        ShieldGroupResource {
            shield_datas: shield_datas,
            count: count,
            front: front,
            hop: hop,
            turn: turn,
            no_hop: no_hop
        }
    }
}

//Shield Group Resource 2, reflectors
#[repr(C)]
pub struct ShieldGroupResource2 {
    pub shield_datas: *const ShieldDatas2,
    pub count: u64,
    pub attack_mul: f32,
    pub speed_mul: f32,
    pub attack_limit: f32,
    pub life_mul: f32,
    pub no_m_ball: bool,
    pub front: u8
}

impl ShieldGroupResource2 {
    pub fn new(
        shield_datas: *const ShieldDatas2,
        count: u64,
        attack_mul: f32,
        speed_mul: f32,
        attack_limit: f32,
        life_mul: f32,
        no_m_ball: bool,
        front: u8
    ) -> Self {
        ShieldGroupResource2 {
            shield_datas: shield_datas,
            count: count,
            attack_mul: attack_mul,
            speed_mul: speed_mul,
            attack_limit: attack_limit,
            life_mul: life_mul,
            no_m_ball: no_m_ball,
            front: front
        }
    }
}

#[repr(C)]
pub struct CollisionLogScuffed {
    pub x00: *const u64,
    pub x08: *const u64,
    pub location: smash2::cpp::simd::Vector3,
    pub x20: u8,
    pub x21: u8,
    pub x22: u8,
    pub x23: u8,
    pub opponent_object_id: u32,
    pub opponent_object_category: u8,
    pub x29: u8,
    pub x2a: u8,
    pub x2b: u8,
    pub x2c: u8,
    pub x2d: u8,
    pub x2e: u8,
    pub collision_kind: u8,
    pub receiver_part_id: u8,
    pub collider_part_id: u8,
    pub receiver_id: u8,
    pub collider_id: u8,
    pub x35: bool
}

#[repr(C)]
pub struct ShieldAttackCollisionEvent {
    pub vtable: u64,
    pub shield_id: u32,
    pub unk: u8,
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub attack_module: u64,
    pub raw_power: f32,
    pub real_power: f32,
    pub collision_log: *const CollisionLogScuffed,
    pub group_index: i32,
    pub pos_x: f32,
    pub lr: f32,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: u8,
}

pub fn get_weapon_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CWeaponCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CWeaponCommon))
    }
}

pub fn get_article_module_initialization_offset(weapon_id: i32) -> u64 {
    return 0x5189818+(0xe8*(weapon_id as u64))+0xb8;
}