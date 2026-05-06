use super::*;

unsafe extern "C" fn springtrap_axe_recall_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_recall_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    if LinkModule::is_link(boma, *LINK_NO_ARTICLE) {
        let x = {weapon.clear_lua_stack(); lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_X, LINK_NO_ARTICLE, Hash40::new("havel"), true); FL_sv_module_access::link(weapon.lua_state_agent); weapon.pop_lua_stack(1).get_f32()};
        let y = {weapon.clear_lua_stack(); lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Y, LINK_NO_ARTICLE, Hash40::new("havel"), true); FL_sv_module_access::link(weapon.lua_state_agent); weapon.pop_lua_stack(1).get_f32()};
        let pos = *PostureModule::pos(boma);
        let x_dist = x-pos.x;
        let y_dist = y-pos.y;
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, x_dist/20.0, y_dist/7.5);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.03, 0.03);
    }
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    WorkModule::set_int(boma, -1, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
    ModelModule::set_scale(boma, 0.73);
    GroundModule::set_passable_check(boma, false);
    GroundModule::set_collidable(boma, false);
    JostleModule::set_status(boma, false);
    0.into()
}

unsafe extern "C" fn springtrap_axe_recall_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.set_situation(SITUATION_KIND_AIR.into());
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_recall_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_recall_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    let x_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    let y_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    let atan2 = y_speed.atan2(x_speed);
    let vector = atan2.to_degrees();
    (*AttackModule::attack_data(boma, 0, false)).vector = vector as i32;
    if should_remove_axe(weapon) || owner_status_kind != *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
        remove_axe(weapon);
    }
    if MotionModule::is_end(boma) {
        MotionModule::change_motion(boma, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn springtrap_axe_recall_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    if LinkModule::is_link(boma, *LINK_NO_ARTICLE) {
        let x = {weapon.clear_lua_stack(); lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_X, LINK_NO_ARTICLE, Hash40::new("havel"), true); FL_sv_module_access::link(weapon.lua_state_agent); weapon.pop_lua_stack(1).get_f32()};
        let y = {weapon.clear_lua_stack(); lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Y, LINK_NO_ARTICLE, Hash40::new("havel"), true); FL_sv_module_access::link(weapon.lua_state_agent); weapon.pop_lua_stack(1).get_f32()};
        let pos = *PostureModule::pos(boma);
        let x_dist = x-pos.x;
        let y_dist = y-pos.y;
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, (x_dist/20.0).min(2.5), y_dist/7.5);
    }
    WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_recall_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_axe_recall_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume(get_costumes())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_exit_status)
    .install()
    ;
}