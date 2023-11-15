use mint::{Vector3, Vector2};
use crate::cheat::classes::{entity::Entity, offsets::PAWN_OFFSETS, view::View};
use crate::utils::{config::Config, process_manager::{read_memory_auto, trace_address}};

pub fn is_enemy_at_crosshair(window_info: ((i32, i32), (i32, i32)), local_entity_pawn_address: u64, local_entity_pawn_team_id: i32, game_address_entity_list: u64, game_view: View, config: Config) -> (bool, bool) {
    let mut u_handle: u32 = 0;
    
    if !read_memory_auto(local_entity_pawn_address + (*PAWN_OFFSETS.lock().unwrap()).ent_index as u64, &mut u_handle) {
        return (false, false);
    }

    if !read_memory_auto(local_entity_pawn_address + (*PAWN_OFFSETS.lock().unwrap()).ent_index as u64, &mut u_handle) {
        return (false, false);
    }

    let list_entry: u64 = trace_address(game_address_entity_list, &[0x8 * (u_handle >> 9) + 0x10, 0x0]);

    if list_entry == 0 {
        return (false, false);
    }

    let mut pawn_address: u64 = 0;

    if !read_memory_auto(list_entry + 0x78 * (u_handle & 0x1FF) as u64, &mut pawn_address) {
        return (false, false);
    }

    let mut entity = Entity::default();

    if !entity.update_pawn(pawn_address, window_info, game_view) {
        return (false, false);
    }

    let allow_shoot = {
        if config.misc.enabled && config.misc.exclude_team {
            local_entity_pawn_team_id != entity.pawn.team_id && entity.pawn.health > 0
        } else {
            entity.pawn.health > 0
        }
    };

    return (true, allow_shoot);
}

pub fn is_enemy_in_fov(config: Config, aim_pos: Vector3<f32>, camera_pos: Vector3<f32>, view_angle: Vector2<f32>) -> Option<(f32, f32)> {
    let pos = Vector3 { x: aim_pos.x - camera_pos.x, y: aim_pos.y - camera_pos.y, z: aim_pos.z - camera_pos.z };
    let distance = f32::sqrt(f32::powf(pos.x, 2.0) + f32::powf(pos.y, 2.0));
    let yaw = f32::atan2(pos.y, pos.x) * 57.295779513 - view_angle.y;
    let pitch = -f32::atan(pos.z / distance) * 57.295779513 - view_angle.x;
    let norm = f32::sqrt(f32::powf(yaw, 2.0) + f32::powf(pitch, 2.0));

    if norm > config.aimbot.fov {
        return None;
    }

    return Some((yaw, pitch));
}