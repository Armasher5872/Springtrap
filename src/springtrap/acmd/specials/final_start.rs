use super::*;

//Final Smash Start
unsafe extern "C" fn ssbexo_springtrap_final_smash_start_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(agent, 4.0, 30.0);
        REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04finalstart.nuanmb"), false, false);
    }
    frame(lua_state, 5.0);
    if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        if is_excute(agent) {
            SlowModule::set_whole(boma, 2, 0);
            FT_SET_FINAL_FEAR_FACE(agent, 30);
        }
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        CAM_ZOOM_OUT(agent);
        SlowModule::clear_whole(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        CAM_ZOOM_OUT(agent);
        ATTACK(agent, 0, 0, Hash40::new("top"), 35.0, 361, 100, 0, 20, 20.0, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(20.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(boma, true, false);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Final Smash Start Effect
unsafe extern "C" fn ssbexo_springtrap_final_smash_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        EffectModule::req_screen(boma, Hash40::new("bg_ganon_final"), false, false, false);
    }
    for _ in 0..8 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 15, 0, 10, 0, 0, 0, false);
        }
        wait(lua_state, 5.0);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_air_distort"), Hash40::new("top"), 0, 12, 20.0, 0, 0, 0, 2.0, true);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 59.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 62.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Final Smash Start Sound
unsafe extern "C" fn ssbexo_springtrap_final_smash_start_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        SoundModule::set_se_pitch_ratio(boma, Hash40::new("vc_ganon_special_s01"), 0.8+sv_math::randf(hash40("fighter"), 0.2));
    }
}

//Final Smash Start Expression
unsafe extern "C" fn ssbexo_springtrap_final_smash_start_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        START_INFO_FLASH_EYE(agent);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll_l"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    for _ in 0..2 {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        wait(lua_state, 10.0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_costumes())
    .game_acmd("game_finalstart", ssbexo_springtrap_final_smash_start_acmd, Low)
    .game_acmd("game_finalairstart", ssbexo_springtrap_final_smash_start_acmd, Low)
    .effect_acmd("effect_finalstart", ssbexo_springtrap_final_smash_start_effect, Low)
    .effect_acmd("effect_finalairstart", ssbexo_springtrap_final_smash_start_effect, Low)
    .sound_acmd("sound_finalstart", ssbexo_springtrap_final_smash_start_sound, Low)
    .sound_acmd("sound_finalairstart", ssbexo_springtrap_final_smash_start_sound, Low)
    .expression_acmd("expression_finalstart", ssbexo_springtrap_final_smash_start_expression, Low)
    .expression_acmd("expression_finalairstart", ssbexo_springtrap_final_smash_start_expression, Low)
    .install()
    ;
}