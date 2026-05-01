use super::*;

unsafe extern "C" fn springtrap_axe_hit_stick_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_hit_stick_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_hit_stick_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("hit_stick"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_hit_stick_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_hit_stick_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let object_id = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    if should_remove_axe(weapon) {
        remove_axe(weapon);
    }
    if object_id != *BATTLE_OBJECT_ID_INVALID {
        let opponent_boma = sv_battle_object::module_accessor(object_id as u32);
        let opponent_status_kind = StatusModule::status_kind(opponent_boma);
        if [
            *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_DEMO, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DOLLY_STAGE_DEAD, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_PLATE_WAIT,
            *FIGHTER_STATUS_KIND_ROULETTE_FURAFURA, *FIGHTER_STATUS_KIND_ROULETTE
        ].contains(&opponent_status_kind) {
            remove_axe(weapon);
        }
    }
    if owner_status_kind == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
        weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        if [hash40("hit_stick"), hash40("hit_stuck")].contains(&motion_kind) {
            MotionModule::change_motion(boma, Hash40::new("hit_stuck"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    0.into()
}

unsafe extern "C" fn springtrap_axe_hit_stick_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_hit_stick_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let object_id = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
    if WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED) {
        LinkModule::remove_model_constraint(boma, true);
        if LinkModule::is_link(boma, *WEAPON_LINK_NO_CONSTRAINT) {
            LinkModule::unlink(boma, *WEAPON_LINK_NO_CONSTRAINT);
        }
        if object_id != *BATTLE_OBJECT_ID_INVALID {
            let opponent_boma = sv_battle_object::module_accessor(object_id as u32);
            let opponent_pos = *PostureModule::pos(opponent_boma);
            let opponent_scale = PostureModule::scale(opponent_boma);
            PostureModule::set_pos(boma, &Vector3f{x: opponent_pos.x, y: opponent_pos.y+(7.0*opponent_scale), z: opponent_pos.z});
        }
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
        WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED);
    }
    0.into()
}

unsafe extern "C" fn springtrap_axe_hit_stick_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let object_id = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
    if WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED) {
        LinkModule::remove_model_constraint(boma, true);
        if LinkModule::is_link(boma, *WEAPON_LINK_NO_CONSTRAINT) {
            LinkModule::unlink(boma, *WEAPON_LINK_NO_CONSTRAINT);
        }
        if object_id != *BATTLE_OBJECT_ID_INVALID {
            let opponent_boma = sv_battle_object::module_accessor(object_id as u32);
            let opponent_pos = *PostureModule::pos(opponent_boma);
            let opponent_scale = PostureModule::scale(opponent_boma);
            PostureModule::set_pos(boma, &Vector3f{x: opponent_pos.x, y: opponent_pos.y+(7.0*opponent_scale), z: opponent_pos.z});
        }
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
        WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED);
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK, springtrap_axe_hit_stick_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK, springtrap_axe_hit_stick_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK, springtrap_axe_hit_stick_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK, springtrap_axe_hit_stick_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK, springtrap_axe_hit_stick_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK, springtrap_axe_hit_stick_exit_status)
    .install()
    ;
}