use super::*;

unsafe extern "C" fn springtrap_phantom_phantom_explode_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_explode_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let phantom_type = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_BALLOON_BOY {
        WorkModule::set_int(boma, 4, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
        WorkModule::set_int(boma, 10, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
        WorkModule::set_int(boma, 10, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    ModelModule::set_scale(boma, 0.001);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_explode_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let phantom_type = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    let pos = *PostureModule::pos(boma);
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_BALLOON_BOY {
        SoundModule::play_se(boma, Hash40::new("se_ganon_special_l02"), true, false, false, false, enSEType(0));
        ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0, 80, 80, 0, 40, 15.0, 0.0, 12.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
        SoundModule::play_se(boma, Hash40::new("se_common_bomb_m"), true, false, false, false, enSEType(0));
        ATTACK(weapon, 0, 0, Hash40::new("top"), 12.0, 361, 80, 0, 40, 18.0, 0.0, 12.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
        SoundModule::play_se(boma, Hash40::new("se_common_bomb_m"), true, false, false, false, enSEType(0));
        ATTACK(weapon, 0, 0, Hash40::new("top"), 6.0, 270, 70, 0, 0, 18.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    EffectModule::req(boma, Hash40::new("springtrap_phantom_detonate"), &Vector3f{x: pos.x, y: pos.y+12.0, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.1, 0, -1, false, 0);
    EffectModule::req(boma, Hash40::new("springtrap_phantom_detonate_shock"), &Vector3f{x: pos.x, y: pos.y+12.0, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
    ReflectorModule::set_status(boma, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NONE), *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
    weapon.fastshift(L2CValue::Ptr(springtrap_phantom_phantom_explode_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_phantom_phantom_explode_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let current_frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let pos_z = PostureModule::pos_z(boma);
    if current_frame > 1.0 {
        AttackModule::clear_all(boma);
    }
    PostureModule::set_pos(boma, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
    if should_remove_phantom(weapon) {
        remove_phantom(weapon);
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_explode_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_explode_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    WorkModule::set_int(boma, 0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_explode_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    WorkModule::set_int(boma, 0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    0.into()
}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_EXPLODE, springtrap_phantom_phantom_explode_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_EXPLODE, springtrap_phantom_phantom_explode_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_EXPLODE, springtrap_phantom_phantom_explode_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_EXPLODE, springtrap_phantom_phantom_explode_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_EXPLODE, springtrap_phantom_phantom_explode_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_EXPLODE, springtrap_phantom_phantom_explode_exit_status)
    .install()
    ;
}