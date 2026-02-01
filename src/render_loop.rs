use hudhook::ImguiRenderLoop;
use imgui::Condition;
use mem_rs::memory::ReadWrite;
use std::sync::{Arc, Mutex};

use crate::config::{Config, string_to_imgui_key};
use crate::memory::constants::{CharData2, CharPosData};
use crate::memory::{Ds1, ds1};
use crate::ui::Bonfire;
use crate::ui::DebugInfo;
use crate::ui::Items;
#[cfg(windows)]
use windows_sys::Win32::Foundation::RECT;
#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowRect, SetCursorPos,
};

use crate::ui::{self, Player};

static mut DS1: Option<Arc<Mutex<Ds1>>> = None;

pub fn get_ds1_instance() -> Arc<Mutex<Ds1>> {
    unsafe {
        if DS1.is_none() {
            DS1 = Some(Arc::new(Mutex::new(Ds1::new())));
        }
        return Arc::clone(DS1.as_mut().unwrap());
    };
}

pub struct RenderLoop {
    config: Arc<Mutex<Config>>,
    no_stamina_consume: bool,
    infinite_magic: bool,
    infinite_goods: bool,
    player_hide: bool,
    player_silence: bool,
    no_death: bool,
    player_exterminate: bool,
    no_damage: bool,
    no_hit: bool,
    no_attack: bool,
    no_move: bool,
    no_update_ai: bool,
    disable_collision: bool,
    no_gravity: bool,
    draw_hit: bool,
    draw_direction: bool,
    draw_counter: bool,
    draw_stable_pos: bool,
    stored_positions: [(f32, f32, f32, f32, i32); 3],
    input_was_disabled: bool,
    show_console: bool,

    stored_bonfire: i32,
    selected_bonfire_id: i32,
    bonfire_search: String,
    menu_open: bool,
    selected_item_index: usize,
    item_quantity: i32,
    item_search: String,
    selected_ring_index: usize,
    ring_quantity: i32,
    ring_search: String,
    selected_weapon_index: usize,
    weapon_quantity: i32,
    weapon_search: String,
    weapon_upgrade_level: i32,
    selected_infusion_index: usize,
    selected_armor_index: usize,
    armor_quantity: i32,
    armor_search: String,
    armor_upgrade_level: i32,
    give_item_type: usize, // 0 = items, 1 = rings, 2 = weapons, 3 = armor
    show_animation_popup: bool,
    debug_info: crate::ui::DebugInfo,
    anim_speed: f32,
    saved_anim_speed: f32,
    anim_speed_toggled: bool,
    last_flag_sync_time: std::time::Instant,
    positions_header_open: bool,
    stats_header_open: bool,
    give_item_header_open: bool,
    header_reset_counter: u32,
}
impl RenderLoop {
    pub fn new() -> Self {
        RenderLoop {
            config: Arc::new(Mutex::new(Config::load_or_default())),
            no_stamina_consume: false,
            infinite_magic: false,
            infinite_goods: false,
            player_hide: false,
            player_silence: false,
            no_death: false,
            player_exterminate: false,
            no_damage: false,
            no_hit: false,
            no_attack: false,
            no_move: false,
            no_update_ai: false,
            disable_collision: false,
            no_gravity: false,
            draw_hit: false,
            draw_direction: false,
            draw_counter: false,
            draw_stable_pos: false,
            stored_positions: [(0.0, 0.0, 0.0, 0.0, 0); 3],
            input_was_disabled: false,
            show_console: false,
            stored_bonfire: 0,
            selected_bonfire_id: -1,
            bonfire_search: String::new(),
            menu_open: false,
            selected_item_index: 0,
            item_quantity: 1,
            item_search: String::new(),
            selected_ring_index: 0,
            ring_quantity: 1,
            ring_search: String::new(),
            selected_weapon_index: 0,
            weapon_quantity: 1,
            weapon_search: String::new(),
            weapon_upgrade_level: 0,
            selected_infusion_index: 0,
            selected_armor_index: 0,
            armor_quantity: 1,
            armor_search: String::new(),
            armor_upgrade_level: 0,
            give_item_type: 0,
            show_animation_popup: false,
            debug_info: crate::ui::DebugInfo::new(),
            anim_speed: 1.0,
            saved_anim_speed: 1.0,
            anim_speed_toggled: false,
            last_flag_sync_time: std::time::Instant::now(),
            positions_header_open: false,
            stats_header_open: false,
            give_item_header_open: false,
            header_reset_counter: 0,
        }
    }
}

impl ImguiRenderLoop for RenderLoop {
    fn render(&mut self, ui: &mut imgui::Ui) {
        let instance = get_ds1_instance();
        let mut ds1 = instance.lock().unwrap();

        // Check if toggle_menu key is pressed
        {
            let config = self.config.lock().unwrap();
            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_menu) {
                if ui.is_key_pressed(key) {
                    let was_open = self.menu_open;
                    self.menu_open = !self.menu_open;
                    
                    // Reset headers when reopening the menu
                    if !was_open && self.menu_open {
                        self.positions_header_open = false;
                        self.stats_header_open = false;
                        self.give_item_header_open = false;
                        // Increment counter to create new unique IDs and reset imgui state
                        self.header_reset_counter = self.header_reset_counter.wrapping_add(1);
                    }
                }
            }
        } // Drop config lock

        // Check if user is interacting with any UI or if menu is open
        let io = ui.io();
        let ui_wants_input = io.want_capture_keyboard || io.want_text_input;

        // Disable game input when menu is open OR when actively typing/interacting with UI
        if ui_wants_input || self.menu_open {
            ds1.input_state.write_u8_rel(None, 0x0);
            if !self.input_was_disabled {
                self.input_was_disabled = true;
            }
        } else {
            ds1.input_state.write_u8_rel(None, 0x1);
            if self.input_was_disabled {
                self.input_was_disabled = false;
            }
        }

        // Lock config once for styling and keybind checks
        let config = self.config.lock().unwrap();
        
        // Set custom button colors from config
        let button_color = config.colors.button.to_float4();
        let button_hovered = config.colors.button_hovered.to_float4();
        let button_active = config.colors.button_active.to_float4();
        let text_color = config.colors.text.to_float4();

        let _text_style = ui.push_style_color(imgui::StyleColor::Text, text_color);
        let _text_disabled_style =
            ui.push_style_color(imgui::StyleColor::TextDisabled, [0.0, 0.0, 0.0, 1.0]);
        let _border_style = ui.push_style_color(imgui::StyleColor::Border, [0.0, 0.0, 0.0, 1.0]);
        let _border_shadow_style =
            ui.push_style_color(imgui::StyleColor::BorderShadow, [0.0, 0.0, 0.0, 0.5]);
        let _button_style = ui.push_style_color(imgui::StyleColor::Button, button_color);
        let _button_hovered_style =
            ui.push_style_color(imgui::StyleColor::ButtonHovered, button_hovered);
        let _button_active_style =
            ui.push_style_color(imgui::StyleColor::ButtonActive, button_active);
        let _header_style = ui.push_style_color(imgui::StyleColor::Header, button_color);
        let _header_hovered_style =
            ui.push_style_color(imgui::StyleColor::HeaderHovered, button_hovered);
        let _header_active_style =
            ui.push_style_color(imgui::StyleColor::HeaderActive, button_active);
        let _title_style = ui.push_style_color(imgui::StyleColor::TitleBg, button_color);
        let _title_active_style =
            ui.push_style_color(imgui::StyleColor::TitleBgActive, button_color);
        let _title_collapsed_style =
            ui.push_style_color(imgui::StyleColor::TitleBgCollapsed, button_active);
        let _frame_bg_style = ui.push_style_color(imgui::StyleColor::FrameBg, button_hovered);
        let _frame_bg_hovered_style =
            ui.push_style_color(imgui::StyleColor::FrameBgHovered, button_hovered);
        let _frame_bg_active_style =
            ui.push_style_color(imgui::StyleColor::FrameBgActive, button_hovered);
        let _check_mark_style =
            ui.push_style_color(imgui::StyleColor::CheckMark, [1.0, 0.0, 0.0, 1.0]);
        let _text_input_style = ui.push_style_color(imgui::StyleColor::Text, [1.0, 1.0, 1.0, 1.0]);

        // Process keybinds unless actively typing in input fields
        if !ui_wants_input {
            // Instantiate player once for all keybinds that need position data
            // This is lazy - only created if a position-related keybind is pressed
            let mut player_for_keybinds: Option<Player> = None;

            if let Some(key) = string_to_imgui_key(&config.keybinds.quitout) {
                if ui.is_key_pressed(key) {
                    ds1.quitout.write_u32_rel(Some(0x0), 0x2);
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.moveswap) {
                if ui.is_key_pressed(key) {
                    if player_for_keybinds.is_none() {
                        let mut p = Player::new();
                        p.instantiate_position_only(&mut ds1);
                        player_for_keybinds = Some(p);
                    }
                    player_for_keybinds.as_mut().unwrap().moveswap(&mut ds1);
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_no_gravity) {
                if ui.is_key_pressed(key) {
                    ds1.set_no_gravity();
                    self.no_gravity = !self.no_gravity;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_no_collision) {
                if ui.is_key_pressed(key) {
                    ds1.set_disable_collision();
                    self.disable_collision = !self.disable_collision;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.load_position_1) {
                if ui.is_key_pressed(key) {
                    ds1.teleport_player(
                        self.stored_positions[0].0,
                        self.stored_positions[0].1,
                        self.stored_positions[0].2,
                        self.stored_positions[0].3,
                    );
                    // Restore HP using chr_data_1 offset (current HP)
                    ds1.chr_data_1
                        .write_i32_rel(Some(0x2D4), self.stored_positions[0].4);
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_no_update_ai) {
                if ui.is_key_pressed(key) {
                    ds1.set_no_update_ai();
                    self.no_update_ai = !self.no_update_ai;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.teleport_down) {
                if ui.is_key_pressed(key) {
                    if player_for_keybinds.is_none() {
                        let mut p = Player::new();
                        p.instantiate_position_only(&mut ds1);
                        player_for_keybinds = Some(p);
                    }
                    let player = player_for_keybinds.as_ref().unwrap();
                    ds1.teleport_player(
                        player.x_pos,
                        player.y_pos - 5.0,
                        player.z_pos,
                        player.angle,
                    );
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.teleport_up) {
                if ui.is_key_pressed(key) {
                    if player_for_keybinds.is_none() {
                        let mut p = Player::new();
                        p.instantiate_position_only(&mut ds1);
                        player_for_keybinds = Some(p);
                    }
                    let player = player_for_keybinds.as_ref().unwrap();
                    ds1.teleport_player(
                        player.x_pos,
                        player.y_pos + 5.0,
                        player.z_pos,
                        player.angle,
                    );
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.store_position_1) {
                if ui.is_key_pressed(key) {
                    if player_for_keybinds.is_none() {
                        let mut p = Player::new();
                        p.instantiate_position_only(&mut ds1);
                        player_for_keybinds = Some(p);
                    }
                    let player = player_for_keybinds.as_ref().unwrap();
                    self.stored_positions[0] = (
                        player.x_pos,
                        player.y_pos,
                        player.z_pos,
                        player.angle,
                        player.hp,
                    );
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.restore_full_hp) {
                if ui.is_key_pressed(key) {
                    // Read max HP from CharData2 and write it to current HP in CharData1
                    let max_hp = ds1.chr_data_2.read_i32_rel(Some(CharData2::MAX_HP));
                    ds1.chr_data_1.write_i32_rel(Some(0x2D4), max_hp);
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.rtsr_range) {
                if ui.is_key_pressed(key) {
                    // Set HP to 19% of max HP (under 20% for RTSR activation)
                    let max_hp = ds1.chr_data_2.read_i32_rel(Some(CharData2::MAX_HP));
                    let rtsr_hp = (max_hp as f32 * 0.19) as i32;
                    ds1.chr_data_1.write_i32_rel(Some(0x2D4), rtsr_hp);
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_no_stamina) {
                if ui.is_key_pressed(key) {
                    ds1.set_no_stam_consume();
                    self.no_stamina_consume = !self.no_stamina_consume;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_infinite_magic) {
                if ui.is_key_pressed(key) {
                    ds1.set_all_no_magic_quantity_consume();
                    self.infinite_magic = !self.infinite_magic;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_infinite_goods) {
                if ui.is_key_pressed(key) {
                    ds1.set_no_goods_consume();
                    self.infinite_goods = !self.infinite_goods;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_player_hide) {
                if ui.is_key_pressed(key) {
                    ds1.set_player_hide();
                    self.player_hide = !self.player_hide;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_player_silence) {
                if ui.is_key_pressed(key) {
                    ds1.set_player_silence();
                    self.player_silence = !self.player_silence;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_no_death) {
                if ui.is_key_pressed(key) {
                    ds1.set_no_death();
                    self.no_death = !self.no_death;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_no_damage) {
                if ui.is_key_pressed(key) {
                    ds1.set_no_damage();
                    self.no_damage = !self.no_damage;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_no_hit) {
                if ui.is_key_pressed(key) {
                    ds1.set_no_hit();
                    self.no_hit = !self.no_hit;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_no_attack) {
                if ui.is_key_pressed(key) {
                    ds1.set_no_attack();
                    self.no_attack = !self.no_attack;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_no_move) {
                if ui.is_key_pressed(key) {
                    ds1.set_no_move();
                    self.no_move = !self.no_move;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_draw_direction) {
                if ui.is_key_pressed(key) {
                    ds1.set_draw_direction();
                    self.draw_direction = !self.draw_direction;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_draw_counter) {
                if ui.is_key_pressed(key) {
                    ds1.set_draw_counter();
                    self.draw_counter = !self.draw_counter;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_draw_stable_pos) {
                if ui.is_key_pressed(key) {
                    ds1.set_draw_stable_pos();
                    self.draw_stable_pos = !self.draw_stable_pos;
                }
            }

            if let Some(key) = string_to_imgui_key(&config.keybinds.toggle_debug_info) {
                if ui.is_key_pressed(key) {
                    self.debug_info.toggle();
                }
            }
        } // End of keybind processing when not typing

        drop(config); // Drop config lock before debug window operations
        
        // Debug Info Window - Update when visible
        if self.debug_info.is_open() {
            self.debug_info.update(&ds1);
        }
        self.debug_info.render_window(ui, &mut ds1, &self.config);

        // Toggle animation speed with Keypad2
        if ui.is_key_pressed(imgui::Key::Keypad2) {
            if self.anim_speed_toggled {
                // Return to saved speed
                self.anim_speed = self.saved_anim_speed;
                self.anim_speed_toggled = false;
            } else {
                // Save current speed and set to 1.0
                self.saved_anim_speed = self.anim_speed;
                self.anim_speed = 1.0;
                self.anim_speed_toggled = true;
            }
            ds1.anim_data.write_f32_rel(
                Some(crate::memory::constants::AnimData::PLAY_SPEED),
                self.anim_speed,
            );
        }
        
        if !self.menu_open {
            // Sync flags even when menu is closed, but only if needed
            self.sync_flags_if_needed(&mut ds1);
            return;
        }

        // Only initialize player and bonfire when menu is open
        let mut player = Player::new();
        let mut bonfire = Bonfire::new();

        let config = self.config.lock().unwrap();
        let main_window_layout = config.window_layout.main_window.clone();
        drop(config); // Drop before window operations

        let mut main_window_open = true;
        let mut main_window_changed = false;
        let mut new_main_pos = [0.0, 0.0];
        let mut new_main_size = [0.0, 0.0];
        
        ui.window("Toolbox Menu")
            .size([main_window_layout.width, main_window_layout.height], Condition::FirstUseEver)
            .position([main_window_layout.pos_x, main_window_layout.pos_y], Condition::FirstUseEver)
            .draw_background(false)
            .opened(&mut main_window_open)
            .build(|| {
                // Capture window position/size at the start of the frame
                new_main_pos = ui.window_pos();
                new_main_size = ui.window_size();
                main_window_changed = true;
                
                let mut items_handler = Items::new();
                if ui.button("Eject") {
                    println!("Eject button pressed!");
                    hudhook::eject();
                }

                if ui
                    .input_float("Animation Speed", &mut self.anim_speed)
                    .step(0.1)
                    .step_fast(1.0)
                    .build()
                {
                    self.anim_speed = self.anim_speed.max(0.0);
                    ds1.anim_data.write_f32_rel(
                        Some(crate::memory::constants::AnimData::PLAY_SPEED),
                        self.anim_speed,
                    );
                }
                ui.separator();

                let positions_flags = if self.positions_header_open {
                    imgui::TreeNodeFlags::DEFAULT_OPEN
                } else {
                    imgui::TreeNodeFlags::empty()
                };
                let positions_id = format!("Positions##reset{}", self.header_reset_counter);
                if ui.collapsing_header(&positions_id, positions_flags) {
                    self.positions_header_open = true;
                    // Update player position data (lightweight operation)
                    player.instantiate_position_only(&mut ds1);

                    for i in 0..3 {
                        ui.text(format!(
                            "Slot {} - X: {:.2}, Y: {:.2}, Z: {:.2}, Angle: {:.2}, HP: {}",
                            i + 1,
                            self.stored_positions[i].0,
                            self.stored_positions[i].1,
                            self.stored_positions[i].2,
                            self.stored_positions[i].3,
                            self.stored_positions[i].4
                        ));

                        ui.same_line();
                        if ui.button(format!("Store##{}", i)) {
                            self.stored_positions[i] = (
                                player.x_pos,
                                player.y_pos,
                                player.z_pos,
                                player.angle,
                                player.hp,
                            );
                        }

                        ui.same_line();
                        if ui.button(format!("Restore##{}", i)) {
                            ds1.teleport_player(
                                self.stored_positions[i].0,
                                self.stored_positions[i].1,
                                self.stored_positions[i].2,
                                self.stored_positions[i].3,
                            );
                            // Restore HP using chr_data_1 offset (current HP)
                            ds1.chr_data_1
                                .write_i32_rel(Some(0x2D4), self.stored_positions[i].4);
                        }

                        ui.separator();
                    }
                } else {
                    self.positions_header_open = false;
                }

                if ui.collapsing_header("Debug Flags", imgui::TreeNodeFlags::DEFAULT_OPEN) {
                    if ui.checkbox("show console", &mut self.show_console) {
                        if self.show_console {
                            hudhook::alloc_console().ok();
                        } else {
                            hudhook::free_console().ok();
                        }
                    }

                    if ui.checkbox("inf stam", &mut self.no_stamina_consume) {
                        ds1.set_no_stam_consume();
                    }

                    if ui.checkbox("infinite magic", &mut self.infinite_magic) {
                        ds1.set_all_no_magic_quantity_consume();
                    }

                    if ui.checkbox("infinite goods", &mut self.infinite_goods) {
                        ds1.set_no_goods_consume();
                    }

                    if ui.checkbox("player hide", &mut self.player_hide) {
                        ds1.set_player_hide();
                    }

                    if ui.checkbox("player silence", &mut self.player_silence) {
                        ds1.set_player_silence();
                    }

                    if ui.checkbox("no death", &mut self.no_death) {
                        ds1.set_no_death();
                    }

                    if ui.checkbox("player exterminate", &mut self.player_exterminate) {
                        ds1.set_player_exterminate(self.player_exterminate);
                    }

                    if ui.checkbox("no damage", &mut self.no_damage) {
                        ds1.set_no_damage();
                    }

                    if ui.checkbox("no hit", &mut self.no_hit) {
                        ds1.set_no_hit();
                    }

                    if ui.checkbox("no attack", &mut self.no_attack) {
                        ds1.set_no_attack();
                    }

                    if ui.checkbox("no move", &mut self.no_move) {
                        ds1.set_no_move();
                    }

                    if ui.checkbox("no update ai", &mut self.no_update_ai) {
                        ds1.set_no_update_ai();
                    }

                    if ui.checkbox("disable collision", &mut self.disable_collision) {
                        ds1.set_disable_collision();
                    }

                    if ui.checkbox("no gravity", &mut self.no_gravity) {
                        ds1.set_no_gravity();
                    }

                    if ui.checkbox("draw direction", &mut self.draw_direction) {
                        ds1.set_draw_direction();
                    }

                    if ui.checkbox("draw counter", &mut self.draw_counter) {
                        ds1.set_draw_counter();
                    }

                    if ui.checkbox("draw stable pos", &mut self.draw_stable_pos) {
                        ds1.set_draw_stable_pos();
                    }
                }

                let stats_flags = if self.stats_header_open {
                    imgui::TreeNodeFlags::DEFAULT_OPEN
                } else {
                    imgui::TreeNodeFlags::empty()
                };
                let stats_id = format!("Stats##reset{}", self.header_reset_counter);
                if ui.collapsing_header(&stats_id, stats_flags) {
                    self.stats_header_open = true;
                    // Instantiate player stats when Stats section is visible
                    player.instantiate(&mut ds1);

                    if ui.input_int("Vitality", &mut player.vitality).build() {
                        player.vitality = player.vitality.max(1);
                        player.set_player_stat(&mut ds1, CharData2::VITALITY, player.vitality);
                    }

                    if ui.input_int("Attunement", &mut player.attunement).build() {
                        player.attunement = player.attunement.max(1);
                        player.set_player_stat(&mut ds1, CharData2::ATTUNEMENT, player.attunement);
                    }

                    if ui.input_int("Endurance", &mut player.endurance).build() {
                        player.endurance = player.endurance.max(1);
                        player.set_player_stat(&mut ds1, CharData2::ENDURANCE, player.endurance);
                    }

                    if ui.input_int("Strength", &mut player.strength).build() {
                        player.strength = player.strength.max(1);
                        player.set_player_stat(&mut ds1, CharData2::STRENGTH, player.strength);
                    }

                    if ui.input_int("Dexterity", &mut player.dexterity).build() {
                        player.dexterity = player.dexterity.max(1);
                        player.set_player_stat(&mut ds1, CharData2::DEXTERITY, player.dexterity);
                    }

                    if ui
                        .input_int("Intelligence", &mut player.intelligence)
                        .build()
                    {
                        player.intelligence = player.intelligence.max(1);
                        player.set_player_stat(
                            &mut ds1,
                            CharData2::INTELLIGENCE,
                            player.intelligence,
                        );
                    }

                    if ui.input_int("Faith", &mut player.faith).build() {
                        player.faith = player.faith.max(1);
                        player.set_player_stat(&mut ds1, CharData2::FAITH, player.faith);
                    }

                    if ui
                        .input_int("Souls", &mut player.souls)
                        .step(100)
                        .step_fast(1000)
                        .build()
                    {
                        player.souls = player.souls.max(1);
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::SOULS), player.souls);
                    }
                } else {
                    self.stats_header_open = false;
                }

                if ui.button("Moveswap") {
                    player.moveswap(&mut ds1);
                }

                ui.same_line();
                if ui.button("Swap Gender") {
                    player.swap_gender(&mut ds1);
                }

                ui.same_line();
                if ui.button("Restore Full HP") {
                    // Read max HP from CharData2 and write it to current HP in CharData1
                    let max_hp = ds1.chr_data_2.read_i32_rel(Some(CharData2::MAX_HP));
                    ds1.chr_data_1.write_i32_rel(Some(0x2D4), max_hp);
                }

                ui.same_line();
                if ui.button("RTSR RANGE") {
                    // Set HP to 19% of max HP (under 20% for RTSR activation)
                    let max_hp = ds1.chr_data_2.read_i32_rel(Some(CharData2::MAX_HP));
                    let rtsr_hp = (max_hp as f32 * 0.19) as i32;
                    ds1.chr_data_1.write_i32_rel(Some(0x2D4), rtsr_hp);
                }

                if ui.collapsing_header("Select bonfire", imgui::TreeNodeFlags::empty()) {
                    ui.set_next_item_width(400.0);
                    ui.input_text("Search", &mut self.bonfire_search).build();

                    let selected_name = if self.selected_bonfire_id >= 0 {
                        Bonfire::get_bonfires()
                            .iter()
                            .find(|(_, id)| *id == self.selected_bonfire_id)
                            .map(|(name, _)| *name)
                            .unwrap_or("Select a bonfire...")
                    } else {
                        "Select a bonfire..."
                    };

                    ui.set_next_item_width(400.0);
                    if let Some(_combo) = ui.begin_combo("##bonfire_combo", selected_name) {
                        let search_lower = self.bonfire_search.to_lowercase();

                        for (bonfire_name, bonfire_id) in Bonfire::get_bonfires() {
                            // Filter by search text
                            if !search_lower.is_empty()
                                && !bonfire_name.to_lowercase().contains(&search_lower)
                            {
                                continue;
                            }

                            let is_selected = self.selected_bonfire_id == bonfire_id;

                            if ui
                                .selectable_config(bonfire_name)
                                .selected(is_selected)
                                .build()
                            {
                                self.selected_bonfire_id = bonfire_id;
                            }

                            // Set focus on selected item
                            if is_selected {
                                ui.set_item_default_focus();
                            }
                        }
                    }

                    if ui.button("Warp to Selected Bonfire") && self.selected_bonfire_id >= 0 {
                        bonfire.set_last_bonfire(&mut ds1, self.selected_bonfire_id as u32);
                        bonfire.inject_bonfire_function(&mut ds1);
                    }
                }

                let give_item_flags = if self.give_item_header_open {
                    imgui::TreeNodeFlags::DEFAULT_OPEN
                } else {
                    imgui::TreeNodeFlags::empty()
                };
                let give_item_id = format!("Give item##reset{}", self.header_reset_counter);
                if ui.collapsing_header(&give_item_id, give_item_flags) {
                    self.give_item_header_open = true;
                    // Tab bar for selecting item type
                    if ui.radio_button("Items", &mut self.give_item_type, 0) {}
                    ui.same_line();
                    if ui.radio_button("Rings", &mut self.give_item_type, 1) {}
                    ui.same_line();
                    if ui.radio_button("Weapons", &mut self.give_item_type, 2) {}
                    ui.same_line();
                    if ui.radio_button("Armor", &mut self.give_item_type, 3) {}

                    ui.separator();

                    // Items tab
                    if self.give_item_type == 0 {
                        ui.set_next_item_width(400.0);
                        ui.input_text("Search", &mut self.item_search).build();

                        let item_data = Items::get_item_data();
                        let selected_item = &item_data[self.selected_item_index];
                        let preview_text = format!("{} (ID: {})", selected_item.3, selected_item.0);

                        ui.set_next_item_width(400.0);
                        if let Some(_combo) = ui.begin_combo("##item_combo", &preview_text) {
                            let search_lower = self.item_search.to_lowercase();

                            for (index, item) in item_data.iter().enumerate() {
                                let item_name = item.3;

                                if !search_lower.is_empty()
                                    && !item_name.to_lowercase().contains(&search_lower)
                                {
                                    continue;
                                }

                                let is_selected = self.selected_item_index == index;
                                let label = format!("{} (ID: {})", item_name, item.0);

                                if ui.selectable_config(&label).selected(is_selected).build() {
                                    self.selected_item_index = index;
                                    self.item_quantity = item.1;
                                }

                                if is_selected {
                                    ui.set_item_default_focus();
                                }
                            }
                        }

                        if ui
                            .input_int("Quantity", &mut self.item_quantity)
                            .step(1)
                            .step_fast(10)
                            .build()
                        {
                            self.item_quantity = self.item_quantity.max(1);
                        }

                        if ui.button("Give Selected Item") {
                            let selected = &item_data[self.selected_item_index];
                            items_handler.execute_get_item(
                                &mut ds1,
                                0x40000000,
                                selected.0,
                                self.item_quantity,
                            );
                        }
                    }

                    // Rings tab
                    if self.give_item_type == 1 {
                        ui.set_next_item_width(400.0);
                        ui.input_text("Search", &mut self.ring_search).build();

                        let ring_data = Items::get_ring_data();
                        let selected_ring = &ring_data[self.selected_ring_index];
                        let preview_text = format!("{} (ID: {})", selected_ring.3, selected_ring.0);

                        ui.set_next_item_width(400.0);
                        if let Some(_combo) = ui.begin_combo("##ring_combo", &preview_text) {
                            let search_lower = self.ring_search.to_lowercase();

                            for (index, ring) in ring_data.iter().enumerate() {
                                let ring_name = ring.3;

                                if !search_lower.is_empty()
                                    && !ring_name.to_lowercase().contains(&search_lower)
                                {
                                    continue;
                                }

                                let is_selected = self.selected_ring_index == index;
                                let label = format!("{} (ID: {})", ring_name, ring.0);

                                if ui.selectable_config(&label).selected(is_selected).build() {
                                    self.selected_ring_index = index;
                                    self.ring_quantity = ring.1;
                                }
                                if is_selected {
                                    ui.set_item_default_focus();
                                }
                            }
                        }

                        if ui
                            .input_int("Quantity", &mut self.ring_quantity)
                            .step(1)
                            .step_fast(10)
                            .build()
                        {
                            self.ring_quantity = self.ring_quantity.max(1);
                        }

                        if ui.button("Give Selected Ring") {
                            let selected = &ring_data[self.selected_ring_index];
                            items_handler.execute_get_item(
                                &mut ds1,
                                0x20000000,
                                selected.0,
                                self.ring_quantity,
                            );
                        }
                    }

                    // Weapons tab
                    if self.give_item_type == 2 {
                        ui.set_next_item_width(400.0);
                        ui.input_text("Search", &mut self.weapon_search).build();

                        let weapon_data = Items::get_weapon_data();
                        let selected_weapon = &weapon_data[self.selected_weapon_index];
                        let preview_text =
                            format!("{} (ID: {})", selected_weapon.3, selected_weapon.0);

                        ui.set_next_item_width(400.0);
                        if let Some(_combo) = ui.begin_combo("##weapon_combo", &preview_text) {
                            let search_lower = self.weapon_search.to_lowercase();

                            for (index, weapon) in weapon_data.iter().enumerate() {
                                let weapon_name = weapon.3;

                                if !search_lower.is_empty()
                                    && !weapon_name.to_lowercase().contains(&search_lower)
                                {
                                    continue;
                                }

                                let is_selected = self.selected_weapon_index == index;
                                let label = format!("{} (ID: {})", weapon_name, weapon.0);

                                if ui.selectable_config(&label).selected(is_selected).build() {
                                    self.selected_weapon_index = index;
                                    self.weapon_quantity = weapon.1;
                                }

                                if is_selected {
                                    ui.set_item_default_focus();
                                }
                            }
                        }

                        if ui
                            .input_int("Quantity", &mut self.weapon_quantity)
                            .step(1)
                            .step_fast(10)
                            .build()
                        {
                            self.weapon_quantity = self.weapon_quantity.max(1);
                        }

                        let infusion_data = Items::get_infusion_data();
                        let selected_infusion = &infusion_data[self.selected_infusion_index];

                        ui.set_next_item_width(200.0);
                        if let Some(_combo) = ui.begin_combo("Infusion", selected_infusion.0) {
                            for (index, infusion) in infusion_data.iter().enumerate() {
                                let is_selected = self.selected_infusion_index == index;

                                if ui
                                    .selectable_config(infusion.0)
                                    .selected(is_selected)
                                    .build()
                                {
                                    self.selected_infusion_index = index;
                                    if self.weapon_upgrade_level > infusion.2 {
                                        self.weapon_upgrade_level = infusion.2;
                                    }
                                }

                                if is_selected {
                                    ui.set_item_default_focus();
                                }
                            }
                        }

                        ui.set_next_item_width(200.0);
                        ui.slider(
                            "Upgrade Level",
                            0,
                            selected_infusion.2,
                            &mut self.weapon_upgrade_level,
                        );

                        if ui.button("Give Selected Weapon") {
                            let selected = &weapon_data[self.selected_weapon_index];
                            let upgraded_weapon_id =
                                selected.0 + selected_infusion.1 + self.weapon_upgrade_level;
                            items_handler.execute_get_item(
                                &mut ds1,
                                0x00000000,
                                upgraded_weapon_id,
                                self.weapon_quantity,
                            );
                        }
                    }

                    // Armor tab
                    if self.give_item_type == 3 {
                        ui.set_next_item_width(400.0);
                        ui.input_text("Search", &mut self.armor_search).build();

                        let armor_data = Items::get_armor_data();
                        let selected_armor = &armor_data[self.selected_armor_index];
                        let preview_text =
                            format!("{} (ID: {})", selected_armor.3, selected_armor.0);

                        ui.set_next_item_width(400.0);
                        if let Some(_combo) = ui.begin_combo("##armor_combo", &preview_text) {
                            let search_lower = self.armor_search.to_lowercase();

                            for (index, armor) in armor_data.iter().enumerate() {
                                let armor_name = armor.3;

                                if !search_lower.is_empty()
                                    && !armor_name.to_lowercase().contains(&search_lower)
                                {
                                    continue;
                                }

                                let is_selected = self.selected_armor_index == index;
                                let label = format!("{} (ID: {})", armor_name, armor.0);

                                if ui.selectable_config(&label).selected(is_selected).build() {
                                    self.selected_armor_index = index;
                                    self.armor_quantity = armor.1;
                                    // Reset upgrade level when selecting new armor
                                    let max_upgrade = match armor.2 {
                                        0 => 0,
                                        1 => 5,
                                        2 => 10,
                                        _ => 0,
                                    };
                                    if self.armor_upgrade_level > max_upgrade {
                                        self.armor_upgrade_level = max_upgrade;
                                    }
                                }

                                if is_selected {
                                    ui.set_item_default_focus();
                                }
                            }
                        }

                        if ui
                            .input_int("Quantity", &mut self.armor_quantity)
                            .step(1)
                            .step_fast(10)
                            .build()
                        {
                            self.armor_quantity = self.armor_quantity.max(1);
                        }

                        // Determine max upgrade level based on armor type (0=not upgradeable, 1=+5, 2=+10)
                        let max_upgrade_level = match selected_armor.2 {
                            0 => 0,
                            1 => 5,
                            2 => 10,
                            _ => 0,
                        };

                        // Only show upgrade slider if armor can be upgraded
                        if max_upgrade_level > 0 {
                            ui.set_next_item_width(200.0);
                            ui.slider(
                                "Upgrade Level",
                                0,
                                max_upgrade_level,
                                &mut self.armor_upgrade_level,
                            );
                        }

                        if ui.button("Give Selected Armor") {
                            let selected = &armor_data[self.selected_armor_index];
                            let upgraded_armor_id = selected.0 + self.armor_upgrade_level;
                            items_handler.execute_get_item(
                                &mut ds1,
                                0x10000000,
                                upgraded_armor_id,
                                self.armor_quantity,
                            );
                        }
                    }
                } else {
                    self.give_item_header_open = false;
                }
            });

        // Save main window layout if changed
        if main_window_changed {
            let mut config = self.config.lock().unwrap();
            let layout = &mut config.window_layout.main_window;
            if (layout.pos_x - new_main_pos[0]).abs() > 1.0
                || (layout.pos_y - new_main_pos[1]).abs() > 1.0
                || (layout.width - new_main_size[0]).abs() > 1.0
                || (layout.height - new_main_size[1]).abs() > 1.0
            {
                layout.pos_x = new_main_pos[0];
                layout.pos_y = new_main_pos[1];
                layout.width = new_main_size[0];
                layout.height = new_main_size[1];
                let _ = config.save();
            }
            drop(config); // Drop config before mutable borrow
        }

        // Sync flags at end of render (non-blocking position)
        self.sync_flags_if_needed(&mut ds1);
    }
}

impl RenderLoop {
    fn sync_flags_if_needed(&mut self, ds1: &mut Ds1) {
        // Only sync flags every 3 seconds and only if at least one flag is enabled
        let any_flag_enabled = self.no_stamina_consume
            || self.infinite_magic
            || self.infinite_goods
            || self.player_hide
            || self.player_silence
            || self.no_death
            || self.player_exterminate
            || self.no_damage
            || self.no_hit
            || self.no_attack
            || self.no_move
            || self.no_update_ai
            || self.disable_collision
            || self.no_gravity
            || self.draw_direction
            || self.draw_counter
            || self.draw_stable_pos;

        if !any_flag_enabled || self.last_flag_sync_time.elapsed().as_secs() < 3 {
            return;
        }

        // Batch all flag writes together
        if self.no_stamina_consume {
            ds1.set_no_stam_consume_to(true);
        }
        if self.infinite_magic {
            ds1.set_all_no_magic_quantity_consume_to(true);
        }
        if self.infinite_goods {
            ds1.set_no_goods_consume_to(true);
        }
        if self.player_hide {
            ds1.set_player_hide_to(true);
        }
        if self.player_silence {
            ds1.set_player_silence_to(true);
        }
        if self.no_death {
            ds1.set_no_death_to(true);
        }
        if self.player_exterminate {
            ds1.set_player_exterminate(true);
        }
        if self.no_damage {
            ds1.set_no_damage_to(true);
        }
        if self.no_hit {
            ds1.set_no_hit_to(true);
        }
        if self.no_attack {
            ds1.set_no_attack_to(true);
        }
        if self.no_move {
            ds1.set_no_move_to(true);
        }
        if self.no_update_ai {
            ds1.set_no_update_ai_to(true);
        }
        if self.disable_collision {
            ds1.set_disable_collision_to(true);
        }
        if self.no_gravity {
            ds1.set_no_gravity_to(true);
        }
        if self.draw_direction {
            ds1.set_draw_direction_to(true);
        }
        if self.draw_counter {
            ds1.set_draw_counter_to(true);
        }
        if self.draw_stable_pos {
            ds1.set_draw_stable_pos_to(true);
        }
        self.last_flag_sync_time = std::time::Instant::now();
    }
}
