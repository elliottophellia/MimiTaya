use std::{f32::consts::PI, sync::{Arc, Mutex}, time::{Instant, Duration}};
use imgui::Ui;
use mint::{Vector3, Vector2};
use lazy_static::lazy_static;
use crate::{utils::{config::{Config, CHEAT_DELAYS}, mouse::move_mouse}, ui::functions::{hotkey_index_to_io, distance_between_vec2, color_with_masked_alpha, color_u32_to_f32}, cheat::classes::{bone::{BoneIndex, aim_position_to_bone_index, BoneJointPos}, view::View}};

lazy_static! {
    pub static ref AIMBOT_TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref TOGGLE_CHANGED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref LAST_MOVED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
}

pub fn get_aimbot_toggled(config: Config) -> bool {
    match hotkey_index_to_io(config.aimbot.key) {
        Ok(aimbot_button) => {
            if config.aimbot.mode == 0 {
                return aimbot_button.is_pressed();
            } else {
                let aimbot_toggled = *AIMBOT_TOGGLED.lock().unwrap();
                let toggle_changed = *TOGGLE_CHANGED.lock().unwrap();

                if aimbot_button.is_pressed() && toggle_changed.elapsed() > Duration::from_millis(250) {
                    *AIMBOT_TOGGLED.lock().unwrap() = !aimbot_toggled;
                    *TOGGLE_CHANGED.lock().unwrap() = Instant::now();

                    return !aimbot_toggled;
                } else {
                    return aimbot_toggled;
                }
            }
        },
        Err(aimbot_key) => {
            if config.aimbot.mode == 0 {
                return aimbot_key.is_pressed();
            } else {
                let aimbot_toggled = *AIMBOT_TOGGLED.lock().unwrap();
                let toggle_changed = *TOGGLE_CHANGED.lock().unwrap();

                if aimbot_key.is_pressed() && toggle_changed.elapsed() > Duration::from_millis(250) {
                    *AIMBOT_TOGGLED.lock().unwrap() = !aimbot_toggled;
                    *TOGGLE_CHANGED.lock().unwrap() = Instant::now();
                    
                    return !aimbot_toggled;
                } else {
                    return aimbot_toggled;
                }
            }
        }
    }
}

pub fn run_aimbot(config: Config, aimbot_info: f32, window_info: ((i32, i32), (i32, i32)), game_view: View, aim_pos: Vector3<f32>) {
    if (*LAST_MOVED.lock().unwrap()).elapsed().as_millis() < CHEAT_DELAYS.aimbot.as_millis() {
        return;
    }
    
    let norm = aimbot_info;
    let smooth = config.aimbot.smooth;

    let (screen_center_x, screen_center_y) = ((window_info.1.0 / 2) as f32, (window_info.1.1 / 2) as f32);
    let mut screen_pos = Vector2 { x: 0.0, y: 0.0 };

    let mut target_x = 0.0;
    let mut target_y = 0.0;

    if !game_view.world_to_screen(aim_pos, &mut screen_pos, window_info) {
        return;
    }

    let x_diff = (screen_center_x - screen_pos.x).abs();
    let y_diff = (screen_center_y - screen_pos.y).abs();

    if x_diff <= 1.0 && y_diff <= 1.0 {
        return;
    }

    if screen_pos.x != 0.0 {
        if screen_pos.x > screen_center_x {
            target_x = -(screen_center_x - screen_pos.x);

            if smooth != 0.0 {
                target_x /= smooth;
            }
    
            if target_x + screen_center_x > screen_center_x * 2.0 {
                target_x = 0.0;
            }
        } else {
            target_x = screen_pos.x - screen_center_x;

            if smooth != 0.0 {
                target_x /= smooth;
            }
    
            if target_x + screen_center_x < 0.0 {
                target_x = 0.0;
            }
        }
    }

    if screen_pos.y != 0.0 {
        if screen_pos.y > screen_center_y {
            target_y = -(screen_center_y - screen_pos.y);

            if smooth != 0.0 {
                target_y /= smooth;
            }

            if target_y + screen_center_y > screen_center_y * 2.0 {
                target_y = 0.0;
            }
        } else {
            target_y = screen_pos.y - screen_center_y;

            if smooth != 0.0 {
                target_y /= smooth;
            }

            if target_y + screen_center_y < 0.0 {
                target_y = 0.0;
            }
        }
    }

    if smooth == 0.0 {
        *LAST_MOVED.lock().unwrap() = Instant::now();
        move_mouse(target_x as i32, target_y as i32);
        return;
    }

    let distance_ratio = norm / config.aimbot.fov;
    let speed_factor = 1.0 + (1.0 - distance_ratio);

    target_x /= smooth * speed_factor;
    target_y /= smooth * speed_factor;

    if f32::abs(target_x) < 1.0 {
        if target_x > 0.0 {
            target_x = 1.0;
        } else {
            target_x = -1.0;
        }
    }

    if f32::abs(target_y) < 1.0 {
        if target_y > 0.0 {
            target_y = 1.0;
        } else {
            target_y = -1.0;
        }
    }

    *LAST_MOVED.lock().unwrap() = Instant::now();
    move_mouse(target_x as i32, target_y as i32);
}

pub fn aimbot_check(bone_pos_list: [BoneJointPos; 30], window_width: i32, window_height: i32, aim_pos: &mut Option<Vector3<f32>>, max_aim_distance: &mut f32, b_spotted_by_mask: u64, local_b_spotted_by_mask: u64, local_player_controller_index: u64, i: u64, in_air: bool, config: Config) {
    let pos = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let bone_index = aim_position_to_bone_index(config.aimbot.bone);
    let distance_to_sight = distance_between_vec2(bone_pos_list[bone_index].screen_pos, pos);

    if config.aimbot.only_grounded && in_air {
        return;
    }

    if distance_to_sight < *max_aim_distance {
        *max_aim_distance = distance_to_sight;

        if !config.aimbot.only_visible || b_spotted_by_mask & (1 << local_player_controller_index) != 0 || local_b_spotted_by_mask & (1 << i) != 0 {
            *aim_pos = Some(bone_pos_list[bone_index].pos);

            if bone_index as usize == BoneIndex::Head as usize {
                if let Some(aim_pos) = aim_pos {
                    aim_pos.z -= -1.0;
                }
            }
        }
    }
}

pub fn render_fov_circle(ui: &mut Ui, window_width: i32, window_height: i32, fov: i32, color: (u32, u32, u32, u32), config: Config) {
    let center_point: Vector2<f32> = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let radius = (config.aimbot.fov / 180.0 * PI / 2.0).tan() / (fov as f32 / 180.0 * PI / 2.0).tan() * window_width as f32;

    if config.aimbot.fov_circle_outline_enabled {
        ui.get_background_draw_list().add_circle(center_point, radius, color_with_masked_alpha(color, 0xFF000000)).thickness(3.0).build();
    }

    ui.get_background_draw_list().add_circle(center_point, radius, color_u32_to_f32(color)).build();
}