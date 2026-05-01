use {
    crate::common::{
        getter_funcs::*,
        globals::*,
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

mod special_hi_end;
mod special_hi_move;
mod special_hi;
mod special_lw;
mod special_n_charge_loop;
mod special_n_high_fire;
mod special_n_low_fire;
mod special_n_recall_end;
mod special_n_recall_loop;
mod special_n_recall_start;
mod special_n;
mod special_s_attack;
mod special_s_hold;
mod special_s;

pub fn install() {
    special_hi_end::install();
    special_hi_move::install();
    special_hi::install();
    special_lw::install();
    special_n_charge_loop::install();
    special_n_high_fire::install();
    special_n_low_fire::install();
    special_n_recall_end::install();
    special_n_recall_loop::install();
    special_n_recall_start::install();
    special_n::install();
    special_s_attack::install();
    special_s_hold::install();
    special_s::install();
}