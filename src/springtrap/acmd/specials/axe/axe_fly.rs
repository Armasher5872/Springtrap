use super::*;

//Axe Fly ACMD
unsafe extern "C" fn springtrap_axe_fly_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("have"), 7.0, 30, 45, 0, 50, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(boma, 0, 361, 45, 1.0, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("sys_hit_fire"));
    }
}

//Axe Fly Effect
unsafe extern "C" fn springtrap_axe_fly_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("have"), 0, 13, 0, 0, 0, 0, 0.5, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_springtrap_axe1"), Hash40::new("tex_springtrap_axe2"), 4, Hash40::new("have"), 0, 8.0, 0, Hash40::new("have"), 0, 14.5, 0, true, Hash40::new("null"), Hash40::new("have"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_springtrap_axe3"), Hash40::new("tex_springtrap_axe4"), 4, Hash40::new("haver"), 0, 12.0, 0, Hash40::new("haver"), 0, 4.0, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
}

//Axe Fly Sound
unsafe extern "C" fn springtrap_axe_fly_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_attackair_s01"));
    }
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("game_fly", springtrap_axe_fly_acmd, Low)
    .acmd("effect_fly", springtrap_axe_fly_effect, Low)
    .acmd("sound_fly", springtrap_axe_fly_sound, Low)
    .install()
    ;
}