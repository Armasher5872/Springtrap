use {
    crate::common::{
        common_func::*,
        getter_funcs::*,
        springtrap_var::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::{
        Priority::*,
        *
    },
};

mod axe;
mod final_start;
mod phantoms;
mod special_hi_end;
mod special_hi_move;
mod special_hi;
mod special_lw;
mod special_n_high_fire;
mod special_n_hold;
mod special_n_low_fire;
mod special_n_recall_end;
mod special_n_recall_loop;
mod special_n_start;
mod special_s_attack;
mod special_s_hold;
mod special_s;

pub fn install() {
    axe::install();
    final_start::install();
    phantoms::install();
    special_hi_end::install();
    special_hi_move::install();
    special_hi::install();
    special_lw::install();
    special_n_high_fire::install();
    special_n_hold::install();
    special_n_low_fire::install();
    special_n_recall_end::install();
    special_n_recall_loop::install();
    special_n_start::install();
    special_s_attack::install();
    special_s_hold::install();
    special_s::install();
}