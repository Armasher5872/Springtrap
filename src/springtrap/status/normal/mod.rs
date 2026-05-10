use {
    crate::common::{
        common_func::*,
        getter_funcs::*,
        globals::*,
        springtrap_var::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::Vector3f
    },
    smashline::*,
    smash_script::macros::*,
};

mod attack_s4_hold;
mod attack_s4_start;
mod attack_s4;
mod throw;

pub fn install() {
    attack_s4_hold::install();
    attack_s4_start::install();
    attack_s4::install();
    throw::install();
}