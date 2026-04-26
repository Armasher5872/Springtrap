use super::*;

unsafe extern "C" fn springtrap_special_n_recall_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn springtrap_special_n_recall_loop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    0.into()
}

unsafe extern "C" fn springtrap_special_n_recall_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let active_axe_count = ArticleModule::get_active_num(fighter.module_accessor, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE);
    let active_axe_battle_object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_ACTIVE_AXE_BATTLE_OBJECT_ID);
    let mut should_end = true;
    for idx in 0..active_axe_count {
        println!("Active Axe Battle Object ID: {}", active_axe_battle_object_id);
        println!("Is Axe Active: {}", sv_battle_object::is_active(active_axe_battle_object_id as u32));
        if sv_battle_object::is_active(active_axe_battle_object_id as u32) {
            let axe_article = get_article_from_no(fighter.module_accessor, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE, idx as i32);
            let axe_battle_object_id = smash::app::lua_bind::Article::get_battle_object_id(axe_article) as u32;
            println!("Axe Battle Object ID: {}", axe_battle_object_id);
            if axe_battle_object_id == active_axe_battle_object_id as u32 {
                let axe_boma = sv_battle_object::module_accessor(axe_battle_object_id as u32);
                let axe_status_kind = StatusModule::status_kind(axe_boma);
                println!("Axe Status Kind: {}", axe_status_kind);
                if [*WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK].contains(&axe_status_kind) {
                    StatusModule::change_status_request_from_script(axe_boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, false);
                    should_end = false;
                    break;
                }
            }
        }
    }
    if should_end {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_recall_loop"), L2CValue::Hash40s("special_air_n_recall_loop"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(springtrap_special_n_recall_loop_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_special_n_recall_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let axe_battle_object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_ACTIVE_AXE_BATTLE_OBJECT_ID);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_recall_loop"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_recall_loop"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if axe_battle_object_id == *BATTLE_OBJECT_ID_INVALID || !sv_battle_object::is_active(axe_battle_object_id as u32) {
        fighter.change_status(FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn springtrap_special_n_recall_loop_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_special_n_recall_loop_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_special_n_recall_loop_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP, springtrap_special_n_recall_loop_pre_status)
    .status(Init, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP, springtrap_special_n_recall_loop_init_status)
    .status(Main, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP, springtrap_special_n_recall_loop_main_status)
    .status(Exec, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP, springtrap_special_n_recall_loop_exec_status)
    .status(End, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP, springtrap_special_n_recall_loop_end_status)
    .status(Exit, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP, springtrap_special_n_recall_loop_exit_status)
    .install()
    ;
}