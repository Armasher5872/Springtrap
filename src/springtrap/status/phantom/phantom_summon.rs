use super::*;

unsafe extern "C" fn springtrap_phantom_phantom_summon_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_situation_kind = StatusModule::situation_kind(owner_boma);
    let owner_ground_correct = GroundModule::get_correct(owner_boma);
    StatusModule::init_settings(weapon.module_accessor, SituationKind(owner_situation_kind), *WEAPON_KINETIC_TYPE_NONE, owner_ground_correct as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_summon_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let phantom_type = WorkModule::get_int(owner_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_LW_PHANTOM_TYPE);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    WorkModule::set_float(boma, owner_lr, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_int(boma, phantom_type, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    PostureModule::set_pos(boma, &Vector3f{x: owner_pos_x+(18.0*owner_lr), y: owner_pos_y+3.0, z: owner_pos_z});
    ModelModule::set_scale(boma, 1.75);
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FOXY {
        StatusModule::change_status_force(boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FOXY_ATTACK, false);
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_BALLOON_BOY {
        StatusModule::change_status_force(boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, false);
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_summon_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let team_no = TeamModule::team_no(owner_boma) as i32;
    let phantom_type = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    ReflectorModule::set_status(boma, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NORMAL), *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
    ReflectorModule::set_no_team(boma, false);
    TeamModule::set_team_owner_id(boma, (*owner_boma).battle_object_id);
    TeamModule::set_team(boma, team_no, true);
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
        WorkModule::set_int(boma, 640, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        MotionModule::change_motion(boma, Hash40::new("freddy_summon"), 0.0, 1.0, false, 0.0, false, false);
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
        WorkModule::set_int(boma, 340, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        MotionModule::change_motion(boma, Hash40::new("chica_summon"), 0.0, 1.0, false, 0.0, false, false);   
    }
    weapon.fastshift(L2CValue::Ptr(springtrap_phantom_phantom_summon_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_phantom_phantom_summon_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let pos_z = PostureModule::pos_z(boma);
    let phantom_type = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    PostureModule::set_pos(boma, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
    if should_remove_phantom(weapon) {
        remove_phantom(weapon);
    }
    if MotionModule::is_end(boma) {
        if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
            weapon.change_status(WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FREDDY_WALK.into(), false.into());
        }
        if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
            weapon.change_status(WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_WALK.into(), false.into()); 
        }
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_summon_exec_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_summon_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_summon_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume(get_costumes())
    .status(Pre, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_SUMMON, springtrap_phantom_phantom_summon_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_SUMMON, springtrap_phantom_phantom_summon_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_SUMMON, springtrap_phantom_phantom_summon_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_SUMMON, springtrap_phantom_phantom_summon_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_SUMMON, springtrap_phantom_phantom_summon_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_SUMMON, springtrap_phantom_phantom_summon_exit_status)
    .install()
    ;
}