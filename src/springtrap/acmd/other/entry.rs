use super::*;

unsafe extern "C" fn springtrap_entry_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let current_frame = MotionModule::frame(boma)-60.0;
    let pos = PostureModule::pos(boma);
    let effect_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.8);
        BURN_COLOR(agent, 1.0, 0.2, 0.0, 0.4);
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
        let portal = EffectModule::req(boma, Hash40::new("springtrap_portal"), &Vector3f{x: (*pos).x, y: (*pos).y+0.1, z: 0.0}, &Vector3f{x: 90.0, y: 0.0, z: 90.0}, 0.5, 0, -1, false, 0);
        EffectModule::set_rot(boma, portal as u32, &Vector3f{x: 90.0, y: 0.0, z: 90.0});
        WorkModule::set_int(agent.module_accessor, portal as i32, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
        EFFECT(agent, Hash40::new("springtrap_axe_ground_crack_stuck"), Hash40::new("top"), 6, -1, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.15);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        BURN_COLOR_FRAME(agent, 30, 1.0, 0.2, 0.0, 0);
        FLASH_FRM(agent, 30, 0, 0, 0, 0);
    }
    for _ in 0..30 {
        if is_excute(agent) {
            EffectModule::set_scale(boma, effect_id as u32, &Vector3f{x: 0.5-(current_frame/60.0), y: 0.5-(current_frame/60.0), z: 0.5-(current_frame/60.0)});
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 91.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
        ColorBlendModule::set_disable_camera_depth_influence(boma, false);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
        EFFECT_OFF_KIND(agent, Hash40::new("springtrap_portal"), true, true);
    }
}

unsafe extern "C" fn springtrap_entry_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_h01"));
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
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 19);
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 10);
    }
    frame(lua_state, 96.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("effect_entryl", springtrap_entry_effect, Low)
    .acmd("sound_entryl", springtrap_entry_sound, Low)
    .acmd("expression_entryl", springtrap_entry_expression, Low)
    .acmd("effect_entryr", springtrap_entry_effect, Low)
    .acmd("sound_entryr", springtrap_entry_sound, Low)
    .acmd("expression_entryr", springtrap_entry_expression, Low)
    .install()
    ;
}