use super::*;

unsafe extern "C" fn springtrap_side_special_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let special_s_charge = WorkModule::get_float(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CHARGE);
    let reflector_threshold = 10.0+(20.0*special_s_charge);
    let mut damage_0 = 9.0+(9.0*special_s_charge);
    let mut damage_1 = 7.0+(7.0*special_s_charge);
    let mut damage_2 = 5.0+(5.0*special_s_charge);
    let size_0 = 4.0+(4.0*special_s_charge);
    let size_1 = 7.0+(3.0*special_s_charge);
    let size_2 = 10.0+(2.0*special_s_charge);
    let z2_coord_0 = 8.0+(8.0*special_s_charge);
    let z2_coord_1 = 8.0+(10.0*special_s_charge);
    let z2_coord_2 = 8.0+(12.0*special_s_charge);
    let saving_level = if special_s_charge > 0.5 {*FIGHTER_RYU_SAVING_LV_2} else {*FIGHTER_RYU_SAVING_LV_1};
    frame(lua_state, 3.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), size_0, 0.0, 9.5, 8.0, 0.0, 9.5, z2_coord_0, 1.0, 1.0, reflector_threshold, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        ATTACK(agent, 0, 0, Hash40::new("top"), damage_0, 20, 25, 0, 50, size_0, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(z2_coord_0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), damage_1, 20, 25, 0, 50, size_1, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(z2_coord_1), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), damage_2, 20, 25, 0, 80, size_2, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(z2_coord_2), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new("top"), damage_1, 60, 25, 0, 50, size_1, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(z2_coord_1), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 4, 0, Hash40::new("top"), damage_2, 60, 25, 0, 80, size_2, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(z2_coord_2), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_attack_level(agent.module_accessor, 0, saving_level as u8);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        damage_0 = 7.0+(7.0*special_s_charge);
        damage_1 = 5.0+(5.0*special_s_charge);
        damage_2 = 3.0+(3.0*special_s_charge);
        ATTACK(agent, 0, 0, Hash40::new("top"), damage_0, 20, 25, 0, 50, size_0, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(z2_coord_0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), damage_1, 20, 25, 0, 50, size_1, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(z2_coord_1), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), damage_2, 20, 25, 0, 80, size_2, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(z2_coord_2), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new("top"), damage_1, 60, 25, 0, 50, size_1, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(z2_coord_1), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 4, 0, Hash40::new("top"), damage_2, 60, 25, 0, 80, size_2, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(z2_coord_2), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn springtrap_side_special_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let special_s_charge = WorkModule::get_float(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CHARGE);
    let effect_scale = 0.5+(1.0*special_s_charge);
    let z_pos = 8.0+(4.0*special_s_charge);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_air_distort"), Hash40::new("top"), 0, 12, z_pos, 0, 0, 0, effect_scale, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn springtrap_side_special_attack_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let special_s_charge = WorkModule::get_float(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CHARGE);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let vc_index = if special_s_charge > 0.75 {Hash40::new("vc_ganon_special_s01")} else if special_s_charge > 0.5 {Hash40::new("se_ganon_special_s03")} else {Hash40::new("se_ganon_special_s02")};
        PLAY_SE(agent, vc_index);
        SoundModule::set_se_pitch_ratio(boma, vc_index, 0.8+sv_math::randf(hash40("fighter"), 0.2));
    }
}

unsafe extern "C" fn springtrap_side_special_attack_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 3.0);
    for _ in 0..2 {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        wait(lua_state, 10.0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_specialsattack", springtrap_side_special_attack_acmd, Low)
    .effect_acmd("effect_specialsattack", springtrap_side_special_attack_effect, Low)
    .sound_acmd("sound_specialsattack", springtrap_side_special_attack_sound, Low)
    .expression_acmd("expression_specialsattack", springtrap_side_special_attack_expression, Low)
    .game_acmd("game_specialairsattack", springtrap_side_special_attack_acmd, Low)
    .effect_acmd("effect_specialairsattack", springtrap_side_special_attack_effect, Low)
    .sound_acmd("sound_specialairsattack", springtrap_side_special_attack_sound, Low)
    .expression_acmd("expression_specialairsattack", springtrap_side_special_attack_expression, Low)
    .install()
    ;
}