use {
    crate::common::{
        getter_funcs::*,
        globals::*,
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
};

mod attack_s4_hold;
mod attack_s4_start;
mod attack_s4;

pub fn install() {
    attack_s4_hold::install();
    attack_s4_start::install();
    attack_s4::install();
}