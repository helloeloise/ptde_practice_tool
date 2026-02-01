use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub keybinds: Keybinds,
    pub colors: ColorScheme,
    #[serde(default)]
    pub window_layout: WindowLayout,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WindowLayout {
    #[serde(default = "default_main_window")]
    pub main_window: WindowState,
    #[serde(default = "default_debug_window")]
    pub debug_window: WindowState,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WindowState {
    pub width: f32,
    pub height: f32,
    pub pos_x: f32,
    pub pos_y: f32,
}

fn default_main_window() -> WindowState {
    WindowState {
        width: 600.0,
        height: 800.0,
        pos_x: 16.0,
        pos_y: 16.0,
    }
}

fn default_debug_window() -> WindowState {
    WindowState {
        width: 700.0,
        height: 850.0,
        pos_x: 400.0,
        pos_y: 16.0,
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Keybinds {
    pub toggle_menu: String,
    pub toggle_debug_info: String,
    pub quitout: String,
    pub moveswap: String,
    pub toggle_no_gravity: String,
    pub toggle_no_collision: String,
    pub load_position_1: String,
    pub toggle_no_update_ai: String,
    pub teleport_down: String,
    pub teleport_up: String,
    pub store_position_1: String,
    pub restore_full_hp: String,
    pub rtsr_range: String,
    pub toggle_no_stamina: String,
    pub toggle_infinite_magic: String,
    pub toggle_infinite_goods: String,
    pub toggle_player_hide: String,
    pub toggle_player_silence: String,
    pub toggle_no_death: String,
    pub toggle_no_damage: String,
    pub toggle_no_hit: String,
    pub toggle_no_attack: String,
    pub toggle_no_move: String,
    pub toggle_draw_direction: String,
    pub toggle_draw_counter: String,
    pub toggle_draw_stable_pos: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColorScheme {
    pub button: ColorRGB,
    pub button_hovered: ColorRGB,
    pub button_active: ColorRGB,
    #[serde(default = "default_text_color")]
    pub text: ColorRGB,
}

fn default_text_color() -> ColorRGB {
    ColorRGB {
        r: 255,
        g: 255,
        b: 255,
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB {
    pub fn to_float4(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            1.0,
        ]
    }
}

impl Config {
    pub fn load_or_default() -> Self {
        let config_path = "config.toml";

        if Path::new(config_path).exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!("Failed to parse config.toml: {}. Using defaults.", e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read config.toml: {}. Using defaults.", e);
                }
            }
        }

        // Return default config and create the file
        let default_config = Self::default();
        let _ = default_config.save();
        default_config
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write("config.toml", toml_string)?;
        Ok(())
    }
}

impl Default for WindowLayout {
    fn default() -> Self {
        WindowLayout {
            main_window: default_main_window(),
            debug_window: default_debug_window(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            keybinds: Keybinds {
                toggle_menu: "".to_string(),
                toggle_debug_info: "".to_string(),
                quitout: "".to_string(),
                moveswap: "".to_string(),
                toggle_no_gravity: "".to_string(),
                toggle_no_collision: "".to_string(),
                load_position_1: "".to_string(),
                toggle_no_update_ai: "".to_string(),
                teleport_down: "".to_string(),
                teleport_up: "".to_string(),
                store_position_1: "".to_string(),
                restore_full_hp: "".to_string(),
                rtsr_range: "".to_string(),
                toggle_no_stamina: "".to_string(),
                toggle_infinite_magic: "".to_string(),
                toggle_infinite_goods: "".to_string(),
                toggle_player_hide: "".to_string(),
                toggle_player_silence: "".to_string(),
                toggle_no_death: "".to_string(),
                toggle_no_damage: "".to_string(),
                toggle_no_hit: "".to_string(),
                toggle_no_attack: "".to_string(),
                toggle_no_move: "".to_string(),
                toggle_draw_direction: "".to_string(),
                toggle_draw_counter: "".to_string(),
                toggle_draw_stable_pos: "".to_string(),
            },
            colors: ColorScheme {
                button: ColorRGB {
                    r: 91,
                    g: 206,
                    b: 250,
                },
                button_hovered: ColorRGB {
                    r: 245,
                    g: 169,
                    b: 184,
                },
                button_active: ColorRGB {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                text: ColorRGB {
                    r: 255,
                    g: 50,
                    b: 100,
                },
            },
            window_layout: WindowLayout::default(),
        }
    }
}

// Helper function to convert string key names to imgui::Key
pub fn string_to_imgui_key(key_str: &str) -> Option<imgui::Key> {
    match key_str {
        "Keypad0" => Some(imgui::Key::Keypad0),
        "Keypad1" => Some(imgui::Key::Keypad1),
        "Keypad2" => Some(imgui::Key::Keypad2),
        "Keypad3" => Some(imgui::Key::Keypad3),
        "Keypad4" => Some(imgui::Key::Keypad4),
        "Keypad5" => Some(imgui::Key::Keypad5),
        "Keypad6" => Some(imgui::Key::Keypad6),
        "Keypad7" => Some(imgui::Key::Keypad7),
        "Keypad8" => Some(imgui::Key::Keypad8),
        "Keypad9" => Some(imgui::Key::Keypad9),
        "Alpha0" => Some(imgui::Key::Alpha0),
        "Alpha1" => Some(imgui::Key::Alpha1),
        "Alpha2" => Some(imgui::Key::Alpha2),
        "Alpha3" => Some(imgui::Key::Alpha3),
        "Alpha4" => Some(imgui::Key::Alpha4),
        "Alpha5" => Some(imgui::Key::Alpha5),
        "Alpha6" => Some(imgui::Key::Alpha6),
        "Alpha7" => Some(imgui::Key::Alpha7),
        "Alpha8" => Some(imgui::Key::Alpha8),
        "Alpha9" => Some(imgui::Key::Alpha9),
        "F1" => Some(imgui::Key::F1),
        "F2" => Some(imgui::Key::F2),
        "F3" => Some(imgui::Key::F3),
        "F4" => Some(imgui::Key::F4),
        "F5" => Some(imgui::Key::F5),
        "F6" => Some(imgui::Key::F6),
        "F7" => Some(imgui::Key::F7),
        "F8" => Some(imgui::Key::F8),
        "F9" => Some(imgui::Key::F9),
        "F10" => Some(imgui::Key::F10),
        "F11" => Some(imgui::Key::F11),
        "F12" => Some(imgui::Key::F12),
        "LeftCtrl" => Some(imgui::Key::LeftCtrl),
        "RightCtrl" => Some(imgui::Key::RightCtrl),
        "LeftShift" => Some(imgui::Key::LeftShift),
        "RightShift" => Some(imgui::Key::RightShift),
        "LeftAlt" => Some(imgui::Key::LeftAlt),
        "RightAlt" => Some(imgui::Key::RightAlt),
        "Space" => Some(imgui::Key::Space),
        "Enter" => Some(imgui::Key::Enter),
        "Escape" => Some(imgui::Key::Escape),
        "Tab" => Some(imgui::Key::Tab),
        "Backspace" => Some(imgui::Key::Backspace),
        "Insert" => Some(imgui::Key::Insert),
        "Delete" => Some(imgui::Key::Delete),
        "Home" => Some(imgui::Key::Home),
        "End" => Some(imgui::Key::End),
        "PageUp" => Some(imgui::Key::PageUp),
        "PageDown" => Some(imgui::Key::PageDown),
        "LeftArrow" => Some(imgui::Key::LeftArrow),
        "RightArrow" => Some(imgui::Key::RightArrow),
        "UpArrow" => Some(imgui::Key::UpArrow),
        "DownArrow" => Some(imgui::Key::DownArrow),
        _ => None,
    }
}
