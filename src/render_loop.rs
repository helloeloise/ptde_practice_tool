use hudhook::ImguiRenderLoop;
use imgui::Condition;
use std::sync::{Arc, Mutex};

use crate::memory::constants::CharData2;
use crate::memory::{Ds1, ds1};
use crate::ui::Player;

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
    no_stamina_consume: bool,
    no_update_ai: bool,
    x_stored_pos: f32,
    y_stored_pos: f32,
    z_stored_pos: f32,
}

impl RenderLoop {
    pub fn new() -> Self {
        RenderLoop {
            no_stamina_consume: false,
            no_update_ai: false,
            x_stored_pos: 0.0,
            y_stored_pos: 0.0,
            z_stored_pos: 0.0,
        }
    }
}

impl ImguiRenderLoop for RenderLoop {
    fn render(&mut self, ui: &mut imgui::Ui) {
        let instance = get_ds1_instance();
        let mut ds1 = instance.lock().unwrap();
        let mut player = Player::new();
        player.instantiate(&mut ds1);

        ui.window("Hello hudhook")
            .size([368.0, 568.0], Condition::FirstUseEver)
            .position([16.0, 16.0], Condition::FirstUseEver)
            .build(|| {
                ui.text(format!("HP {:?}", player.hp));
                ui.text(format!("Stamina {:?}", player.stamina));

                ui.text(format!("Pos X {:?}", player.x_pos));
                ui.text(format!("Pos Y {:?}", player.y_pos));
                ui.text(format!("Pos Z {:?}", player.z_pos));

                ui.text(format!("Stored Pos X {:?}", self.x_stored_pos));
                ui.text(format!("Stored Pos Y {:?}", self.y_stored_pos));
                ui.text(format!("Stored Pos Z {:?}", self.z_stored_pos));

                if ui.button("Eject") {
                    print!("test");
                    hudhook::eject();
                }

                if ui.button("Store player position") {
                    self.x_stored_pos = player.x_pos;
                    self.y_stored_pos = player.y_pos;
                    self.z_stored_pos = player.z_pos;
                }

                if ui.button("Teleport player") {
                    ds1.teleport_player(self.x_stored_pos, self.y_stored_pos, self.z_stored_pos);
                }

                if ui.checkbox("inf stam", &mut self.no_stamina_consume) {
                    ds1.set_no_stam_consume();
                }

                if ui.checkbox("no update ai", &mut self.no_update_ai) {
                    ds1.set_no_update_ai();
                }

                if ui.button("Stats") {
                    ui.open_popup("stats_popup");
                }
                if let Some(_popup) = ui.begin_popup("stats_popup") {
                    if (ui.input_int(
                        format!("Vitality {:?}", player.vitality),
                        &mut player.vitality,
                    ))
                    .build()
                    {
                        player.set_player_stat(&mut ds1, CharData2::VITALITY, player.vitality);
                    }
                    if (ui.input_int(
                        format!("Attunement {:?}", player.attunement),
                        &mut player.attunement,
                    ))
                    .build()
                    {
                        player.set_player_stat(&mut ds1, CharData2::ATTUNEMENT, player.attunement);
                    }
                    if (ui.input_int(
                        format!("Endurance {:?}", player.endurance),
                        &mut player.endurance,
                    ))
                    .build()
                    {
                        player.set_player_stat(&mut ds1, CharData2::ENDURANCE, player.endurance);
                    }
                    if (ui.input_int(
                        format!("Strength {:?}", player.strength),
                        &mut player.strength,
                    ))
                    .build()
                    {
                        player.set_player_stat(&mut ds1, CharData2::STRENGTH, player.strength);
                    }
                    if (ui.input_int(
                        format!("Dexterity {:?}", player.dexterity),
                        &mut player.dexterity,
                    ))
                    .build()
                    {
                        player.set_player_stat(&mut ds1, CharData2::DEXTERITY, player.dexterity);
                    }
                    if (ui.input_int(
                        format!("Intelligence {:?}", player.intelligence),
                        &mut player.intelligence,
                    ))
                    .build()
                    {
                        player.set_player_stat(
                            &mut ds1,
                            CharData2::INTELLIGENCE,
                            player.intelligence,
                        );
                    }
                    if (ui.input_int(format!("Faith {:?}", player.faith), &mut player.faith))
                        .build()
                    {
                        player.set_player_stat(&mut ds1, CharData2::FAITH, player.faith);
                    }
                }
            });
    }
}
