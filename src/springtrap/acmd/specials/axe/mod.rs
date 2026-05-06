use {
    crate::common::common_func::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::frame,
        },
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

mod axe_fly;
mod axe_hit_stick;
mod axe_hit_stuck;
mod axe_stick;
mod axe_stuck;

pub fn install() {
    axe_fly::install();
    axe_hit_stick::install();
    axe_hit_stuck::install();
    axe_stick::install();
    axe_stuck::install();
}