use {
    crate::common::{
        common_func::*,
        getter_funcs::*,
        hooks::*,
        module_init::*,
        springtrap_func::*, 
        springtrap_var::*,
        structs::*,
    },
    smash::{
        app::{
            lua_bind::*, 
            *
        },
        hash40,
        lib::lua_const::*,
        phx::*,
    }
};

mod springtrap_axe;
mod springtrap_phantom;
mod springtrap;

pub fn install() {
    springtrap_axe::install();
    springtrap_phantom::install();
    springtrap::install();
}