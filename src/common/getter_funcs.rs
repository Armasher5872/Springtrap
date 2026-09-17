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
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_DX);
        }
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_GIRL);
        }
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_BIG);
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

pub fn weapon_initialise_module(weapon_id: i32, module: ModuleInitModules) {
    let module_init_offset = match module {
        ModuleInitModules::KineticModule => 0x33b89a0,
        ModuleInitModules::ArticleModule => 0x33b97b0,
        ModuleInitModules::AttackModule => 0x33b8af0,
        ModuleInitModules::ControlModule => 0x33b95b0,
        ModuleInitModules::EffectModule => 0x33b8ce0,
        ModuleInitModules::GroundModule => 0x33b8e00,
        ModuleInitModules::MotionModule => 0x33b8300,
        ModuleInitModules::ReflectModule => 0x33b8f80,
        ModuleInitModules::SearchModule => 0x33b9030,
        ModuleInitModules::SoundModule => 0x33b9210,
        ModuleInitModules::VisibilityModule => 0x33b8470,
        ModuleInitModules::ColorBlendModule => 0x33b8510,
        ModuleInitModules::ShakeModule => 0x33b85a0,
        ModuleInitModules::AreaModule => 0x33b9500,
        ModuleInitModules::SlopeModule => 0x33b9720,
        ModuleInitModules::ReflectorModule => 0x33b9de0,
        ModuleInitModules::SlowModule => 0x33b9900,
        ModuleInitModules::MotionAnimcmdModule => 0x33b8780,
        ModuleInitModules::TurnModule => 0x33b9ef0,
        ModuleInitModules::LuaModule => 0x33b93f0,
        _ => 0x0,
    };
    if module_init_offset == 0 {
        return;
    }
    let offset = 0x518a818+(0xe8*(weapon_id as usize))+(module as usize*0x8);
    let module_init = unsafe {skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64+module_init_offset};
    let _ = skyline::patching::Patch::in_text(offset).data(module_init);
}

/*
Credited to IncrediblePlays for the original logic of the function (Of which this is a modified form of). Used to get either the address or pointer address of an agent (fighter/weapon/item) virtual function. 

kind: The respective fighter/weapon kind, takes the dereferenced lua_const value (E.G. *FIGHTER_KIND_SNAKE)
entry: The entry of the virtual function you want to hook. Documentation can be found here: https://github.com/theincredibleplayer/smash-vtables/blob/main/Vtable/fighter_vtable_documentation.txt
is_weapon: Indicates if the agent is a weapon
is_pointer: Determines whether or not it should get the address hook itself, or the pointer to the address hook. The pointer can be seen as the "index" of the agent's virtual table.

An example of its usage would be: get_agent_virtual_function(*FIGHTER_KIND_SNAKE, 13, false, false); This would get the 13th (0-Indexed) virtual function of Snake, which is his OPFF.

This function also has additional checks that will forcibly close the game if the function params are out of bounds
*/
pub fn get_agent_virtual_function(kind: i32, entry: usize, is_weapon: bool, is_pointer: bool) -> usize {
    if kind < 0 {
        std::process::abort()
    }
    unsafe {
        if is_weapon {
            if kind >= 0x267 || entry >= 104 {
                std::process::abort();
            }
        }
        else{
            if kind >= 0x5E || entry >= 146 {
                std::process::abort();
            }
        }
        let vtable = if is_weapon {get_weapon_vtable(kind as u32)} else {get_fighter_vtable(kind as u32)};
        let first_entry_ptr = *(vtable as *const u64) as *const usize;
        let main = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text);
        if is_pointer {
            return first_entry_ptr.add(entry) as usize-main as usize;
        }
        else {
            return *first_entry_ptr.add(entry)-main as usize;
        }
    }
}