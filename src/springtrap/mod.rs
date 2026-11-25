#![allow(static_mut_refs, unused_mut)]
use {
    crate::common::{
        globals::*,
        springtrap_func::*, 
        springtrap_var::*
    },
    param_config::*,
    smash::{
        app::{
            lua_bind::*, 
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smashline::*,
    smash_script::macros::*
};

mod acmd;
mod opff;
mod status;
mod vtable;

pub fn install() {
    let axe = clone_weapon("ganon", "axe", *WEAPON_KIND_KROOL_IRONBALL, false);
    let phantom = clone_weapon("ganon", "phantom", *WEAPON_KIND_KOOPAJR_CANNONBALL, false);
    unsafe {
        FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE = axe.generate_id;
        FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_PHANTOM = phantom.generate_id;
        WEAPON_KIND_SPRINGTRAP_AXE = axe.weapon_id;
        WEAPON_KIND_SPRINGTRAP_PHANTOM = phantom.weapon_id;
    }
    disable_kirby_copy(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd());
    disable_villager_pocket(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), 0);
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("walk_accel_mul"), 0, 0.104)); //Vanilla Value is 0.084
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("walk_accel_add"), 0, 0.06)); //Vanilla Value is 0.0315
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("walk_speed_max"), 0, 0.89)); //Vanilla Value is 0.767
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("dash_speed"), 0, 1.77)); //Vanilla Value is 1.87
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("run_accel_mul"), 0, 0.0995)); //Vanilla Value is 0.10593
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("run_accel_add"), 0, 0.044)); //Vanilla Value is 0.033
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("run_speed_max"), 0, 1.55)); //Vanilla Value is 1.34
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("jump_speed_x_mul"), 0, 0.8)); //Vanilla Value is 0.75
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("jump_speed_x_max"), 0, 1.77)); //Vanilla Value is 1.8
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("jump_initial_y"), 0, 1.0)); //Vanilla Value is 14.0195
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("jump_y"), 0, 33.03)); //Vanilla Value is 25.49
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("mini_jump_y"), 0, 9.55)); //Vanilla Value is 12.24
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("jump_aerial_y"), 0, 30.25)); //Vanilla Value is 26
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("air_accel_x_mul"), 0, 0.03)); //Vanilla Value is 0.03
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("air_speed_x_stable"), 0, 1.09)); //Vanilla Value is 0.83
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("air_brake_x"), 0, 0.0035)); //Vanilla Value is 0.015
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("air_accel_y"), 0, 0.11)); //Vanilla Value is 0.108
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("air_speed_y_stable"), 0, 1.66)); //Vanilla Value is 1.65
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("air_brake_y"), 0, 0.015)); //Vanilla Value is 0.015
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("dive_speed_y"), 0, 3.0)); //Vanilla Value is 2.64
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("weight"), 0, 119.0)); //Vanilla Value is 118
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("landing_attack_air_frame_n"), 0, 12.0));
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("landing_attack_air_frame_f"), 0, 16.0));
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("landing_attack_air_frame_b"), 0, 14.0));
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("landing_attack_air_frame_hi"), 0, 18.0));
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("landing_attack_air_frame_lw"), 0, 15.0));
    update_int_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("attack_combo_max"), 0, 3)); //Vanilla Value is 1
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("combo_attack_12_end"), 0, 30.0)); //Vanilla Value is 0
    update_float_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("combo_attack_13_end"), 0, 35.0)); //Vanilla Value is 0
    update_int_2(*FIGHTER_KIND_GANON, get_springtrap_costumes_non_acmd(), (hash40("squat_walk_type"), 0, 1)); //Vanilla Value is 0 (false)
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
}