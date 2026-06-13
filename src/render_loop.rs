use hudhook::ImguiRenderLoop;
use imgui::Condition;
use mem_rs::memory::ReadWrite;
use std::sync::{Arc, Mutex};

use crate::config::{Config, ResolvedKeybinds};
use crate::memory::{Ds1, ds1};
use crate::ui::Bonfire;
use crate::ui::DebugInfo;
#[cfg(windows)]
use windows_sys::Win32::Foundation::RECT;
#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowRect, SetCursorPos,
};

use crate::ui::{self, Player, TasRunner};

mod feature_sync;
mod frame_tasks;
mod input_state;
mod keybinds;
mod layout_persistence;
mod menu_state;
mod pending_angle;
mod positions_section;
mod debug_flags_section;
mod stats_section;
mod utility_actions_section;
mod bonfire_section;
mod give_item_section;
mod ui_style;
mod window_actions;

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
    infinite_stamina: bool,
    freeze_poise: bool,
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
    disable_enemies: bool,
    disable_events: bool,
    auto_save_disabled: bool,
    offline_mode: bool,
    free_cam: bool,
    stored_positions: [Option<(f32, f32, f32, f32, i32)>; 3],
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
    last_process_refresh_time: std::time::Instant,
    positions_header_open: bool,
    stats_header_open: bool,
    give_item_header_open: bool,
    header_reset_counter: u32,
    last_main_window_save_time: std::time::Instant,
    resolved_keybinds: ResolvedKeybinds,
    pending_angle_write: Option<(f32, u32)>,
    tas_runner: TasRunner,
}
impl RenderLoop {
    pub fn new() -> Self {
        let config = Config::load_or_default();
        let resolved_keybinds = ResolvedKeybinds::from_config(&config);
        RenderLoop {
            config: Arc::new(Mutex::new(config)),
            resolved_keybinds,
            no_stamina_consume: false,
            infinite_stamina: false,
            freeze_poise: false,
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
            disable_enemies: false,
            disable_events: false,
            auto_save_disabled: false,
            offline_mode: false,
            free_cam: false,
            stored_positions: [None; 3],
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
            last_process_refresh_time: std::time::Instant::now(),
            positions_header_open: false,
            stats_header_open: false,
            give_item_header_open: false,
            header_reset_counter: 0,
            last_main_window_save_time: std::time::Instant::now(),
            pending_angle_write: None,
            tas_runner: TasRunner::new(),
        }
    }
}

impl ImguiRenderLoop for RenderLoop {
    fn render(&mut self, ui: &mut imgui::Ui) {
        self.handle_menu_toggle_input(ui);

        // Acquire ds1 lock
        let instance = get_ds1_instance();
        let mut ds1 = instance.lock().unwrap();

        self.run_pre_ui_frame_tasks(&mut ds1);

        // Check if user is interacting with any UI or if menu is open
        let io = ui.io();
        let ui_wants_input = io.want_capture_keyboard || io.want_text_input;

        self.update_game_input_state(&mut ds1, ui_wants_input);

        self.apply_pending_angle_write(&mut ds1);

        // Lock config for styling only — keybinds now use cached resolved_keybinds
        let config = self.config.lock().unwrap();
        let _ui_style_tokens = self.push_ui_styles(ui, &config);
        drop(config); // Release config lock before mutable self borrows.

        self.process_keybinds(ui, &mut ds1, ui_wants_input);

        // Debug Info Window - Update when visible
        if self.debug_info.is_open() {
            self.debug_info.update(&ds1);
            self.debug_info.set_stored_position(
                self.stored_positions[0].map(|(x, y, z, a, _)| (x, y, z, a)),
            );
        }
        self.debug_info.render_window(ui, &mut ds1, &self.config);

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
            .size(
                [main_window_layout.width, main_window_layout.height],
                Condition::FirstUseEver,
            )
            .position(
                [main_window_layout.pos_x, main_window_layout.pos_y],
                Condition::FirstUseEver,
            )
            .draw_background(false)
            .opened(&mut main_window_open)
            .build(|| {
                // Capture window position/size at the start of the frame
                new_main_pos = ui.window_pos();
                new_main_size = ui.window_size();
                // Only mark as changed if values actually differ
                if (main_window_layout.pos_x - new_main_pos[0]).abs() > 1.0
                    || (main_window_layout.pos_y - new_main_pos[1]).abs() > 1.0
                    || (main_window_layout.width - new_main_size[0]).abs() > 1.0
                    || (main_window_layout.height - new_main_size[1]).abs() > 1.0
                {
                    main_window_changed = true;
                }

                self.handle_eject_action(ui, &mut ds1);
                self.render_animation_speed_control(ui, &mut ds1);
                ui.separator();

                self.render_positions_section(ui, &mut ds1, &mut player);
                self.render_debug_flags_section(ui, &mut ds1);
                self.render_stats_section(ui, &mut ds1, &mut player);
                self.render_utility_actions_section(ui, &mut ds1, &mut player);
                self.render_bonfire_section(ui, &mut ds1, &mut bonfire);
                self.render_give_item_section(ui, &mut ds1);

                self.tas_runner.render_section(ui);
            });

        // If the user clicked the X button on the main window, close the menu.
        if !main_window_open {
            self.menu_open = false;
        }

        self.persist_main_window_layout_if_changed(main_window_changed, new_main_pos, new_main_size);

        // Sync flags at end of render (non-blocking position)
        self.sync_flags_if_needed(&mut ds1);
    }
}

impl Drop for RenderLoop {
    fn drop(&mut self) {
        // Clean up XInput hook before DLL unload to prevent crashes
        let instance = get_ds1_instance();
        let mut ds1 = instance.lock().unwrap();
        ds1.disable_xinput_hook();
        eprintln!("[CLEANUP] XInput hook disabled before unload");
    }
}
