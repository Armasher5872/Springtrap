use {
    crate::common::{
        common_func::*,
        getter_funcs::*,
        globals::*,
        springtrap_func::*,
        springtrap_var::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

mod bb_fall;
mod bb_idle;
mod foxy_attack;
mod phantom_attack;
mod phantom_break;
mod phantom_explode;
mod phantom_move;
mod phantom_summon;
mod phantom_turn;

pub fn install() {
    bb_fall::install();
    bb_idle::install();
    foxy_attack::install();
    phantom_attack::install();
    phantom_break::install();
    phantom_explode::install();
    phantom_move::install();
    phantom_summon::install();
    phantom_turn::install();
}