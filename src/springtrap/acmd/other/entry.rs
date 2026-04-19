use super::*;

unsafe extern "C" fn springtrap_entry_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let pos = PostureModule::pos(boma);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.8);
        BURN_COLOR(agent, 1.0, 0.2, 0.0, 0.4);
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
        EffectModule::set_rot(boma, EffectModule::req(boma, Hash40::new("springtrap_portal"), &Vector3f{x: (*pos).x, y: (*pos).y+0.1, z: 0.0}, &Vector3f{x: 90.0, y: 0.0, z: 90.0}, 0.5, 0, -1, false, 0) as u32, &Vector3f{x: 90.0, y: 0.0, z: 90.0});
        EFFECT(agent, Hash40::new("springtrap_axe_ground_crack_stuck"), Hash40::new("top"), 6, -1, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.15);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        BURN_COLOR_FRAME(agent, 30, 1.0, 0.2, 0.0, 0);
        FLASH_FRM(agent, 30, 0, 0, 0, 0);
        EFFECT_OFF_KIND(agent, Hash40::new("springtrap_portal"), true, true);
    }
    wait(lua_state, 30.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
        ColorBlendModule::set_disable_camera_depth_influence(boma, false);
    }
}

unsafe extern "C" fn springtrap_entry_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
    frame(lua_state, 95.0);
    if is_excute(agent) {
        let vc_index = if sv_math::randf(hash40("fighter"), 1.0) > 0.5 {Hash40::new("vc_ganon_appeal_h01")} else {Hash40::new("vc_ganon_attackhard_h01")};
        let scream = SoundModule::play_se(boma, vc_index, true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, scream as i32, 0.8, 0);
    }
}

unsafe extern "C" fn springtrap_entry_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 1);
    }
    frame(lua_state, 140.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 1);
    }
    frame(lua_state, 171.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 1);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .effect_acmd("effect_entryl", springtrap_entry_effect, Low)
    .sound_acmd("sound_entryl", springtrap_entry_sound, Low)
    .expression_acmd("expression_entryl", springtrap_entry_expression, Low)
    .effect_acmd("effect_entryr", springtrap_entry_effect, Low)
    .sound_acmd("sound_entryr", springtrap_entry_sound, Low)
    .expression_acmd("expression_entryr", springtrap_entry_expression, Low)
    .install()
    ;
}