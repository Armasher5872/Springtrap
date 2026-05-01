use {
    smash::{
        app::sv_animcmd::frame,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::macros::*,
    smashline::{
        Priority::*,
        *
    },
};

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

pub fn install() {
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
}