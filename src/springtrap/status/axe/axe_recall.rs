use super::*;

unsafe extern "C" fn springtrap_axe_recall_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_recall_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    ModelModule::set_scale(boma, 0.73);
    GroundModule::set_passable_check(boma, false);
    GroundModule::set_collidable(boma, false);
    JostleModule::set_status(boma, false);
    0.into()
}

unsafe extern "C" fn springtrap_axe_recall_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let prev_status_kind = weapon.global_table[PREV_STATUS_KIND].get_i32();
    let attack_data = AttackModule::attack_data(boma, 0, false);
    if prev_status_kind == *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK {
        (*attack_data).size = 7.0;
    }
    HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
    MotionModule::change_motion(boma, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_recall_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_recall_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let attack_data = AttackModule::attack_data(boma, 0, false);
    let axe_dist = calculate_axe_distance(weapon);
    let axe_stats = calculate_axe_stats(axe_dist.x, axe_dist.y);
    (*attack_data).vector = axe_stats.z as i32;
    (*attack_data).r_add = 0;
    (*attack_data).r_fix = axe_stats.w.round() as i32;
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, axe_stats.x, axe_stats.y);
    if should_remove_axe(weapon) || StatusModule::status_kind(owner_boma) != *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
        remove_axe(weapon);
    }
    if MotionModule::is_end(boma) {
        MotionModule::change_motion(boma, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn calculate_axe_distance(weapon: &mut L2CWeaponCommon) -> Vector2f {
    let boma = weapon.module_accessor;
    let mut ret = Vector2f{x: 0.0, y: 0.0};
    if LinkModule::is_link(boma, *LINK_NO_ARTICLE) {
        let x = {weapon.clear_lua_stack(); lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_X, LINK_NO_ARTICLE, Hash40::new("havel"), true); FL_sv_module_access::link(weapon.lua_state_agent); weapon.pop_lua_stack(1).get_f32()};
        let y = {weapon.clear_lua_stack(); lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Y, LINK_NO_ARTICLE, Hash40::new("havel"), true); FL_sv_module_access::link(weapon.lua_state_agent); weapon.pop_lua_stack(1).get_f32()};
        let pos = *PostureModule::pos(boma);
        let x_dist = x-pos.x;
        let y_dist = y-pos.y;
        ret = Vector2f{x: x_dist, y: y_dist};
    }
    ret
}

unsafe extern "C" fn calculate_axe_stats(x_dist: f32, y_dist: f32) -> Vector4f {
    let x_speed = if x_dist > 0.0 {(x_dist/20.0).clamp(2.5, 10.0)} else {(x_dist/20.0).clamp(-10.0, -2.5)};
    let y_speed = y_dist/7.5;
    let vector = (y_speed.abs()).atan2(x_speed.abs()).to_degrees();
    let fixed_knockback = (x_speed.abs()*80.0).clamp(0.0, 200.0);
    Vector4f{x: x_speed, y: y_speed, z: vector, w: fixed_knockback}
}

unsafe extern "C" fn springtrap_axe_recall_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_recall_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_axe_recall_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_ironballcloned")
    .set_costume(get_springtrap_costumes_acmd())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, springtrap_axe_recall_exit_status)
    .install()
    ;
}