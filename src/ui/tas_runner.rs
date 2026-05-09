use std::path::PathBuf;

pub struct TasRunner {
    pub scripts: Vec<PathBuf>,
    pub selected_script: usize,
    tas_folder: String,
    soulstas_exe: String,
    running_process: Option<std::process::Child>,
}

impl TasRunner {
    pub fn new() -> Self {
        let mut runner = TasRunner {
            scripts: Vec::new(),
            selected_script: 0,
            tas_folder: "tas_scripts".to_string(),
            soulstas_exe: "soulstas_x86.exe".to_string(),
            running_process: None,
        };
        runner.refresh_scripts();
        runner
    }

    pub fn refresh_scripts(&mut self) {
        self.scripts.clear();
        let folder = std::path::Path::new(&self.tas_folder);
        let _ = std::fs::create_dir_all(folder);
        if let Ok(entries) = std::fs::read_dir(folder) {
            let mut paths: Vec<PathBuf> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("txt"))
                .collect();
            paths.sort();
            self.scripts = paths;
        }
        if !self.scripts.is_empty() && self.selected_script >= self.scripts.len() {
            self.selected_script = self.scripts.len() - 1;
        }
    }

    /// Returns true if the spawned SoulsTAS process is still running.
    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut child) = self.running_process {
            match child.try_wait() {
                Ok(None) => return true,
                _ => {}
            }
        } else {
            return false;
        }
        self.running_process = None;
        false
    }

    pub fn run_selected(&mut self) {
        if self.scripts.is_empty() {
            return;
        }
        self.stop();

        let script_path = &self.scripts[self.selected_script];
        let script_str = script_path.to_string_lossy().into_owned();

        match std::process::Command::new(&self.soulstas_exe)
            .args(["ds1", &script_str])
            .spawn()
        {
            Ok(child) => self.running_process = Some(child),
            Err(e) => eprintln!("TasRunner: failed to spawn process: {}", e),
        }
    }

    pub fn stop(&mut self) {
        if let Some(ref mut child) = self.running_process {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.running_process = None;
    }

    pub fn render_section(&mut self, ui: &imgui::Ui) {
        if ui.collapsing_header("TAS Runner", imgui::TreeNodeFlags::empty()) {
            let is_running = self.is_running();

            if ui.button("Refresh##tas") {
                self.refresh_scripts();
            }
            ui.same_line();
            if is_running {
                ui.text_colored([0.2, 0.9, 0.2, 1.0], "Running");
            } else {
                ui.text_colored([0.6, 0.6, 0.6, 1.0], "Idle");
            }

            if self.scripts.is_empty() {
                ui.text("No .txt scripts found in tas_scripts/");
            } else {
                let preview = self.scripts[self.selected_script]
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?")
                    .to_string();
                ui.set_next_item_width(400.0);
                if let Some(_combo) = ui.begin_combo("##tas_combo", &preview) {
                    for i in 0..self.scripts.len() {
                        let name = self.scripts[i]
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("?")
                            .to_string();
                        let is_selected = self.selected_script == i;
                        if ui.selectable_config(&name).selected(is_selected).build() {
                            self.selected_script = i;
                        }
                        if is_selected {
                            ui.set_item_default_focus();
                        }
                    }
                }
            }

            {
                let _d = ui.begin_disabled(self.scripts.is_empty() || is_running);
                if ui.button("Run##tas") {
                    self.run_selected();
                }
            }
            ui.same_line();
            {
                let _d = ui.begin_disabled(!is_running);
                if ui.button("Stop##tas") {
                    self.stop();
                }
            }
        }
    }
}
