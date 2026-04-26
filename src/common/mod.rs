use {
    crate::{
        common::{
            common_func::*,
            springtrap_var::*,
        },
        MARKED_COLORS
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValue,
            LuaConst,
            lua_const::*,
        },
        lua2cpp::*,
        phx::{
            Hash40,
            Vector3f,
            Vector4f
        }
    },
    smash2::cpp::simd::*,
    smash_script::{
        macros::*,
        *
    },
};

pub mod common_func;
pub mod globals;
pub mod springtrap_func;
pub mod springtrap_var;