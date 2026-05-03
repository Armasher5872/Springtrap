use super::*;

unsafe extern "C" fn springtrap_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_AXE);
    WorkModule::off_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
    WorkModule::off_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_PHANTOM);
    WorkModule::set_float(boma, 0.0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CHARGE);
    WorkModule::set_int(boma, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    WorkModule::set_int(boma, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_TIME);
    WorkModule::set_int(boma, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    WorkModule::set_int(boma, *SPRINGTRAP_PHANTOM_TYPE_BALLOON_BOY, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_LW_PHANTOM_TYPE);
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
}

unsafe extern "C" fn springtrap_opff(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let current_frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let motion_kind = MotionModule::motion_kind(boma);
    let effect_id = WorkModule::get_int(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    //Handles rescaling
    if ModelModule::scale(boma) == WorkModule::get_param_float(boma, hash40("scale"), 0) {
        ModelModule::set_scale(boma, 1.1);
        AttackModule::set_attack_scale(boma, 1.1, true);
        GrabModule::set_size_mul(boma, 1.1);
    }
    //Axe Stuff
    if !ArticleModule::is_exist(boma, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE) {
        if WorkModule::is_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_AXE) {
            WorkModule::off_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_AXE);
        }
    }
    //Phantom Stuff
    if !ArticleModule::is_exist(boma, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_PHANTOM) {
        if WorkModule::is_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_PHANTOM) {
            WorkModule::off_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_PHANTOM);
        }
    }
    //Win Static Sound
    if [hash40("win_1_wait"), hash40("win_2_wait"), hash40("win_3_wait")].contains(&motion_kind) {
        if current_frame % 25 == 0 {
            let end_static = SoundModule::play_se(boma, Hash40::new("se_ganon_special_h02"), true, false, false, false, enSEType(0));
            SoundModule::set_se_vol(boma, end_static as i32, 0.75, 0);
        }
        if motion_kind == hash40("win_3_wait") {
            if (70..=90).contains(&current_frame) {
                EffectModule::set_alpha(boma, effect_id as u32, (current_frame/20) as f32);
            }
        }
    }
}

unsafe extern "C" fn springtrap_axe_on_start(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
}

unsafe extern "C" fn springtrap_axe_opff(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    if WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED) {
        let defender_object_id = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
        if defender_object_id != *BATTLE_OBJECT_ID_INVALID {
            let defender_boma = sv_battle_object::module_accessor(defender_object_id as u32);
            let defender_scale = PostureModule::scale(defender_boma);
            LinkModule::set_model_constraint_pos_ort(boma, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("bust"), (*CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_SCALE | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_ORIENTATION) as u32, true);
            ModelModule::set_scale(boma, 0.73*defender_scale);
            LinkModule::set_constraint_rot_offset(boma, &Vector3f{x: -90.0, y: 0.0, z: -90.0});
            LinkModule::set_constraint_translate_offset(boma, &Vector3f{x: -2.0*defender_scale, y: -7.0*defender_scale, z: -4.0*defender_scale});
        }
    }
}

unsafe extern "C" fn springtrap_phantom_on_start(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    WorkModule::set_int(boma, 0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
}

unsafe extern "C" fn springtrap_phantom_opff(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    println!("Current Motion Kind: {}", MotionModule::motion_kind(boma));
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .on_start(springtrap_on_start)
    .on_line(Main, springtrap_opff)
    .install()
    ;
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .on_start(springtrap_axe_on_start)
    .on_line(Main, springtrap_axe_opff)
    .install()
    ;
    Agent::new("ganon_phantom")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .on_start(springtrap_phantom_on_start)
    .on_line(Main, springtrap_phantom_opff)
    .install()
    ;
}