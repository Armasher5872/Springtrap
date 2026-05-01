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
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::Vector3f
    },
    smash_script::*,
    smashline::*,
};

mod axe_fly;
mod axe_hit_stick;
mod axe_recall;
mod axe_stick;

pub fn install() {
    axe_fly::install();
    axe_hit_stick::install();
    axe_recall::install();
    axe_stick::install();
}