use {
    crate::common::{
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
        phx::Vector3f
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

mod bb_fall;
mod bb_idle;
mod chica_attack;
mod chica_fall;
mod chica_turn;
mod chica_walk;
mod foxy_attack;
mod freddy_attack;
mod freddy_fall;
mod freddy_turn;
mod freddy_walk;
mod phantom_break;
mod phantom_explode;
mod phantom_summon;

pub fn install() {
    bb_fall::install();
    bb_idle::install();
    chica_attack::install();
    chica_fall::install();
    chica_turn::install();
    chica_walk::install();
    foxy_attack::install();
    freddy_attack::install();
    freddy_fall::install();
    freddy_turn::install();
    freddy_walk::install();
    phantom_break::install();
    phantom_explode::install();
    phantom_summon::install();
}