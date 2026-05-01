use {
    crate::common::{
        getter_funcs::*,
        hooks::*,
        springtrap_func::*, 
        springtrap_var::*,
        structs::*,
    },
    smash::{
        app::{
            lua_bind::*, 
            *
        },
        lib::lua_const::*,
        phx::*,
    }
};

mod common;
mod springtrap_phantom;
mod springtrap;

pub fn install() {
    common::install();
    springtrap_phantom::install();
    springtrap::install();
}