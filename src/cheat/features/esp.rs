use std::f32::consts::PI;
use imgui::{Ui, ImColor32};
use mint::{Vector3, Vector2, Vector4};
use crate::{cheat::classes::{bone::{BoneJointPos, bone_joint_list, BoneIndex}, view::View}, utils::config::Config, ui::functions::{color_u32_to_f32, color_with_masked_alpha, rectangle, distance_between_vec3, stroke_text, mix_colors, color_with_alpha}};

pub fn render_bones(ui: &mut Ui, bone_pos_list: [BoneJointPos; 30], config: Config) {
    let mut previous: BoneJointPos = BoneJointPos { pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, screen_pos: Vector2 { x: 0.0, y: 0.0 }, is_visible: false };

    for bone_joint in bone_joint_list::LIST {
        previous.pos = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        
        for joint in bone_joint {
            let current = bone_pos_list[joint as usize];

            if (previous.pos == Vector3 { x: 0.0, y: 0.0, z: 0.0 }) {
                previous = current;
                continue;
            }

            if previous.is_visible && current.is_visible {
                ui.get_background_draw_list().add_line(previous.screen_pos, current.screen_pos, color_u32_to_f32(config.esp.skeleton_color)).thickness(1.3).build();
            }

            previous = current;
        }
    }
}

pub fn render_head(ui: &mut Ui, bone_pos_list: [BoneJointPos; 30], config: Config) {
    let head = bone_pos_list[BoneIndex::Head as usize];
    let neck = bone_pos_list[BoneIndex::Neck0 as usize];
    
    let center_pos = head.screen_pos;
    let radius = (head.screen_pos.y - neck.screen_pos.y).abs() + 2.0;

    if radius > 200.0 {
        return;
    }

    if config.esp.head_mode == 0 {
        ui.get_background_draw_list().add_circle(center_pos, radius, color_u32_to_f32(config.esp.head_color)).thickness(1.2).build();
    } else {
        ui.get_background_draw_list().add_circle(center_pos, radius + 1.0, color_with_masked_alpha(config.esp.head_color, 0xFF000000)).filled(true).build();
        ui.get_background_draw_list().add_circle(center_pos, radius, color_u32_to_f32(config.esp.head_color)).filled(true).build();
    }
}

pub fn render_eye_ray(ui: &mut Ui, bone_pos_list: [BoneJointPos; 30], view_angle: Vector2<f32>, config: Config, view: View, window_info: ((i32, i32), (i32, i32))) {
    let line_length = (view_angle.x * PI / 180.0).cos() * 30.0;
    let head = bone_pos_list[BoneIndex::Head as usize];
    let mut pos = Vector2 { x: 0.0, y: 0.0 };

    if !view.world_to_screen(Vector3 { x: head.pos.x + (view_angle.y * PI / 180.0).cos() * line_length, y: head.pos.y + (view_angle.y * PI / 180.0).sin() * line_length, z: head.pos.z - (view_angle.x * PI / 180.0).sin() * 30.0 }, &mut pos, window_info) {
        return;
    }

    ui.get_background_draw_list().add_line(head.screen_pos, pos, color_u32_to_f32(config.esp.eye_ray_color)).thickness(1.3).build();
}

fn calculate_size(initial: Vector2<f32>, distance: f32, max_distance: f32) -> Vector2<f32> {
    let scale = 1.0 - (distance / max_distance);
    return Vector2 { x: initial.x * scale, y: initial.y * scale };
}

pub fn get_2d_box_non_player(size: Vector2<f32>, screen_pos: Vector2<f32>, distance: f32) -> Vector4<f32> {
    let size = calculate_size(size, distance, 300.0);
    let pos = Vector2 { x: screen_pos.x - size.x / 2.0, y: screen_pos.y - size.y / 2.0 };
    return Vector4 { x: pos.x, y: pos.y, z: size.x, w: size.y };
}

pub fn get_2d_box(bone_pos_list: [BoneJointPos; 30], screen_pos: Vector2<f32>) -> Vector4<f32> {
    let head = bone_pos_list[BoneIndex::Head as usize];
    let size = Vector2 { x: ((screen_pos.y - head.screen_pos.y) * 1.09) * 0.6, y: (screen_pos.y - head.screen_pos.y) * 1.09 };
    let pos = Vector2 { x: screen_pos.x - size.x / 2.0, y: head.screen_pos.y - size.y * 0.08 };

    return Vector4 { x: pos.x, y: pos.y, z: size.x, w: size.y };
}

pub fn get_2d_bone_rect(bone_pos_list: [BoneJointPos; 30]) -> Vector4<f32> {
    let mut min = bone_pos_list[0].screen_pos;
    let mut max = bone_pos_list[0].screen_pos;

    for joint in bone_pos_list {
        if !joint.is_visible {
            continue;
        }

        min.x = joint.screen_pos.x.min(min.x);
        min.y = joint.screen_pos.y.min(min.y);

        max.x = joint.screen_pos.x.max(max.x);
        max.y = joint.screen_pos.y.max(max.y);
    }

    return Vector4 { x: min.x, y: min.y, z: (max.x - min.x), w: (max.y - min.y) };
}

pub fn render_snap_line(ui: &mut Ui, rect: Vector4<f32>, config: Config, window_width: i32, window_height: i32) {
    let from_line = Vector2 { x: rect.x + rect.z / 2.0, y: rect.y };
    let to_line = {
        if config.esp.snap_line_mode == 0 {
            Vector2 { x: window_width as f32 / 2.0, y: 0.0 }
        } else if config.esp.snap_line_mode == 1 {
            Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 }
        } else {
            Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 }
        }
    };

    ui.get_background_draw_list().add_line(from_line, to_line, color_u32_to_f32(config.esp.snap_line_color)).thickness(1.2).build();
}

pub fn render_box_bomb(ui: &mut Ui, rect: Vector4<f32>, config: Config) {
    rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_with_masked_alpha(config.esp.bomb_color, 0xFF000000).into(), 3.0, config.esp.bomb_rounding, false);
    rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_u32_to_f32(config.esp.bomb_color).into(), 1.3, config.esp.bomb_rounding, false);

    if config.esp.filled_bomb_enabled {
        rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_with_alpha(config.esp.filled_bomb_color, config.esp.filled_bomb_alpha).into(), 1.0, config.esp.bomb_rounding, true);
    }
}

pub fn render_box(ui: &mut Ui, rect: Vector4<f32>, b_spotted_by_mask: u64, local_b_spotted_by_mask: u64, local_player_controller_index: u64, i: u64, config: Config) {
    rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_with_masked_alpha(config.esp.box_color, 0xFF000000).into(), 3.0, config.esp.box_rounding, false);
    
    if config.esp.box_target_enabled && (b_spotted_by_mask & (1 << local_player_controller_index) != 0 || local_b_spotted_by_mask & (1 << i) != 0) {
        rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_u32_to_f32(config.esp.box_target_color).into(), 1.3, config.esp.box_rounding, false);
    } else {
        rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_u32_to_f32(config.esp.box_color).into(), 1.3, config.esp.box_rounding, false);
    }

    if config.esp.filled_box_enabled {
        rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_with_alpha(config.esp.filled_box_color, config.esp.filled_box_alpha).into(), 1.0, config.esp.box_rounding, true);
    }
}

pub fn render_weapon_name(ui: &mut Ui, weapon_name: &str, rect: Vector4<f32>, config: Config) {
    stroke_text(ui, weapon_name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y + rect.w }, color_u32_to_f32(config.esp.weapon_name_color).into(), true);
}

pub fn render_distance(ui: &mut Ui, pawn_pos: Vector3<f32>, local_pawn_pos: Vector3<f32>, rect: Vector4<f32>, config: Config) {
    let distance = distance_between_vec3(pawn_pos, local_pawn_pos) as u32 / 100;
    stroke_text(ui, format!("{}m", distance), Vector2 { x: rect.x + rect.z + 4.0, y: rect.y }, color_u32_to_f32(config.esp.distance_color).into(), false);
}

pub fn render_bomb_name(ui: &mut Ui, name: &str, rect: Vector4<f32>, config: Config) {
    stroke_text(ui, name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y - 13.0 - 14.0 }, color_u32_to_f32(config.esp.bomb_color).into(), true);
}

pub fn render_name(ui: &mut Ui, name: &str, rect: Vector4<f32>, config: Config) {
    if config.esp.health_bar_enabled && config.esp.health_bar_mode == 1 {
        stroke_text(ui, name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y - 13.0 - 14.0 }, color_u32_to_f32(config.esp.name_color).into(), true);
    } else {
        stroke_text(ui, name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y - 14.0 }, color_u32_to_f32(config.esp.name_color).into(), true);
    }
}

pub fn render_health_bar(ui: &mut Ui, current_health: f32, rect_pos: Vector2<f32>, rect_size: Vector2<f32>, config: Config) {
    let max_health = 100.0;

    let background_color = ImColor32::from_rgba(90, 90, 90, 220);
    let frame_color = ImColor32::from_rgba(45, 45, 45, 220);

    let first_stage_color = ImColor32::from(color_u32_to_f32(config.esp.health_bar_first_color));
    let second_stage_color = ImColor32::from(color_u32_to_f32(config.esp.health_bar_second_color));
    let third_stage_color = ImColor32::from(color_u32_to_f32(config.esp.health_bar_third_color));

    let in_range = |value: f32, min: f32, max: f32| -> bool {
        value > min && value <= max
    };

    let proportion = current_health / max_health;
    let (height, width) = ((rect_size.y * proportion), (rect_size.x * proportion));
    let color = {
        if in_range(proportion, 0.5, 1.0) {
            mix_colors(first_stage_color, second_stage_color, proportion.powf(2.5) * 3.0 - 1.0)
        } else {
            mix_colors(second_stage_color, third_stage_color, proportion.powf(2.5) * 4.0)
        }
    };
    
    rectangle(ui, rect_pos, rect_size, background_color, 1.0, config.esp.health_bar_rounding, true);
    
    if config.esp.health_bar_mode == 0 {
        // Vertical
        ui.get_background_draw_list().add_rect(Vector2 { x: rect_pos.x, y: rect_pos.y + rect_size.y - height }, Vector2 { x: rect_pos.x + rect_size.x, y: rect_pos.y + rect_size.y }, color).filled(true).rounding(config.esp.health_bar_rounding as f32).build();
    } else {
        // Horizontal
        ui.get_background_draw_list().add_rect(rect_pos, Vector2 { x: rect_pos.x + width, y: rect_pos.y + rect_size.y }, color).filled(true).rounding(config.esp.health_bar_rounding as f32).build();
    }

    rectangle(ui, rect_pos, rect_size, frame_color, 1.0, config.esp.health_bar_rounding, false);
}

pub fn render_bomb(ui: &mut Ui, pos: Vector3<f32>, local_pawn_pos: Vector3<f32>, screen_pos: Vector2<f32>, bomb_site: &str, config: Config) {
    let distance = distance_between_vec3(pos, local_pawn_pos) as u32 / 100;
    let rect = get_2d_box_non_player(Vector2 { x: 20.0, y: 20.0 }, screen_pos, distance as f32);

    render_box_bomb(ui, rect, config);

    if config.esp.name_enabled {
        render_bomb_name(ui, format!("Bomb ({})", bomb_site).as_str(), rect, config);
    }

    if config.esp.distance_enabled {
        render_distance(ui, pos, local_pawn_pos, rect, config);
    }
}