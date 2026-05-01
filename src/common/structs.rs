//Credited to WuBoyTH
use super::*;

//Collision Log
#[repr(C)]
pub struct CollisionLog {
    pub _next: *mut CollisionLog,
    pub _end: *mut CollisionLog,
    pub location: Vector3f,
    pub _padding_0: u32,
    pub _padding_1: u32,
    pub opponent_battle_object_id: u32,
    pub _padding_2: [u8;7],
    pub _collision_kind: u8,
    pub _receiver_part_id: u8,
    pub _collider_part_id: u8,
    pub _receiver_id: u8,
    pub _collider_id: u8,
    pub _padding_3: [u8;10]
}

#[repr(C)]
pub struct CollisionLogScuffed {
    pub x00: *const u64,
    pub x08: *const u64,
    pub location: Vector3,
    pub x20: u8,
    pub x21: u8,
    pub x22: u8,
    pub x23: u8,
    pub opponent_object_id: u32,
    pub opponent_object_category: u8,
    pub x29: u8,
    pub x2a: u8,
    pub x2b: u8,
    pub x2c: u8,
    pub x2d: u8,
    pub x2e: u8,
    pub collision_kind: u8,
    pub receiver_part_id: u8,
    pub collider_part_id: u8,
    pub receiver_id: u8,
    pub collider_id: u8,
    pub x35: bool
}

#[repr(C)]
pub struct ShieldAttackCollisionEvent {
    pub vtable: u64,
    pub shield_id: u32,
    pub unk: u8,
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub attack_module: u64,
    pub raw_power: f32,
    pub real_power: f32,
    pub collision_log: *const CollisionLogScuffed,
    pub group_index: i32,
    pub pos_x: f32,
    pub lr: f32,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: u8,
}

//Shield Data, reflectorboxes

#[repr(C)]
pub struct ShieldData {
    pub offset: Vector3,
    pub offset2: Vector3,
    pub size: f32,
    pub x24: u32,
    pub joint: Hash40,
    pub shape: u8,
    pub shield_type: u8,
    pub x32: u16,
    pub x34: u32,
    pub x38: u64,
    pub status: i32,
    pub x44: u32,
    pub x48: u64,
}

impl ShieldData {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        size: f32,
        joint: Hash40,
        shape: u8,
        shield_type: u8
    ) -> Self {
        ShieldData {
            offset: Vector3{vec: [x, y, z]},
            offset2: Vector3{vec: [x2, y2, z2]},
            size: size,
            x24: 0,
            joint: joint,
            shape: shape,
            shield_type: shield_type,
            x32: 0,
            x34: 0,
            x38: 0,
            status: 0,
            x44: 0,
            x48: 0,
        }
    }
}

//Shield Datas 2, reflectors
#[repr(C)]
pub struct ShieldDatas2 {
    pub datas: [ShieldData; 8]
}

impl ShieldDatas2 {
    pub fn new() -> ShieldDatas2 {
        ShieldDatas2 { datas: [
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Hash40::new_raw(0), 0, 0)
        ] }
    }
    pub fn add(mut self, shield_data: ShieldData, index: usize) -> ShieldDatas2 {
        self.datas[index] = shield_data;
        self
    }
}

//Shield Group Resource 2, reflectors
#[repr(C)]
pub struct ShieldGroupResource2 {
    pub shield_datas: *const ShieldDatas2,
    pub count: u64,
    pub attack_mul: f32,
    pub speed_mul: f32,
    pub attack_limit: f32,
    pub life_mul: f32,
    pub no_m_ball: bool,
    pub front: u8
}

impl ShieldGroupResource2 {
    pub fn new(
        shield_datas: *const ShieldDatas2,
        count: u64,
        attack_mul: f32,
        speed_mul: f32,
        attack_limit: f32,
        life_mul: f32,
        no_m_ball: bool,
        front: u8
    ) -> Self {
        ShieldGroupResource2 {
            shield_datas: shield_datas,
            count: count,
            attack_mul: attack_mul,
            speed_mul: speed_mul,
            attack_limit: attack_limit,
            life_mul: life_mul,
            no_m_ball: no_m_ball,
            front: front
        }
    }
}