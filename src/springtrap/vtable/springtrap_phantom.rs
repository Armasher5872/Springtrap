use super::*;

const KOOPAJR_CANNONBALL_VTABLE_INITIALIZATION_EVENT_OFFSET: usize = 0x3425870;
const KOOPAJR_CANNONBALL_VTABLE_WEAPON_MODULE_ACCESSOR_INITIALIZATION_EVENT_OFFSET: usize = 0x34257f0;

//Bowser Jr Cannonball Initialization Event Offset
#[skyline::hook(offset = KOOPAJR_CANNONBALL_VTABLE_INITIALIZATION_EVENT_OFFSET)]
unsafe extern "C" fn koopajr_cannonball_initialization_event(vtable: u64, weapon: *mut smash::app::Weapon, param_3: u64) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = *(param_3 as *mut u32).add(0x2c/4);
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let ptr = get_module_vtable_func(boma, 0x108, 0x60);
    let set_shield_group2: extern "C" fn(*mut u64, *mut ShieldGroupResource2, i32) = std::mem::transmute(ptr);
    let reflector_module = *(boma as *mut *mut u64).add(0x108/8);
    if owner_kind == *FIGHTER_KIND_GANON {
        let bb_shield_data = ShieldData::new(0.0, 6.0, 0.0, 0.0, 6.0, 0.0, 4.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
        let bb_shield_datas = &mut (ShieldDatas2::new().add(bb_shield_data, 0));
        let bb_resource = &mut ShieldGroupResource2::new(bb_shield_datas, 1, 1.0, 1.0, 50.0, 0.0, false, 0);
        set_shield_group2(reflector_module, bb_resource, *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
        ReflectorModule::set_status(boma, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NONE), *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
    }
    call_original!(vtable, weapon, param_3)
}

//Bowser Jr Cannonball Reflector Clean Event Offset
unsafe extern "C" fn koopajr_cannonball_reflector_clean_event(_vtable: u64, weapon: *mut smash::app::Weapon) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_GANON {
        ReflectorModule::clean(boma);
    }
}

//Bowser Jr Cannonball On Reflection Event Offset
unsafe extern "C" fn koopajr_cannonball_on_reflection_event(_vtable: u64, weapon: *mut smash::app::Weapon, log: *mut ShieldAttackCollisionEvent) {
    let boma = (*weapon).battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let opponent_id = (*(*log).collision_log).opponent_object_id;
    let opponent_battle_object = get_battle_object_from_id(opponent_id);
    let opponent_boma = (*opponent_battle_object).module_accessor;
    let opponent_power = (*log).real_power;
    let opponent_attack_data = AttackModule::attack_data(owner_boma, (*(*log).collision_log).collider_id as i32, (*(*log).collision_log).x35);
    let vec = (*opponent_attack_data).vector;
    let opponent_angle = if vec > 360 {32} else {(*opponent_attack_data).vector} as f32;
    let opponent_lr = PostureModule::lr(opponent_boma);
    let speed = opponent_power/8.0;
    let speed_x = ((opponent_angle+90.0).to_radians().sin()*speed)*opponent_lr;
    let speed_y = (opponent_angle-90.0).to_radians().cos()*speed;
    if owner_kind == *FIGHTER_KIND_GANON {
        WorkModule::set_float(boma, speed_x, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
        WorkModule::set_float(boma, speed_y, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
        if [*WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_FALL].contains(&status_kind) {
            StatusModule::change_status_request_from_script(boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_FALL, false);
        }
        if [
            *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_WALK, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_FALL, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_TURN, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FREDDY_WALK, 
            *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FREDDY_FALL, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FREDDY_TURN
        ].contains(&status_kind) {
            StatusModule::change_status_request_from_script(boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_BREAK, false);
        }
    }
}

//Bowser Jr Cannonball Initialize Weapon Module Accessor Event Offset
#[skyline::hook(offset = KOOPAJR_CANNONBALL_VTABLE_WEAPON_MODULE_ACCESSOR_INITIALIZATION_EVENT_OFFSET)]
unsafe extern "C" fn koopajr_cannonball_initialize_weapon_module_accessor(vtable: u64, boma: *mut BattleObjectModuleAccessor, param_3: u64) -> u64 {
    *(param_3 as *mut i32).add(0x288/4) = *COLLISION_KIND_SHIELD;
    call_original!(vtable, boma, param_3)
}

pub fn install() {
    //Fuck it we ball type code (Patches the initialization of Bowser Jr's Cannonball modules to instead use Palutena's Reflection Board Module Initialization so that the former can call to ReflectorModule functions correctly)
    let koopajr_cannonball_module_initialization_offset = get_article_module_initialization_offset(*WEAPON_KIND_KOOPAJR_CANNONBALL) as usize;
    let initialize_reflectormodule = unsafe {skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64+0x33b9830};
    let _ = skyline::patching::Patch::in_text(koopajr_cannonball_module_initialization_offset).data(initialize_reflectormodule);
    let _ = skyline::patching::Patch::in_text(0x51d8348).data(koopajr_cannonball_reflector_clean_event as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x51d8468).data(koopajr_cannonball_on_reflection_event as *const () as u64);
    skyline::install_hooks!(
        koopajr_cannonball_initialization_event,
        koopajr_cannonball_initialize_weapon_module_accessor
    );
}