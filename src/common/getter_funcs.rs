#![allow(improper_ctypes_definitions)]
use super::*;

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

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

//Gets Article Boma
pub unsafe fn get_article_boma(boma: *mut BattleObjectModuleAccessor, article_type: skyline::libc::c_int) -> *mut BattleObjectModuleAccessor {
    let article = ArticleModule::get_article(boma, article_type);
    let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    return sv_battle_object::module_accessor(object_id);
}

pub fn get_weapon_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CWeaponCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CWeaponCommon))
    }
}

//Gets the article owner boma
pub unsafe fn get_owner_boma(weapon: &mut L2CAgentBase) -> *mut BattleObjectModuleAccessor {
    return &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
}

//Used to get the pointer for a vtable function within a specific module.
pub unsafe fn get_module_vtable_func(boma: *mut BattleObjectModuleAccessor, module_offset: usize, func_offset: u64) -> u64 {
    let module = (boma as *mut u64).add(module_offset/0x8);
    let vtable = *module as *const u64;
    *((*vtable + func_offset) as *const u64)
}

pub fn get_article_module_initialization_offset(weapon_id: i32) -> u64 {
    return 0x5189818+(0xe8*(weapon_id as u64))+0xb8;
}