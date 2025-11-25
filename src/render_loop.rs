
use std::sync::{Arc, Mutex};
use hudhook::ImguiRenderLoop;
use imgui::Condition;
use crate::memory::Ds1;

static mut DS1: Option<Arc<Mutex<Ds1>>> = None;

pub fn get_ds1_instance() -> Arc<Mutex<Ds1>>
{
    unsafe
    {
        if DS1.is_none()
        {
            DS1 = Some(Arc::new(Mutex::new(Ds1::new())));
        }
        return Arc::clone(DS1.as_mut().unwrap());
    };
}

pub struct RenderLoop
{
    no_stamina_consume: bool,
    no_update_ai: bool,
    x_stored_pos: f32,
    y_stored_pos: f32,
    z_stored_pos: f32,
}

impl RenderLoop
{
    pub fn new() -> Self
    {
        RenderLoop
        {
            no_stamina_consume: false,
            no_update_ai: false,
            x_stored_pos : 0.,
            y_stored_pos: 0.,
            z_stored_pos: 0.,
        }
    }
}

impl ImguiRenderLoop for RenderLoop
{
    fn render(&mut self, ui: &mut imgui::Ui)
    {
        let instance = get_ds1_instance();
        let mut ds1 = instance.lock().unwrap();

        ui.window("Hello hudhook")
            .size([368.0, 568.0], Condition::FirstUseEver)
            .position([16.0, 16.0], Condition::FirstUseEver)
            .build(|| {
                ui.text(format!("HP {:?}", ds1.get_hp()));
                ui.text(format!("Stamina {:?}", ds1.get_stamina()));
                ui.text(format!("Pos X {:?}", ds1.get_x_pos()));
                ui.text(format!("Pos Y {:?}", ds1.get_y_pos()));
                ui.text(format!("Pos Z {:?}", ds1.get_z_pos()));

                ui.text(format!("Stored Pos X {:?}", self.x_stored_pos));
                ui.text(format!("Stored Pos Y {:?}", self.y_stored_pos));
                ui.text(format!("Stored Pos Z {:?}", self.z_stored_pos));

                if ui.button("Eject") {
                    print!("test");
                    hudhook::eject();
                }

                if ui.button("Store player position")
                {
                    self.x_stored_pos = ds1.get_x_pos();
                    self.y_stored_pos = ds1.get_y_pos();
                    self.z_stored_pos = ds1.get_z_pos();
                }

                if ui.button("Teleport player") {
                    ds1.teleport_player(self.x_stored_pos, self.y_stored_pos, self.z_stored_pos);
                }

                if ui.checkbox("inf stam", &mut self.no_stamina_consume)
                {
                    ds1.set_no_stam_consume();
                }

                if ui.checkbox("no update ai", &mut self.no_update_ai)
                {
                    ds1.set_no_update_ai();
                }


            });
    }
}